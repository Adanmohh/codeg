import { act, fireEvent, render, screen, waitFor } from "@testing-library/react"
import { afterEach, beforeEach, expect, it, vi } from "vitest"
import type { ReactNode } from "react"
import { OpsPage } from "./ops-page"
import { WorkbenchRouteProvider } from "@/contexts/workbench-route-context"
import { useRemoteConnection } from "@/contexts/remote-connection-context"
import { ops, type OpsContext, type Proposal, type Thread } from "@/lib/ops/api"
import { AppI18nProvider } from "@/components/i18n-provider"
import { getSystemLanguageSettings } from "@/lib/api"
import { getMessagesForLocale } from "@/i18n/messages"
import { LANGUAGE_SETTINGS_STORAGE_KEY } from "@/lib/i18n"
import type { SystemLanguageSettings } from "@/lib/types"
import type { AbstractIntlMessages } from "next-intl"

vi.mock("@/lib/api", () => ({ getSystemLanguageSettings: vi.fn() }))
vi.mock("@/i18n/messages", () => ({
  getFallbackMessages: () => ({}),
  getMessagesForLocale: vi.fn(),
}))
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(async () => () => {}),
}))

vi.mock("@/contexts/remote-connection-context", () => ({
  useRemoteConnection: vi.fn(() => null),
}))
vi.mock("@/lib/ops/api", () => ({
  ops: {
    context: vi.fn(),
    tickets: vi.fn(),
    thread: vi.fn(),
    proposals: vi.fn(),
    proposal: vi.fn(),
    emailStatus: vi.fn(),
    saveDraft: vi.fn(),
    note: vi.fn(),
    approve: vi.fn(),
  },
}))
vi.mock("@/components/ai-elements/message", () => ({
  Message: ({ children }: { children: ReactNode }) => <div>{children}</div>,
  MessageContent: ({ children }: { children: ReactNode }) => (
    <div>{children}</div>
  ),
}))

const context: OpsContext = {
  accountId: 1,
  operator: "operator:http",
  emailTransport: "per_inbox",
  transportMessage: "Human review required",
  inboxes: [{ id: 1, name: "Support", email: "support@example.com" }],
}
const reply = {
  inboxId: 1,
  conversationId: 1,
  from: "support@example.com",
  to: ["reader@example.com"],
  cc: [],
  bcc: [],
  subject: "Re: Request",
  text: "Saved server body",
  inReplyTo: "parent@example.com",
  references: ["parent@example.com"],
}
const thread: Thread = {
  ticket: {
    id: 1,
    inboxId: 1,
    subject: "Request",
    contact: "Reader",
    status: 0,
    updatedAt: "2026-09-08T10:00:00Z",
  },
  contactEmail: "reader@example.com",
  messages: [],
  draft: { id: 1, revision: 1, reply, updatedAt: "2026-09-08T10:00:00Z" },
  suggestedReply: reply,
}
const proposal: Proposal = {
  id: 1,
  inboxId: 1,
  conversationId: 1,
  taskId: 1,
  runSeq: 1,
  status: "pending",
  stale: false,
  reason: null,
  createdAt: "2026-09-08T10:00:00Z",
  delivery: null,
  payload: { draftId: 1, draftRevision: 1, reply },
}
beforeEach(() => {
  vi.clearAllMocks()
  vi.mocked(getSystemLanguageSettings).mockResolvedValue({
    mode: "manual",
    language: "en",
  })
  vi.mocked(getMessagesForLocale).mockResolvedValue({})
  vi.mocked(useRemoteConnection).mockReturnValue(null)
  vi.mocked(ops.context).mockResolvedValue(context)
  vi.mocked(ops.tickets).mockResolvedValue({
    items: [thread.ticket],
    hasMore: false,
  })
  vi.mocked(ops.thread).mockResolvedValue(thread)
  vi.mocked(ops.proposals).mockResolvedValue([proposal])
  vi.mocked(ops.proposal).mockResolvedValue(proposal)
  vi.mocked(ops.emailStatus).mockResolvedValue({
    inboxId: 1,
    configured: false,
    lastPullAt: null,
    lastPullStatus: null,
    lastPullError: null,
  })
})

function LocaleShell({ mobile }: { mobile: boolean }) {
  return (
    <AppI18nProvider initialMessages={{}}>
      <Shell mobile={mobile} />
    </AppI18nProvider>
  )
}
function deferred<T>() {
  let resolve!: (value: T) => void
  let reject!: (error: Error) => void
  const promise = new Promise<T>((yes, no) => {
    resolve = yes
    reject = no
  })
  return { promise, resolve, reject }
}
function changeLocale(language: SystemLanguageSettings["language"]) {
  // The browser dispatches this to the workspace when the separate Settings
  // tab persists its language preference. No private editor value is stored.
  fireEvent(
    window,
    new StorageEvent("storage", {
      key: LANGUAGE_SETTINGS_STORAGE_KEY,
      newValue: JSON.stringify({ mode: "manual", language }),
    })
  )
}
async function openReply() {
  fireEvent.click(
    await screen.findByRole("button", { name: /Reader Open Request/ })
  )
  return screen.findByLabelText("Reply message")
}

it("keeps the initial boot guard until both settings and their messages load", async () => {
  const settings = deferred<SystemLanguageSettings>()
  const messages = deferred<AbstractIntlMessages>()
  vi.mocked(getSystemLanguageSettings).mockReturnValue(settings.promise)
  vi.mocked(getMessagesForLocale).mockReturnValue(messages.promise)
  render(<LocaleShell mobile={false} />)
  expect(
    screen.queryByRole("region", { name: "Ops desk" })
  ).not.toBeInTheDocument()
  expect(ops.context).not.toHaveBeenCalled()
  await act(async () => settings.resolve({ mode: "manual", language: "ar" }))
  expect(ops.context).not.toHaveBeenCalled()
  expect(document.documentElement.dir).toBe("ltr")
  await act(async () => messages.resolve({}))
  await openReply()
  expect(document.documentElement.lang).toBe("ar")
  expect(document.documentElement.dir).toBe("rtl")
})

it.each([false, true])(
  "preserves actual reply, note and complete review across Settings locale changes (mobile=%s)",
  async (mobile) => {
    const storage = vi.spyOn(Storage.prototype, "setItem")
    vi.spyOn(window, "confirm").mockReturnValue(true)
    const arabic = deferred<AbstractIntlMessages>()
    vi.mocked(getMessagesForLocale).mockImplementation((locale) =>
      locale === "ar" ? arabic.promise : Promise.resolve({})
    )
    const { rerender } = render(<LocaleShell mobile={mobile} />)
    const replyInput = await openReply()
    fireEvent.change(replyInput, {
      target: { value: "Private reply sentinel (42)" },
    })
    fireEvent.click(screen.getByRole("button", { name: "Private note" }))
    const note = screen.getByLabelText("Private note · never emailed")
    fireEvent.change(note, { target: { value: "ملاحظة sentinel" } })
    changeLocale("ar")
    expect(screen.getByLabelText("Private note · never emailed")).toBe(note)
    expect(note).toHaveValue("ملاحظة sentinel")
    expect(document.documentElement.dir).toBe("ltr")
    await act(async () => arabic.resolve({}))
    expect(document.documentElement.dir).toBe("rtl")
    expect(screen.getByLabelText("Private note · never emailed")).toBe(note)
    rerender(<LocaleShell mobile={!mobile} />)
    expect(
      await screen.findByLabelText("Private note · never emailed")
    ).toHaveValue("ملاحظة sentinel")
    fireEvent.click(screen.getByRole("button", { name: "Reply draft" }))
    expect(screen.getByLabelText("Reply message")).toHaveValue(
      "Private reply sentinel (42)"
    )
    fireEvent.click(screen.getByRole("button", { name: "Approvals" }))
    fireEvent.click(
      await screen.findByRole("button", { name: /Re: Request Task/ })
    )
    await screen.findByLabelText("Reply message")
    const edits = {
      To: "new-sentinel@example.com, second@example.com",
      Cc: "cc-sentinel@example.com",
      "Bcc · hidden from other recipients": "bcc-sentinel@example.com",
      Subject: "مراجعة sentinel (42)",
      "Reply message": "Private complete review sentinel",
      "In-Reply-To · message ID": "parent-sentinel@example.com",
      "References · one message ID per line":
        "first-sentinel@example.com\nparent-sentinel@example.com",
    }
    for (const [label, value] of Object.entries(edits))
      fireEvent.change(screen.getByLabelText(label), { target: { value } })
    const reviewInput = screen.getByLabelText("Reply message")
    changeLocale("en")
    expect(screen.getByLabelText("Reply message")).toBe(reviewInput)
    await waitFor(() => expect(document.documentElement.lang).toBe("en"))
    rerender(<LocaleShell mobile={mobile} />)
    await screen.findByLabelText("Reply message")
    for (const [label, value] of Object.entries(edits))
      expect(screen.getByLabelText(label)).toHaveValue(value)
    expect(screen.getByLabelText("From · inbox identity")).toHaveValue(
      reply.from
    )
    for (const method of [ops.saveDraft, ops.note, ops.approve])
      expect(method).not.toHaveBeenCalled()
    expect(
      storage.mock.calls.every(
        ([key, value]) =>
          key === LANGUAGE_SETTINGS_STORAGE_KEY && !value.includes("sentinel")
      )
    ).toBe(true)
  }
)

it("retains edits after a failed bundle and ignores a superseded locale response", async () => {
  const error = vi.spyOn(console, "error").mockImplementation(() => {})
  const failed = deferred<AbstractIntlMessages>()
  const french = deferred<AbstractIntlMessages>()
  const arabic = deferred<AbstractIntlMessages>()
  vi.mocked(getMessagesForLocale)
    .mockReturnValueOnce(failed.promise)
    .mockReturnValueOnce(french.promise)
    .mockReturnValueOnce(arabic.promise)
  render(<LocaleShell mobile={false} />)
  const input = await openReply()
  fireEvent.change(input, { target: { value: "Still private" } })
  changeLocale("ar")
  await act(async () =>
    failed.reject(new Error("Synthetic unavailable bundle"))
  )
  expect(error).toHaveBeenCalled()
  expect(screen.getByLabelText("Reply message")).toBe(input)
  expect(input).toHaveValue("Still private")
  expect(document.documentElement.lang).toBe("en")
  changeLocale("fr")
  changeLocale("ar")
  await act(async () => arabic.resolve({}))
  await act(async () => french.resolve({}))
  expect(document.documentElement.lang).toBe("ar")
  expect(screen.getByLabelText("Reply message")).toBe(input)
  expect(input).toHaveValue("Still private")
})

it.each(["id", "base_url"] as const)(
  "still isolates colliding account/object IDs when backend %s changes during a locale load",
  async (key) => {
    const connection = {
      id: 1,
      name: "Backend A",
      base_url: "https://a.invalid",
      token: "synthetic",
      headers: [],
      sort_order: 0,
      created_at: "",
      updated_at: "",
    }
    const remote = { connection, expired: false, markExpired: vi.fn() }
    vi.mocked(useRemoteConnection).mockReturnValue(remote)
    const arabic = deferred<AbstractIntlMessages>()
    vi.mocked(getMessagesForLocale).mockReturnValue(arabic.promise)
    const { rerender } = render(<LocaleShell mobile={false} />)
    fireEvent.change(await openReply(), {
      target: { value: "Backend A private edit" },
    })
    changeLocale("ar")
    vi.mocked(useRemoteConnection).mockReturnValue({
      ...remote,
      connection: {
        ...connection,
        [key]: key === "id" ? 2 : "https://b.invalid",
      },
    })
    rerender(<LocaleShell mobile />)
    expect(
      screen.queryByDisplayValue("Backend A private edit")
    ).not.toBeInTheDocument()
    expect(await openReply()).toHaveValue("Saved server body")
    await act(async () => arabic.resolve({}))
    expect(screen.getByLabelText("Reply message")).toHaveValue(
      "Saved server body"
    )
    expect(
      screen.queryByDisplayValue("Backend A private edit")
    ).not.toBeInTheDocument()
  }
)
afterEach(() => vi.restoreAllMocks())
function Shell({ mobile }: { mobile: boolean }) {
  // Mirrors FolderLayoutShell's different parent types, causing an actual
  // unmount/remount of the entire Ops subtree while its provider survives.
  return (
    <WorkbenchRouteProvider>
      {mobile ? (
        <section>
          <OpsPage />
        </section>
      ) : (
        <div>
          <OpsPage />
        </div>
      )}
    </WorkbenchRouteProvider>
  )
}

it("preserves selection, draft, private note and complete review across layout remounts without writing them", async () => {
  const storage = vi.spyOn(Storage.prototype, "setItem")
  vi.spyOn(window, "confirm").mockReturnValue(true)
  const { rerender } = render(<Shell mobile={false} />)
  fireEvent.click(
    await screen.findByRole("button", { name: /Reader Open Request/ })
  )
  fireEvent.change(await screen.findByLabelText("Reply message"), {
    target: { value: "Unsaved draft sentinel" },
  })
  fireEvent.click(screen.getByRole("button", { name: "Private note" }))
  fireEvent.change(screen.getByLabelText("Private note · never emailed"), {
    target: { value: "Private note sentinel" },
  })
  rerender(<Shell mobile />)
  expect(
    await screen.findByLabelText("Private note · never emailed")
  ).toHaveValue("Private note sentinel")
  expect(screen.getByRole("heading", { name: "Request" })).toBeInTheDocument()
  fireEvent.click(screen.getByRole("button", { name: "Reply draft" }))
  expect(screen.getByLabelText("Reply message")).toHaveValue(
    "Unsaved draft sentinel"
  )
  rerender(<Shell mobile={false} />)
  expect(await screen.findByLabelText("Reply message")).toHaveValue(
    "Unsaved draft sentinel"
  )
  fireEvent.click(screen.getByRole("button", { name: "Approvals" }))
  fireEvent.click(
    await screen.findByRole("button", { name: /Re: Request Task/ })
  )
  fireEvent.change(await screen.findByLabelText("Reply message"), {
    target: { value: "Unsaved review sentinel" },
  })
  fireEvent.change(
    screen.getByLabelText("Bcc · hidden from other recipients"),
    { target: { value: "copy@example.com" } }
  )
  rerender(<Shell mobile />)
  expect(await screen.findByLabelText("Reply message")).toHaveValue(
    "Unsaved review sentinel"
  )
  expect(
    screen.getByLabelText("Bcc · hidden from other recipients")
  ).toHaveValue("copy@example.com")
  expect(
    screen.getByRole("heading", { name: "Pending human review" })
  ).toBeInTheDocument()
  for (const method of [ops.saveDraft, ops.note, ops.approve])
    expect(method).not.toHaveBeenCalled()
  expect(
    storage.mock.calls.some((call) =>
      call.some((value) => String(value).includes("sentinel"))
    )
  ).toBe(false)
})

it("drops in-memory edits synchronously when the backend changes, even when object IDs collide", async () => {
  const { rerender } = render(<Shell mobile={false} />)
  fireEvent.click(
    await screen.findByRole("button", { name: /Reader Open Request/ })
  )
  fireEvent.change(await screen.findByLabelText("Reply message"), {
    target: { value: "Backend A private edit" },
  })
  vi.mocked(useRemoteConnection).mockReturnValue({
    connection: {
      id: 2,
      name: "Other backend",
      base_url: "https://other.invalid",
      token: "synthetic",
      headers: [],
      sort_order: 0,
      created_at: "",
      updated_at: "",
    },
    expired: false,
    markExpired: vi.fn(),
  })
  rerender(<Shell mobile />)
  expect(
    screen.queryByDisplayValue("Backend A private edit")
  ).not.toBeInTheDocument()
  fireEvent.click(
    await screen.findByRole("button", { name: /Reader Open Request/ })
  )
  await waitFor(() =>
    expect(screen.getByLabelText("Reply message")).toHaveValue(
      "Saved server body"
    )
  )
})
