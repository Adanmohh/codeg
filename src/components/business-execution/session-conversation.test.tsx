import {
  act,
  cleanup,
  fireEvent,
  render,
  screen,
  waitFor,
} from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { afterEach, describe, expect, it, vi } from "vitest"
import {
  createExecutionHttpClient,
  type ExecutionClient,
} from "@/lib/business-execution/client"
import type {
  EventsInput,
  MessagePart,
  SessionFrame,
  SessionSummary,
} from "@/lib/business-execution/types"
import { SessionConversation } from "./session-conversation"

const session: SessionSummary = {
  id: "synthetic-session",
  taskId: "synthetic-task",
  profileId: "synthetic-profile",
  profileRevision: 1,
  revision: 4,
  generation: 1,
  status: "idle",
  mode: "chat",
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
const draft = "Synthetic unsent session draft — مسودة"
const clients: ExecutionClient[] = []
const part = (
  text: string,
  messageId = "synthetic-message",
  index = 0
): MessagePart => ({
  messageId,
  role: "assistant",
  part: index,
  text,
  complete: true,
})
const encode = (...frames: SessionFrame[]) =>
  new TextEncoder().encode(
    frames.map((frame) => JSON.stringify(frame)).join("\n") + "\n"
  )
function setup() {
  const connections: {
    input: EventsInput
    stream: ReadableStreamDefaultController<Uint8Array>
  }[] = []
  const cancel = vi.fn()
  const history = vi.fn(async () => ({
    messages: [part("Synthetic recent saved page")],
    nextBeforeCursor: "opaque-older" as string | null,
  }))
  const get = vi.fn(async () => ({ session }))
  const post = vi.fn(async (path: string, input: object) => {
    if (path === "execution/sessions/events") {
      const binding = input as EventsInput
      return new Response(
        new ReadableStream<Uint8Array>({
          start(stream) {
            connections.push({ input: binding, stream })
            stream.enqueue(
              encode({
                type: "snapshot",
                sessionId: binding.sessionId,
                generation: binding.expectedGeneration,
                operation: {
                  id: binding.operationId,
                  status: "confirmed",
                  reason: null,
                },
                cursor: `opaque-start-${connections.length}`,
                reset: true,
                reason: binding.cursor ? "cursor_expired" : "initial",
                state: {
                  status: "idle",
                  messages: [part(`Synthetic snapshot ${connections.length}`)],
                  tools: [],
                },
                olderCursor: "opaque-earlier",
              })
            )
          },
          cancel,
        }),
        {
          headers: {
            "content-type": "application/x-ndjson; charset=utf-8",
            "cache-control": "no-store",
            "x-content-type-options": "nosniff",
          },
        }
      )
    }
    if (path === "execution/sessions/get") return Response.json(await get())
    if (path === "execution/sessions/history")
      return Response.json(await history())
    throw new Error("Unexpected synthetic mutation")
  })
  const client = createExecutionHttpClient({ post, unauthorized: vi.fn() })
  clients.push(client)
  const emit = (...frames: SessionFrame[]) =>
    connections[connections.length - 1].stream.enqueue(encode(...frames))
  return { client, post, connections, emit, cancel, history, get }
}
function surface(
  client: ExecutionClient,
  options: {
    locale?: "en" | "ar"
    visible?: boolean
    current?: SessionSummary
    onGuard?: ReturnType<typeof vi.fn>
  } = {}
) {
  return (
    <NextIntlClientProvider
      locale={options.locale ?? "en"}
      messages={{}}
      timeZone="UTC"
    >
      <div style={options.visible === false ? { display: "none" } : undefined}>
        <SessionConversation
          client={client}
          session={options.current ?? session}
          references={[]}
          initialDraft={draft}
          onGuard={options.onGuard}
        />
      </div>
    </NextIntlClientProvider>
  )
}
async function ready() {
  await screen.findByText("Connected to this session")
  const editor = await screen.findByRole("textbox", { name: "Prompt" })
  await waitFor(() =>
    expect(screen.getByRole("button", { name: "Send prompt" })).toBeEnabled()
  )
  return editor
}
afterEach(() => {
  cleanup()
  clients.splice(0).forEach((client) => client.close())
  vi.restoreAllMocks()
})

describe("persistent E1 conversation using the existing message/composer surfaces", () => {
  it.each([
    { ...session, capabilities: { ...session.capabilities, read: false } },
    { ...session, status: "revoked" as const },
    { ...session, mode: "terminal" as const },
  ])("does not attach a missing or incompatible read capability", (current) => {
    const { client, post } = setup()
    render(surface(client, { current }))
    expect(screen.getByRole("alert")).toBeVisible()
    expect(post).not.toHaveBeenCalled()
    expect(screen.queryByRole("textbox")).toBeNull()
  })

  it("keeps the exact editor through locale, hidden panes and an explicitly requested snapshot reset", async () => {
    const { client, post, connections } = setup()
    const view = render(surface(client))
    const editor = await ready()
    expect(editor.textContent).toBe(draft)
    view.rerender(surface(client, { locale: "ar", visible: false }))
    expect(editor.isConnected).toBe(true)
    view.rerender(surface(client))
    expect(screen.getByRole("textbox", { name: "Prompt" })).toBe(editor)
    expect(post).toHaveBeenCalledOnce()
    await act(async () => connections[0].stream.close())
    fireEvent.click(
      await screen.findByRole("button", { name: "Check session and reconnect" })
    )
    await screen.findByText("Synthetic snapshot 2")
    expect(screen.queryByText("Synthetic snapshot 1")).toBeNull()
    expect(screen.getByRole("textbox", { name: "Prompt" })).toBe(editor)
    expect(editor.textContent).toBe(draft)
    expect(connections[1].input.cursor).toBe("opaque-start-1")
    expect(post.mock.calls.map(([path]) => path)).toEqual([
      "execution/sessions/events",
      "execution/sessions/get",
      "execution/sessions/events",
    ])
    expect(JSON.stringify(localStorage)).not.toContain(draft)
  })

  it("deduplicates cursor and message part while keeping identical text from distinct messages", async () => {
    const { client, emit } = setup()
    render(surface(client))
    await ready()
    const event: SessionFrame = {
      type: "event",
      sessionId: session.id,
      generation: 1,
      cursor: "opaque-next",
      event: {
        kind: "message",
        message: part("Synthetic repeated text", "second"),
      },
    }
    await act(async () =>
      emit(
        event,
        event,
        { ...event, cursor: "opaque-third" },
        {
          ...event,
          cursor: "opaque-fourth",
          event: {
            kind: "message",
            message: part("Synthetic repeated text", "third"),
          },
        }
      )
    )
    expect(screen.getAllByText("Synthetic repeated text")).toHaveLength(2)
  })

  it("clears the private draft and discards a buffered message after actual stream revocation", async () => {
    const { client, emit, cancel, post } = setup()
    const guard = vi.fn()
    render(surface(client, { onGuard: guard }))
    await ready()
    await act(async () =>
      emit(
        {
          type: "event",
          sessionId: session.id,
          generation: 1,
          cursor: "opaque-revoked",
          event: { kind: "status", status: "revoked" },
        },
        {
          type: "event",
          sessionId: session.id,
          generation: 1,
          cursor: "opaque-late",
          event: {
            kind: "message",
            message: part("Withhold buffered private bytes"),
          },
        }
      )
    )
    expect(screen.getByRole("alert")).toBeVisible()
    expect(screen.queryByRole("textbox")).toBeNull()
    expect(screen.queryByText("Synthetic snapshot 1")).toBeNull()
    expect(screen.queryByText("Withhold buffered private bytes")).toBeNull()
    expect(guard).toHaveBeenLastCalledWith({
      dirty: false,
      busy: false,
      unresolved: false,
    })
    expect(cancel).toHaveBeenCalled()
    expect(post).toHaveBeenCalledOnce()
  })

  it("uses bounded opaque-cursor history pages without resetting the draft or launching anything", async () => {
    const { client, post, history } = setup()
    render(surface(client))
    const editor = await ready()
    fireEvent.click(
      screen.getByRole("button", { name: "Browse saved conversation" })
    )
    await screen.findByText("Synthetic recent saved page")
    history.mockResolvedValueOnce({
      messages: [part("Synthetic older saved page")],
      nextBeforeCursor: null,
    })
    fireEvent.click(screen.getByRole("button", { name: "Older messages" }))
    await screen.findByText("Synthetic older saved page")
    expect(screen.queryByText("Synthetic recent saved page")).toBeNull()
    fireEvent.click(
      screen.getByRole("button", { name: "Return to conversation" })
    )
    expect(screen.getByRole("textbox", { name: "Prompt" })).toBe(editor)
    expect(editor.textContent).toBe(draft)
    expect(post.mock.calls.slice(1).map(([, input]) => input)).toEqual([
      { sessionId: session.id, beforeCursor: null, limit: 40 },
      { sessionId: session.id, beforeCursor: "opaque-older", limit: 40 },
    ])
  })

  it("withholds the whole session when reconnect returns a foreign task binding", async () => {
    const { client, connections, get, post } = setup()
    render(surface(client))
    await ready()
    await act(async () => connections[0].stream.close())
    get.mockResolvedValueOnce({
      session: { ...session, taskId: "foreign-task" },
    })
    fireEvent.click(
      await screen.findByRole("button", { name: "Check session and reconnect" })
    )
    await screen.findByRole("alert")
    expect(screen.queryByRole("textbox")).toBeNull()
    expect(screen.queryByText("Synthetic snapshot 1")).toBeNull()
    expect(post).toHaveBeenCalledTimes(2)
  })

  it("aborts only its attachment on pane unmount, without stop, prompt or launch", async () => {
    const { client, post, cancel } = setup()
    const view = render(surface(client))
    await ready()
    view.unmount()
    await waitFor(() => expect(cancel).toHaveBeenCalled())
    expect(post.mock.calls.map(([path]) => path)).toEqual([
      "execution/sessions/events",
    ])
  })
})
