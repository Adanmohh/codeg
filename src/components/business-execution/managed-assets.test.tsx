import { Blob as NodeBlob } from "node:buffer"
import { webcrypto } from "node:crypto"
import type { ReactNode } from "react"
import {
  act,
  cleanup,
  fireEvent,
  render,
  screen,
  waitFor,
  within,
} from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest"
import {
  createBusinessClient,
  type BusinessClient,
} from "@/lib/business/client"
import type {
  ExecutionInputs,
  ExecutionResults,
  SessionSummary,
} from "@/lib/business-execution/types"
import { detail, member } from "@/components/business/test-fixtures"
import { retained, outputCandidate } from "./asset-test-fixtures"
import { AssetSubmission } from "./asset-submission"
import { ImportOutputDialog } from "./import-output"
import { TaskFilesWorkspace } from "./task-files"

vi.mock("@tauri-apps/api/core", () => ({
  isTauri: () => false,
  invoke: vi.fn(),
}))
const clients: BusinessClient[] = []
const fetcher = vi.fn<typeof fetch>()
const submit =
  vi.fn<(input: ExecutionInputs["assets/submit"]) => Promise<Response>>()
const importFile =
  vi.fn<(input: ExecutionInputs["assets/import-output"]) => Promise<Response>>()
const receipt =
  vi.fn<(input: ExecutionInputs["operations/get"]) => Promise<Response>>()
const assetGet =
  vi.fn<(input: ExecutionInputs["assets/get"]) => Promise<Response>>()
const getTask = vi.fn<() => Promise<Response>>()
const createURL = vi.fn<(blob: Blob) => string>(
  () => "blob:synthetic-private-version"
)
const revokeURL = vi.fn()
let task = detail()
let asset: ExecutionResults["assets/get"]
let session: SessionSummary
const content =
  "# Synthetic file\n<script>syntheticOnly()</script>\n![image](https://invalid.example/synthetic)"

function client() {
  const value = createBusinessClient(
    {
      kind: "http",
      address: "http://127.0.0.1:4359",
      token: "synthetic-operator-only",
    },
    vi.fn()
  )
  clients.push(value)
  return value
}
function wrap(children: ReactNode, locale: "en" | "ar" = "en", visible = true) {
  return (
    <NextIntlClientProvider locale={locale} messages={{}} timeZone="UTC">
      <div style={visible ? undefined : { display: "none" }}>{children}</div>
    </NextIntlClientProvider>
  )
}
function published(input: ExecutionInputs["assets/submit"]) {
  const result = structuredClone(task)
  result.task.revision = input.expectedTaskRevision + 1
  result.task.status = "review"
  result.task.currentDeliverableId = "synthetic-deliverable"
  result.deliverables = [
    {
      id: "synthetic-deliverable",
      revision: result.task.revision,
      author: { id: member.id, displayName: member.displayName, kind: "human" },
      body: input.body,
      createdAt: "2026-09-09T00:00:00Z",
      assets: input.versions.map((selection) => ({
        ...selection,
        title: asset.asset.title,
        mediaType: asset.version.mediaType,
        byteSize: asset.version.byteSize,
        sha256: asset.version.sha256,
      })),
    },
  ]
  return result
}
beforeEach(async () => {
  vi.clearAllMocks()
  vi.stubGlobal("fetch", fetcher)
  vi.stubGlobal("crypto", webcrypto)
  vi.stubGlobal("Blob", NodeBlob)
  vi.stubGlobal(
    "URL",
    class extends URL {
      static createObjectURL = createURL
      static revokeObjectURL = revokeURL
    }
  )
  task = detail()
  asset = retained("synthetic-asset", "synthetic-version", task.task.id)
  const bytes = new TextEncoder().encode(content)
  asset.version.byteSize = bytes.byteLength
  asset.version.sha256 = Array.from(
    new Uint8Array(await webcrypto.subtle.digest("SHA-256", bytes)),
    (byte) => byte.toString(16).padStart(2, "0")
  ).join("")
  session = {
    id: "synthetic-session",
    taskId: task.task.id,
    profileId: "synthetic-profile",
    profileRevision: 1,
    revision: 2,
    generation: 1,
    mode: "chat",
    status: "idle",
    title: "Synthetic saved conversation",
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
  getTask.mockImplementation(async () => Response.json(task))
  assetGet.mockImplementation(async () => Response.json(asset))
  importFile.mockImplementation(async (input) =>
    Response.json({
      asset: asset.asset,
      version: asset.version,
      operation: { id: input.operationId, status: "confirmed", reason: null },
    })
  )
  submit.mockImplementation(async (input) =>
    Response.json({
      detail: published(input),
      operation: { id: input.operationId, status: "confirmed", reason: null },
    })
  )
  receipt.mockImplementation(async (input) =>
    Response.json({
      operation: { id: input.operationId, status: "confirmed", reason: null },
      resourceId: "synthetic-deliverable",
    })
  )
  fetcher.mockImplementation(async (url, options) => {
    const path = new URL(String(url)).pathname
    const { input } = JSON.parse(String(options?.body))
    if (path.endsWith("/tasks/get")) return getTask()
    if (path.endsWith("/assets/list"))
      return Response.json({ items: [asset.asset], nextCursor: null })
    if (path.endsWith("/assets/get")) return assetGet(input)
    if (path.endsWith("/assets/versions"))
      return Response.json({ items: [asset.version], nextCursor: null })
    if (path.endsWith("/assets/import-output")) return importFile(input)
    if (path.endsWith("/assets/submit")) return submit(input)
    if (path.endsWith("/operations/get")) return receipt(input)
    if (path.endsWith("/sessions/get")) return Response.json({ session })
    if (path.endsWith("/outputs/list"))
      return Response.json({ items: [outputCandidate()], nextCursor: null })
    if (path.endsWith("/assets/content"))
      return new Response(content, {
        headers: {
          "content-type": asset.version.mediaType,
          "content-length": String(asset.version.byteSize),
          etag: `"sha256-${asset.version.sha256}"`,
          "cache-control": "no-store",
          "x-content-type-options": "nosniff",
          "content-disposition":
            input.disposition === "preview" ? "inline" : "attachment",
        },
      })
    throw new Error("Unexpected synthetic asset route")
  })
})
afterEach(() => {
  cleanup()
  clients.splice(0).forEach((value) => value.close())
  vi.unstubAllGlobals()
})

function importView(
  business: BusinessClient,
  props: Partial<Parameters<typeof ImportOutputDialog>[0]> = {},
  locale: "en" | "ar" = "en"
) {
  return wrap(
    <ImportOutputDialog
      client={business.execution()}
      taskId={task.task.id}
      sessionId={session.id}
      output={outputCandidate()}
      assets={[asset.asset]}
      onClose={vi.fn()}
      onImported={vi.fn()}
      onGuard={vi.fn()}
      onDenied={vi.fn()}
      {...props}
    />,
    locale
  )
}
function submissionView(
  business: BusinessClient,
  props: Partial<Parameters<typeof AssetSubmission>[0]> = {},
  locale: "en" | "ar" = "en",
  visible = true
) {
  return wrap(
    <AssetSubmission
      business={business}
      client={business.execution()}
      taskId={task.task.id}
      members={[member]}
      selection={[asset]}
      onRemove={vi.fn()}
      onClear={vi.fn()}
      onGuard={vi.fn()}
      onPublished={vi.fn()}
      onTask={vi.fn()}
      onDenied={vi.fn()}
      {...props}
    />,
    locale,
    visible
  )
}

describe("private import with an exact durable intent", () => {
  it("imports only after explicit selection and sends IDs/revisions without paths or caller provenance", async () => {
    const imported = vi.fn()
    render(importView(client(), { onImported: imported }))
    expect(importFile).not.toHaveBeenCalled()
    fireEvent.change(screen.getByLabelText("File title"), {
      target: { value: "My synthetic result" },
    })
    fireEvent.click(screen.getByRole("button", { name: "Retain file" }))
    await waitFor(() => expect(imported).toHaveBeenCalledOnce())
    expect(importFile.mock.calls[0][0]).toEqual({
      operationId: expect.any(String),
      sessionId: session.id,
      outputId: "synthetic-output",
      expectedOutputRevision: 3,
      title: "My synthetic result",
      assetId: null,
      expectedAssetRevision: null,
    })
    expect(submit).not.toHaveBeenCalled()
  })
  it("revalidates adding a version and refuses a missing addVersion capability before importing", async () => {
    asset.capabilities.addVersion = false
    const denied = vi.fn()
    render(importView(client(), { onDenied: denied }))
    fireEvent.change(screen.getByLabelText("Save as"), {
      target: { value: asset.asset.id },
    })
    fireEvent.click(screen.getByRole("button", { name: "Retain file" }))
    await waitFor(() => expect(denied).toHaveBeenCalledOnce())
    expect(assetGet).toHaveBeenCalledOnce()
    expect(importFile).not.toHaveBeenCalled()
  })
  it("holds a lost import through a missing receipt and only replays the identical confirmed operation", async () => {
    importFile.mockRejectedValueOnce(new Error("Synthetic response lost"))
    const imported = vi.fn()
    const close = vi.fn()
    render(importView(client(), { onImported: imported, onClose: close }))
    fireEvent.click(screen.getByRole("button", { name: "Retain file" }))
    await screen.findByRole("button", { name: "Check receipt" })
    fireEvent.click(screen.getByRole("button", { name: "Close" }))
    expect(close).not.toHaveBeenCalled()
    receipt.mockResolvedValueOnce(new Response(null, { status: 404 }))
    fireEvent.click(screen.getByRole("button", { name: "Check receipt" }))
    await screen.findByRole("button", { name: "Check receipt" })
    expect(importFile).toHaveBeenCalledOnce()
    expect(screen.getByLabelText("File title")).toBeDisabled()
    // A newer current asset version cannot change the original receipt target.
    asset.asset.latestVersionId = "synthetic-later-version"
    fireEvent.click(screen.getByRole("button", { name: "Check receipt" }))
    await waitFor(() => expect(imported).toHaveBeenCalledOnce())
    expect(importFile).toHaveBeenCalledTimes(2)
    expect(importFile.mock.calls[1][0]).toEqual(importFile.mock.calls[0][0])
    expect(
      receipt.mock.calls.every(
        ([input]) =>
          input.operationId === importFile.mock.calls[0][0].operationId &&
          input.kind === "import_output"
      )
    ).toBe(true)
  })
  it("keeps an edited title when closing is cancelled and rejects the pinned title length bound", async () => {
    const close = vi.fn()
    render(importView(client(), { onClose: close }))
    const title = screen.getByLabelText("File title")
    fireEvent.change(title, { target: { value: "Synthetic unsaved title" } })
    fireEvent.click(screen.getByRole("button", { name: "Close" }))
    fireEvent.click(screen.getByRole("button", { name: "Keep editing" }))
    expect(title).toHaveValue("Synthetic unsaved title")
    expect(close).not.toHaveBeenCalled()
    fireEvent.change(title, { target: { value: "界".repeat(241) } })
    expect(screen.getByRole("button", { name: "Retain file" })).toBeDisabled()
    expect(importFile).not.toHaveBeenCalled()
  })
})

describe("human selected-version submission", () => {
  it("submits two explicit immutable versions without following a newer latest pointer", async () => {
    const first = structuredClone(asset)
    const second = retained(
      "synthetic-deck",
      "synthetic-deck-version",
      task.task.id
    )
    second.asset.title = "Synthetic presentation.pptx"
    second.asset.mediaType =
      "application/vnd.openxmlformats-officedocument.presentationml.presentation"
    second.version.mediaType = second.asset.mediaType
    first.asset.latestVersionId = "synthetic-newer-unselected-version"
    const selected = [first, second]
    assetGet.mockImplementation(async (input) =>
      Response.json(
        selected.find((item) => item.version.id === input.versionId)
      )
    )
    submit.mockImplementationOnce(async (input) => {
      const result = published(input)
      result.deliverables[0].assets = selected.map((item) => ({
        assetId: item.asset.id,
        versionId: item.version.id,
        title: item.asset.title,
        mediaType: item.version.mediaType,
        byteSize: item.version.byteSize,
        sha256: item.version.sha256,
      }))
      return Response.json({
        detail: result,
        operation: { id: input.operationId, status: "confirmed", reason: null },
      })
    })
    const done = vi.fn()
    render(submissionView(client(), { selection: selected, onPublished: done }))
    fireEvent.click(
      screen.getByRole("button", { name: "Review files and audience" })
    )
    fireEvent.click(await screen.findByRole("checkbox"))
    fireEvent.click(
      screen.getByRole("button", { name: "Submit files for human review" })
    )
    await waitFor(() => expect(done).toHaveBeenCalledOnce())
    expect(submit.mock.calls[0][0].versions).toEqual(
      selected.map((item) => ({
        assetId: item.asset.id,
        versionId: item.version.id,
      }))
    )
    expect(JSON.stringify(submit.mock.calls)).not.toContain(
      "synthetic-newer-unselected-version"
    )
  })
  it("shows the current changed task audience and revision before asking for confirmation", async () => {
    render(submissionView(client()))
    task.task.domain = "website"
    task.task.revision = 9
    fireEvent.click(
      screen.getByRole("button", { name: "Review files and audience" })
    )
    const confirm = await screen.findByRole("checkbox")
    expect(screen.getByText("Website")).toBeVisible()
    expect(screen.getByText("Saved task revision: 9")).toBeVisible()
    expect(confirm).not.toBeChecked()
    expect(submit).not.toHaveBeenCalled()
  })
  it("requires current file/task review plus explicit audience confirmation before the one exact submission", async () => {
    const done = vi.fn()
    render(submissionView(client(), { onPublished: done }))
    fireEvent.change(screen.getByLabelText("Note for the reviewer"), {
      target: { value: "Synthetic human note" },
    })
    expect(assetGet).not.toHaveBeenCalled()
    expect(submit).not.toHaveBeenCalled()
    fireEvent.click(
      screen.getByRole("button", { name: "Review files and audience" })
    )
    const confirm = await screen.findByRole("checkbox")
    expect(screen.getByText("Destination audience")).toBeVisible()
    expect(screen.getByText("Marketing")).toBeVisible()
    expect(confirm).not.toBeChecked()
    expect(
      screen.getByRole("button", { name: "Submit files for human review" })
    ).toBeDisabled()
    fireEvent.click(confirm)
    fireEvent.click(
      screen.getByRole("button", { name: "Submit files for human review" })
    )
    await waitFor(() => expect(done).toHaveBeenCalledOnce())
    expect(submit).toHaveBeenCalledOnce()
    expect(submit.mock.calls[0][0]).toEqual({
      operationId: expect.any(String),
      taskId: task.task.id,
      expectedTaskRevision: task.task.revision,
      versions: [{ assetId: asset.asset.id, versionId: asset.version.id }],
      body: "Synthetic human note",
    })
    expect(
      screen.getByText(/They are not approved or marked done/)
    ).toBeVisible()
    expect(
      fetcher.mock.calls.some(([url]) =>
        /\/tasks\/(submit|review|progress)$/.test(String(url))
      )
    ).toBe(false)
  })
  it("invalidates confirmation after a note edit and preserves that exact draft across same-scope locale/visibility changes", async () => {
    const business = client()
    const execution = business.execution()
    const view = render(submissionView(business, { client: execution }))
    const note = screen.getByLabelText("Note for the reviewer")
    fireEvent.change(note, {
      target: { value: "Synthetic retained private note" },
    })
    fireEvent.click(
      screen.getByRole("button", { name: "Review files and audience" })
    )
    fireEvent.click(await screen.findByRole("checkbox"))
    fireEvent.change(note, { target: { value: "Synthetic changed note" } })
    expect(
      screen.queryByRole("button", { name: "Submit files for human review" })
    ).toBeNull()
    view.rerender(submissionView(business, { client: execution }, "ar", false))
    view.rerender(submissionView(business, { client: execution }, "ar", true))
    expect(screen.getByLabelText("ملاحظة للمراجع")).toBe(note)
    expect(note).toHaveValue("Synthetic changed note")
    expect(submit).not.toHaveBeenCalled()
  })
  it("reconciles a lost submission using its receipt and selected historical deliverable without submitting twice", async () => {
    submit.mockRejectedValueOnce(new Error("Synthetic response lost"))
    const done = vi.fn()
    render(submissionView(client(), { onPublished: done }))
    fireEvent.click(
      screen.getByRole("button", { name: "Review files and audience" })
    )
    fireEvent.click(await screen.findByRole("checkbox"))
    fireEvent.click(
      screen.getByRole("button", { name: "Submit files for human review" })
    )
    await screen.findByRole("button", { name: "Check receipt" })
    receipt.mockResolvedValueOnce(new Response(null, { status: 404 }))
    fireEvent.click(screen.getByRole("button", { name: "Check receipt" }))
    await screen.findByRole("button", { name: "Check receipt" })
    expect(screen.getByLabelText("Note for the reviewer")).toBeDisabled()
    task = published(submit.mock.calls[0][0])
    task.task.revision += 1
    task.task.currentDeliverableId = null
    fireEvent.click(screen.getByRole("button", { name: "Check receipt" }))
    await waitFor(() => expect(done).toHaveBeenCalledOnce())
    expect(submit).toHaveBeenCalledOnce()
    expect(
      receipt.mock.calls.every(
        ([input]) =>
          input.operationId === submit.mock.calls[0][0].operationId &&
          input.kind === "submit"
      )
    ).toBe(true)
  })
  it("withholds a tampered version before exposing confirmation or submitting", async () => {
    const expected = structuredClone(asset)
    asset.version.sha256 = "b".repeat(64)
    render(submissionView(client(), { selection: [expected] }))
    fireEvent.click(
      screen.getByRole("button", { name: "Review files and audience" })
    )
    await screen.findByRole("alert")
    expect(screen.queryByRole("checkbox")).toBeNull()
    expect(submit).not.toHaveBeenCalled()
  })
  it("ignores a late successful publication reply after the private scope is removed", async () => {
    let resolve: ((response: Response) => void) | null = null
    submit.mockImplementationOnce(
      () =>
        new Promise((done) => {
          resolve = done
        })
    )
    const done = vi.fn()
    const view = render(submissionView(client(), { onPublished: done }))
    fireEvent.click(
      screen.getByRole("button", { name: "Review files and audience" })
    )
    fireEvent.click(await screen.findByRole("checkbox"))
    fireEvent.click(
      screen.getByRole("button", { name: "Submit files for human review" })
    )
    await waitFor(() => expect(submit).toHaveBeenCalledOnce())
    view.unmount()
    await act(async () => {
      const input = submit.mock.calls[0][0]
      resolve?.(
        Response.json({
          detail: published(input),
          operation: {
            id: input.operationId,
            status: "confirmed",
            reason: null,
          },
        })
      )
    })
    expect(done).not.toHaveBeenCalled()
  })
})

describe("private task file workspace", () => {
  function page(
    business: BusinessClient,
    originalOperator = true,
    sessionId: string | null = session.id
  ) {
    return wrap(
      <TaskFilesWorkspace
        business={business}
        originalOperator={originalOperator}
        taskId={task.task.id}
        sessionId={sessionId}
        members={[member]}
        onGuard={vi.fn()}
        onPublished={vi.fn()}
        onTask={vi.fn()}
      />
    )
  }
  it.each(["member", "native"])(
    "does not load private files at the %s boundary",
    (kind) => {
      const business = client()
      render(
        page(
          kind === "native" ? { ...business, native: true } : business,
          kind !== "member"
        )
      )
      expect(screen.getByRole("alert")).toBeVisible()
      expect(fetcher).not.toHaveBeenCalled()
    }
  )
  it("loads retained metadata without scanning/importing output, then displays verified escaped content and disposes its download", async () => {
    const view = render(page(client()))
    fireEvent.click(
      await screen.findByRole("button", { name: /Synthetic launch brief.md/ })
    )
    fireEvent.click(await screen.findByRole("button", { name: "Read file" }))
    await waitFor(() =>
      expect(document.querySelector("pre")?.textContent).toBe(content)
    )
    expect(document.querySelector("img, iframe, script")).toBeNull()
    expect(
      fetcher.mock.calls.some(([url]) => String(url).endsWith("/outputs/list"))
    ).toBe(false)
    expect(importFile).not.toHaveBeenCalled()
    fireEvent.click(screen.getByRole("button", { name: "Prepare download" }))
    const save = await screen.findByRole("link", { name: "Save file" })
    expect(save).toHaveAttribute("href", "blob:synthetic-private-version")
    view.unmount()
    expect(revokeURL).toHaveBeenCalledWith("blob:synthetic-private-version")
    expect(
      fetcher.mock.calls.every(([url]) =>
        String(url).startsWith("http://127.0.0.1:4359/api/business/")
      )
    ).toBe(true)
  })
  it("checks actual session output only on request and keeps import separate from review selection", async () => {
    render(page(client()))
    fireEvent.click(
      await screen.findByRole("button", { name: "Check session files" })
    )
    fireEvent.click(await screen.findByRole("button", { name: "Retain file" }))
    const dialog = await screen.findByRole("dialog")
    fireEvent.click(within(dialog).getByRole("button", { name: "Retain file" }))
    await waitFor(() => expect(screen.queryByRole("dialog")).toBeNull())
    expect(importFile).toHaveBeenCalledOnce()
    expect(submit).not.toHaveBeenCalled()
    expect(
      within(
        screen.getByRole("list", { name: "Selected file versions" })
      ).queryAllByRole("listitem")
    ).toHaveLength(0)
  })
  it("removes private file content and actions after an authenticated denial", async () => {
    render(page(client(), true, null))
    fireEvent.click(
      await screen.findByRole("button", { name: /Synthetic launch brief.md/ })
    )
    await screen.findByRole("button", { name: "Read file" })
    fetcher.mockResolvedValueOnce(new Response(null, { status: 403 }))
    fireEvent.click(screen.getByRole("button", { name: "Read file" }))
    await screen.findByRole("alert")
    expect(screen.queryByText("Synthetic launch brief.md")).toBeNull()
    expect(
      screen.queryByRole("button", { name: "Select this version for review" })
    ).toBeNull()
    expect(createURL).not.toHaveBeenCalled()
  })
  it("does not deliver a late file result into a different selected version", async () => {
    const first = structuredClone(asset)
    let resolveFirst: ((response: Response) => void) | null = null
    assetGet.mockImplementationOnce(
      () =>
        new Promise((resolve) => {
          resolveFirst = resolve
        })
    )
    render(page(client(), true, null))
    fireEvent.click(
      await screen.findByRole("button", { name: /Synthetic launch brief.md/ })
    )
    await waitFor(() => expect(assetGet).toHaveBeenCalledOnce())
    asset = retained(
      "synthetic-second-asset",
      "synthetic-second-version",
      task.task.id
    )
    asset.asset.title = "Synthetic second file"
    fireEvent.change(screen.getByLabelText("Find a saved file"), {
      target: { value: "second" },
    })
    fireEvent.click(screen.getByRole("button", { name: "Search files" }))
    fireEvent.click(
      await screen.findByRole("button", { name: /Synthetic second file/ })
    )
    await screen.findByRole("heading", { name: "Synthetic second file" })
    await act(async () => {
      resolveFirst?.(Response.json(first))
    })
    expect(
      screen.getByRole("heading", { name: "Synthetic second file" })
    ).toBeVisible()
    expect(
      screen.queryByRole("heading", { name: "Synthetic launch brief.md" })
    ).toBeNull()
  })
})
