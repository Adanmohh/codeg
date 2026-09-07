"use client"

import { useOpsSessionState } from "@/components/ops/session"
import type {
  Detail,
  Draft,
  EvidenceField,
  Source,
} from "@/lib/ops-intake/types"
import { intake } from "@/lib/ops-intake/api"
import { Action, Choice, Field, Notice, Panel, TextField } from "./ui"

export const fields: EvidenceField[] = ["build", "screen", "reciter", "log"]
export function EvidenceEditor({
  source,
  detail,
  stale,
  busy,
  run,
  update,
}: {
  source: Source
  detail: Detail
  stale: boolean
  busy: boolean
  run: (action: () => Promise<void>) => void
  update: (draft: Draft) => void
}) {
  const key = `intake:evidence:${detail.draft.id}:${detail.draft.source_revision}`
  const [field, setField] = useOpsSessionState<EvidenceField>(
    `${key}:field`,
    "build"
  )
  const [values, setValues] = useOpsSessionState(`${key}:values`, {
    build: "",
    screen: "",
    reciter: "",
    log: "",
  })
  const [contents, setContents] = useOpsSessionState(`${key}:content`, {
    build: "",
    screen: "",
    reciter: "",
    log: "",
  })
  const [captured, setCaptured] = useOpsSessionState(`${key}:captured`, "")
  const [session, setSession] = useOpsSessionState(`${key}:session`, "")
  const missing = fields.filter((f) => !detail.draft.proofs[f])
  const locked =
    detail.proposals.some((p) => p.status === "pending") ||
    detail.handoff_unknown ||
    detail.receipt?.state === "unknown" ||
    detail.receipt?.state === "created"
  return (
    <Panel title="Required evidence">
      <ul
        aria-label="Evidence checklist"
        className="grid grid-cols-2 gap-2 sm:grid-cols-4"
      >
        {fields.map((f) => (
          <li key={f} className="border-border rounded-lg border p-3 text-sm">
            <span className="font-medium capitalize">{f}</span>
            <span className="text-muted-foreground mt-1 block">
              {detail.draft.proofs[f] ? "Attached" : "Required"}
            </span>
          </li>
        ))}
      </ul>
      {missing.length > 0 && (
        <p className="text-sm">
          Cannot prepare or file: add {missing.join(", ")} evidence.
        </p>
      )}
      {stale && (
        <Notice>
          Refresh this report before attaching proof. The host must verify its
          current source revision.
        </Notice>
      )}
      <form
        className="grid gap-4"
        onSubmit={(e) => {
          e.preventDefault()
          run(async () => {
            const draft = await intake.attach({
              source,
              expected_revision: detail.draft.revision,
              field,
              value: values[field],
              content: contents[field],
              captured_at: captured || null,
              session_ulid: field === "log" ? session || null : null,
            })
            update(draft)
            setContents((previous) => ({ ...previous, [field]: "" }))
          })
        }}
      >
        <fieldset
          disabled={busy || stale || locked}
          className="grid min-w-0 gap-4"
        >
          <Choice
            label="Evidence type"
            value={field}
            onChange={(e) => setField(e.target.value as EvidenceField)}
          >
            {fields.map((f) => (
              <option key={f} value={f}>
                {f[0].toUpperCase() + f.slice(1)}
              </option>
            ))}
          </Choice>
          {field === "build" && detail.snapshot.record.build_number && (
            <p className="text-muted-foreground text-sm">
              TestFlight reports build{" "}
              <strong className="text-foreground">
                {detail.snapshot.record.build_number}
              </strong>
              . Confirm that build in the proof content.
            </p>
          )}
          <Field
            label={`${field[0].toUpperCase() + field.slice(1)} evidence summary`}
            value={values[field]}
            required
            maxLength={field === "log" ? 8192 : 300}
            onChange={(e) =>
              setValues((previous) => ({
                ...previous,
                [field]: e.target.value,
              }))
            }
          />
          <TextField
            label="Sanitized proof content"
            value={contents[field]}
            required
            maxLength={8192}
            onChange={(e) =>
              setContents((previous) => ({
                ...previous,
                [field]: e.target.value,
              }))
            }
            hint="Paste reviewed UTF-8 text, up to 8 KiB, including the exact summary above. Remove names, contact details, credentials and private paths. A URL, screenshot or session ID alone is not a diagnostic log."
          />
          <Field
            label="Captured at (optional)"
            value={captured}
            onChange={(e) => setCaptured(e.target.value)}
            hint="An actual ISO timestamp, including timezone, when known. Leave blank otherwise."
          />
          {field === "log" && (
            <Field
              label="Recitation session ID (if applicable)"
              value={session}
              onChange={(e) => setSession(e.target.value)}
              maxLength={26}
              hint="The actual session ULID for a session diagnostic. Leave blank for a local diagnostic."
            />
          )}
          <Action type="submit" className="justify-self-start">
            {detail.draft.proofs[field]
              ? "Replace reviewed proof"
              : "Attach reviewed proof"}
          </Action>
        </fieldset>
      </form>
      {fields.some((f) => detail.draft.proofs[f]) && (
        <dl className="grid gap-3 text-sm">
          {fields.map((f) => {
            const p = detail.draft.proofs[f]
            return p ? (
              <div key={f} className="border-border min-w-0 border-t pt-3">
                <dt className="font-medium capitalize">{f}</dt>
                <dd className="mt-1 whitespace-pre-wrap break-words">
                  {p.proof.value}
                </dd>
                {p.session_ulid && (
                  <dd className="mt-1 break-all">Session: {p.session_ulid}</dd>
                )}
                <dd className="text-muted-foreground mt-2 break-all text-xs">
                  SHA-256 {p.proof.sha256}
                </dd>
              </div>
            ) : null
          })}
        </dl>
      )}
    </Panel>
  )
}
