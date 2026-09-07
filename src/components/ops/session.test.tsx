import { fireEvent, render, screen, waitFor } from "@testing-library/react"
import { afterEach, beforeEach, expect, it, vi } from "vitest"
import type { ReactNode } from "react"
import { OpsPage } from "./ops-page"
import { WorkbenchRouteProvider } from "@/contexts/workbench-route-context"
import { useRemoteConnection } from "@/contexts/remote-connection-context"
import { ops, type OpsContext, type Proposal, type Thread } from "@/lib/ops/api"

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
