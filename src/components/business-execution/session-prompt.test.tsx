import { act, fireEvent, render, screen, waitFor } from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { afterEach, describe, expect, it, vi } from "vitest"
import {
  createExecutionHttpClient,
  type ExecutionClient,
  type ExecutionHttpTransport,
} from "@/lib/business-execution/client"
import type { SessionSummary } from "@/lib/business-execution/types"
import { SessionPrompt, type PromptReference } from "./session-prompt"

const session: SessionSummary = {
  id: "synthetic-session",
  taskId: "synthetic-task",
  profileId: "synthetic-profile",
  profileRevision: 1,
  revision: 4,
  generation: 2,
  status: "idle",
  mode: "chat",
  title: "Synthetic business brief",
  createdAt: "2026-09-09T00:00:00Z",
  updatedAt: "2026-09-09T00:00:00Z",
  lastActivityAt: "2026-09-09T00:00:00Z",
  capabilities: {
    read: true,
    prompt: true,
    continue: false,
    stop: false,
    terminalWrite: false,
    importOutput: true,
  },
  reason: null,
}
const reference: PromptReference = {
  label: "Synthetic task, revision 3",
  input: { kind: "task", taskId: session.taskId, expectedRevision: 3 },
}
const draft =
  "  Synthetic launch brief — خطة 🧭\nKeep this exact second line.  "
const scopes: ExecutionClient[] = []
function setup(post: ExecutionHttpTransport["post"]) {
  const host = { post: vi.fn(post), unauthorized: vi.fn() }
  const client = createExecutionHttpClient(host)
  scopes.push(client)
  return { client, host }
}
function deferred() {
  let resolve!: (response: Response) => void
  const promise = new Promise<Response>((done) => {
    resolve = done
  })
  return { promise, resolve }
}
function receipt(operationId: string) {
  return Response.json({
    operation: { id: operationId, status: "confirmed", reason: null },
    messageId: "synthetic-message",
    inputHash: "a".repeat(64),
  })
}
function surface(
  client: ExecutionClient,
  onAccepted = vi.fn(),
  options: {
    locale?: "en" | "ar"
    visible?: boolean
    references?: PromptReference[]
    initialDraft?: string
    session?: SessionSummary
  } = {}
) {
  return (
    <NextIntlClientProvider
      locale={options.locale ?? "en"}
      messages={{}}
      timeZone="UTC"
    >
      <div style={options.visible === false ? { display: "none" } : undefined}>
        <SessionPrompt
          client={client}
          session={options.session ?? session}
          initialDraft={options.initialDraft ?? draft}
          references={options.references ?? [reference]}
          onAccepted={onAccepted}
        />
      </div>
    </NextIntlClientProvider>
  )
}
async function ready() {
  const editor = await screen.findByRole("textbox", { name: "Prompt" })
  await waitFor(
    () =>
      expect(screen.getByRole("button", { name: "Send prompt" })).toBeEnabled(),
    { timeout: 5000 }
  )
  return editor
}
afterEach(() => {
  for (const scope of scopes.splice(0)) scope.close()
  vi.restoreAllMocks()
})

describe("existing RichComposer bound to an E1 prompt receipt", () => {
  it("refuses prompt intent when the current session capability is absent", async () => {
    const { client, host } = setup(async () => Response.json({}))
    render(
      surface(client, vi.fn(), {
        session: {
          ...session,
          capabilities: { ...session.capabilities, prompt: false },
        },
      })
    )
    const editor = await screen.findByRole("textbox", { name: "Prompt" })
    await waitFor(() =>
      expect(editor.textContent).toContain("Synthetic launch brief")
    )
    fireEvent.keyDown(editor, { key: "Enter", code: "Enter" })
    expect(screen.getByRole("button", { name: "Send prompt" })).toBeDisabled()
    expect(
      screen.getByText("This action is not available for this session.")
    ).toBeInTheDocument()
    expect(host.post).not.toHaveBeenCalled()
  })
  it("keeps the submitted reference review unchanged while newer task metadata arrives", async () => {
    const held = deferred()
    const { client } = setup(() => held.promise)
    const callback = vi.fn()
    const view = render(surface(client, callback))
    await ready()
    fireEvent.click(screen.getByRole("checkbox", { name: reference.label }))
    fireEvent.click(screen.getByRole("button", { name: "Send prompt" }))
    const newer: PromptReference = {
      label: "Synthetic task, revision 4",
      input: { kind: "task", taskId: session.taskId, expectedRevision: 4 },
    }
    view.rerender(surface(client, callback, { references: [newer] }))
    expect(
      screen.getByRole("checkbox", { name: reference.label })
    ).toBeChecked()
    expect(
      screen.queryByRole("checkbox", { name: newer.label })
    ).not.toBeInTheDocument()
    await act(async () => {
      held.resolve(new Response(null, { status: 503 }))
    })
    await screen.findByRole("button", { name: "Check receipt" })
    expect(
      screen.getByRole("checkbox", { name: reference.label })
    ).toBeChecked()
  })
  it("does not launch or post on mount and retains its editor through locale and pane visibility", async () => {
    const { client, host } = setup(async () => Response.json({}))
    const callback = vi.fn()
    const view = render(surface(client, callback))
    const editor = await ready()
    const originalText = editor.textContent
    view.rerender(surface(client, callback, { locale: "ar", visible: false }))
    expect(editor.isConnected).toBe(true)
    expect(editor.textContent).toBe(originalText)
    view.rerender(surface(client, callback, { locale: "en" }))
    expect(screen.getByRole("textbox", { name: "Prompt" })).toBe(editor)
    expect(editor.textContent).toBe(originalText)
    expect(host.post).not.toHaveBeenCalled()
    expect(JSON.stringify(localStorage)).not.toContain("Synthetic launch brief")
  })

  it("sends one exact prompt with explicit references despite repeated Enter intent", async () => {
    const held = deferred()
    const { client, host } = setup(() => held.promise)
    const accepted = vi.fn()
    render(surface(client, accepted))
    const editor = await ready()
    fireEvent.click(screen.getByRole("checkbox", { name: reference.label }))
    fireEvent.keyDown(editor, { key: "Enter", code: "Enter" })
    fireEvent.keyDown(editor, { key: "Enter", code: "Enter" })
    expect(host.post).toHaveBeenCalledOnce()
    const input = host.post.mock.calls[0][1] as {
      operationId: string
      text: string
    }
    expect(input).toMatchObject({
      sessionId: session.id,
      expectedSessionRevision: 4,
      text: draft,
      inputs: [reference.input],
    })
    expect(screen.getByRole("button", { name: "Send prompt" })).toBeDisabled()
    expect(editor.textContent).toContain("Keep this exact second line.")
    await act(async () => {
      held.resolve(receipt(input.operationId))
    })
    await waitFor(() => expect(accepted).toHaveBeenCalledOnce())
    expect(editor.textContent).toBe("")
    expect(
      screen.getByText(
        "Prompt accepted. The response will appear in the conversation."
      )
    ).toBeInTheDocument()
    expect(host.post).toHaveBeenCalledOnce()
  })

  it("keeps an uncertain prompt through missing receipt lookup and reconciles without resend", async () => {
    const { client, host } = setup(async () => {
      throw new Error("synthetic private transport diagnostic")
    })
    const accepted = vi.fn()
    render(surface(client, accepted))
    const editor = await ready()
    fireEvent.click(screen.getByRole("button", { name: "Send prompt" }))
    const check = await screen.findByRole("button", { name: "Check receipt" })
    const original = host.post.mock.calls[0][1] as { operationId: string }
    expect(editor.textContent).toContain("Synthetic launch brief")
    expect(
      screen.queryByText("synthetic private transport diagnostic")
    ).not.toBeInTheDocument()
    host.post.mockResolvedValueOnce(new Response(null, { status: 404 }))
    fireEvent.click(check)
    await screen.findByRole("button", { name: "Check receipt" })
    expect(editor.textContent).toContain("Synthetic launch brief")
    expect(screen.getByRole("button", { name: "Send prompt" })).toBeDisabled()
    host.post.mockResolvedValueOnce(
      Response.json({
        operation: {
          id: original.operationId,
          status: "confirmed",
          reason: null,
        },
        resourceId: "synthetic-message",
      })
    )
    fireEvent.click(screen.getByRole("button", { name: "Check receipt" }))
    await waitFor(() => expect(accepted).toHaveBeenCalledOnce())
    expect(editor.textContent).toBe("")
    expect(host.post.mock.calls.map(([path]) => path)).toEqual([
      "execution/sessions/prompt",
      "execution/operations/get",
      "execution/operations/get",
    ])
    expect(
      host.post.mock.calls
        .slice(1)
        .every(
          ([, input]) =>
            (input as { operationId: string }).operationId ===
            original.operationId
        )
    ).toBe(true)
  })

  it("requires explicit draft review after a confirmed failure and never automatically sends again", async () => {
    const { client, host } = setup(async () => {
      throw new Error("lost response")
    })
    render(surface(client))
    const editor = await ready()
    fireEvent.click(screen.getByRole("button", { name: "Send prompt" }))
    await screen.findByRole("button", { name: "Check receipt" })
    const original = host.post.mock.calls[0][1] as { operationId: string }
    host.post.mockResolvedValueOnce(
      Response.json({
        operation: {
          id: original.operationId,
          status: "failed",
          reason: "cancelled",
        },
        resourceId: null,
      })
    )
    fireEvent.click(screen.getByRole("button", { name: "Check receipt" }))
    fireEvent.click(
      await screen.findByRole("button", { name: "Review prompt" })
    )
    await waitFor(() =>
      expect(screen.getByRole("button", { name: "Send prompt" })).toBeEnabled()
    )
    expect(editor.textContent).toContain("Synthetic launch brief")
    expect(host.post).toHaveBeenCalledTimes(2)
  })

  it("withholds a mismatched receipt and retains the original request for lookup", async () => {
    const { client, host } = setup(async () => receipt("foreign-operation"))
    render(surface(client))
    const editor = await ready()
    fireEvent.click(screen.getByRole("button", { name: "Send prompt" }))
    await screen.findByRole("button", { name: "Check receipt" })
    expect(editor.textContent).toContain("Synthetic launch brief")
    expect(screen.getByRole("button", { name: "Send prompt" })).toBeDisabled()
    expect(host.post).toHaveBeenCalledOnce()
  })

  it("requires a new explicit selection after a task reference revision changes", async () => {
    const { client, host } = setup(async () => Response.json({}))
    const callback = vi.fn()
    const view = render(surface(client, callback))
    const editor = await ready()
    fireEvent.click(screen.getByRole("checkbox", { name: reference.label }))
    const newer: PromptReference = {
      label: "Synthetic task, revision 4",
      input: { kind: "task", taskId: session.taskId, expectedRevision: 4 },
    }
    view.rerender(surface(client, callback, { references: [newer] }))
    expect(editor.textContent).toContain("Synthetic launch brief")
    expect(screen.getByRole("button", { name: "Send prompt" })).toBeDisabled()
    expect(
      screen.getByRole("checkbox", { name: newer.label })
    ).not.toBeChecked()
    fireEvent.click(screen.getByRole("button", { name: "Clear selection" }))
    expect(screen.getByRole("button", { name: "Send prompt" })).toBeEnabled()
    expect(host.post).not.toHaveBeenCalled()
  })

  it("detaches a held prompt on unmount without delivery into a new session pane", async () => {
    const held = deferred()
    const { client, host } = setup(() => held.promise)
    const accepted = vi.fn()
    const first = render(surface(client, accepted))
    await ready()
    fireEvent.click(screen.getByRole("button", { name: "Send prompt" }))
    const input = host.post.mock.calls[0][1] as { operationId: string }
    first.unmount()
    render(
      surface(client, accepted, {
        initialDraft: "New synthetic session draft",
        session: { ...session, id: "other-synthetic-session" },
      })
    )
    await ready()
    await act(async () => {
      held.resolve(receipt(input.operationId))
    })
    expect(screen.getByRole("textbox", { name: "Prompt" }).textContent).toBe(
      "New synthetic session draft"
    )
    expect(accepted).not.toHaveBeenCalled()
    expect(host.post.mock.calls[0][2].aborted).toBe(true)
    expect(host.post).toHaveBeenCalledOnce()
  })
})
