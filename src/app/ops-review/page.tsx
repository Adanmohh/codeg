"use client"

import { Suspense } from "react"
import { useSearchParams } from "next/navigation"
import { ReviewLinkPage } from "@/components/ops-telegram/review-link-page"
import { OpsSessionProvider } from "@/components/ops/session"
import { Loading } from "@/components/ops/ui"
import { reviewNotice } from "@/lib/ops-telegram/locator"

function ReviewLocator() {
  const search = useSearchParams()
  const notice = reviewNotice(search.toString())
  return (
    <OpsSessionProvider key={notice ?? "invalid"}>
      <ReviewLinkPage notice={notice} />
    </OpsSessionProvider>
  )
}
export default function Page() {
  return (
    <Suspense fallback={<Loading label="Opening your review…" />}>
      <ReviewLocator />
    </Suspense>
  )
}
