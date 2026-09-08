"use client"

import { useState } from "react"
import { Input } from "@/components/ui/input"
import type { BusinessClient } from "@/lib/business/client"
import type { BusinessContext } from "@/lib/business/identity"
import { useBusinessCopy } from "@/lib/business/copy"
import { Action, Brand, ErrorNotice, Field } from "./ui"
import { BusinessPreferences } from "./preferences"

export function BootstrapWorkspace({
  client,
  onReady,
  disconnect,
}: {
  client: BusinessClient
  onReady: (context: BusinessContext) => void
  disconnect: () => void
}) {
  const copy = useBusinessCopy()
  const [organizationName, setOrganizationName] = useState("")
  const [ownerName, setOwnerName] = useState("")
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<unknown>(null)
  return (
    <main className="bg-background h-full overflow-y-auto p-6 sm:p-10">
      <header className="mx-auto flex max-w-6xl flex-wrap items-center justify-between gap-4">
        <Brand />
        <BusinessPreferences />
      </header>
      <section className="border-border bg-card mx-auto mt-12 max-w-xl rounded-3xl border p-6 shadow-xs sm:mt-20 sm:p-10">
        <p className="text-primary mb-4 text-sm font-semibold">
          {copy.operator}
        </p>
        <h1 className="text-3xl leading-tight font-semibold tracking-tight">
          {copy.bootstrapTitle}
        </h1>
        <p className="text-muted-foreground mt-4 text-sm leading-relaxed">
          {copy.bootstrapHint}
        </p>
        <form
          className="mt-8 space-y-5"
          onSubmit={async (event) => {
            event.preventDefault()
            setBusy(true)
            setError(null)
            try {
              onReady(
                await client.identity("bootstrap", {
                  organizationName: organizationName.trim(),
                  ownerName: ownerName.trim(),
                })
              )
            } catch (caught) {
              setError(caught)
            } finally {
              setBusy(false)
            }
          }}
        >
          <Field label={copy.organizationName}>
            {(id) => (
              <Input
                id={id}
                value={organizationName}
                onChange={(event) => setOrganizationName(event.target.value)}
                required
                maxLength={120}
                className="min-h-11 rounded-xl"
                disabled={busy}
                autoComplete="organization"
              />
            )}
          </Field>
          <Field label={copy.yourName}>
            {(id) => (
              <Input
                id={id}
                value={ownerName}
                onChange={(event) => setOwnerName(event.target.value)}
                required
                maxLength={120}
                className="min-h-11 rounded-xl"
                disabled={busy}
                autoComplete="name"
              />
            )}
          </Field>
          {error != null && <ErrorNotice error={error} />}
          <Action
            type="submit"
            className="w-full"
            disabled={busy || !ownerName.trim() || !organizationName.trim()}
          >
            {busy ? copy.saving : copy.createWorkspace}
          </Action>
          <Action
            type="button"
            variant="ghost"
            className="w-full"
            onClick={disconnect}
            disabled={busy}
          >
            {copy.disconnect}
          </Action>
        </form>
      </section>
    </main>
  )
}
