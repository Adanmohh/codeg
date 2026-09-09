import {
  act,
  cleanup,
  fireEvent,
  render,
  screen,
  waitFor,
} from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest"
import {
  createBusinessClient,
  type BusinessClient,
} from "@/lib/business/client"
import type {
  ExecutionInputs,
  ProfileSummary,
  SessionSummary,
} from "@/lib/business-execution/types"
import { BusinessWorkspace } from "@/components/business/workspace"
import { context, detail, member } from "@/components/business/test-fixtures"
import { BusinessSessionPane, TaskAiWorkspace } from "./session-workspace"
import { ExecutionPaneGuard } from "./pane-guard"
import type { PromptGuard } from "./session-prompt"

vi.mock("@tauri-apps/api/core", () => ({
  isTauri: () => false,
  invoke: vi.fn(),
}))
vi.mock("@/hooks/use-media-query", () => ({ useMediaQuery: () => true }))
vi.mock("@/components/i18n-provider", () => ({
  useAppI18n: () => ({ appLocale: "en", setLanguageSettings: vi.fn() }),
}))
vi.mock("next-themes", () => ({
  useTheme: () => ({ theme: "light", setTheme: vi.fn() }),
}))
const clients: BusinessClient[] = []
const fetcher = vi.fn<typeof fetch>()
const start =
  vi.fn<(input: ExecutionInputs["sessions/start"]) => Promise<Response>>()
const resume =
  vi.fn<(input: ExecutionInputs["sessions/continue"]) => Promise<Response>>()
const lookup =
  vi.fn<(input: ExecutionInputs["operations/get"]) => Promise<Response>>()
let profile: ProfileSummary
let session: SessionSummary
let saved: SessionSummary[]
let task = detail()

function client() {
  const value = createBusinessClient(
    {
      kind: "http",
      address: "http://127.0.0.1:4359",
      token: "synthetic-original-operator",
    },
    vi.fn()
  )
  clients.push(value)
  return value
}
function rootContext() {
  return {
    ...context,
    operator: true,
    capabilities: { ...context.capabilities, legacyOperator: true },
  }
}
function wrap(children: React.ReactNode) {
  return (
    <NextIntlClientProvider locale="en" messages={{}} timeZone="UTC">
      {children}
    </NextIntlClientProvider>
  )
}
function choices(
  business: BusinessClient,
  onOpen = vi.fn(),
  originalOperator = true
) {
  return wrap(
    <TaskAiWorkspace
      client={business}
      taskId={task.task.id}
      originalOperator={originalOperator}
      onGuard={vi.fn()}
      onOpen={onOpen}
    />
  )
}
function workspace(business: BusinessClient, operator = true) {
  return wrap(
    <BusinessWorkspace
      client={business}
      context={operator ? rootContext() : context}
      onContext={vi.fn()}
      disconnect={vi.fn()}
    />
  )
}
beforeEach(() => {
  vi.clearAllMocks()
  vi.stubGlobal("fetch", fetcher)
  task = detail()
  profile = {
    id: "synthetic-profile",
    revision: 7,
    label: "Synthetic writing profile",
    clientId: "synthetic-client",
    modes: ["chat"],
    custody: "original_operator",
    model: { id: "synthetic-model", reasoning: "max" },
    readiness: "ready",
    reason: null,
    capabilities: {
      start: true,
      continue: true,
      managedOutput: true,
      officePreview: false,
    },
  }
  session = {
    id: "synthetic-session",
    taskId: task.task.id,
    profileId: profile.id,
    profileRevision: 7,
    revision: 4,
    generation: 1,
    status: "idle",
    mode: "chat",
    title: "Synthetic persistent conversation",
    createdAt: "2026-09-09T00:00:00Z",
    updatedAt: "2026-09-09T00:00:00Z",
    lastActivityAt: "2026-09-09T00:00:00Z",
    capabilities: {
      read: true,
      prompt: true,
      continue: false,
      stop: true,
      terminalWrite: false,
      importOutput: true,
    },
    reason: null,
  }
  saved = []
  start.mockImplementation(async (input) =>
    Response.json({
      session,
      operation: { id: input.operationId, status: "confirmed", reason: null },
    })
  )
  resume.mockImplementation(async (input) =>
    Response.json({
      session,
      operation: { id: input.operationId, status: "confirmed", reason: null },
    })
  )
  lookup.mockImplementation(async (input) =>
    Response.json({
      operation: { id: input.operationId, status: "confirmed", reason: null },
      resourceId: session.id,
    })
  )
  fetcher.mockImplementation(async (url, options) => {
    const path = new URL(String(url)).pathname
    const { input } = JSON.parse(String(options?.body))
    if (path.endsWith("/tasks/get")) return Response.json(task)
    if (path.endsWith("/tasks/list"))
      return Response.json({
        tasks: [task.task],
        page: 0,
        hasMore: false,
        canCreate: true,
      })
    if (path.endsWith("/members/list")) return Response.json([member])
    if (path.endsWith("/settings/get"))
      return Response.json({
        organizationId: member.organizationId,
        revision: 1,
        settings: {
          displayName: context.organization!.name,
          palette: "neutral",
          workspaceLayout: "split",
          defaultWorkArea: "tasks",
        },
      })
    if (path.endsWith("/context")) return Response.json(rootContext())
    if (path.endsWith("/profiles/list"))
      return Response.json({ profiles: [profile], unavailableReason: null })
    if (path.endsWith("/sessions/list"))
      return Response.json({ items: saved, nextCursor: null })
    if (path.endsWith("/sessions/get")) return Response.json({ session })
    if (path.endsWith("/sessions/start")) return start(input)
    if (path.endsWith("/sessions/continue")) return resume(input)
    if (path.endsWith("/operations/get")) return lookup(input)
    if (path.endsWith("/sessions/events"))
      return new Response(
        new ReadableStream({
          start(controller) {
            controller.enqueue(
              new TextEncoder().encode(
                JSON.stringify({
                  type: "snapshot",
                  sessionId: session.id,
                  generation: session.generation,
                  operation: {
                    id: input.operationId,
                    status: "confirmed",
                    reason: null,
                  },
                  cursor: "opaque-position",
                  reset: true,
                  reason: "initial",
                  state: { status: "idle", messages: [], tools: [] },
                  olderCursor: null,
                }) + "\n"
              )
            )
          },
        }),
        {
          headers: {
            "content-type": "application/x-ndjson; charset=utf-8",
            "cache-control": "no-store",
            "x-content-type-options": "nosniff",
          },
        }
      )
    throw new Error("Unexpected synthetic route")
  })
})
afterEach(() => {
  cleanup()
  clients.splice(0).forEach((business) => business.close())
  vi.unstubAllGlobals()
})

describe("actual original-operator session entry", () => {
  it.each(["member", "native"])(
    "does not load private task/profile/session data for a %s boundary",
    (boundary) => {
      const business = client()
      const factory = vi.spyOn(business, "execution")
      const input =
        boundary === "native" ? { ...business, native: true } : business
      render(choices(input, vi.fn(), boundary !== "member"))
      expect(screen.getByRole("alert")).toBeVisible()
      expect(fetcher).not.toHaveBeenCalled()
      expect(factory).not.toHaveBeenCalled()
    }
  )

  it("requires a deliberate ready-profile choice and sends one exact start input", async () => {
    const onOpen = vi.fn()
    render(choices(client(), onOpen))
    const select = await screen.findByLabelText("Choose an AI profile")
    expect(
      screen.getByRole("button", { name: "Start conversation" })
    ).toBeDisabled()
    expect(start).not.toHaveBeenCalled()
    fireEvent.change(select, { target: { value: profile.id } })
    const button = screen.getByRole("button", { name: "Start conversation" })
    fireEvent.click(button)
    fireEvent.click(button)
    await waitFor(() => expect(onOpen).toHaveBeenCalledOnce())
    expect(start).toHaveBeenCalledOnce()
    expect(start.mock.calls[0][0]).toEqual({
      operationId: expect.any(String),
      taskId: task.task.id,
      expectedTaskRevision: task.task.revision,
      profileId: profile.id,
      expectedProfileRevision: 7,
      mode: "chat",
    })
    expect(onOpen).toHaveBeenCalledWith({
      taskId: task.task.id,
      sessionId: session.id,
      title: session.title,
    })
  })

  it.each(["blocked", "member_custody", "no_contribution"])(
    "refuses a %s profile/task admission",
    async (reason) => {
      if (reason === "blocked") {
        profile.readiness = "blocked"
        profile.reason = "missing_configuration"
      }
      if (reason === "member_custody") profile.custody = "isolated_member"
      if (reason === "no_contribution") task.task.capabilities.submit = false
      render(choices(client()))
      fireEvent.change(await screen.findByLabelText("Choose an AI profile"), {
        target: { value: profile.id },
      })
      expect(
        screen.getByRole("button", { name: "Start conversation" })
      ).toBeDisabled()
      expect(start).not.toHaveBeenCalled()
    }
  )

  it("holds an ambiguous start through a missing receipt and close request, then reconciles without relaunch", async () => {
    start.mockRejectedValueOnce(new Error("Synthetic response lost"))
    let guard: PromptGuard = { dirty: false, busy: false, unresolved: false }
    let requestClose: (() => void) | null = null
    const close = vi.fn()
    const onOpen = vi.fn()
    const business = client()
    render(
      wrap(
        <ExecutionPaneGuard
          onClose={close}
          getGuard={() => guard}
          registerClose={(request) => {
            requestClose = request
          }}
        >
          <TaskAiWorkspace
            client={business}
            taskId={task.task.id}
            originalOperator
            onGuard={(value) => {
              guard = value
            }}
            onOpen={onOpen}
          />
        </ExecutionPaneGuard>
      )
    )
    fireEvent.change(await screen.findByLabelText("Choose an AI profile"), {
      target: { value: profile.id },
    })
    fireEvent.click(screen.getByRole("button", { name: "Start conversation" }))
    await screen.findByRole("button", { name: "Check receipt" })
    act(() => {
      requestClose?.()
    })
    expect(close).not.toHaveBeenCalled()
    expect(
      screen.getByText(
        "Resolve the pending request here before closing this pane. No request will be repeated automatically."
      )
    ).toBeVisible()
    lookup.mockResolvedValueOnce(new Response(null, { status: 404 }))
    fireEvent.click(screen.getByRole("button", { name: "Check receipt" }))
    await waitFor(() => expect(lookup).toHaveBeenCalledOnce())
    await screen.findByRole("button", { name: "Check receipt" })
    expect(
      screen.getByRole("button", { name: "Start conversation" })
    ).toBeDisabled()
    fireEvent.click(screen.getByRole("button", { name: "Check receipt" }))
    await waitFor(() => expect(onOpen).toHaveBeenCalledOnce())
    expect(start).toHaveBeenCalledOnce()
    expect(lookup.mock.calls).toEqual([
      [{ operationId: start.mock.calls[0][0].operationId, kind: "start" }],
      [{ operationId: start.mock.calls[0][0].operationId, kind: "start" }],
    ])
    act(() => {
      requestClose?.()
    })
    expect(close).toHaveBeenCalledOnce()
  })

  it("continues only an explicit existing session with its captured revision", async () => {
    session.capabilities.continue = true
    saved = [session]
    const onOpen = vi.fn()
    render(choices(client(), onOpen))
    fireEvent.click(
      await screen.findByRole("button", { name: "Continue session" })
    )
    await waitFor(() => expect(onOpen).toHaveBeenCalledOnce())
    expect(resume).toHaveBeenCalledOnce()
    expect(resume.mock.calls[0][0]).toEqual({
      operationId: expect.any(String),
      sessionId: session.id,
      expectedSessionRevision: 4,
    })
    expect(start).not.toHaveBeenCalled()
  })

  it("never opens a session from a foreign profile result even if its operation claims confirmation", async () => {
    const onOpen = vi.fn()
    start.mockImplementationOnce(async (input) =>
      Response.json({
        session: { ...session, profileId: "foreign-profile" },
        operation: { id: input.operationId, status: "confirmed", reason: null },
      })
    )
    render(choices(client(), onOpen))
    fireEvent.change(await screen.findByLabelText("Choose an AI profile"), {
      target: { value: profile.id },
    })
    fireEvent.click(screen.getByRole("button", { name: "Start conversation" }))
    await screen.findByRole("button", { name: "Check receipt" })
    expect(onOpen).not.toHaveBeenCalled()
    expect(start).toHaveBeenCalledOnce()
  })

  it("shows profile and conversation tabs beside a retained task draft in the actual workbench", async () => {
    render(workspace(client()))
    fireEvent.click(
      await screen.findByRole("button", { name: /Synthetic launch brief/ })
    )
    fireEvent.click(await screen.findByRole("button", { name: "Edit task" }))
    const brief = screen.getByRole("textbox", { name: "Brief" })
    fireEvent.change(brief, {
      target: { value: "Synthetic unsaved human plan" },
    })
    fireEvent.click(screen.getByRole("button", { name: "AI workspace" }))
    fireEvent.change(await screen.findByLabelText("Choose an AI profile"), {
      target: { value: profile.id },
    })
    fireEvent.click(screen.getByRole("button", { name: "Start conversation" }))
    await screen.findByText("Connected to this session")
    expect(screen.getByRole("tab", { name: session.title })).toHaveAttribute(
      "aria-selected",
      "true"
    )
    fireEvent.click(screen.getByRole("tab", { name: task.task.title }))
    expect(screen.getByRole("textbox", { name: "Brief" })).toBe(brief)
    expect(brief).toHaveValue("Synthetic unsaved human plan")
    expect(start).toHaveBeenCalledOnce()
    expect(
      fetcher.mock.calls.some(([url]) => String(url).includes("/tasks/update"))
    ).toBe(false)
    expect(
      fetcher.mock.calls.every(([url]) =>
        String(url).startsWith("http://127.0.0.1:4359/api/business/")
      )
    ).toBe(true)
  })

  it("does not expose or preload the AI entry for a personal owner credential", async () => {
    render(workspace(client(), false))
    fireEvent.click(
      await screen.findByRole("button", { name: /Synthetic launch brief/ })
    )
    await screen.findByRole("heading", { name: task.task.title })
    expect(screen.queryByRole("button", { name: "AI workspace" })).toBeNull()
    expect(
      fetcher.mock.calls.some(([url]) => String(url).includes("/execution/"))
    ).toBe(false)
  })

  it("refuses direct personal access to a session pane without issuing a request", () => {
    render(
      wrap(
        <BusinessSessionPane
          client={client()}
          taskId={task.task.id}
          sessionId={session.id}
          originalOperator={false}
          onGuard={vi.fn()}
        />
      )
    )
    expect(screen.getByRole("alert")).toBeVisible()
    expect(fetcher).not.toHaveBeenCalled()
  })
})
