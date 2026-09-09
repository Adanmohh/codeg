"use client"

import { useEffect, useRef, useState } from "react"
import { Input } from "@/components/ui/input"
import { Action, controlClass, Field, Modal } from "@/components/business/ui"
import { useBusinessCopy } from "@/lib/business/copy"
import { useIntakeCopy } from "@/lib/business/intake-copy"
import type { BusinessClient } from "@/lib/business/client"
import type { Member } from "@/lib/business/identity"
import type {
  BindingAdmin,
  Grant,
  Page,
  SourceKind,
  SourceSetup,
} from "@/lib/business/intake"
import {
  BUSINESS_DOMAINS,
  type BusinessDomain,
} from "@/lib/business/presentation"
import { Check, IntakeError, accessWasLost, outcomeIsUncertain } from "./ui"

export function SourceSetupDialog({
  client,
  initial,
  setupKinds,
  setupDomains,
  members,
  onClose,
  onChanged,
}: {
  client: BusinessClient
  initial?: BindingAdmin
  setupKinds: SourceKind[]
  setupDomains: BusinessDomain[]
  members: Member[]
  onClose: () => void
  onChanged: () => void
}) {
  const copy = useIntakeCopy()
  const common = useBusinessCopy()
  const [admin, setAdmin] = useState<BindingAdmin | null>(initial ?? null)
  const [kind, setKind] = useState<SourceKind>(initial?.kind ?? setupKinds[0])
  const [label, setLabel] = useState(initial?.label ?? "")
  const [domain, setDomain] = useState<BusinessDomain>(
    initial?.domain ?? setupDomains[0]
  )
  const [owner, setOwner] = useState(initial?.sourceOwnerId ?? "")
  const [secret, setSecret] = useState("")
  const [resource, setResource] = useState("")
  const [destinations, setDestinations] = useState<BusinessDomain[]>(
    initial?.publicationDomains ?? []
  )
  const [retained, setRetained] = useState(initial?.retainedTaskText ?? false)
  const [grants, setGrants] = useState<Page<Grant> | null>(null)
  const [grantee, setGrantee] = useState("")
  const [grantRead, setGrantRead] = useState(false)
  const [grantImport, setGrantImport] = useState(false)
  const [grantTriage, setGrantTriage] = useState(false)
  const [grantDomains, setGrantDomains] = useState<BusinessDomain[]>([])
  const [expiry, setExpiry] = useState("")
  const [confirmed, setConfirmed] = useState(false)
  const [busy, setBusy] = useState(false)
  const [uncertain, setUncertain] = useState(false)
  const [error, setError] = useState<unknown>(null)
  const [leave, setLeave] = useState(false)
  const alive = useRef(true)
  const working = useRef(false)
  const replay = useRef<(() => Promise<BindingAdmin>) | null>(null)
  const owners = members.filter(
    (member) =>
      member.kind === "human" &&
      member.status === "active" &&
      member.domains.includes(domain)
  )
  const currentGrant = grants?.items.find((grant) => grant.memberId === grantee)
  const bindingId = admin?.id
  const validSecret = /^[\x21-\x7e]{1,4096}$/.test(secret)
  const validResource =
    kind === "email"
      ? /^\d+$/.test(resource) &&
        Number(resource) > 0 &&
        Number(resource) <= 2147483647
      : resource.length > 0 && resource.length <= 128
  const validExpiry =
    !expiry ||
    (/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?Z$/.test(expiry) &&
      Date.parse(expiry) > Date.now())
  const validSetup =
    setupKinds.includes(kind) &&
    setupDomains.includes(domain) &&
    destinations.every((value) => setupDomains.includes(value)) &&
    !!label.trim() &&
    (destinations.length === 0 || retained) &&
    (admin
      ? !secret || validSecret
      : !!owner && (kind === "fireflies" ? validSecret : validResource))
  const validGrantDomains = grantDomains.every(
    (value) =>
      setupDomains.includes(value) && admin?.publicationDomains.includes(value)
  )
  useEffect(() => {
    alive.current = true
    return () => {
      alive.current = false
      replay.current = null
    }
  }, [])
  async function loadGrants(bindingId: string, page = 0) {
    try {
      const result = await client.intake("grants/list", { bindingId, page })
      if (alive.current)
        setGrants((old) =>
          page && old
            ? {
                ...result,
                items: [
                  ...old.items,
                  ...result.items.filter(
                    (item) => !old.items.some((prior) => prior.id === item.id)
                  ),
                ],
              }
            : result
        )
    } catch (caught) {
      if (alive.current) {
        setError(caught)
        if (accessWasLost(caught)) {
          setGrants(null)
          setSecret("")
          replay.current = null
        }
      }
    }
  }
  useEffect(() => {
    if (!bindingId) return
    let cancelled = false
    client
      .intake("grants/list", { bindingId })
      .then((result) => {
        if (!cancelled) setGrants(result)
      })
      .catch((caught) => {
        if (!cancelled) setError(caught)
      })
    return () => {
      cancelled = true
    }
  }, [client, bindingId])
  function chooseGrantee(id: string) {
    const grant = grants?.items.find((entry) => entry.memberId === id)
    setGrantee(id)
    setGrantRead(grant?.read ?? false)
    setGrantImport(grant?.import ?? false)
    setGrantTriage(grant?.triage ?? false)
    setGrantDomains(grant?.publicationDomains ?? [])
    setExpiry(grant?.expiresAt ?? "")
    setConfirmed(false)
  }
  async function run(operation: () => Promise<BindingAdmin>) {
    if (working.current) return
    working.current = true
    replay.current = operation
    setBusy(true)
    setError(null)
    setConfirmed(false)
    try {
      const result = await operation()
      if (!alive.current) return
      setAdmin(result)
      setSecret("")
      setUncertain(false)
      replay.current = null
      onChanged()
      await loadGrants(result.id)
    } catch (caught) {
      if (!alive.current) return
      setError(caught)
      setUncertain(outcomeIsUncertain(caught))
      if (!outcomeIsUncertain(caught)) replay.current = null
      if (accessWasLost(caught)) {
        setSecret("")
        setGrants(null)
      }
    } finally {
      working.current = false
      if (alive.current) setBusy(false)
    }
  }
  async function checkSetup() {
    if (!admin || busy) return
    setBusy(true)
    try {
      const result = await client.intake("bindings/status", {
        bindingId: admin.id,
      })
      if (!alive.current) return
      if (!result.admin) {
        setSecret("")
        setGrants(null)
        onClose()
        return
      }
      // Reading does not rewrite the frozen uncertain request. Retry still uses
      // its original revision, operation ID and write-only credential.
      if (!uncertain) {
        setAdmin(result.admin)
        setLabel(result.admin.label)
        setDestinations(result.admin.publicationDomains)
        setRetained(result.admin.retainedTaskText)
      }
      await loadGrants(admin.id)
    } catch (caught) {
      if (alive.current) setError(caught)
    } finally {
      if (alive.current) setBusy(false)
    }
  }
  function saveSetup(enabled?: boolean) {
    if (busy || uncertain || !validSetup) return
    const operationId = crypto.randomUUID()
    if (admin) {
      const input = {
        operationId,
        bindingId: admin.id,
        expectedRevision: admin.revision,
        label,
        enabled: enabled ?? admin.enabled,
        publicationDomains: destinations,
        retainedTaskText: retained,
        ...(secret
          ? { credential: { kind: "fireflies" as const, apiKey: secret } }
          : {}),
      }
      void run(() => client.intake("bindings/update", input))
    } else {
      const source: SourceSetup =
        kind === "fireflies"
          ? { kind, apiKey: secret }
          : kind === "email"
            ? { kind, inboxId: Number(resource) }
            : { kind, productId: resource }
      const input = {
        operationId,
        label,
        domain,
        sourceOwnerId: owner,
        source,
        publicationDomains: destinations,
        retainedTaskText: retained,
      }
      void run(() => client.intake("bindings/create", input))
    }
  }
  return (
    <Modal
      title={admin ? copy.reviewSetup : copy.setup}
      onClose={() => {
        if (!busy) setLeave(true)
      }}
      wide
    >
      <p className="text-muted-foreground text-sm leading-relaxed">
        {copy.disabledFirst}
      </p>
      {error != null && <IntakeError error={error} />}
      {uncertain && (
        <div role="status" className="space-y-3 text-sm">
          <p>{copy.setupUncertain}</p>
          <Action
            variant="outline"
            disabled={busy || !replay.current}
            onClick={() => {
              if (replay.current) void run(replay.current)
            }}
          >
            {copy.retryExact}
          </Action>
        </div>
      )}
      <form
        className="space-y-5"
        onSubmit={(event) => {
          event.preventDefault()
          saveSetup()
        }}
      >
        <fieldset disabled={busy || uncertain} className="space-y-5">
          <Field label={copy.label}>
            {(id) => (
              <Input
                id={id}
                dir="auto"
                value={label}
                onChange={(event) => setLabel(event.target.value)}
                className="min-h-11 rounded-xl"
                maxLength={240}
                required
              />
            )}
          </Field>
          {!admin ? (
            <>
              <div className="grid gap-4 sm:grid-cols-2">
                <Field label={copy.sourceType}>
                  {(id) => (
                    <select
                      id={id}
                      className={controlClass}
                      value={kind}
                      onChange={(event) => {
                        setKind(event.target.value as SourceKind)
                        setSecret("")
                        setResource("")
                      }}
                    >
                      {setupKinds.map((value) => (
                        <option key={value} value={value}>
                          {copy[value]}
                        </option>
                      ))}
                    </select>
                  )}
                </Field>
                <Field label={common.domain}>
                  {(id) => (
                    <select
                      id={id}
                      className={controlClass}
                      value={domain}
                      onChange={(event) => {
                        setDomain(event.target.value as BusinessDomain)
                        setOwner("")
                      }}
                    >
                      {setupDomains.map((value) => (
                        <option key={value} value={value}>
                          {common[value]}
                        </option>
                      ))}
                    </select>
                  )}
                </Field>
              </div>
              <Field label={copy.sourceOwner} hint={copy.sourceOwnerHint}>
                {(id) => (
                  <select
                    id={id}
                    className={controlClass}
                    value={owner}
                    onChange={(event) => setOwner(event.target.value)}
                    required
                  >
                    <option value="">{copy.sourceOwner}</option>
                    {owners.map((member) => (
                      <option key={member.id} value={member.id}>
                        {member.displayName}
                      </option>
                    ))}
                  </select>
                )}
              </Field>
            </>
          ) : (
            <dl className="grid gap-4 text-sm sm:grid-cols-2">
              <div>
                <dt className="text-muted-foreground">{copy.sourceType}</dt>
                <dd>{copy[admin.kind]}</dd>
              </div>
              <div>
                <dt className="text-muted-foreground">{common.domain}</dt>
                <dd>{common[admin.domain]}</dd>
              </div>
              <div>
                <dt className="text-muted-foreground">{copy.sourceOwner}</dt>
                <dd>
                  {members.find((member) => member.id === admin.sourceOwnerId)
                    ?.displayName ?? common.memberUnavailable}
                </dd>
              </div>
              <div>
                <dt className="text-muted-foreground">{common.status}</dt>
                <dd>
                  {admin.enabled ? copy.enabled : copy.disabled} ·{" "}
                  {common.sourceVersion} {admin.revision}
                </dd>
              </div>
            </dl>
          )}
          {kind === "fireflies" ? (
            <Field
              label={admin ? copy.replaceCredential : copy.apiKey}
              hint={copy.secretHint}
            >
              {(id) => (
                <Input
                  id={id}
                  type="password"
                  autoComplete="new-password"
                  dir="ltr"
                  value={secret}
                  onChange={(event) => setSecret(event.target.value)}
                  maxLength={4096}
                  required={!admin}
                  className="min-h-11 rounded-xl"
                />
              )}
            </Field>
          ) : (
            !admin && (
              <Field
                label={kind === "email" ? copy.inboxId : copy.productId}
                hint={copy.existingSetup}
              >
                {(id) => (
                  <Input
                    id={id}
                    value={resource}
                    onChange={(event) => setResource(event.target.value)}
                    dir="ltr"
                    maxLength={128}
                    className="min-h-11 rounded-xl"
                    required
                  />
                )}
              </Field>
            )
          )}
          <fieldset className="space-y-3">
            <legend className="mb-2 text-sm font-medium">
              {copy.publicationAreas}
            </legend>
            <p className="text-muted-foreground text-sm">
              {copy.publicationHint}
            </p>
            {destinations.some((value) => !setupDomains.includes(value)) && (
              <p role="status" className="text-sm">
                {copy.removeUnavailableDestinations}
              </p>
            )}
            <div className="grid gap-2 sm:grid-cols-2">
              {BUSINESS_DOMAINS.filter(
                (value) =>
                  setupDomains.includes(value) || destinations.includes(value)
              ).map((value) => (
                <Check
                  key={value}
                  checked={destinations.includes(value)}
                  onChange={(checked) =>
                    setDestinations(
                      checked
                        ? [...destinations, value]
                        : destinations.filter((entry) => entry !== value)
                    )
                  }
                >
                  {common[value]}
                </Check>
              ))}
            </div>
          </fieldset>
          <Check checked={retained} onChange={setRetained}>
            {copy.retainedConsent}
          </Check>
        </fieldset>
        <div className="flex flex-wrap gap-2">
          <Action type="submit" disabled={busy || uncertain || !validSetup}>
            {admin ? common.save : copy.createConnection}
          </Action>
          {admin && (
            <Action
              variant="outline"
              disabled={busy || uncertain || !validSetup}
              onClick={() => saveSetup(!admin.enabled)}
            >
              {admin.enabled ? copy.disable : copy.enable}
            </Action>
          )}
          {admin && (
            <Action
              variant="outline"
              disabled={busy || uncertain || !validSetup}
              onClick={() => saveSetup()}
            >
              {copy.revalidate}
            </Action>
          )}
          {admin && (
            <Action
              variant="ghost"
              disabled={busy}
              onClick={() => void checkSetup()}
            >
              {common.refresh}
            </Action>
          )}
        </div>
        {admin && (
          <p className="text-muted-foreground text-sm leading-relaxed">
            {copy.revalidateHint}
          </p>
        )}
      </form>
      {admin && (
        <section
          className="border-border space-y-4 border-t pt-5"
          aria-label={copy.grants}
        >
          <h2 className="font-semibold">{copy.grants}</h2>
          {grants?.items.length === 0 && (
            <p className="text-muted-foreground text-sm">{copy.noGrants}</p>
          )}
          <ul className="divide-border divide-y">
            {grants?.items.map((grant) => (
              <li
                key={grant.id}
                className="flex flex-wrap items-center justify-between gap-3 py-3 text-sm"
              >
                <div className="min-w-0">
                  <p className="break-words">
                    {members.find((member) => member.id === grant.memberId)
                      ?.displayName ?? common.memberUnavailable}
                  </p>
                  <p className="text-muted-foreground text-xs">
                    {grant.state === "active" ? copy.grantActive : copy.revoked}{" "}
                    · {common.sourceVersion} {grant.revision}
                  </p>
                </div>
                {grant.state === "active" && (
                  <Action
                    variant="outline"
                    disabled={busy || uncertain}
                    onClick={() => {
                      const input = {
                        operationId: crypto.randomUUID(),
                        bindingId: admin.id,
                        expectedBindingRevision: admin.revision,
                        grantId: grant.id,
                        expectedGrantRevision: grant.revision,
                      }
                      void run(
                        async () =>
                          (await client.intake("grants/revoke", input)).binding
                      )
                    }}
                  >
                    {copy.grantRevoke}
                  </Action>
                )}
              </li>
            ))}
          </ul>
          {grants?.hasMore && (
            <Action
              variant="outline"
              disabled={busy}
              onClick={() => void loadGrants(admin.id, grants.page + 1)}
            >
              {common.more}
            </Action>
          )}
          <form
            className="space-y-4"
            onSubmit={(event) => {
              event.preventDefault()
              if (
                !grantee ||
                !confirmed ||
                !validExpiry ||
                !validGrantDomains ||
                busy ||
                uncertain ||
                !grantRead
              )
                return
              const input = {
                operationId: crypto.randomUUID(),
                bindingId: admin.id,
                expectedBindingRevision: admin.revision,
                memberId: grantee,
                expectedGrantRevision: currentGrant?.revision ?? null,
                scope: "binding_current_and_future_sources" as const,
                read: grantRead,
                import: grantImport,
                triage: grantTriage,
                publicationDomains: grantDomains,
                expiresAt: expiry || null,
              }
              void run(
                async () =>
                  (await client.intake("grants/upsert", input)).binding
              )
            }}
          >
            <fieldset
              disabled={busy || uncertain}
              className="space-y-4"
              onChange={() => setConfirmed(false)}
            >
              <Field label={copy.grantPerson}>
                {(id) => (
                  <select
                    id={id}
                    className={controlClass}
                    value={grantee}
                    onChange={(event) => chooseGrantee(event.target.value)}
                    required
                  >
                    <option value="">{copy.grantPerson}</option>
                    {members
                      .filter(
                        (member) =>
                          member.kind === "human" &&
                          member.status === "active" &&
                          member.domains.includes(admin.domain)
                      )
                      .map((member) => (
                        <option key={member.id} value={member.id}>
                          {member.displayName}
                        </option>
                      ))}
                  </select>
                )}
              </Field>
              <Check
                checked={grantRead}
                onChange={(value) => {
                  setGrantRead(value)
                  if (!value) {
                    setGrantImport(false)
                    setGrantTriage(false)
                    setGrantDomains([])
                  }
                }}
              >
                {copy.grantRead}
              </Check>
              <Check
                checked={grantImport}
                disabled={
                  !grantRead ||
                  members.find((member) => member.id === grantee)?.role ===
                    "viewer"
                }
                onChange={setGrantImport}
              >
                {copy.grantImport}
              </Check>
              <Check
                checked={grantTriage}
                disabled={
                  !grantRead ||
                  members.find((member) => member.id === grantee)?.role ===
                    "viewer"
                }
                onChange={(value) => {
                  setGrantTriage(value)
                  if (!value) setGrantDomains([])
                }}
              >
                {copy.grantTriage}
              </Check>
              <fieldset className="space-y-2">
                <legend className="mb-2 text-sm font-medium">
                  {copy.publicationAreas}
                </legend>
                {!validGrantDomains && (
                  <p role="status" className="text-sm">
                    {copy.removeUnavailableDestinations}
                  </p>
                )}
                {BUSINESS_DOMAINS.filter(
                  (value) =>
                    (setupDomains.includes(value) &&
                      admin.publicationDomains.includes(value)) ||
                    grantDomains.includes(value)
                ).map((value) => (
                  <Check
                    key={value}
                    checked={grantDomains.includes(value)}
                    disabled={!grantRead || !grantTriage}
                    onChange={(checked) =>
                      setGrantDomains(
                        checked
                          ? [...grantDomains, value]
                          : grantDomains.filter((entry) => entry !== value)
                      )
                    }
                  >
                    {common[value]}
                  </Check>
                ))}
              </fieldset>
              <Field label={copy.grantExpires} hint={copy.grantExpiryHint}>
                {(id) => (
                  <Input
                    id={id}
                    value={expiry}
                    onChange={(event) => setExpiry(event.target.value)}
                    dir="ltr"
                    maxLength={40}
                    className="min-h-11 rounded-xl"
                  />
                )}
              </Field>
            </fieldset>
            <Check
              checked={confirmed}
              disabled={busy || uncertain}
              onChange={setConfirmed}
            >
              {copy.grantConfirm}
            </Check>
            <Action
              type="submit"
              disabled={
                busy ||
                uncertain ||
                !grantee ||
                !grantRead ||
                !validExpiry ||
                !validGrantDomains ||
                !confirmed
              }
            >
              {copy.grantSave}
            </Action>
          </form>
        </section>
      )}
      {leave && (
        <Modal title={common.discardTitle} onClose={() => setLeave(false)}>
          <p className="text-sm leading-relaxed">
            {uncertain ? copy.setupUncertain : common.discardHint}
          </p>
          <div className="flex flex-wrap gap-2">
            <Action variant="outline" onClick={() => setLeave(false)}>
              {common.stay}
            </Action>
            <Action
              variant="destructive"
              disabled={uncertain}
              onClick={() => {
                setSecret("")
                replay.current = null
                onClose()
              }}
            >
              {common.close}
            </Action>
          </div>
        </Modal>
      )}
    </Modal>
  )
}
