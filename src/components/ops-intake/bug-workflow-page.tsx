"use client"

import { useCallback, useEffect, useRef, useState } from "react"
import { useOpsSessionState } from "@/components/ops/session"
import { intake, intakeError } from "@/lib/ops-intake/api"
import type {
  Detail,
  Draft,
  Listing,
  Product,
  Source,
  Status,
} from "@/lib/ops-intake/types"
import { cn } from "@/lib/utils"
import { EvidenceEditor } from "./evidence"
import { IssueReview } from "./issue-review"
import { IntakeSettings } from "./settings"
import { Action, Choice, Notice, Panel } from "./ui"

export function BugWorkflowPageTitle() {
  return (
    <span className="text-muted-foreground px-4 text-xs font-medium">
      Bug intake
    </span>
  )
}

export function BugWorkflowPage() {
  const [status, setStatus] = useOpsSessionState<Status | null>(
    "intake:status",
    null
  )
  const [productId, setProductId] = useOpsSessionState("intake:product", "")
  const [selected, setSelected] = useOpsSessionState<Source | null>(
    "intake:selected",
    null
  )
  const [listing, setListing] = useOpsSessionState<Listing | null>(
    `intake:list:${productId}`,
    null
  )
  const [settings, setSettings] = useOpsSessionState("intake:settings", false)
  const [busy, setBusy] = useOpsSessionState("intake:list-busy", false)
  const [error, setError] = useState<string | null>(null)
  const inFlight = useRef(false)
  const product = status?.products.find(
    (p) => p.binding.product_id === productId
  )
  const loadStatus = useCallback(async () => {
    try {
      const status = await intake.status()
      setStatus(status)
      setProductId(
        (current) => current || status.products[0]?.binding.product_id || ""
      )
      setError(null)
    } catch (error) {
      setError(intakeError(error))
    }
  }, [setStatus, setProductId])
  useEffect(() => {
    void loadStatus()
  }, [loadStatus]) // This provider is destroyed on backend change.
  const read = async (cursor: string | null = null) => {
    if (busy || inFlight.current) return
    inFlight.current = true
    setBusy(true)
    setError(null)
    try {
      const result = await intake.list(productId, cursor)
      setListing((previous) =>
        cursor && previous
          ? {
              ...result,
              records: [
                ...previous.records,
                ...result.records.filter(
                  (r) =>
                    !previous.records.some(
                      (p) =>
                        p.record.source_ref.ulid === r.record.source_ref.ulid
                    )
                ),
              ],
            }
          : result
      )
    } catch (error) {
      setError(intakeError(error))
    } finally {
      inFlight.current = false
      setBusy(false)
    }
  }
  return (
    <div
      className="bg-background text-foreground flex min-h-0 min-w-0 flex-1 flex-col overflow-y-auto pb-[max(24px,env(safe-area-inset-bottom))]"
      data-testid="bug-workflow"
    >
      <header className="border-border flex flex-wrap items-start justify-between gap-4 border-b p-4 sm:p-6">
        <div>
          <h1 className="text-xl font-semibold tracking-tight">Bug intake</h1>
          <p className="text-muted-foreground mt-1 text-sm">
            TestFlight report → reviewed evidence → GitHub issue
          </p>
        </div>
        <Action variant="outline" onClick={() => setSettings(!settings)}>
          {settings ? "Close settings" : "Product settings"}
        </Action>
      </header>
      <div className="mx-auto grid w-full max-w-7xl gap-4 p-4 sm:gap-6 sm:p-6">
        {error && (
          <Notice error>
            {error}{" "}
            <Action variant="outline" onClick={() => void loadStatus()}>
              Reload connection
            </Action>
          </Notice>
        )}
        {!status && !error && <Notice>Loading intake settings…</Notice>}
        {status && (settings || status.products.length === 0) && (
          <IntakeSettings
            key={product?.binding.product_id ?? "new"}
            status={status}
            product={product}
            onSaved={(next) => {
              setStatus(next)
              setProductId(next.products[0]?.binding.product_id ?? "")
              setSettings(false)
              setListing(null)
            }}
          />
        )}
        {status?.products.length === 0 && (
          <Notice>
            No product is connected. Add the existing Hafidh origin, project and
            GitHub App configuration above. In-app feedback is unavailable
            because the current Hafidh contract has no read endpoint.
          </Notice>
        )}
        {product && (
          <>
            <div className="flex flex-wrap items-end justify-between gap-4">
              <div className="min-w-0 flex-1 sm:max-w-sm">
                <Choice
                  label="Product"
                  value={productId}
                  onChange={(e) => {
                    setProductId(e.target.value)
                    setSelected(null)
                  }}
                >
                  {status?.products.map((p) => (
                    <option
                      key={p.binding.product_id}
                      value={p.binding.product_id}
                    >
                      {p.binding.product_id} · {p.binding.full_name}
                    </option>
                  ))}
                </Choice>
              </div>
              <Action
                disabled={
                  busy ||
                  !product.binding.enabled ||
                  !product.intake_credential_present ||
                  !status?.adapter_installed
                }
                onClick={() => void read()}
              >
                {busy ? "Reading TestFlight…" : "Read TestFlight"}
              </Action>
            </div>
            {(!product.intake_credential_present ||
              !product.app_key_present ||
              !product.binding.enabled ||
              !status?.adapter_installed) && (
              <Notice>
                {!product.binding.enabled && "Product access is disabled. "}
                {!product.intake_credential_present &&
                  "Hafidh read credential is missing. "}
                {!product.app_key_present &&
                  "GitHub App key is missing; filing is unavailable. "}
                {!status?.adapter_installed &&
                  "The intake adapter is not installed. "}
                <Action variant="outline" onClick={() => setSettings(true)}>
                  Review settings
                </Action>
              </Notice>
            )}
            <p className="text-muted-foreground text-sm">
              Read-only TestFlight intake. In-app feedback is unavailable. A
              list read does not confirm evidence freshness.
            </p>
            <div className="grid min-w-0 items-start gap-6 lg:grid-cols-[minmax(15rem,0.8fr)_minmax(0,2fr)]">
              <section
                aria-label="TestFlight reports"
                className={cn(
                  "border-border min-w-0 overflow-hidden rounded-xl border",
                  selected && "hidden lg:block"
                )}
              >
                <h2 className="border-border border-b px-4 py-3 text-sm font-semibold">
                  Source reports
                </h2>
                {!listing && (
                  <p className="text-muted-foreground p-4 text-sm">
                    Read TestFlight to load source reports.
                  </p>
                )}
                {listing?.records.length === 0 && (
                  <p className="text-muted-foreground p-4 text-sm">
                    No reports were returned by this read.
                  </p>
                )}
                {listing?.records.map(({ record }) => (
                  <button
                    key={record.source_ref.ulid}
                    type="button"
                    aria-pressed={selected?.ulid === record.source_ref.ulid}
                    onClick={() =>
                      setSelected({
                        product_id: productId,
                        ulid: record.source_ref.ulid,
                      })
                    }
                    className="border-border hover:bg-muted/60 focus-visible:ring-ring aria-pressed:bg-primary/10 flex min-h-20 w-full min-w-0 flex-col gap-2 border-b p-4 text-start focus-visible:ring-2 focus-visible:ring-inset focus-visible:outline-none"
                  >
                    <span className="line-clamp-2 text-sm font-medium [overflow-wrap:anywhere]">
                      {record.title}
                    </span>
                    <span className="text-foreground text-xs">
                      {record.source_status} ·{" "}
                      {record.build_number
                        ? `Build ${record.build_number}`
                        : "Build unavailable"}
                    </span>
                  </button>
                ))}
                {listing?.next_cursor && (
                  <div className="p-4">
                    <Action
                      variant="outline"
                      disabled={busy}
                      onClick={() => void read(listing.next_cursor)}
                    >
                      Read next page
                    </Action>
                  </div>
                )}
                {listing && !listing.scan_complete && !listing.next_cursor && (
                  <p className="text-muted-foreground p-4 text-sm">
                    The bounded scan is incomplete. Start another read with
                    overlap; older records may be outside this window.
                  </p>
                )}
              </section>
              {selected?.product_id === productId ? (
                <BugDetail
                  key={`${selected.product_id}:${selected.ulid}`}
                  source={selected}
                  product={product}
                  back={() => setSelected(null)}
                />
              ) : (
                <div className="hidden lg:block">
                  <Panel title="Select a report">
                    <p className="text-muted-foreground text-sm">
                      Review source details, attach the four required proofs,
                      then prepare the exact issue for human approval.
                    </p>
                  </Panel>
                </div>
              )}
            </div>
          </>
        )}
      </div>
    </div>
  )
}

function BugDetail({
  source,
  product,
  back,
}: {
  source: Source
  product: Product
  back: () => void
}) {
  const key = `intake:detail:${source.product_id}:${source.ulid}`
  const [detail, setDetail] = useOpsSessionState<Detail | null>(key, null)
  const [busy, setBusy] = useOpsSessionState(`${key}:busy`, false)
  const [error, setError] = useOpsSessionState<string | null>(
    `${key}:error`,
    null
  )
  const [saved, setSaved] = useOpsSessionState(`${key}:saved`, "")
  const [now, setNow] = useState(Date.now())
  const inFlight = useRef(false)
  useEffect(() => {
    let active = true
    intake
      .detail(source)
      .then((d) => {
        if (active) setDetail(d)
      })
      .catch((e) => {
        if (active) setError(intakeError(e))
      })
    const clock = window.setInterval(() => setNow(Date.now()), 10000)
    return () => {
      active = false
      window.clearInterval(clock)
    }
  }, [source, setDetail, setError])
  const run = async (action: () => Promise<void>) => {
    if (busy || inFlight.current) return
    inFlight.current = true
    setBusy(true)
    setError(null)
    setSaved("")
    try {
      await action()
    } catch (error) {
      setError(intakeError(error))
      try {
        setDetail(await intake.detail(source))
      } catch {
        /* Keep the original fixed error. */
      }
    } finally {
      inFlight.current = false
      setBusy(false)
    }
  }
  const update = (draft: Draft) => {
    setDetail((old) => (old ? { ...old, draft } : old))
    setSaved("Saved locally.")
  }
  const stale =
    !detail?.snapshot.verified_at ||
    !!detail.snapshot.error ||
    now / 1000 - detail.snapshot.verified_at > 900
  const r = detail?.snapshot.record
  return (
    <div className="min-w-0 space-y-6" aria-busy={busy}>
      <div className="flex flex-wrap gap-3">
        <Action variant="outline" className="lg:hidden" onClick={back}>
          Back to reports
        </Action>
        <Action
          variant="outline"
          disabled={busy}
          onClick={() =>
            void run(async () => setDetail(await intake.detail(source)))
          }
        >
          Reload status
        </Action>
      </div>
      {error && <Notice error>{error}</Notice>}
      {saved && (
        <p className="text-muted-foreground text-sm" role="status">
          {saved}
        </p>
      )}
      {!detail && !error && <Notice>Loading report…</Notice>}
      {detail && r && (
        <>
          <Panel title="Source report">
            <p className="text-sm leading-relaxed whitespace-pre-wrap [overflow-wrap:anywhere]">
              {r.description || "No written description was provided."}
            </p>
            <dl className="grid grid-cols-2 gap-4 text-sm">
              {[
                ["Device", r.device],
                ["OS version", r.os_version],
                ["App version", r.app_version],
                ["Build", r.build_number],
                ["Platform", r.platform],
                [
                  "Submitted",
                  r.submitted_at
                    ? new Date(r.submitted_at).toLocaleString()
                    : null,
                ],
              ].map(([label, value]) => (
                <div key={label}>
                  <dt className="text-muted-foreground">{label}</dt>
                  <dd className="mt-1 [overflow-wrap:anywhere]">
                    {value || "Not provided"}
                  </dd>
                </div>
              ))}
            </dl>
            <p className="text-muted-foreground text-xs [overflow-wrap:anywhere]">
              TestFlight {r.source_ref.ulid}
            </p>
            <div className="border-border flex flex-wrap items-center justify-between gap-3 border-t pt-4">
              <p className="text-sm">
                {stale
                  ? "Source refresh required"
                  : `Revalidated ${new Date(detail.snapshot.verified_at! * 1000).toLocaleTimeString()}`}
              </p>
              <Action
                disabled={busy}
                onClick={() =>
                  void run(async () => {
                    setDetail(await intake.refresh(source))
                    setNow(Date.now())
                    setSaved("Source revalidated from Hafidh.")
                  })
                }
              >
                {busy ? "Working…" : "Refresh source"}
              </Action>
            </div>
            <p className="text-muted-foreground text-xs">
              A changed source revision resets its evidence and severity
              confirmation. Saved edits remain available for comparison until
              you choose the current draft.
            </p>
          </Panel>
          <EvidenceEditor
            source={source}
            detail={detail}
            stale={stale}
            busy={busy}
            run={run}
            update={update}
          />
          <IssueReview
            source={source}
            detail={detail}
            product={product}
            stale={stale}
            busy={busy}
            run={run}
            update={update}
            replace={setDetail}
          />
        </>
      )}
    </div>
  )
}
