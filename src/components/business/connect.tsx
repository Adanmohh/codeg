"use client"

import { useState, useSyncExternalStore } from "react"
import { isTauri } from "@tauri-apps/api/core"
import { ArrowRight, KeyRound } from "lucide-react"
import { Input } from "@/components/ui/input"
import { getCodegToken } from "@/lib/transport/web-auth"
import { type BusinessConnection } from "@/lib/business/client"
import { useBusinessCopy } from "@/lib/business/copy"
import { Action, Brand, ErrorNotice, Field } from "./ui"
import { BusinessPreferences } from "./preferences"

function subscribeHints(changed: () => void) {
  window.addEventListener("storage", changed)
  return () => window.removeEventListener("storage", changed)
}
function connectionHints() {
  const native = isTauri()
  let savedOperator = false
  try {
    savedOperator = !native && !!getCodegToken()
  } catch {
    /* disabled storage */
  }
  // Stable, nonsecret snapshot. The token itself is never part of view state.
  return JSON.stringify({
    native,
    savedOperator,
    origin: native ? "" : window.location.origin,
  })
}
const serverHints = () => '{"native":false,"savedOperator":false,"origin":""}'

export function ConnectWorkspace({
  connect,
  busy,
  error,
}: {
  connect: (connection: BusinessConnection) => Promise<boolean>
  busy: boolean
  error: unknown
}) {
  const copy = useBusinessCopy()
  const [addressDraft, setAddress] = useState<string | null>(null)
  const [token, setToken] = useState("")
  const [operator, setOperator] = useState(false)
  const { native, savedOperator, origin } = JSON.parse(
    useSyncExternalStore(subscribeHints, connectionHints, serverHints)
  ) as { native: boolean; savedOperator: boolean; origin: string }
  const address = addressDraft ?? origin
  async function submit(connection: BusinessConnection) {
    if (await connect(connection)) setToken("")
  }
  return (
    <main className="bg-background h-full overflow-y-auto">
      <header className="mx-auto flex max-w-7xl flex-wrap items-center justify-between gap-4 px-6 py-4 sm:px-10 sm:py-6">
        <Brand />
        <BusinessPreferences />
      </header>
      <div className="mx-auto grid max-w-6xl gap-6 px-6 pt-4 pb-10 md:grid-cols-[minmax(0,1fr)_minmax(0,440px)] md:items-center md:gap-16 md:pt-20 md:pb-16">
        <section className="min-w-0">
          <p className="text-primary mb-3 text-sm font-semibold md:mb-5">
            {copy.workspace}
          </p>
          <h1 className="max-w-xl text-3xl leading-[1.16] font-semibold tracking-tight sm:text-5xl">
            {copy.signInTitle}
          </h1>
          <p className="text-muted-foreground mt-3 max-w-md text-sm leading-relaxed md:mt-6 md:text-base">
            {copy.signInLead}
          </p>
          <div className="border-border mt-9 hidden flex-wrap gap-x-6 gap-y-3 border-t pt-6 text-sm md:flex">
            <span>{copy.owner}</span>
            <ArrowRight
              className="text-muted-foreground size-4 rtl:rotate-180"
              aria-hidden="true"
            />
            <span>{copy.assignee}</span>
            <ArrowRight
              className="text-muted-foreground size-4 rtl:rotate-180"
              aria-hidden="true"
            />
            <span>{copy.reviewer}</span>
          </div>
        </section>
        <section
          className="border-border bg-card min-w-0 rounded-3xl border p-6 shadow-xs sm:p-8"
          aria-labelledby="business-signin"
        >
          <div className="bg-primary/10 text-primary mb-5 flex size-11 items-center justify-center rounded-xl">
            <KeyRound aria-hidden="true" className="size-5" />
          </div>
          <h2
            id="business-signin"
            className="text-xl font-semibold tracking-tight"
          >
            {copy.signIn}
          </h2>
          <p className="text-muted-foreground mt-2 text-sm leading-relaxed">
            {copy.signInHint}
          </p>
          <form
            className="mt-6 space-y-5"
            onSubmit={(event) => {
              event.preventDefault()
              void submit({ kind: "http", address, token })
            }}
          >
            <Field label={copy.server} hint={copy.addressHint}>
              {(id) => (
                <Input
                  id={id}
                  className="min-h-11 rounded-xl"
                  type="url"
                  dir="ltr"
                  value={address}
                  onChange={(event) => setAddress(event.target.value)}
                  required
                  autoComplete="url"
                  disabled={busy}
                />
              )}
            </Field>
            <Field label={operator ? copy.operatorToken : copy.memberToken}>
              {(id) => (
                <Input
                  id={id}
                  className="min-h-11 rounded-xl"
                  type="password"
                  dir="ltr"
                  value={token}
                  onChange={(event) => setToken(event.target.value)}
                  required
                  autoComplete="off"
                  spellCheck={false}
                  disabled={busy}
                />
              )}
            </Field>
            {error != null && <ErrorNotice error={error} />}
            <Action
              className="w-full"
              type="submit"
              disabled={busy || !token.trim() || !address.trim()}
            >
              {busy ? copy.connecting : copy.connect}
              <ArrowRight className="rtl:rotate-180" aria-hidden="true" />
            </Action>
            <p className="text-muted-foreground text-xs leading-relaxed">
              {copy.sessionHint}
            </p>
          </form>
          <details className="mt-6 border-t pt-4">
            <summary className="focus-visible:ring-ring flex min-h-11 cursor-pointer items-center rounded-lg text-sm font-medium outline-none focus-visible:ring-2">
              {copy.operator}
            </summary>
            <div className="grid gap-3 pt-2">
              {native && (
                <Action
                  variant="outline"
                  disabled={busy}
                  onClick={() => void submit({ kind: "native" })}
                >
                  {copy.localDesk}
                </Action>
              )}
              {savedOperator && (
                <Action
                  variant="outline"
                  className="h-auto whitespace-normal py-3 text-start"
                  disabled={busy}
                  onClick={() => {
                    void submit({
                      kind: "http",
                      address: window.location.origin,
                      token: getCodegToken(),
                    })
                  }}
                >
                  {copy.existingOperator}
                </Action>
              )}
              <label className="flex min-h-11 cursor-pointer items-center gap-3 text-sm">
                <input
                  type="checkbox"
                  checked={operator}
                  onChange={(event) => setOperator(event.target.checked)}
                  className="accent-primary size-4"
                />
                {copy.operatorToken}
              </label>
            </div>
          </details>
        </section>
      </div>
    </main>
  )
}
