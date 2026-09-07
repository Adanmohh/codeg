"use client"

import { useCallback, useRef, useState } from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { ops } from "@/lib/ops/api"
import {
  LoadError,
  Loading,
  Notice,
  touchButton,
  destructiveButton,
} from "./ui"
import { opsError, useOpsResource } from "./use-ops-resource"

export function EmailSettings({
  inboxId,
  onPulled,
}: {
  inboxId: number
  onPulled: () => void
}) {
  const load = useCallback(() => ops.emailStatus(inboxId), [inboxId])
  const resource = useOpsResource(`email:${inboxId}`, load)
  const [key, setKey] = useState("")
  const [busy, setBusy] = useState("")
  const [error, setError] = useState("")
  const [notice, setNotice] = useState("")
  const inFlight = useRef(false)
  const run = async (action: "configure" | "disconnect" | "pull") => {
    if (inFlight.current) return
    inFlight.current = true
    setBusy(action)
    setError("")
    setNotice("")
    try {
      if (action === "configure") {
        await ops.configureEmail(inboxId, key)
        setKey("")
        setNotice(
          "Key saved in the existing credential store. Pull now to check access."
        )
      } else if (action === "disconnect") {
        await ops.disconnectEmail(inboxId)
        setNotice("Inbox key removed. Local threads and drafts are preserved.")
      } else {
        const result = await ops.pullEmail(inboxId)
        setNotice(
          `${result.inserted} messages imported · ${result.duplicates} already present.`
        )
        onPulled()
      }
      resource.reload()
    } catch (e) {
      setError(opsError(e))
    } finally {
      inFlight.current = false
      setBusy("")
    }
  }
  if (resource.loading) return <Loading label="Checking inbox connection…" />
  if (resource.error)
    return <LoadError error={resource.error} retry={resource.reload} />
  const status = resource.data
  return (
    <div className="space-y-3">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <span className="text-xs font-medium">
          {status?.configured ? "Resend key configured" : "Email not connected"}
        </span>
        <Button
          className={touchButton}
          variant="outline"
          disabled={!!busy || !status?.configured}
          onClick={() => void run("pull")}
        >
          {busy === "pull" ? "Pulling…" : "Pull now"}
        </Button>
      </div>
      <details className="rounded-lg border p-3">
        <summary className="min-h-11 cursor-pointer text-sm font-medium focus-visible:outline-2 focus-visible:outline-ring">
          {status?.configured ? "Manage email connection" : "Connect Resend"}
        </summary>
        <form
          className="space-y-3 pt-2"
          onSubmit={(e) => {
            e.preventDefault()
            void run("configure")
          }}
        >
          <p className="text-xs leading-relaxed text-muted-foreground">
            Use this inbox’s Resend key. It is stored on this backend and never
            returned to agents or shown again here. Sender-domain and receiving
            access remain provider configuration.
          </p>
          <label
            className="block text-sm font-medium"
            htmlFor={`ops-resend-key-${inboxId}`}
          >
            {status?.configured ? "Replacement API key" : "Resend API key"}
          </label>
          <Input
            id={`ops-resend-key-${inboxId}`}
            type="password"
            autoComplete="new-password"
            spellCheck={false}
            value={key}
            onChange={(e) => setKey(e.target.value)}
            className={touchButton}
            disabled={!!busy}
            required
          />
          <Button
            type="submit"
            className={touchButton}
            disabled={!!busy || !key}
          >
            {busy === "configure" ? "Saving key…" : "Save inbox key"}
          </Button>
        </form>
        {status?.configured && (
          <Button
            className={`${destructiveButton} mt-3`}
            variant="destructive"
            disabled={!!busy}
            onClick={() => void run("disconnect")}
          >
            Remove inbox key
          </Button>
        )}
      </details>
      {status?.lastPullAt && (
        <p className="text-xs text-muted-foreground">
          Last pull: {status.lastPullStatus} ·{" "}
          {new Date(status.lastPullAt).toLocaleString()}
        </p>
      )}
      {status?.lastPullError && <Notice error>{status.lastPullError}</Notice>}
      {error && <Notice error>{error}</Notice>}
      {notice && <Notice>{notice}</Notice>}
    </div>
  )
}
