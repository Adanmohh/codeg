import {
  act,
  fireEvent,
  render,
  renderHook,
  screen,
  waitFor,
} from "@testing-library/react"
import { beforeEach, describe, expect, it, vi } from "vitest"
import {
  ops,
  type DeliveryStatus,
  type OpsContext,
  type Proposal,
} from "@/lib/ops/api"
import { ReviewCard, proposalStatus } from "./proposals-view"
import { useOpsResource } from "./use-ops-resource"
import {
  WorkbenchRouteProvider,
  useWorkbenchRoute,
} from "@/contexts/workbench-route-context"

vi.mock("@/lib/ops/api", () => ({
  ops: {
    emailStatus: vi.fn(),
    approve: vi.fn(),
    deny: vi.fn(),
    reconcileReceipt: vi.fn(),
  },
}))
const context: OpsContext = {
  accountId: 1,
  operator: "operator:http",
  inboxes: [],
  emailTransport: "per_inbox",
  transportMessage: "Every send requires human approval",
}
const proposal: Proposal = {
  id: 1,
  taskId: 4,
  runSeq: 2,
  inboxId: 3,
  conversationId: 5,
  status: "pending",
  stale: false,
  reason: null,
  createdAt: "2026-09-08T08:00:00Z",
  delivery: null,
  payload: {
    draftId: 6,
    draftRevision: 1,
    reply: {
      inboxId: 3,
      conversationId: 5,
      from: "support@example.com",
      to: ["reader@example.com"],
      cc: [],
      bcc: ["copy@example.com"],
      subject: "Re: a question",
      text: "Original",
      inReplyTo: "parent@example.com",
      references: ["parent@example.com"],
    },
  },
}
const props = () => ({
  proposal,
  context,
  onDirty: vi.fn(),
  onThread: vi.fn(),
  onResolved: vi.fn(),
})
beforeEach(() => {
  vi.clearAllMocks()
  vi.mocked(ops.emailStatus).mockResolvedValue({
    inboxId: 3,
    configured: true,
    lastPullAt: null,
    lastPullStatus: null,
    lastPullError: null,
  })
})

describe("Ops review component (mocked facade, not E2E)", () => {
  it("shows every recipient and binds an edited approval to the original snapshot", async () => {
    let complete!: (value: Proposal) => void
    vi.mocked(ops.approve).mockImplementation(
      () =>
        new Promise((resolve) => {
          complete = resolve
        })
    )
    const callbacks = props()
    render(<ReviewCard {...callbacks} />)
    expect(
      screen.getByLabelText("Bcc · hidden from other recipients")
    ).toHaveValue("copy@example.com")
    fireEvent.change(screen.getByLabelText("To"), {
      target: { value: "replacement@example.com, second@example.com" },
    })
    fireEvent.change(screen.getByLabelText("Reply message"), {
      target: { value: "Human edit" },
    })
    const button = screen.getByRole("button", {
      name: "Approve and send reply",
    })
    await waitFor(() => expect(button).toBeEnabled())
    fireEvent.click(button)
    fireEvent.click(button)
    expect(ops.approve).toHaveBeenCalledTimes(1)
    expect(ops.approve).toHaveBeenCalledWith(1, proposal.payload, {
      ...proposal.payload,
      reply: {
        ...proposal.payload!.reply,
        to: ["replacement@example.com", "second@example.com"],
        text: "Human edit",
      },
    })
    expect(callbacks.onResolved).not.toHaveBeenCalled()
    await act(async () =>
      complete({ ...proposal, status: "approved", payload: null })
    )
    expect(callbacks.onResolved).toHaveBeenCalledOnce()
  })
  it("missing configuration and stale payloads disable sending while denial remains available", async () => {
    vi.mocked(ops.emailStatus).mockResolvedValue({
      inboxId: 3,
      configured: false,
      lastPullAt: null,
      lastPullStatus: null,
      lastPullError: null,
    })
    vi.mocked(ops.deny).mockResolvedValue({
      ...proposal,
      status: "denied",
      payload: null,
    })
    render(
      <ReviewCard
        {...props()}
        proposal={{
          ...proposal,
          stale: true,
          reason: "A newer draft was saved",
        }}
      />
    )
    await screen.findByText(/Open the original thread and connect/)
    expect(
      screen.getByRole("button", { name: "Approve and send reply" })
    ).toBeDisabled()
    fireEvent.click(screen.getByRole("button", { name: "Deny proposal" }))
    await waitFor(() =>
      expect(ops.deny).toHaveBeenCalledWith(1, proposal.payload)
    )
    expect(ops.approve).not.toHaveBeenCalled()
  })
  it("approved and unknown records never claim sent or offer another send", async () => {
    const unknown: Proposal = {
      ...proposal,
      status: "approved",
      payload: null,
      delivery: {
        id: 1,
        proposalId: 1,
        status: "unknown",
        messageId: "x@example.com",
        providerId: null,
        error: "Do not resend",
        updatedAt: proposal.createdAt,
      },
    }
    render(<ReviewCard {...props()} proposal={unknown} />)
    await screen.findByText("Delivery unconfirmed · do not resend")
    expect(
      screen.queryByRole("button", { name: /Approve and send/ })
    ).not.toBeInTheDocument()
    expect(proposalStatus({ ...unknown, delivery: null })).toBe(
      "Approved · delivery unconfirmed"
    )
    expect(
      proposalStatus({
        ...unknown,
        delivery: {
          ...unknown.delivery!,
          status: "sent",
          providerId: "receipt",
        },
      })
    ).toBe("Sent · provider accepted")
  })
  it.each([
    ["sent", /Provider acceptance does not confirm recipient delivery/],
    ["receipt_recorded", /it does not send again/],
    ["failed", /The provider did not accept this reply/],
    ["not_sent", /This attempt stopped before sending/],
    ["unknown", /Reconcile the provider outcome first/],
    ["reserved", /Do not resend this proposal/],
    ["sending", /Do not resend this proposal/],
  ] as [DeliveryStatus["status"], RegExp][])(
    "explains actual %s receipt state without a contradictory decision disclaimer",
    async (status, explanation) => {
      const terminal: Proposal = {
        ...proposal,
        status: "approved",
        payload: null,
        delivery: {
          id: 1,
          proposalId: 1,
          status,
          messageId: "synthetic@example.com",
          providerId:
            status === "sent" || status === "receipt_recorded"
              ? "synthetic-receipt"
              : null,
          error: status === "sent" ? null : "Synthetic backend detail",
          updatedAt: proposal.createdAt,
        },
      }
      render(<ReviewCard {...props()} proposal={terminal} />)
      await screen.findByText(explanation)
      expect(
        screen.queryByText(
          /No delivery receipt is implied|This decision has no recorded delivery receipt/
        )
      ).not.toBeInTheDocument()
      expect(
        screen.queryByRole("button", { name: /Approve and send|Deny proposal/ })
      ).not.toBeInTheDocument()
      expect(
        screen.queryByRole("button", { name: "Finish recording receipt" }) !==
          null
      ).toBe(status === "receipt_recorded")
      expect(ops.approve).not.toHaveBeenCalled()
    }
  )
  it("keeps denial and an approval without a receipt distinct", async () => {
    const { rerender } = render(
      <ReviewCard
        {...props()}
        proposal={{ ...proposal, status: "denied", payload: null }}
      />
    )
    await screen.findByRole("heading", { name: "Denied · not sent" })
    expect(
      screen.getByText(/private reply content has been redacted/)
    ).toBeInTheDocument()
    rerender(
      <ReviewCard
        {...props()}
        proposal={{ ...proposal, status: "approved", payload: null }}
      />
    )
    await screen.findByRole("heading", {
      name: "Approved · delivery unconfirmed",
    })
    expect(
      screen.getByText(/This decision has no recorded delivery receipt/)
    ).toBeInTheDocument()
    expect(
      screen.queryByRole("button", {
        name: /Approve and send|Finish recording/,
      })
    ).not.toBeInTheDocument()
  })
  it("finishes recording through the receipt-only callback once, without approving or sending again", async () => {
    let finish!: (value: DeliveryStatus) => void
    vi.mocked(ops.reconcileReceipt).mockImplementation(
      () =>
        new Promise((resolve) => {
          finish = resolve
        })
    )
    const callbacks = props()
    const recorded: Proposal = {
      ...proposal,
      status: "approved",
      payload: null,
      delivery: {
        id: 1,
        proposalId: 1,
        status: "receipt_recorded",
        messageId: "synthetic@example.com",
        providerId: "synthetic-receipt",
        error: "Synthetic local insert failed",
        updatedAt: proposal.createdAt,
      },
    }
    render(<ReviewCard {...callbacks} proposal={recorded} />)
    const button = await screen.findByRole("button", {
      name: "Finish recording receipt",
    })
    fireEvent.click(button)
    fireEvent.click(button)
    expect(ops.reconcileReceipt).toHaveBeenCalledTimes(1)
    expect(ops.reconcileReceipt).toHaveBeenCalledWith(proposal.id)
    expect(callbacks.onResolved).not.toHaveBeenCalled()
    await act(async () =>
      finish({ ...recorded.delivery!, status: "sent", error: null })
    )
    expect(callbacks.onResolved).toHaveBeenCalledOnce()
    expect(ops.approve).not.toHaveBeenCalled()
    expect(ops.deny).not.toHaveBeenCalled()
  })
})

it("a changed inbox key never paints old data or a late earlier response", async () => {
  let finishOld!: (value: string) => void
  let finishNew!: (value: string) => void
  const oldLoad = () =>
    new Promise<string>((resolve) => {
      finishOld = resolve
    })
  const newLoad = () =>
    new Promise<string>((resolve) => {
      finishNew = resolve
    })
  const { result, rerender } = renderHook(
    ({ id, load }) => useOpsResource(id, load),
    { initialProps: { id: "old", load: oldLoad } }
  )
  rerender({ id: "new", load: newLoad })
  await act(async () => finishOld("private old inbox"))
  expect(result.current.data).toBeUndefined()
  await act(async () => finishNew("current inbox"))
  expect(result.current.data).toBe("current inbox")
})

it("workbench sidebar and conversation navigation honor and release the Ops leave guard", () => {
  const { result } = renderHook(useWorkbenchRoute, {
    wrapper: WorkbenchRouteProvider,
  })
  act(() => result.current.setRoute("ops"))
  const guard = vi.fn(() => false)
  const release = result.current.registerLeaveGuard(guard)
  act(() => result.current.setRoute("tasks"))
  expect(result.current.routeId).toBe("ops")
  act(() => result.current.openConversations())
  expect(result.current.routeId).toBe("ops")
  expect(guard).toHaveBeenCalledWith("conversations")
  release()
  act(() => result.current.setRoute("tasks"))
  expect(result.current.routeId).toBe("tasks")
})
