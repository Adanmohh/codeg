// Explicit mirrors of ops_intake_host/types.rs and accepted ops_intake/types.rs.
export type Severity = "high" | "medium" | "low"
export type EvidenceField = "build" | "screen" | "reciter" | "log"
export interface Source {
  product_id: string
  ulid: string
}
export interface SourceRef extends Source {
  source: "testflight" | "in_app"
}
export interface Binding {
  product_id: string
  folder_id: number
  app_id: string
  installation_id: number
  repository_id: number
  full_name: string
  enabled: boolean
}
export interface Product {
  binding: Binding
  origin: string
  intake_credential_present: boolean
  app_key_present: boolean
}
export interface Status {
  products: Product[]
  folders: { id: number; name: string }[]
  adapter_installed: boolean
  in_app_available: false
}
export interface IntakeRecord {
  source_ref: SourceRef & { external_id: string | null }
  source_revision: string
  fetched_at: string
  title: string
  description: string
  source_status: string
  submitted_at: string | null
  source_updated_at: string
  device: string | null
  os_version: string | null
  app_version: null
  build_number: string | null
  platform: string | null
  locale: string | null
  screenshots: {
    ulid: string
    content_type: string
    width: number | null
    height: number | null
  }[]
  triage: {
    seeded_tags: string[]
    seeded_severity: Severity
    source_tags: string[]
    source_severity: Severity
  }
}
export interface Snapshot {
  record: IntakeRecord
  verified_at: number | null
  error: string | null
}
export interface Listing {
  records: Snapshot[]
  next_cursor: string | null
  scan_complete: boolean
}
export interface EvidenceRef {
  artifact_id: string
  sha256: string
  value: string
}
export interface Proof {
  proof: EvidenceRef
  captured_at: string | null
  session_ulid: string | null
}
export interface PreparedIssue {
  draft: {
    schema_version: number
    template_version: string
    task_id: number
    run_seq: number
    source_ref: SourceRef
    source_revision: string
    title: string
    summary: string
    labels: string[]
    evidence: Record<EvidenceField, EvidenceRef>
  }
  repository_id: number
  repository: string
  binding_digest: string
  outgoing: { title: string; body: string; labels: string[] }
}
export interface Draft {
  id: string
  revision: number
  source_revision: string
  title: string
  summary: string
  labels: string[]
  confirmed_severity: Severity | null
  proofs: Partial<Record<EvidenceField, Proof>>
  prepared: PreparedIssue | null
}
export interface Receipt {
  attempt_id: number
  proposal_id: number
  state: "unknown" | "failed" | "created"
  issue: {
    id: number
    number: number
    html_url: string
    labels: string[]
    labels_match: boolean
  } | null
  error_code: string | null
  retry_after: number | null
}
export interface Proposal {
  id: number
  status: string
  payload: PreparedIssue | null
  stale: boolean
}
export interface Detail {
  snapshot: Snapshot
  draft: Draft
  tasks: { id: number; title: string; run_seq: number }[]
  proposals: Proposal[]
  receipt: Receipt | null
  handoff_unknown: boolean
  fix_task_id: number | null
}
// Write-only operator credential input, never included in a response or tool.
export interface ConfigureInput {
  binding: Binding
  origin: string
  intake_bearer: string | null
  app_private_key: string | null
}
export interface SaveInput {
  source: Source
  expected_revision: number
  title: string
  summary: string
  labels: string[]
  confirmed_severity: Severity | null
}
export interface AttachInput {
  source: Source
  expected_revision: number
  field: EvidenceField
  value: string
  content: string
  captured_at: string | null
  session_ulid: string | null
}
