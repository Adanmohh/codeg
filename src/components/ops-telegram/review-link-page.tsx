"use client"

import { useCallback, useEffect, useState } from "react"
import { Button } from "@/components/ui/button"
import { ReviewCard } from "@/components/ops/proposals-view"
import { InboxView } from "@/components/ops/inbox-view"
import { useOpsSession } from "@/components/ops/session"
import { LoadError, Loading, Notice, touchButton } from "@/components/ops/ui"
import { opsError, useOpsResource } from "@/components/ops/use-ops-resource"
import { ops, type Proposal, type ThreadKey } from "@/lib/ops/api"
import { telegram, type ReviewResolution } from "@/lib/ops-telegram/api"
import { getCodegToken, redirectToCodegLogin } from "@/lib/transport/web-auth"
import { isDesktop } from "@/lib/platform"
import { IssuePhoneCard } from "./issue-review"

export function ReviewLinkPage({ notice }: { notice: string | null }) {
  const [authenticated, setAuthenticated] = useState(false)
  useEffect(() => {
    if (!isDesktop() && !getCodegToken()) redirectToCodegLogin()
    else setAuthenticated(true)
  }, [])
  return authenticated ? (
    <ReviewContent notice={notice} />
  ) : (
    <Loading label="Opening protected review…" />
  )
}

function ReviewContent({ notice }: { notice: string | null }) {
  const session = useOpsSession()
  const [dirty, setDirty] = useState(false)
  const [thread, setThread] = useState<ThreadKey | null>(null)
  const [decided, setDecided] = useState<Proposal | null>(null)
  const [error, setError] = useState("")
  const context = useOpsResource("review-context", ops.context)
  const load = useCallback(
    () =>
      notice
        ? telegram.resolve(notice)
        : Promise.resolve<ReviewResolution>({
            state: "unavailable",
            proposal: null,
            issue: null,
          }),
    [notice]
  )
  const review = useOpsResource(`notice:${notice}`, load)
  const proposal = decided ?? review.data?.proposal
  useEffect(() => {
    const warn = (event: BeforeUnloadEvent) => {
      event.preventDefault()
      event.returnValue = ""
    }
    if (dirty) window.addEventListener("beforeunload", warn)
    return () => window.removeEventListener("beforeunload", warn)
  }, [dirty])
  const navigate = (action: () => void) => {
    if (
      dirty &&
      !window.confirm(
        "Discard unsaved changes? Save the draft or private note before leaving to keep it."
      )
    )
      return
    session.discardEdits()
    setDirty(false)
    action()
  }
  const resolved = () => {
    if (!proposal) return
    void ops
      .proposal(proposal.id)
      .then(setDecided)
      .catch((e: unknown) => setError(opsError(e)))
  }
  return (
    <main
      aria-label="Protected Ops review"
      className="flex min-h-dvh min-w-0 flex-col bg-background pb-[env(safe-area-inset-bottom)]"
    >
      <header className="flex flex-wrap items-center justify-between gap-2 border-b px-4 py-3 sm:px-6">
        <p className="font-semibold">Hafidh Ops desk</p>
        <div className="flex flex-wrap gap-2">
          {thread && (
            <Button
              variant="outline"
              className={touchButton}
              onClick={() => navigate(() => setThread(null))}
            >
              Back to review
            </Button>
          )}
          <Button
            variant="ghost"
            className={touchButton}
            onClick={() =>
              navigate(() => {
                window.location.href = "/workspace"
              })
            }
          >
            Open workspace
          </Button>
        </div>
      </header>
      {context.loading || review.loading ? (
        <Loading label="Loading the complete review payload…" />
      ) : context.error || review.error ? (
        <LoadError
          error={context.error ?? review.error ?? "Review unavailable"}
          retry={() =>
            navigate(() => {
              context.reload()
              review.reload()
            })
          }
        />
      ) : review.data?.issue && notice ? (
        <IssuePhoneCard notice={notice} review={review.data.issue} />
      ) : context.data && proposal ? (
        <>
          {error && (
            <div className="p-4">
              <Notice error>{error}</Notice>
            </div>
          )}
          {thread ? (
            <div className="flex h-[calc(100dvh-6rem)] min-h-0 flex-col">
              <InboxView
                context={context.data}
                selected={thread}
                onSelect={(key) => navigate(() => setThread(key))}
                onDirty={setDirty}
              />
            </div>
          ) : (
            <ReviewCard
              key={`${proposal.id}:${proposal.status}:${proposal.stale}`}
              context={context.data}
              proposal={proposal}
              onDirty={setDirty}
              onThread={(key) => navigate(() => setThread(key))}
              onResolved={resolved}
            />
          )}
        </>
      ) : (
        <section className="mx-auto max-w-xl space-y-4 px-5 py-12">
          <h1 className="text-2xl font-semibold">
            This review link is no longer current
          </h1>
          <p className="text-sm leading-relaxed text-muted-foreground">
            The proposal or recipient settings may have changed, or a decision
            was already recorded. Open the Ops workspace to check the current
            queue. This link cannot approve or send anything.
          </p>
          <Button
            variant="outline"
            className={touchButton}
            onClick={review.reload}
          >
            Check link again
          </Button>
        </section>
      )}
    </main>
  )
}
