import type { BusinessDomain } from "./presentation"
import type {
  Assignment,
  TaskDetail,
  TaskFields,
  TaskOperations,
} from "./tasks"

// Closed contract670af9ca + access18be55ed. IDs and revisions stay opaque;
// identity, provider credentials and source epochs are never inferred by UI.
export const INTAKE_REASONS = [
  "binding_missing",
  "binding_disabled",
  "credential_unavailable",
  "binding_unavailable",
  "source_expired",
  "rebase_required",
  "import_busy",
  "retry_later",
  "source_denied",
  "publication_not_allowed",
  "unsupported_schema",
  "provider_unavailable",
  "request_timeout",
] as const
export type IntakeReason = (typeof INTAKE_REASONS)[number]
export function intakeReason(
  key: string | null | undefined
): IntakeReason | undefined {
  return INTAKE_REASONS.find((reason) => key === `business.intake.${reason}`)
}
export type SourceKind = "fireflies" | "email" | "hafidh_testflight"
export type Disclosure = "fresh" | "metadata_only"
export type CandidateState = "pending" | "accepted" | "linked" | "discarded"
export type PreparedTask = TaskFields & Assignment
export interface BindingCapabilities {
  read: boolean
  import: boolean
  triage: boolean
  publicationDomains: BusinessDomain[]
}
export interface BindingSummary {
  id: string
  kind: SourceKind
  label: string
  domain: BusinessDomain
  revision: number
  accessEpoch: number
  enabled: boolean
  credentialState: "missing" | "present"
  capabilities: BindingCapabilities
}
export type ResourceIdentity =
  | { kind: "fireflies"; providerUserId: string; mine: boolean }
  | {
      kind: "email"
      accountId: number
      inboxId: number
      configurationIdentity: string
    }
  | {
      kind: "hafidh_testflight"
      accountId: number
      productId: string
      configurationIdentity: string
    }
export interface BindingAdmin extends Omit<BindingSummary, "capabilities"> {
  sourceOwnerId: string
  publicationDomains: BusinessDomain[]
  retainedTaskText: boolean
  resource: ResourceIdentity
}
export interface BindingView {
  binding: BindingSummary
  admin: BindingAdmin | null
}
export interface Page<T> {
  items: T[]
  page: number
  hasMore: boolean
}
export interface BindingList extends Page<BindingView> {
  canManageSetup: boolean
}
export type GrantScope = "binding_current_and_future_sources"
export interface Grant {
  id: string
  bindingId: string
  memberId: string
  revision: number
  state: "active" | "revoked"
  scope: GrantScope
  read: boolean
  import: boolean
  triage: boolean
  publicationDomains: BusinessDomain[]
  expiresAt: string | null
}
export type SourceSetup =
  | { kind: "fireflies"; apiKey: string }
  | { kind: "email"; inboxId: number }
  | { kind: "hafidh_testflight"; productId: string }
export interface SourceSummary {
  id: string
  bindingId: string
  kind: SourceKind
  title: string
  revision: number | null
  observedAt: string | null
  accessValidUntil: string | null
  access: "unverified" | "fresh" | "expired" | "denied"
  content: "missing" | "available" | "unsupported"
  summary: "missing" | "empty" | "available" | "unsupported"
  providerSummaryStatus: string | null
  requiresRefresh: boolean
}
export interface Passage {
  id: string
  sourceRevision: number
  kind: "sentence" | "summary" | "email_message" | "feedback"
  text: string
  index: number | null
  start: number | null
  end: number | null
}
export interface SourceDetail {
  source: SourceSummary
  disclosure: Disclosure
  passages: Passage[]
  candidateCount: number
}
export interface CandidateCapabilities {
  select: boolean
  edit: boolean
  accept: boolean
  link: boolean
  discard: boolean
  publicationDomains: BusinessDomain[]
}
export interface Candidate {
  id: string
  sourceId: string
  revision: number
  sourceRevision: number
  state: CandidateState
  origin: "source_review" | "human_selection"
  requiresRebase: boolean
  disclosure: Disclosure
  hasPreparedDraft: boolean
  draft: PreparedTask | null
  ownerSuggestion: string | null
  dueSuggestion: string | null
  capabilities: CandidateCapabilities
}
export interface CandidateDetail {
  candidate: Candidate
  passages: Passage[]
  source: SourceSummary
  decision: Decision | null
}
export interface Decision {
  id: string
  candidateId: string
  fromRevision: number
  sourceRevision: number
  kind: CandidateState
  actorId: string
  task:
    | { state: "none" | "restricted" }
    | { state: "accessible"; taskId: string; taskRevision: number }
  createdAt: string
}
export interface DecisionResult {
  decision: Decision
  task: TaskDetail
  replayed: boolean
}
export interface IntakeImport {
  id: string
  bindingId: string
  revision: number
  state: "queued" | "running" | "waiting" | "complete" | "failed" | "cancelled"
  coverage: "not_started" | "partial" | "bounded_end" | "capped"
  discovered: number
  completed: number
  failed: number
  nextAttemptAt: string | null
  errorCode: IntakeReason | null
  capabilities: { advance: boolean; cancel: boolean }
}
export type ImportSelection =
  | { kind: "window"; fromDate: string; toDate: string }
  | { kind: "record"; sourceId: string }
export type LegacyRef =
  | { kind: "email"; conversationId: number; messageId: number }
  | { kind: "hafidh_testflight"; ulid: string }
export interface SourceLinkView {
  linkId: string
  accessible: boolean
  source: SourceSummary | null
}
type Op<I, R> = { input: I; result: R }
type Operation = { operationId: string }
type BindingInput = { bindingId: string }
type BindingRevision = Operation & BindingInput & { expectedRevision: number }
type CandidateRevision = Operation & {
  candidateId: string
  expectedRevision: number
}
type SelectionInput = CandidateRevision & {
  expectedSourceRevision: number
  passageIds: string[]
}
type PublicationInput = CandidateRevision & {
  expectedSourceRevision: number
  publishToDomain: BusinessDomain
}
type ImportRevision = Operation & { importId: string; expectedRevision: number }
type GrantResult = { binding: BindingAdmin; grant: Grant }
export interface IntakeOperations {
  "bindings/list": Op<{ page?: number }, BindingList>
  "bindings/status": Op<BindingInput, BindingView>
  "bindings/create": Op<
    Operation & {
      label: string
      domain: BusinessDomain
      sourceOwnerId: string
      source: SourceSetup
      publicationDomains: BusinessDomain[]
      retainedTaskText: boolean
    },
    BindingAdmin
  >
  "bindings/update": Op<
    BindingRevision & {
      label: string
      enabled: boolean
      publicationDomains: BusinessDomain[]
      retainedTaskText: boolean
      credential?: { kind: "fireflies"; apiKey: string }
    },
    BindingAdmin
  >
  "bindings/disable": Op<BindingRevision, BindingAdmin>
  "grants/list": Op<BindingInput & { page?: number }, Page<Grant>>
  "grants/upsert": Op<
    Operation &
      BindingInput & {
        expectedBindingRevision: number
        memberId: string
        expectedGrantRevision: number | null
        scope: GrantScope
        read: boolean
        import: boolean
        triage: boolean
        publicationDomains: BusinessDomain[]
        expiresAt: string | null
      },
    GrantResult
  >
  "grants/revoke": Op<
    Operation &
      BindingInput & {
        expectedBindingRevision: number
        grantId: string
        expectedGrantRevision: number
      },
    GrantResult
  >
  "sources/list": Op<BindingInput & { page?: number }, Page<SourceSummary>>
  "sources/get": Op<{ sourceId: string }, SourceDetail>
  "imports/start": Op<
    Operation & BindingInput & { selection: ImportSelection },
    IntakeImport
  >
  "imports/capture": Op<
    Operation & BindingInput & { ref: LegacyRef },
    IntakeImport
  >
  "imports/list": Op<
    BindingInput & { view?: "unfinished" | "all"; page?: number },
    Page<IntakeImport>
  >
  "imports/get": Op<{ importId: string }, IntakeImport>
  "imports/advance": Op<ImportRevision, IntakeImport>
  "imports/cancel": Op<ImportRevision, IntakeImport>
  "candidates/list": Op<
    { sourceId: string; state?: CandidateState; page?: number },
    Page<Candidate>
  >
  "candidates/get": Op<{ candidateId: string }, CandidateDetail>
  "candidates/create": Op<
    Operation & {
      sourceId: string
      expectedSourceRevision: number
      passageIds: string[]
    },
    Candidate
  >
  "candidates/select": Op<SelectionInput, CandidateDetail>
  "candidates/edit": Op<
    SelectionInput & {
      task: TaskOperations["create"]["input"]
      ownerSuggestion?: string | null
      dueSuggestion?: string | null
    },
    CandidateDetail
  >
  "candidates/accept": Op<PublicationInput, DecisionResult>
  "candidates/link": Op<
    PublicationInput & { taskId: string; expectedTaskRevision: number },
    DecisionResult
  >
  "candidates/discard": Op<CandidateRevision, Decision>
  "tasks/sources": Op<{ taskId: string }, { links: SourceLinkView[] }>
}

export const intakeCommands = {
  "bindings/list": "business_intake_bindings_list",
  "bindings/status": "business_intake_bindings_status",
  "bindings/create": "business_intake_bindings_create",
  "bindings/update": "business_intake_bindings_update",
  "bindings/disable": "business_intake_bindings_disable",
  "grants/list": "business_intake_grants_list",
  "grants/upsert": "business_intake_grants_upsert",
  "grants/revoke": "business_intake_grants_revoke",
  "sources/list": "business_intake_sources_list",
  "sources/get": "business_intake_sources_get",
  "imports/start": "business_intake_imports_start",
  "imports/capture": "business_intake_imports_capture",
  "imports/list": "business_intake_imports_list",
  "imports/get": "business_intake_imports_get",
  "imports/advance": "business_intake_imports_advance",
  "imports/cancel": "business_intake_imports_cancel",
  "candidates/list": "business_intake_candidates_list",
  "candidates/get": "business_intake_candidates_get",
  "candidates/create": "business_intake_candidates_create",
  "candidates/select": "business_intake_candidates_select",
  "candidates/edit": "business_intake_candidates_edit",
  "candidates/accept": "business_intake_candidates_accept",
  "candidates/link": "business_intake_candidates_link",
  "candidates/discard": "business_intake_candidates_discard",
  "tasks/sources": "business_intake_tasks_sources",
} as const satisfies Record<keyof IntakeOperations, string>

// A client clock may only remove disclosure. It cannot mint upstream freshness.
export function sourceIsFresh(
  source: SourceSummary,
  now = Date.now()
): boolean {
  return (
    source.access === "fresh" &&
    !source.requiresRefresh &&
    source.revision !== null &&
    source.accessValidUntil !== null &&
    Date.parse(source.accessValidUntil) > now
  )
}
export function validPassages(
  passages: Passage[],
  ids: string[],
  revision: number | null
): boolean {
  if (
    revision === null ||
    ids.length < 1 ||
    ids.length > 20 ||
    new Set(ids).size !== ids.length
  )
    return false
  let length = 0
  for (const id of ids) {
    const passage = passages.find(
      (entry) => entry.id === id && entry.sourceRevision === revision
    )
    if (!passage) return false
    length += Array.from(passage.text).length
  }
  return length <= 20000
}
