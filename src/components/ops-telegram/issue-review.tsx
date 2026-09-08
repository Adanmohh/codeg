"use client"

import { useEffect, useRef, useState } from "react"
import { IssueReviewCard } from "@/components/ops-intake/issue-review"
import { fields } from "@/components/ops-intake/evidence"
import { Panel } from "@/components/ops-intake/ui"
import { intake, intakeError } from "@/lib/ops-intake/api"
import { type Detail } from "@/lib/ops-intake/types"
import { type IssuePhoneReview } from "@/lib/ops-telegram/api"
import { LoadError, Loading, Notice } from "@/components/ops/ui"
import { useOpsResource } from "@/components/ops/use-ops-resource"

export function IssuePhoneCard({
  notice,
  review,
}: {
  notice: string
  review: IssuePhoneReview
}) {
  const [detail, setDetail] = useState<Detail>(review.detail)
  const [error, setError] = useState("")
  const [busy, setBusy] = useState(false)
  const [now, setNow] = useState(Date.now())
  const inFlight = useRef(false)
  const settings = useOpsResource("issue-phone-settings", intake.status)
  useEffect(() => {
    const clock = window.setInterval(() => setNow(Date.now()), 10000)
    return () => window.clearInterval(clock)
  }, [])
  const product = settings.data?.products.find(
    (p) => p.binding.product_id === review.source.product_id
  )
  const run = async (action: () => Promise<void>) => {
    if (inFlight.current) return
    inFlight.current = true
    setBusy(true)
    setError("")
    try {
      await action()
    } catch (e) {
      setError(intakeError(e))
    } finally {
      inFlight.current = false
      setBusy(false)
    }
  }
  const stale =
    !detail.snapshot.verified_at ||
    !!detail.snapshot.error ||
    now / 1000 - detail.snapshot.verified_at > 900 ||
    JSON.stringify(product?.binding) !== JSON.stringify(review.binding)
  if (settings.loading) return <Loading label="Loading issue configuration…" />
  if (settings.error)
    return <LoadError error={settings.error} retry={settings.reload} />
  if (!product)
    return (
      <Notice error>This product is no longer available for review.</Notice>
    )
  return (
    <div
      className="mx-auto w-full min-w-0 max-w-3xl space-y-6 p-4 sm:p-6"
      aria-busy={busy}
    >
      <h1 className="text-2xl font-semibold">Review GitHub issue</h1>
      {error && <Notice error>{error}</Notice>}
      <Panel title="Source report">
        <p className="whitespace-pre-wrap text-sm leading-relaxed [overflow-wrap:anywhere]">
          {detail.snapshot.record.description}
        </p>
        <p className="text-sm [overflow-wrap:anywhere]">
          Product {review.source.product_id} · TestFlight {review.source.ulid}
        </p>
      </Panel>
      <Panel title="Required evidence">
        <dl className="space-y-4 text-sm">
          {fields.map((field) => (
            <div key={field}>
              <dt className="font-semibold capitalize">{field}</dt>
              <dd className="mt-1 whitespace-pre-wrap [overflow-wrap:anywhere]">
                {detail.draft.proofs[field]?.proof.value ?? "Missing"}
              </dd>
              <dd className="mt-2 break-all text-xs text-muted-foreground">
                SHA-256{" "}
                {detail.draft.proofs[field]?.proof.sha256 ?? "Unavailable"}
              </dd>
            </div>
          ))}
        </dl>
      </Panel>
      <IssueReviewCard
        source={review.source}
        detail={detail}
        product={product}
        stale={stale}
        busy={busy}
        run={run}
        update={(draft) => setDetail((current) => ({ ...current, draft }))}
        replace={setDetail}
        reviewNotice={notice}
      />
    </div>
  )
}
