import { getTransport } from "@/lib/transport"
import { extractAppCommandError } from "@/lib/app-error"
import type {
  AttachInput,
  ConfigureInput,
  Detail,
  Draft,
  Listing,
  PreparedIssue,
  SaveInput,
  Source,
  Status,
} from "./types"

function call<T>(command: string, input: object = {}): Promise<T> {
  return getTransport().call<T>(
    `ops_intake_${command}`,
    { input },
    { timeoutMs: 60000 }
  )
}
export const intake = {
  status: () => call<Status>("status"),
  configure: (input: ConfigureInput) => call<Status>("configure", input),
  list: (product_id: string, cursor: string | null = null) =>
    call<Listing>("list", { product_id, cursor }),
  detail: (source: Source) => call<Detail>("detail", source),
  refresh: (source: Source) => call<Detail>("refresh", source),
  save: (input: SaveInput) => call<Draft>("save", input),
  attach: (input: AttachInput) => call<Draft>("attach", input),
  prepare: (source: Source, expected_revision: number, task_id: number) =>
    call<Draft>("prepare", { source, expected_revision, task_id }),
  approve: (
    source: Source,
    proposal_id: number,
    expected_payload: PreparedIssue,
    approved_payload: PreparedIssue
  ) =>
    call<Detail>("approve", {
      source,
      proposal_id,
      expected_payload,
      approved_payload,
    }),
  deny: (
    source: Source,
    proposal_id: number,
    expected_payload: PreparedIssue
  ) => call<Detail>("deny", { source, proposal_id, expected_payload }),
  reconcile: (source: Source) => call<Detail>("reconcile", source),
  fix: (source: Source) => call<Detail>("fix", source),
}
const errors: Record<string, string> = {
  not_configured:
    "Complete the product, repository and credential settings, then try again. A GitHub App key is required to file.",
  adapter_missing:
    "The Hafidh intake adapter is not installed for this Desk host. Install its pinned package in a dedicated Python environment and set CODEG_INTAKE_PYTHON on the host.",
  access_denied:
    "Access is unavailable. Check that this product is enabled and its Hafidh credential has permission. Refresh after correcting access.",
  invalid_input:
    "Check the entered values. Outward text must be bounded and contain no credentials, contact details, private paths or URLs.",
  invalid_evidence:
    "Proof needs sanitized UTF-8 content up to 8 KiB containing the exact evidence summary. Check the build number, capture time and session ID. URLs and private paths are not proof.",
  stale_source:
    "Refresh this report from Hafidh before continuing. Previous freshness cannot be reused after a failed read.",
  source_unavailable:
    "Hafidh could not be revalidated. Check access and retry the read; this report has not been marked fresh.",
  conflict:
    "This draft, task or filing changed. Reload its status before continuing. An unknown filing must never be retried.",
  storage_unavailable:
    "Local storage could not confirm the operation. Reload status before trying another change.",
  task_required:
    "Choose an active task in the configured project. Task creation and starting remain in the Tasks view.",
  severity_required:
    "Confirm the severity suggestion or choose a different severity before preparing the issue.",
}
export function intakeError(error: unknown): string {
  const parsed = extractAppCommandError(error)
  return (
    (parsed && errors[parsed.message]) ||
    "The Desk could not complete this request. Check your connection and reload status before continuing."
  )
}
