import { getTransport } from "@/lib/transport"
import type { WorkTask } from "@/lib/types"

export interface Inbox {
  id: number
  name: string
  email: string
}
export interface OpsContext {
  accountId: number
  operator: string
  inboxes: Inbox[]
  emailTransport: "per_inbox"
  transportMessage: string
}
export interface ThreadKey {
  inboxId: number
  conversationId: number
}
export interface Reply extends ThreadKey {
  from: string
  to: string[]
  cc: string[]
  bcc: string[]
  subject: string
  text: string
  inReplyTo: string | null
  references: string[]
}
export interface ReplyPayload {
  draftId: number
  draftRevision: number
  reply: Reply
}
export interface Draft {
  id: number
  revision: number
  reply: Reply
  updatedAt: string
}
export interface Ticket {
  id: number
  inboxId: number
  subject: string
  contact: string
  status: number
  updatedAt: string
}
export interface TicketPage {
  items: Ticket[]
  hasMore: boolean
}
export interface OpsMessage {
  id: number
  content: string
  private: boolean
  outgoing: boolean
  author: string
  status: number
  sourceId: string | null
  createdAt: string
}
export interface Thread {
  ticket: Ticket
  contactEmail: string
  messages: OpsMessage[]
  draft: Draft | null
  suggestedReply: Reply
}
export interface Proposal {
  id: number
  taskId: number
  runSeq: number
  inboxId: number
  conversationId: number
  status: string
  stale: boolean
  reason: string | null
  payload: ReplyPayload | null
  createdAt: string
  delivery: DeliveryStatus | null
}
export interface EmailStatus {
  inboxId: number
  configured: boolean
  lastPullAt: string | null
  lastPullStatus: string | null
  lastPullError: string | null
}
export interface DeliveryStatus {
  id: number
  proposalId: number
  status:
    | "reserved"
    | "sending"
    | "not_sent"
    | "failed"
    | "unknown"
    | "receipt_recorded"
    | "sent"
  messageId: string
  providerId: string | null
  error: string | null
  updatedAt: string
}
export interface Morning {
  tasks: WorkTask[]
  tickets: Ticket[]
  proposals: Proposal[]
}

// Inherited transport owns credentials. Ops never reads or forwards a token.
const call = <T>(command: string, input?: unknown) =>
  getTransport().call<T>(command, input === undefined ? {} : { input })

export const ops = {
  emailStatus: (inboxId: number) =>
    call<EmailStatus>("ops_email_status", { inboxId }),
  configureEmail: (inboxId: number, apiKey: string) =>
    call<EmailStatus>("ops_email_configure", { inboxId, apiKey }),
  disconnectEmail: (inboxId: number) =>
    call<EmailStatus>("ops_email_disconnect", { inboxId }),
  pullEmail: (inboxId: number) =>
    getTransport().call<{ inserted: number; duplicates: number }>(
      "ops_email_pull",
      { input: { inboxId } },
      { timeoutMs: 65000 }
    ),
  reconcileReceipt: (id: number) =>
    call<DeliveryStatus>("ops_email_reconcile_receipt", { id }),
  context: () => call<OpsContext>("ops_context"),
  createInbox: (input: { name: string; email: string }) =>
    call<Inbox>("ops_inbox_create", input),
  tickets: (input: { inboxId: number; status?: number; page: number }) =>
    call<TicketPage>("ops_tickets", input),
  thread: (input: ThreadKey) => call<Thread>("ops_thread", input),
  note: (input: ThreadKey & { content: string }) =>
    call<OpsMessage>("ops_note_add", input),
  saveDraft: (input: ThreadKey & { expectedRevision: number; reply: Reply }) =>
    call<Draft>("ops_draft_save", input),
  proposals: () => call<Proposal[]>("ops_proposals"),
  proposal: (id: number) => call<Proposal>("ops_proposal_get", { id }),
  deny: (id: number, expectedPayload: ReplyPayload) =>
    call<Proposal>("ops_proposal_deny", { id, expectedPayload }),
  approve: (
    id: number,
    expectedPayload: ReplyPayload,
    approvedPayload: ReplyPayload
  ) =>
    call<Proposal>("ops_proposal_approve", {
      id,
      expectedPayload,
      approvedPayload,
    }),
  morning: () => call<Morning>("ops_morning"),
}
