"use client"

import { useCallback, useEffect, useRef, useState } from "react"
import { ArrowLeft, ShieldCheck } from "lucide-react"
import { Button } from "@/components/ui/button"
import {
  ops,
  type OpsContext,
  type Proposal,
  type ThreadKey,
} from "@/lib/ops/api"
import { cn } from "@/lib/utils"
import { LoadError, Loading, Notice, touchButton } from "./ops-page"
import { completeReply, ReplyEditor, replyFields } from "./reply-editor"
import { opsError, useOpsResource } from "./use-ops-resource"

export function proposalStatus(p: Proposal): string {
  if (p.stale) return "Stale · new review needed"
  if (p.status === "approved") return "Approved · delivery unconfirmed"
  if (p.status === "denied") return "Denied · not sent"
  if (p.status === "pending") return "Pending human review"
  return p.status
}

export function ApprovalsView({
  context,
  selectedId,
  onSelect,
  onThread,
  onDirty,
}: {
  context: OpsContext
  selectedId: number | null
  onSelect: (id: number | null) => void
  onThread: (key: ThreadKey) => void
  onDirty: (dirty: boolean) => void
}) {
  const queue = useOpsResource("proposals", ops.proposals)
  return (
    <div className="flex min-h-0 flex-1">
      <aside
        aria-label="Proposal queue"
        className={cn(
          "w-full shrink-0 overflow-y-auto border-e sm:w-72 lg:w-80",
          selectedId !== null && "hidden sm:block"
        )}
      >
        <div className="space-y-1 border-b p-4">
          <h1 className="font-semibold">Review queue</h1>
          <p className="text-xs leading-relaxed text-muted-foreground">
            Pending first, then recent decisions. Approval alone is not
            delivery.
          </p>
        </div>
        {queue.loading ? (
          <Loading label="Loading proposals…" />
        ) : queue.error ? (
          <LoadError error={queue.error} retry={queue.reload} />
        ) : (
          queue.data &&
          (queue.data.length ? (
            <ul className="divide-y">
              {queue.data.map((p) => (
                <li key={p.id}>
                  <button
                    type="button"
                    className={cn(
                      "w-full space-y-1 p-4 text-start hover:bg-muted/50 focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-ring",
                      selectedId === p.id && "bg-primary/8"
                    )}
                    aria-current={selectedId === p.id ? "true" : undefined}
                    onClick={() => onSelect(p.id)}
                  >
                    <span className="block truncate text-sm font-medium">
                      {p.payload?.reply.subject ?? `Reply review #${p.id}`}
                    </span>
                    <span className="block text-xs text-muted-foreground">
                      Task #{p.taskId} · Run {p.runSeq}
                    </span>
                    <span className="block text-xs">{proposalStatus(p)}</span>
                  </button>
                </li>
              ))}
            </ul>
          ) : (
            <div className="space-y-3 p-6">
              <ShieldCheck className="size-7 text-primary" aria-hidden="true" />
              <h2 className="text-sm font-medium">No proposals to review</h2>
              <p className="text-sm leading-relaxed text-muted-foreground">
                Save reply drafts in the inbox. A task-bound proposal adapter
                will bring them here for human review; that agent capability is
                not connected yet.
              </p>
            </div>
          ))
        )}
      </aside>
      {selectedId !== null ? (
        <ProposalDetail
          key={selectedId}
          id={selectedId}
          context={context}
          onDirty={onDirty}
          onBack={() => onSelect(null)}
          onThread={onThread}
          onResolved={queue.reload}
        />
      ) : (
        <div className="hidden flex-1 items-center justify-center p-8 sm:flex">
          <div className="max-w-sm space-y-2 text-center">
            <h2 className="font-medium">Review the complete reply</h2>
            <p className="text-sm text-muted-foreground">
              Check every recipient, edit the content and resolve the proposal.
              A newer draft or task run invalidates an old review.
            </p>
          </div>
        </div>
      )}
    </div>
  )
}

function ProposalDetail({
  id,
  context,
  onBack,
  onThread,
  onDirty,
  onResolved,
}: {
  id: number
  context: OpsContext
  onBack: () => void
  onThread: (key: ThreadKey) => void
  onDirty: (dirty: boolean) => void
  onResolved: () => void
}) {
  const load = useCallback(() => ops.proposal(id), [id])
  const resource = useOpsResource(`proposal:${id}`, load)
  return (
    <div className="min-w-0 flex-1 overflow-y-auto">
      <div className="border-b p-2 sm:hidden">
        <Button className={touchButton} variant="ghost" onClick={onBack}>
          <ArrowLeft aria-hidden="true" />
          Back to review queue
        </Button>
      </div>
      {resource.loading ? (
        <Loading label="Loading the complete review payload…" />
      ) : resource.error ? (
        <LoadError error={resource.error} retry={resource.reload} />
      ) : (
        resource.data && (
          <ReviewCard
            key={`${resource.data.id}:${resource.data.status}:${resource.data.stale}`}
            proposal={resource.data}
            context={context}
            onDirty={onDirty}
            onThread={onThread}
            onResolved={() => {
              resource.reload()
              onResolved()
            }}
          />
        )
      )}
    </div>
  )
}

export function ReviewCard({
  proposal,
  context,
  onDirty,
  onThread,
  onResolved,
}: {
  proposal: Proposal
  context: OpsContext
  onDirty: (dirty: boolean) => void
  onThread: (key: ThreadKey) => void
  onResolved: () => void
}) {
  const original = proposal.payload
  const [fields, setFields] = useState(() =>
    original ? replyFields(original.reply) : null
  )
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState("")
  const inFlight = useRef(false)
  useEffect(() => () => onDirty(false), [onDirty])
  const resolve = async (decision: "approve" | "deny") => {
    if (inFlight.current || !original || !fields) return
    inFlight.current = true
    setBusy(true)
    setError("")
    try {
      if (decision === "deny") await ops.deny(proposal.id, original)
      else
        await ops.approve(proposal.id, original, {
          ...original,
          reply: completeReply(original.reply, fields),
        })
      onDirty(false)
      onResolved()
    } catch (e) {
      setError(opsError(e))
    } finally {
      inFlight.current = false
      setBusy(false)
    }
  }
  return (
    <article className="mx-auto max-w-3xl space-y-5 p-4 sm:p-6">
      <header className="space-y-2">
        <p className="text-xs font-medium text-muted-foreground">
          Proposal #{proposal.id} · Task #{proposal.taskId} · Run{" "}
          {proposal.runSeq}
        </p>
        <h1 className="text-xl font-semibold">{proposalStatus(proposal)}</h1>
        <Button
          className={touchButton}
          variant="outline"
          onClick={() =>
            onThread({
              inboxId: proposal.inboxId,
              conversationId: proposal.conversationId,
            })
          }
        >
          Open original thread
        </Button>
      </header>
      {proposal.reason && (
        <Notice error={proposal.stale}>{proposal.reason}</Notice>
      )}
      {original && fields ? (
        <>
          <p className="text-sm text-muted-foreground">
            Draft #{original.draftId} · Revision {original.draftRevision}.
            Review all recipients and the complete message below. Edits apply to
            this approval; the saved draft is not reread for delivery.
          </p>
          <ReplyEditor
            fields={fields}
            disabled={busy || proposal.stale}
            onChange={(next) => {
              setFields(next)
              onDirty(
                JSON.stringify(next) !==
                  JSON.stringify(replyFields(original.reply))
              )
            }}
          />
          <div id="ops-delivery-state">
            <Notice>{context.transportMessage}</Notice>
          </div>
          {error && (
            <Notice error>
              {error} No delivery is confirmed. Refresh the queue to see the
              current decision before retrying.
            </Notice>
          )}
          <div className="flex flex-wrap gap-3">
            <Button
              className={touchButton}
              disabled={
                busy ||
                proposal.stale ||
                context.emailTransport !== "configured"
              }
              aria-describedby="ops-delivery-state"
              onClick={() => void resolve("approve")}
            >
              {busy ? "Resolving…" : "Approve reply"}
            </Button>
            <Button
              className={touchButton}
              variant="destructive"
              disabled={busy || proposal.status !== "pending"}
              onClick={() => void resolve("deny")}
            >
              Deny proposal
            </Button>
          </div>
        </>
      ) : (
        <Notice>
          {proposal.status === "denied"
            ? "This proposal was denied. Its private reply content has been redacted from the review record."
            : "This record has no editable pending payload. Open the thread for its current draft. No delivery receipt is implied by this decision."}
        </Notice>
      )}
    </article>
  )
}
