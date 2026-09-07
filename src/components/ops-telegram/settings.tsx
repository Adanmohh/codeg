"use client"

import { useRef, useState } from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { useOpsSessionState } from "@/components/ops/session"
import { LoadError, Loading, Notice, touchButton } from "@/components/ops/ui"
import { opsError, useOpsResource } from "@/components/ops/use-ops-resource"
import { telegram, type TelegramStatus } from "@/lib/ops-telegram/api"

const states: Record<TelegramStatus["state"], string> = {
  not_configured:
    "Notifications are off. Choose an existing private Telegram channel to configure them.",
  disabled: "Notifications are off. No Ops notifications will be sent.",
  recipient_changed:
    "The channel recipient or topic settings changed. Save a new private recipient binding before sending.",
  missing_token:
    "The selected channel has no available bot token. Save its token in Chat channels; pending proposals remain unchanged.",
  ready:
    "Notifications are enabled. Pending email proposals are checked by the existing channel scheduler.",
}
const outcomes: Record<TelegramStatus["notices"][number]["status"], string> = {
  preflight_failed:
    "Recipient check failed · no send attempted. The next queue check can retry safely.",
  sent: "Telegram accepted the notification · human review still required",
  failed: "Notification rejected · no automatic resend",
  unknown: "Notification unconfirmed · no automatic resend",
  obsolete: "Proposal or recipient changed · notification not sent",
}

export function TelegramSettings({
  onDirty,
}: {
  onDirty: (dirty: boolean) => void
}) {
  const resource = useOpsResource("telegram-settings", telegram.status)
  return (
    <div className="min-h-0 flex-1 overflow-y-auto p-4 sm:p-6">
      <section
        aria-label="Telegram notifications"
        className="mx-auto max-w-2xl space-y-6"
      >
        <header className="space-y-2">
          <p className="text-xs font-medium text-muted-foreground">
            OPS DESK · NOTIFICATIONS
          </p>
          <h1 className="text-2xl font-semibold">Review from Telegram</h1>
          <p className="text-sm leading-relaxed text-muted-foreground">
            Receive a private link when an email reply needs your review. Open
            the protected page to see every recipient and edit the full message.
            Telegram commands cannot approve an Ops proposal.
          </p>
        </header>
        {resource.loading ? (
          <Loading label="Loading Telegram settings…" />
        ) : resource.error ? (
          <LoadError error={resource.error} retry={resource.reload} />
        ) : (
          resource.data && (
            <SettingsForm
              key={resource.data.configuration?.revision ?? "new"}
              data={resource.data}
              onDirty={onDirty}
              reload={resource.reload}
            />
          )
        )}
      </section>
    </div>
  )
}

function SettingsForm({
  data,
  onDirty,
  reload,
}: {
  data: TelegramStatus
  onDirty: (dirty: boolean) => void
  reload: () => void
}) {
  const initial = {
    channel: data.configuration?.channelId.toString() ?? "",
    user: data.configuration?.privateUserId ?? "",
    origin: data.configuration?.reviewOrigin ?? "",
    enabled: data.enabled,
  }
  const [fields, setFields] = useOpsSessionState(
    `edit:telegram:${data.configuration?.revision ?? "new"}`,
    initial
  )
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState("")
  const inFlight = useRef(false)
  const dirty = JSON.stringify(fields) !== JSON.stringify(initial)
  const update = (next: typeof fields) => {
    setFields(next)
    onDirty(JSON.stringify(next) !== JSON.stringify(initial))
  }
  const perform = async (action: "save" | "disable" | "notify") => {
    if (inFlight.current) return
    inFlight.current = true
    setBusy(true)
    setError("")
    try {
      if (action === "save")
        await telegram.configure({
          channelId: Number(fields.channel),
          privateUserId: fields.user,
          reviewOrigin: fields.origin,
          enabled: fields.enabled,
          expectedRevision: data.configuration?.revision ?? null,
        })
      else if (action === "disable") await telegram.disable()
      else await telegram.notify()
      onDirty(false)
      reload()
    } catch (e) {
      setError(opsError(e))
    } finally {
      inFlight.current = false
      setBusy(false)
    }
  }
  return (
    <>
      <Notice>{states[data.state]}</Notice>
      <p className="text-sm leading-relaxed text-muted-foreground">
        Use Settings → Chat channels to save a Telegram bot token and a numeric
        private chat ID with topics off. Ops reuses that configuration without
        connecting the channel or starting polling. Only the private user you
        explicitly name below can receive notices.
      </p>
      <form
        className="space-y-5"
        onSubmit={(e) => {
          e.preventDefault()
          void perform("save")
        }}
      >
        <div className="space-y-1.5">
          <label htmlFor="ops-telegram-channel" className="text-sm font-medium">
            Telegram channel
          </label>
          <select
            id="ops-telegram-channel"
            required
            disabled={busy}
            value={fields.channel}
            onChange={(e) => update({ ...fields, channel: e.target.value })}
            className="min-h-11 w-full rounded-lg border bg-background px-3 text-base focus-visible:outline-2 focus-visible:outline-ring sm:text-sm"
          >
            <option value="">Choose an existing channel</option>
            {data.channels.map((ch) => (
              <option key={ch.id} value={ch.id}>
                {ch.name}
              </option>
            ))}
          </select>
        </div>
        <div className="space-y-1.5">
          <label htmlFor="ops-telegram-user" className="text-sm font-medium">
            Authorized private user ID
          </label>
          <Input
            id="ops-telegram-user"
            required
            inputMode="numeric"
            pattern="[1-9][0-9]*"
            disabled={busy}
            value={fields.user}
            onChange={(e) => update({ ...fields, user: e.target.value })}
            className={touchButton}
            aria-describedby="telegram-recipient-help"
          />
          <p
            id="telegram-recipient-help"
            className="text-xs leading-relaxed text-muted-foreground"
          >
            Must exactly match the channel’s private chat ID. Groups, topics and
            usernames are not supported.
          </p>
        </div>
        <div className="space-y-1.5">
          <label htmlFor="ops-telegram-origin" className="text-sm font-medium">
            Protected review origin
          </label>
          <Input
            id="ops-telegram-origin"
            required
            type="url"
            disabled={busy}
            value={fields.origin}
            onChange={(e) => update({ ...fields, origin: e.target.value })}
            className={touchButton}
            placeholder="https://desk.example.com"
            aria-describedby="telegram-origin-help"
          />
          <p
            id="telegram-origin-help"
            className="text-xs leading-relaxed text-muted-foreground"
          >
            Phone access requires a reachable HTTPS origin protected by your
            operator login. A loopback link only validates this local browser;
            it will not reach this desk from your phone. Do not include
            credentials, paths or query parameters.
          </p>
        </div>
        <label className="flex min-h-11 cursor-pointer items-center gap-3 rounded-lg border p-3 text-sm">
          <input
            type="checkbox"
            className="size-5 accent-primary"
            checked={fields.enabled}
            disabled={busy}
            onChange={(e) => update({ ...fields, enabled: e.target.checked })}
          />
          Enable email review notifications for this private recipient
        </label>
        {error && <Notice error>{error}</Notice>}
        <div className="flex flex-wrap gap-3">
          <Button
            type="submit"
            className={touchButton}
            disabled={busy || !fields.channel || !fields.user || !fields.origin}
          >
            {busy ? "Working…" : "Save Telegram settings"}
          </Button>
          {data.enabled && (
            <Button
              type="button"
              variant="outline"
              className={touchButton}
              disabled={busy || dirty}
              onClick={() => void perform("disable")}
            >
              Turn notifications off
            </Button>
          )}
        </div>
      </form>
      <section
        className="space-y-3 border-t pt-5"
        aria-label="Notification history"
      >
        <div className="flex flex-wrap items-center justify-between gap-3">
          <h2 className="font-semibold">Recent notifications</h2>
          <Button
            variant="outline"
            className={touchButton}
            disabled={busy || dirty || data.state !== "ready"}
            onClick={() => void perform("notify")}
          >
            Check queue now
          </Button>
        </div>
        <p className="text-xs leading-relaxed text-muted-foreground">
          A queue check sends review links only. Telegram delivery never sends
          email or resolves an approval. Attempted or unconfirmed notifications
          are not resent automatically.
        </p>
        {data.notices.length ? (
          <ul className="divide-y rounded-lg border">
            {data.notices.map((n) => (
              <li className="space-y-1 p-4" key={n.proposalId}>
                <p className="text-sm font-medium">
                  Proposal #{n.proposalId} · Task #{n.taskId} · Run {n.runSeq}
                </p>
                <p className="text-sm leading-relaxed">{outcomes[n.status]}</p>
              </li>
            ))}
          </ul>
        ) : (
          <p className="rounded-lg border border-dashed p-5 text-sm text-muted-foreground">
            No Ops notifications have been attempted.
          </p>
        )}
      </section>
    </>
  )
}
