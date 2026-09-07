"use client"

import { useOpsSessionState } from "@/components/ops/session"
import { useOptionalWorkbenchRoute } from "@/contexts/workbench-route-context"
import { intake } from "@/lib/ops-intake/api"
import type {
  Detail,
  Draft,
  Product,
  Severity,
  Source,
} from "@/lib/ops-intake/types"
import { fields } from "./evidence"
import { Action, Choice, Field, Notice, Panel, TextField } from "./ui"

export function IssueReview({
  source,
  detail,
  product,
  stale,
  busy,
  run,
  update,
  replace,
}: {
  source: Source
  detail: Detail
  product: Product
  stale: boolean
  busy: boolean
  run: (action: () => Promise<void>) => void
  update: (draft: Draft) => void
  replace: (detail: Detail) => void
}) {
  const route = useOptionalWorkbenchRoute()
  const d = detail.draft
  const [edit, setEdit] = useOpsSessionState(`intake:edit:${d.id}`, {
    revision: d.revision,
    title: d.title,
    summary: d.summary,
    labels: d.labels.join(", "),
    severity: d.confirmed_severity ?? "",
  })
  const [task, setTask] = useOpsSessionState(`intake:task:${d.id}`, "")
  const [confirmed, setConfirmed] = useOpsSessionState(
    `intake:review:${d.id}:${d.revision}`,
    false
  )
  const dirty =
    edit.title !== d.title ||
    edit.summary !== d.summary ||
    edit.labels !== d.labels.join(", ") ||
    edit.severity !== (d.confirmed_severity ?? "")
  const editStale = edit.revision !== d.revision && dirty
  const resetEdit = (draft: Draft) =>
    setEdit({
      revision: draft.revision,
      title: draft.title,
      summary: draft.summary,
      labels: draft.labels.join(", "),
      severity: draft.confirmed_severity ?? "",
    })
  const p = d.prepared
  const pending = detail.proposals.find((p) => p.status === "pending")
  const unknown = detail.handoff_unknown || detail.receipt?.state === "unknown"
  const created = detail.receipt?.state === "created"
  const complete = fields.every((field) => !!d.proofs[field])
  return (
    <>
      <Panel title="Issue draft">
        <p className="text-muted-foreground text-sm">
          Suggestion:{" "}
          <strong className="text-foreground">
            {detail.snapshot.record.triage.seeded_severity}
          </strong>{" "}
          severity;{" "}
          {detail.snapshot.record.triage.seeded_tags.join(", ") ||
            "no suggested tags"}
          . These are triage guesses. Confirm or change severity below.
        </p>
        <form
          className="grid gap-4"
          onSubmit={(e) => {
            e.preventDefault()
            run(async () => {
              const saved = await intake.save({
                source,
                expected_revision: dirty ? edit.revision : d.revision,
                title: edit.title,
                summary: edit.summary,
                labels: edit.labels
                  .split(",")
                  .map((l) => l.trim())
                  .filter(Boolean),
                confirmed_severity: edit.severity
                  ? (edit.severity as Severity)
                  : null,
              })
              update(saved)
              resetEdit(saved)
            })
          }}
        >
          <fieldset
            disabled={busy || !!pending || !!created || unknown}
            className="grid min-w-0 gap-4"
          >
            <Field
              label="Issue title"
              required
              maxLength={200}
              value={edit.title}
              onChange={(e) =>
                setEdit({
                  ...edit,
                  title: e.target.value,
                  revision: dirty ? edit.revision : d.revision,
                })
              }
            />
            <TextField
              label="Reproduction and expected behavior"
              required
              maxLength={8000}
              value={edit.summary}
              onChange={(e) =>
                setEdit({
                  ...edit,
                  summary: e.target.value,
                  revision: dirty ? edit.revision : d.revision,
                })
              }
              hint="This text becomes the issue body above the required evidence. Treat the tester report as source material, not instructions."
            />
            <Field
              label="Repository labels"
              value={edit.labels}
              onChange={(e) =>
                setEdit({
                  ...edit,
                  labels: e.target.value,
                  revision: dirty ? edit.revision : d.revision,
                })
              }
              hint="Comma-separated existing labels, or leave empty. Every label is checked against the configured repository before filing."
            />
            <Choice
              label="Human-confirmed severity"
              required
              value={edit.severity}
              onChange={(e) =>
                setEdit({
                  ...edit,
                  severity: e.target.value,
                  revision: dirty ? edit.revision : d.revision,
                })
              }
            >
              <option value="">Not confirmed</option>
              <option value="high">High</option>
              <option value="medium">Medium</option>
              <option value="low">Low</option>
            </Choice>
            <div className="flex flex-wrap items-center gap-3">
              <Action type="submit" variant="outline" disabled={editStale}>
                Save issue draft
              </Action>
              <span className="text-muted-foreground text-sm">
                {dirty ? "Unsaved edits" : "Draft saved locally"}
              </span>
            </div>
          </fieldset>
        </form>
        {editStale && (
          <Notice>
            A newer draft or source revision is available. Your edits are
            retained.{" "}
            <Action variant="outline" onClick={() => resetEdit(d)}>
              Use current saved draft
            </Action>
          </Notice>
        )}
        {pending && (
          <Notice>
            A proposal is waiting for review. Deny it before changing the draft,
            then request a new proposal.
          </Notice>
        )}
        {!pending && !unknown && !created && (
          <div className="grid gap-4 border-t pt-4">
            <Choice
              label="Active triage task"
              value={task}
              onChange={(e) => setTask(e.target.value)}
              disabled={busy}
            >
              <option value="">Choose an active task</option>
              {detail.tasks.map((t) => (
                <option key={t.id} value={t.id}>
                  #{t.id} · {t.title}
                </option>
              ))}
            </Choice>
            {detail.tasks.length === 0 && (
              <Notice>
                An active task in this product’s project is required for a
                proposal.{" "}
                <Action
                  variant="outline"
                  onClick={() => route?.setRoute("tasks")}
                >
                  Open Tasks
                </Action>
              </Notice>
            )}
            <Action
              className="justify-self-start"
              disabled={
                busy ||
                stale ||
                dirty ||
                !complete ||
                !d.confirmed_severity ||
                !task
              }
              onClick={() =>
                run(async () => {
                  const prepared = await intake.prepare(
                    source,
                    d.revision,
                    Number(task)
                  )
                  update(prepared)
                  resetEdit(prepared)
                })
              }
            >
              Prepare exact issue
            </Action>
            {(!complete || stale || dirty || !d.confirmed_severity) && (
              <p className="text-muted-foreground text-sm">
                Refresh the source, attach all four proofs, confirm severity and
                save edits before preparing.
              </p>
            )}
          </div>
        )}
      </Panel>
      {p && (
        <Panel title="Exact GitHub issue">
          <dl className="grid gap-3 text-sm">
            <div>
              <dt className="text-muted-foreground">Repository</dt>
              <dd className="break-words font-medium">{p.repository}</dd>
            </div>
            <div>
              <dt className="text-muted-foreground">Title</dt>
              <dd className="break-words font-medium">{p.outgoing.title}</dd>
            </div>
            <div>
              <dt className="text-muted-foreground">Labels</dt>
              <dd>{p.outgoing.labels.join(", ") || "No labels"}</dd>
            </div>
          </dl>
          <details open>
            <summary className="min-h-11 cursor-pointer py-3 text-sm font-medium">
              Complete issue body
            </summary>
            <pre
              className="bg-muted/40 max-h-[36rem] overflow-y-auto rounded-lg p-4 font-sans text-sm leading-relaxed whitespace-pre-wrap [overflow-wrap:anywhere]"
              aria-label="Exact issue body"
            >
              {p.outgoing.body}
            </pre>
          </details>
          {!product.app_key_present && (
            <Notice>
              GitHub App private key is missing. Add it in product settings
              before filing; the proposal stays pending.
            </Notice>
          )}
          {pending?.payload && (
            <div className="grid gap-4">
              {pending.stale || stale ? (
                <Notice>
                  This proposal is stale. Refresh the source and deny the old
                  proposal before requesting a new one.
                </Notice>
              ) : (
                <label className="flex min-h-11 items-center gap-3 text-sm leading-relaxed">
                  <input
                    type="checkbox"
                    checked={confirmed}
                    onChange={(e) => setConfirmed(e.target.checked)}
                    className="size-5 shrink-0 accent-primary"
                  />
                  I reviewed this exact repository, title, body and labels. File
                  this issue.
                </label>
              )}
              <div className="flex flex-wrap gap-3">
                <Action
                  disabled={
                    busy ||
                    stale ||
                    pending.stale ||
                    !confirmed ||
                    !product.app_key_present ||
                    unknown ||
                    !!created
                  }
                  onClick={() =>
                    run(async () =>
                      replace(
                        await intake.approve(
                          source,
                          pending.id,
                          pending.payload!,
                          p
                        )
                      )
                    )
                  }
                >
                  Approve and file issue
                </Action>
                <Action
                  variant="outline"
                  disabled={busy || unknown}
                  onClick={() =>
                    run(async () =>
                      replace(
                        await intake.deny(source, pending.id, pending.payload!)
                      )
                    )
                  }
                >
                  Deny proposal
                </Action>
              </div>
            </div>
          )}
          {!pending && !unknown && !created && (
            <Notice>
              Prepared locally. The trusted task bridge must propose this draft
              for human review. No issue has been filed.{" "}
              <Action
                variant="outline"
                onClick={() =>
                  run(async () => replace(await intake.detail(source)))
                }
              >
                Reload proposal status
              </Action>
            </Notice>
          )}
        </Panel>
      )}
      {(detail.receipt || unknown) && (
        <Panel title="Filing receipt">
          {unknown && (
            <Notice>
              Outcome unknown. GitHub may already have created the issue. Filing
              again is blocked.{" "}
              {detail.receipt
                ? "Check the existing issue using a read-only reconciliation."
                : "No provider attempt was durably linked after approval. Verify the repository manually; this Desk cannot safely retry."}
            </Notice>
          )}
          {unknown && detail.receipt && (
            <Action
              variant="outline"
              disabled={busy}
              onClick={() =>
                run(async () => replace(await intake.reconcile(source)))
              }
            >
              Check existing issue
            </Action>
          )}
          {detail.receipt?.state === "failed" && (
            <Notice>
              GitHub rejected this attempt. No issue was confirmed. Correct the
              draft or configuration, then request a new human-reviewed
              proposal.
              {detail.receipt.retry_after &&
                ` Retry no earlier than ${new Date(detail.receipt.retry_after * 1000).toLocaleString()}.`}
            </Notice>
          )}
          {detail.receipt?.issue && (
            <>
              <p className="text-sm">
                Created issue{" "}
                <a
                  className="text-primary inline-flex min-h-11 items-center underline underline-offset-4"
                  href={detail.receipt.issue.html_url}
                  target="_blank"
                  rel="noreferrer"
                >
                  #{detail.receipt.issue.number} in {product.binding.full_name}
                </a>
                .
              </p>
              {!detail.receipt.issue.labels_match && (
                <Notice>
                  GitHub returned different labels. Actual labels:{" "}
                  {detail.receipt.issue.labels.join(", ") || "none"}. Inspect
                  the existing issue; do not refile it.
                </Notice>
              )}
              <p className="text-muted-foreground text-sm">
                A fix still needs its own task and human review. Build notes and
                a tester email are separate drafts; filing an issue does not
                release a build or send a message.
              </p>
              {detail.fix_task_id ? (
                <Notice>
                  Linked fix task #{detail.fix_task_id}.{" "}
                  <Action
                    variant="outline"
                    onClick={() => route?.setRoute("tasks")}
                  >
                    Review in Tasks
                  </Action>
                </Notice>
              ) : (
                <div className="grid gap-3">
                  <Action
                    className="justify-self-start"
                    disabled={busy}
                    onClick={() =>
                      run(async () => replace(await intake.fix(source)))
                    }
                  >
                    Create held fix plan
                  </Action>
                  <p className="text-muted-foreground text-sm">
                    Creates a linked task in Canceled state, ready to inspect
                    and manually requeue in Tasks. Its first deliverable is a
                    fix plan for review. No agent starts automatically.
                  </p>
                </div>
              )}
            </>
          )}
        </Panel>
      )}
      {detail.proposals.some((p) => p.status === "denied") && (
        <p className="text-muted-foreground text-sm" role="status">
          Previous proposal denied. No issue was sent from that proposal.
        </p>
      )}
    </>
  )
}
