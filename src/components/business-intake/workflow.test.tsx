import { type ReactNode } from "react"
import { act, fireEvent, render, screen, waitFor } from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest"
import { BusinessError, type BusinessClient } from "@/lib/business/client"
import type { CandidateDetail, IntakeImport } from "@/lib/business/intake"
import { BusinessWorkspace } from "@/components/business/workspace"
import {
  context,
  detail as taskDetail,
  member,
} from "@/components/business/test-fixtures"
import { CandidateReview } from "./candidate"
import { SourceSetupDialog } from "./setup"
import { ImportPanel } from "./imports"
import { TaskTarget } from "./task-target"
import { TaskSources } from "./task-sources"
import { SourceReview } from "./source-review"
import { admin, binding, candidate, decision, source } from "./test-fixtures"

vi.mock("@/components/i18n-provider", () => ({
  useAppI18n: () => ({ appLocale: "en", setLanguageSettings: vi.fn() }),
}))
vi.mock("next-themes", () => ({
  useTheme: () => ({ theme: "light", setTheme: vi.fn() }),
}))
const intake = vi.fn()
const tasks = vi.fn()
const identity = vi.fn()
const client: BusinessClient = {
  native: false,
  label: "http://127.0.0.1:4350",
  close: vi.fn(),
  identity,
  tasks,
  intake,
}
const onDirty = vi.fn()
const onChanged = vi.fn()
const onTask = vi.fn()
function wrapper(children: ReactNode, locale: "en" | "ar" = "en") {
  return (
    <NextIntlClientProvider locale={locale} messages={{}} timeZone="UTC">
      {children}
    </NextIntlClientProvider>
  )
}
function review(
  initial: CandidateDetail,
  reading = {
    source: initial.source,
    passages: initial.passages,
    disclosure: initial.candidate.disclosure,
    candidateCount: 1,
  },
  locale: "en" | "ar" = "en",
  active = true
) {
  return wrapper(
    <CandidateReview
      initial={initial}
      source={reading}
      client={client}
      actor={member}
      members={[member]}
      active={active}
      refresh={0}
      onDirty={onDirty}
      onChanged={onChanged}
      onTask={onTask}
    />,
    locale
  )
}
beforeEach(() => {
  vi.resetAllMocks()
  localStorage.clear()
  identity.mockImplementation(async (operation) =>
    operation === "context" ? context : [member]
  )
  tasks.mockImplementation(async (operation) =>
    operation === "list"
      ? { tasks: [taskDetail().task], page: 0, hasMore: false, canCreate: true }
      : taskDetail()
  )
})
afterEach(() => vi.useRealTimers())

describe("business Sources privacy and exact human decisions", () => {
  it("removes passages and an already-confirmed review at the source expiry deadline", async () => {
    vi.useFakeTimers()
    vi.setSystemTime(new Date("2026-09-08T18:00:00Z"))
    const reading = source()
    reading.source.accessValidUntil = new Date(Date.now() + 1000).toISOString()
    const value = candidate(reading)
    intake.mockImplementation(async (operation) =>
      operation === "sources/get" ? reading : value
    )
    render(review(value, reading))
    await act(async () => {
      await Promise.resolve()
    })
    fireEvent.click(
      screen.getByRole("button", { name: "Review before sharing" })
    )
    fireEvent.click(
      screen.getByRole("checkbox", {
        name: /I have reviewed these exact passages/,
      })
    )
    expect(
      screen.getByRole("button", { name: "Accept into shared work" })
    ).toBeEnabled()
    await act(async () => {
      await vi.advanceTimersByTimeAsync(1001)
    })
    expect(screen.queryByText(reading.passages[0].text)).not.toBeInTheDocument()
    expect(
      screen.queryByText("Private prepared task text")
    ).not.toBeInTheDocument()
    expect(
      screen.queryByRole("button", { name: "Accept into shared work" })
    ).not.toBeInTheDocument()
    expect(
      intake.mock.calls.some(([operation]) => operation === "candidates/accept")
    ).toBe(false)
  })
  it("keeps stale discard usable through the actual source-reader parent", async () => {
    const reading = source()
    reading.source.access = "expired"
    reading.source.requiresRefresh = true
    reading.disclosure = "metadata_only"
    reading.passages = []
    const value = candidate(reading)
    value.candidate.disclosure = "metadata_only"
    value.candidate.draft = null
    value.candidate.capabilities = {
      select: false,
      edit: false,
      accept: false,
      link: false,
      discard: true,
      publicationDomains: [],
    }
    intake.mockImplementation(async (operation) => {
      if (operation === "sources/get") return reading
      if (operation === "candidates/list")
        return { items: [value.candidate], page: 0, hasMore: false }
      if (operation === "candidates/get") return value
      if (operation === "candidates/discard") return decision("discarded")
      if (operation === "imports/list")
        return { items: [], page: 0, hasMore: false }
      throw new Error("Unexpected synthetic operation")
    })
    render(
      wrapper(
        <SourceReview
          client={client}
          binding={binding.binding}
          sourceId={reading.source.id}
          actor={member}
          members={[member]}
          active
          onBack={vi.fn()}
          onDirty={onDirty}
          onTask={onTask}
        />
      )
    )
    fireEvent.click(await screen.findByRole("button", { name: /Pending 1/ }))
    fireEvent.click(
      await screen.findByRole("button", { name: "Discard candidate" })
    )
    fireEvent.click(
      screen.getByRole("checkbox", {
        name: "Discard this candidate without publishing a task.",
      })
    )
    fireEvent.click(screen.getByRole("button", { name: "Discard candidate" }))
    await screen.findByText("Decision recorded")
    expect(intake).toHaveBeenCalledWith(
      "candidates/discard",
      expect.objectContaining({ expectedRevision: 3 })
    )
  })
  it("does not preload source APIs on My work or infer operator setup from an owner role", async () => {
    intake.mockResolvedValue({
      canManageSetup: false,
      items: [],
      page: 0,
      hasMore: false,
    })
    render(
      wrapper(
        <BusinessWorkspace
          client={client}
          context={context}
          onContext={vi.fn()}
          disconnect={vi.fn()}
        />
      )
    )
    await screen.findByText("Synthetic launch brief")
    expect(intake).not.toHaveBeenCalled()
    fireEvent.click(screen.getByRole("button", { name: "Sources" }))
    await screen.findByText("No sources available yet.")
    expect(intake).toHaveBeenCalledWith("bindings/list", { page: 0 })
    expect(
      screen.queryByRole("button", { name: "Set up a source" })
    ).not.toBeInTheDocument()
  })
  it("saves exact task/date/null fields privately and accepts only the reviewed server draft and destination", async () => {
    const reading = source()
    let saved = candidate(reading)
    intake.mockImplementation(async (operation, input) => {
      if (operation === "sources/get") return reading
      if (operation === "candidates/get") return saved
      if (operation === "candidates/edit") {
        saved = {
          ...saved,
          candidate: { ...saved.candidate, revision: 4, draft: input.task },
        }
        return saved
      }
      if (operation === "candidates/accept")
        return { decision: decision(), task: taskDetail(), replayed: false }
      throw new Error("Unexpected synthetic operation")
    })
    render(review(saved, reading))
    const brief = await screen.findByRole("textbox", { name: "Brief" })
    fireEvent.change(brief, {
      target: { value: "Exact human-edited brief.\nSecond line." },
    })
    fireEvent.click(screen.getByRole("button", { name: "Save private draft" }))
    const accept = await screen.findByRole("button", {
      name: "Accept into shared work",
    })
    expect(accept).toBeDisabled()
    expect(screen.getByText("2028-02-29")).toBeInTheDocument()
    expect(
      screen.getByText(/current or future Read access/)
    ).toBeInTheDocument()
    expect(screen.getByText(/Accepted task text remains/)).toBeInTheDocument()
    fireEvent.click(
      screen.getByRole("checkbox", {
        name: /I have reviewed these exact passages/,
      })
    )
    fireEvent.click(accept)
    await screen.findByRole("button", { name: "Open shared task" })
    expect(intake).toHaveBeenCalledWith(
      "candidates/edit",
      expect.objectContaining({
        task: expect.objectContaining({
          notes: "Exact human-edited brief.\nSecond line.",
          dueDate: "2028-02-29",
          assigneeId: null,
          reviewerId: null,
        }),
      })
    )
    const input = intake.mock.calls.find(
      ([operation]) => operation === "candidates/accept"
    )![1]
    expect(Object.keys(input).sort()).toEqual([
      "candidateId",
      "expectedRevision",
      "expectedSourceRevision",
      "operationId",
      "publishToDomain",
    ])
    expect(input).toMatchObject({
      expectedRevision: 4,
      expectedSourceRevision: 2,
      publishToDomain: "marketing",
    })
    expect(tasks).not.toHaveBeenCalled()
  })
  it("keeps local drafts across locale and view changes without browser persistence", async () => {
    const reading = source()
    const value = candidate(reading)
    intake.mockImplementation(async (operation) =>
      operation === "sources/get" ? reading : value
    )
    const rendered = render(review(value, reading))
    fireEvent.change(await screen.findByRole("textbox", { name: "Brief" }), {
      target: { value: "Local private Arabic-ready draft" },
    })
    rendered.rerender(review(value, reading, "ar"))
    expect(screen.getByRole("textbox", { name: "وصف العمل" })).toHaveValue(
      "Local private Arabic-ready draft"
    )
    rendered.rerender(review(value, reading, "ar", false))
    expect(
      screen.queryByDisplayValue("Local private Arabic-ready draft")
    ).not.toBeInTheDocument()
    rendered.rerender(review(value, reading, "en", true))
    await waitFor(() =>
      expect(screen.getByRole("textbox", { name: "Brief" })).toHaveValue(
        "Local private Arabic-ready draft"
      )
    )
    expect(JSON.stringify(localStorage)).not.toContain("Local private")
    expect(JSON.stringify(sessionStorage)).not.toContain("Local private")
  })
  it("recovers a lost acceptance through its durable decision without filing or creating again", async () => {
    const reading = source()
    const value = candidate(reading)
    let accepted = false
    intake.mockImplementation(async (operation) => {
      if (operation === "sources/get") return reading
      if (operation === "candidates/get")
        return accepted ? { ...value, decision: decision() } : value
      if (operation === "candidates/accept") {
        accepted = true
        throw new BusinessError("offline")
      }
      throw new Error("Unexpected synthetic operation")
    })
    render(review(value, reading))
    fireEvent.click(
      await screen.findByRole("button", { name: "Review before sharing" })
    )
    fireEvent.click(
      screen.getByRole("checkbox", {
        name: /I have reviewed these exact passages/,
      })
    )
    fireEvent.click(
      screen.getByRole("button", { name: "Accept into shared work" })
    )
    await screen.findByText(/The decision could not be confirmed/)
    fireEvent.click(screen.getByRole("button", { name: "Check saved outcome" }))
    await screen.findByRole("button", { name: "Open shared task" })
    expect(
      intake.mock.calls.filter(
        ([operation]) => operation === "candidates/accept"
      )
    ).toHaveLength(1)
    expect(tasks).not.toHaveBeenCalled()
  })
  it("withholds a saved destination-private draft despite fresh source read permission", async () => {
    const reading = source()
    const value = candidate(reading)
    value.candidate.draft = null
    value.candidate.capabilities.edit = false
    value.candidate.capabilities.accept = false
    intake.mockImplementation(async (operation) =>
      operation === "sources/get" ? reading : value
    )
    render(review(value, reading))
    await screen.findByText(/A prepared draft exists/)
    expect(screen.getByText(reading.passages[0].text)).toBeInTheDocument()
    expect(
      screen.queryByRole("textbox", { name: "Brief" })
    ).not.toBeInTheDocument()
    expect(
      screen.queryByText("Private prepared task text")
    ).not.toBeInTheDocument()
    expect(
      screen.queryByRole("button", { name: "Accept into shared work" })
    ).not.toBeInTheDocument()
  })
  it("hides cached private text after source access is denied, including unsaved local fields", async () => {
    const reading = source()
    const value = candidate(reading)
    intake.mockImplementation(async (operation) =>
      operation === "sources/get" ? reading : value
    )
    render(review(value, reading))
    fireEvent.change(await screen.findByRole("textbox", { name: "Brief" }), {
      target: { value: "Private local edit before revoke" },
    })
    intake.mockRejectedValue(new BusinessError("forbidden", "source_denied"))
    await act(async () => {
      window.dispatchEvent(new Event("focus"))
    })
    await screen.findByText(/no longer readable with current access/)
    expect(
      screen.queryByDisplayValue("Private local edit before revoke")
    ).not.toBeInTheDocument()
    expect(screen.queryByText(reading.passages[0].text)).not.toBeInTheDocument()
  })
  it("labels current candidate revisions on conflict, preserves edits, and requires explicit adoption", async () => {
    const reading = source()
    const value = candidate(reading)
    let conflict = false
    const changed = {
      ...value,
      candidate: {
        ...value.candidate,
        revision: 8,
        draft: {
          ...value.candidate.draft!,
          notes: "Other person's saved brief",
        },
      },
    }
    intake.mockImplementation(async (operation) => {
      if (operation === "sources/get") return reading
      if (operation === "candidates/get") return conflict ? changed : value
      if (operation === "candidates/edit") {
        conflict = true
        throw new BusinessError("conflict")
      }
      throw new Error("Unexpected synthetic operation")
    })
    render(review(value, reading))
    fireEvent.change(await screen.findByRole("textbox", { name: "Brief" }), {
      target: { value: "My retained brief" },
    })
    fireEvent.click(screen.getByRole("button", { name: "Save private draft" }))
    await screen.findByText(/This candidate or source changed/)
    fireEvent.click(
      screen.getByRole("button", { name: "Load current source and candidate" })
    )
    const comparison = await screen.findByRole("region", {
      name: "Current saved candidate",
    })
    expect(comparison).toHaveTextContent("Revision 8")
    expect(comparison).toHaveTextContent("Other person's saved brief")
    expect(
      screen.getByRole("button", { name: "Save private draft" })
    ).toBeDisabled()
    fireEvent.click(
      screen.getByRole("button", {
        name: "Use current version and keep my draft",
      })
    )
    expect(screen.getByRole("textbox", { name: "Brief" })).toHaveValue(
      "My retained brief"
    )
    expect(
      screen.getByRole("button", { name: "Save private draft" })
    ).toBeEnabled()
  })
  it("supports passage-only rebase and an exact text-free target link without preparing or copying a task", async () => {
    const reading = source()
    let value = candidate(reading)
    value.candidate.draft = null
    value.candidate.hasPreparedDraft = false
    value.candidate.requiresRebase = true
    value.candidate.capabilities.accept = false
    value.candidate.capabilities.link = false
    intake.mockImplementation(async (operation) => {
      if (operation === "sources/get") return reading
      if (operation === "candidates/get") return value
      if (operation === "candidates/select") {
        value = {
          ...value,
          candidate: {
            ...value.candidate,
            revision: 4,
            requiresRebase: false,
            capabilities: { ...value.candidate.capabilities, link: true },
          },
        }
        return value
      }
      if (operation === "candidates/link")
        return {
          decision: decision("linked"),
          task: taskDetail(),
          replayed: false,
        }
      throw new Error("Unexpected synthetic operation")
    })
    render(review(value, reading))
    fireEvent.click(
      await screen.findByRole("button", {
        name: "Confirm this passage selection",
      })
    )
    fireEvent.click(
      await screen.findByRole("button", { name: "Link an existing task" })
    )
    fireEvent.click(screen.getByRole("button", { name: "Find a task" }))
    fireEvent.click(
      await screen.findByRole("button", { name: "Review this task" })
    )
    const target = await screen.findByRole("region", {
      name: "Exact target task",
    })
    expect(target).toHaveTextContent("Revision 1")
    expect(target).toHaveTextContent("To do")
    fireEvent.click(
      screen.getByRole("checkbox", {
        name: /I reviewed this exact task and destination/,
      })
    )
    fireEvent.click(
      screen.getByRole("button", { name: "Confirm text-free link" })
    )
    await screen.findByText("Linked to existing task")
    expect(
      intake.mock.calls.some(
        ([operation]) =>
          operation === "candidates/edit" || operation === "candidates/accept"
      )
    ).toBe(false)
    expect(tasks.mock.calls.some(([operation]) => operation === "create")).toBe(
      false
    )
    const input = intake.mock.calls.find(
      ([operation]) => operation === "candidates/link"
    )![1]
    expect(input).toMatchObject({
      expectedRevision: 4,
      expectedSourceRevision: 2,
      taskId: taskDetail().task.id,
      expectedTaskRevision: 1,
      publishToDomain: "marketing",
    })
    expect(input).not.toHaveProperty("task")
    expect(input).not.toHaveProperty("passageIds")
  })
  it("allows read-authorized stale candidate discard without requiring task publication or freshness", async () => {
    const reading = source()
    const value = candidate(reading)
    reading.disclosure = "metadata_only"
    reading.source.access = "expired"
    reading.source.requiresRefresh = true
    reading.passages = []
    value.candidate.disclosure = "metadata_only"
    value.candidate.draft = null
    value.passages = []
    value.candidate.capabilities = {
      select: false,
      edit: false,
      accept: false,
      link: false,
      discard: true,
      publicationDomains: [],
    }
    intake.mockImplementation(async (operation) =>
      operation === "sources/get"
        ? reading
        : operation === "candidates/discard"
          ? decision("discarded")
          : value
    )
    render(review(value, reading))
    fireEvent.click(
      await screen.findByRole("button", { name: "Discard candidate" })
    )
    fireEvent.click(
      screen.getByRole("checkbox", {
        name: "Discard this candidate without publishing a task.",
      })
    )
    fireEvent.click(screen.getByRole("button", { name: "Discard candidate" }))
    await screen.findByText("Decision recorded")
    expect(intake).toHaveBeenCalledWith(
      "candidates/discard",
      expect.objectContaining({
        candidateId: value.candidate.id,
        expectedRevision: 3,
      })
    )
    expect(tasks).not.toHaveBeenCalled()
  })
  it("does not install a late target snapshot after source access disables linking", async () => {
    const selected = vi.fn()
    let finish!: (value: ReturnType<typeof taskDetail>) => void
    tasks.mockImplementation((operation) =>
      operation === "list"
        ? Promise.resolve({
            tasks: [taskDetail().task],
            page: 0,
            hasMore: false,
            canCreate: true,
          })
        : new Promise((resolve) => {
            finish = resolve
          })
    )
    const element = (disabled: boolean) =>
      wrapper(
        <TaskTarget
          client={client}
          domains={["marketing"]}
          disabled={disabled}
          onSelected={selected}
        />
      )
    const result = render(element(false))
    fireEvent.click(screen.getByRole("button", { name: "Find a task" }))
    fireEvent.click(
      await screen.findByRole("button", { name: "Review this task" })
    )
    result.rerender(element(true))
    await act(async () => {
      finish(taskDetail())
    })
    expect(selected).not.toHaveBeenCalledWith(
      expect.objectContaining({ task: expect.anything() })
    )
    expect(screen.queryByText("Synthetic launch brief")).not.toBeInTheDocument()
  })
  it("loads task references only on demand and keeps inaccessible links text-free", async () => {
    const read = source()
    intake.mockResolvedValue({
      links: [
        { linkId: "one", accessible: false, source: null },
        { linkId: "two", accessible: true, source: read.source },
      ],
    })
    const open = vi.fn()
    render(
      wrapper(<TaskSources taskId="task" client={client} onSource={open} />)
    )
    expect(intake).not.toHaveBeenCalled()
    fireEvent.click(screen.getByText("Source references"))
    await screen.findByText("Private source reference")
    fireEvent.click(
      screen.getByRole("button", { name: /Synthetic customer conversation/ })
    )
    expect(open).toHaveBeenCalledWith(read.source)
  })
})

describe("protected setup and bounded import recovery", () => {
  it("creates disabled/zero-grant setup only after explicit retained-task consent and keeps key write-only", async () => {
    intake.mockImplementation(async (operation) =>
      operation === "bindings/create"
        ? admin
        : { items: [], page: 0, hasMore: false }
    )
    render(
      wrapper(
        <SourceSetupDialog
          client={client}
          members={[member]}
          onClose={vi.fn()}
          onChanged={vi.fn()}
        />
      )
    )
    fireEvent.change(screen.getByRole("textbox", { name: "Connection name" }), {
      target: { value: "Synthetic meetings" },
    })
    fireEvent.change(
      screen.getByRole("combobox", { name: "Source account owner" }),
      { target: { value: member.id } }
    )
    fireEvent.change(screen.getByLabelText("Fireflies API key"), {
      target: { value: "SYNTHETIC_WRITE_ONLY_KEY" },
    })
    fireEvent.click(screen.getByRole("checkbox", { name: "Marketing" }))
    expect(
      screen.getByRole("button", { name: "Create disabled connection" })
    ).toBeDisabled()
    fireEvent.click(
      screen.getByRole("checkbox", {
        name: /I allow explicitly accepted task text/,
      })
    )
    fireEvent.click(
      screen.getByRole("button", { name: "Create disabled connection" })
    )
    await screen.findByText("Nobody has an explicit grant yet.")
    expect(
      screen.getByLabelText("Replace stored Fireflies key (optional)")
    ).toHaveValue("")
    expect(JSON.stringify(localStorage)).not.toContain(
      "SYNTHETIC_WRITE_ONLY_KEY"
    )
    const input = intake.mock.calls.find(
      ([operation]) => operation === "bindings/create"
    )![1]
    expect(input).toMatchObject({
      sourceOwnerId: member.id,
      retainedTaskText: true,
      publicationDomains: ["marketing"],
    })
    expect(input).not.toHaveProperty("enabled")
    expect(
      intake.mock.calls.some(
        ([operation]) =>
          operation.startsWith("grants/") && operation !== "grants/list"
      )
    ).toBe(false)
  })
  it("requires explicit historical/current/future grant confirmation and invalidates it when flags change", async () => {
    intake.mockImplementation(async (operation) =>
      operation === "grants/upsert"
        ? { binding: { ...admin, revision: 2 }, grant: {} }
        : { items: [], page: 0, hasMore: false }
    )
    render(
      wrapper(
        <SourceSetupDialog
          client={client}
          initial={admin}
          members={[member]}
          onClose={vi.fn()}
          onChanged={vi.fn()}
        />
      )
    )
    await screen.findByText("Nobody has an explicit grant yet.")
    fireEvent.change(
      screen.getByRole("combobox", { name: "Person receiving access" }),
      { target: { value: member.id } }
    )
    fireEvent.click(
      screen.getByRole("checkbox", {
        name: "Read retained and newly imported source records",
      })
    )
    const consent = screen.getByRole("checkbox", {
      name: /all retained historical versions, including those imported before this grant/,
    })
    fireEvent.click(consent)
    expect(
      screen.getByRole("button", { name: "Save explicit access" })
    ).toBeEnabled()
    fireEvent.click(
      screen.getByRole("checkbox", {
        name: "Import and refresh source records",
      })
    )
    expect(consent).not.toBeChecked()
    expect(
      screen.getByRole("button", { name: "Save explicit access" })
    ).toBeDisabled()
    fireEvent.click(consent)
    fireEvent.click(
      screen.getByRole("button", { name: "Save explicit access" })
    )
    await waitFor(() =>
      expect(intake).toHaveBeenCalledWith(
        "grants/upsert",
        expect.objectContaining({
          expectedBindingRevision: 1,
          expectedGrantRevision: null,
          memberId: member.id,
          scope: "binding_current_and_future_sources",
          read: true,
          import: true,
          triage: false,
          publicationDomains: [],
          expiresAt: null,
        })
      )
    )
  })
  it("retries an ambiguous import start with the same operation and UTC boundaries, without a second selection", async () => {
    const result: IntakeImport = {
      id: "import",
      bindingId: binding.binding.id,
      revision: 1,
      state: "queued",
      coverage: "not_started",
      discovered: 0,
      completed: 0,
      failed: 0,
      nextAttemptAt: null,
      errorCode: null,
      capabilities: { advance: true, cancel: true },
    }
    let starts = 0
    intake.mockImplementation(async (operation) => {
      if (operation === "imports/list")
        return { items: [], page: 0, hasMore: false }
      if (operation === "imports/start") {
        if (++starts === 1) throw new BusinessError("offline")
        return result
      }
      throw new Error("Unexpected synthetic operation")
    })
    render(
      wrapper(
        <ImportPanel
          client={client}
          binding={binding.binding}
          active
          onRead={vi.fn()}
        />
      )
    )
    fireEvent.change(screen.getByLabelText("From (UTC)"), {
      target: { value: "2026-09-07T10:00" },
    })
    fireEvent.change(screen.getByLabelText("Until (UTC)"), {
      target: { value: "2026-09-08T10:00" },
    })
    fireEvent.click(
      screen.getByRole("button", { name: "Start bounded import" })
    )
    const retry = await screen.findByRole("button", {
      name: "Retry this exact operation",
    })
    expect(
      screen.getByRole("button", { name: "Start bounded import" })
    ).toBeDisabled()
    fireEvent.click(retry)
    await screen.findByText("Queued")
    const calls = intake.mock.calls.filter(
      ([operation]) => operation === "imports/start"
    )
    expect(calls).toHaveLength(2)
    expect(calls[0][1]).toEqual(calls[1][1])
    expect(calls[0][1].selection).toEqual({
      kind: "window",
      fromDate: "2026-09-07T10:00:00.000Z",
      toDate: "2026-09-08T10:00:00.000Z",
    })
    expect(
      intake.mock.calls.some(([operation]) => operation === "imports/advance")
    ).toBe(false)
  })
})
