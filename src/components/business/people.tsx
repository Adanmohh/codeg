"use client"

import { useState } from "react"
import { Plus, Users } from "lucide-react"
import { Input } from "@/components/ui/input"
import { BusinessError, type BusinessClient } from "@/lib/business/client"
import { useBusinessCopy } from "@/lib/business/copy"
import {
  MEMBER_ROLES,
  type BusinessContext,
  type Credential,
  type Member,
  type MemberFields,
  type MemberRole,
} from "@/lib/business/identity"
import { BUSINESS_DOMAINS } from "@/lib/business/presentation"
import {
  Action,
  controlClass,
  ErrorNotice,
  Field,
  Modal,
  Person,
  roleLabel,
} from "./ui"

export function People({
  client,
  context,
  members,
  reload,
}: {
  client: BusinessClient
  context: BusinessContext
  members: Member[]
  reload: () => Promise<void>
}) {
  const copy = useBusinessCopy()
  const [selected, setSelected] = useState<Member | "new" | null>(null)
  return (
    <section className="space-y-8">
      <header className="flex flex-wrap items-end justify-between gap-5">
        <div>
          <p className="text-primary mb-3 text-sm font-semibold">
            {context.organization?.name}
          </p>
          <h1 className="text-3xl font-semibold tracking-tight">{copy.team}</h1>
          <p className="text-muted-foreground mt-3 max-w-xl text-sm leading-relaxed">
            {copy.teamHint}
          </p>
        </div>
        {context.capabilities.manageMembers && (
          <Action onClick={() => setSelected("new")}>
            <Plus aria-hidden="true" />
            {copy.addMember}
          </Action>
        )}
      </header>
      <div className="border-border bg-card overflow-hidden rounded-2xl border">
        <ul className="divide-y">
          {members.map((member) => (
            <li key={member.id}>
              <button
                type="button"
                className="hover:bg-muted/40 focus-visible:ring-ring flex w-full min-w-0 flex-wrap items-center gap-4 p-5 text-start outline-none focus-visible:ring-2 focus-visible:ring-inset"
                onClick={() => setSelected(member)}
              >
                <div className="min-w-0 flex-1">
                  <Person
                    person={{ name: member.displayName, kind: member.kind }}
                    fallback=""
                  />
                  <p className="text-muted-foreground mt-2 text-xs">
                    {roleLabel(member.role, copy)} ·{" "}
                    {member.operatorOwner
                      ? copy.protectedOwner
                      : copy[member.status]}
                  </p>
                </div>
                <div className="flex max-w-full flex-wrap gap-2">
                  {member.domains.map((domain) => (
                    <span
                      key={domain}
                      className="bg-muted rounded-md px-2 py-1 text-xs"
                    >
                      {copy[domain]}
                    </span>
                  ))}
                </div>
              </button>
            </li>
          ))}
        </ul>
        {members.length === 0 && (
          <div className="text-muted-foreground flex items-center gap-3 p-6 text-sm">
            <Users className="size-5" aria-hidden="true" />
            {copy.noMembers}
          </div>
        )}
      </div>
      {selected && (
        <MemberEditor
          key={selected === "new" ? "new" : selected.id}
          client={client}
          context={context}
          member={selected === "new" ? null : selected}
          onClose={() => setSelected(null)}
          onChanged={reload}
        />
      )}
    </section>
  )
}

function MemberEditor({
  client,
  context,
  member,
  onClose,
  onChanged,
}: {
  client: BusinessClient
  context: BusinessContext
  member: Member | null
  onClose: () => void
  onChanged: () => Promise<void>
}) {
  const copy = useBusinessCopy()
  const organizationId = context.organization!.id
  const [base, setBase] = useState(member)
  const [draft, setDraft] = useState<MemberFields>({
    displayName: member?.displayName ?? "",
    role: member?.role ?? "member",
    domains: member?.domains ?? [],
  })
  const [kind, setKind] = useState<Member["kind"]>(member?.kind ?? "human")
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<unknown>(null)
  const [conflicted, setConflicted] = useState(false)
  const [current, setCurrent] = useState<Member | null>(null)
  const [credentials, setCredentials] = useState<Credential[] | null>(null)
  const [label, setLabel] = useState("")
  const [secret, setSecret] = useState<string | null>(null)
  const [revoke, setRevoke] = useState<Credential | "member" | null>(null)
  const [confirmDiscard, setConfirmDiscard] = useState(false)
  const canManage =
    context.capabilities.manageMembers &&
    (!base ||
      context.member?.role === "owner" ||
      (!["owner", "admin"].includes(base.role) &&
        base.domains.every((domain) =>
          context.member?.domains.includes(domain)
        )))
  const canEdit =
    canManage && (!base || (!base.operatorOwner && base.status === "active"))
  const canCredentials =
    !!base && kind === "human" && (canManage || base.id === context.member?.id)
  const dirty =
    draft.displayName !== (base?.displayName ?? "") ||
    draft.role !== (base?.role ?? "member") ||
    JSON.stringify(draft.domains) !== JSON.stringify(base?.domains ?? []) ||
    (!base && kind !== "human")
  function close() {
    if (busy) return
    if (dirty) setConfirmDiscard(true)
    else onClose()
  }
  async function run(action: () => Promise<void>) {
    setBusy(true)
    setError(null)
    try {
      await action()
    } catch (caught) {
      setError(caught)
      setRevoke(null)
      if (caught instanceof BusinessError && caught.kind === "conflict")
        setConflicted(true)
    } finally {
      setBusy(false)
    }
  }
  const conflict = conflicted
  return (
    <Modal
      title={base ? copy.memberAccess : copy.addMember}
      description={copy.memberAccessHint}
      onClose={close}
      wide
    >
      <form
        className="space-y-5"
        onSubmit={(event) => {
          event.preventDefault()
          void run(async () => {
            const fields = {
              displayName: draft.displayName.trim(),
              role: kind === "agent" ? ("member" as const) : draft.role,
              domains: draft.domains,
            }
            const saved = base
              ? await client.identity("members/update", {
                  ...fields,
                  organizationId,
                  memberId: base.id,
                  expectedRevision: base.revision,
                })
              : await client.identity("members/create", {
                  ...fields,
                  organizationId,
                  kind,
                })
            setBase(saved)
            setDraft({
              displayName: saved.displayName,
              role: saved.role,
              domains: saved.domains,
            })
            await onChanged()
          })
        }}
      >
        <Field label={copy.displayName}>
          {(id) => (
            <Input
              id={id}
              className="min-h-11 rounded-xl"
              value={draft.displayName}
              onChange={(event) =>
                setDraft({ ...draft, displayName: event.target.value })
              }
              maxLength={120}
              required
              disabled={!canEdit || busy}
            />
          )}
        </Field>
        <div className="grid gap-5 sm:grid-cols-2">
          <Field label={copy.kind}>
            {(id) => (
              <select
                id={id}
                className={controlClass}
                value={kind}
                disabled={!!base || busy || !canEdit}
                onChange={(event) => {
                  setKind(event.target.value as Member["kind"])
                  if (event.target.value === "agent")
                    setDraft({ ...draft, role: "member" })
                }}
              >
                <option value="human">{copy.human}</option>
                <option value="agent">{copy.agent}</option>
              </select>
            )}
          </Field>
          <Field label={copy.role}>
            {(id) => (
              <select
                id={id}
                className={controlClass}
                value={draft.role}
                onChange={(event) =>
                  setDraft({ ...draft, role: event.target.value as MemberRole })
                }
                disabled={!canEdit || busy || kind === "agent"}
              >
                {MEMBER_ROLES.filter(
                  (role) =>
                    !canEdit ||
                    context.member?.role === "owner" ||
                    !["owner", "admin"].includes(role)
                ).map((role) => (
                  <option key={role} value={role}>
                    {roleLabel(role, copy)}
                  </option>
                ))}
              </select>
            )}
          </Field>
        </div>
        <fieldset disabled={!canEdit || busy} className="space-y-2">
          <legend className="mb-2 text-sm font-medium">{copy.domain}</legend>
          <div className="grid grid-cols-2 gap-x-4 sm:grid-cols-3">
            {BUSINESS_DOMAINS.map((domain) => (
              <label
                key={domain}
                className="flex min-h-11 cursor-pointer items-center gap-3 text-sm"
              >
                <input
                  type="checkbox"
                  className="accent-primary size-4"
                  checked={draft.domains.includes(domain)}
                  disabled={
                    context.member?.role !== "owner" &&
                    !context.member?.domains.includes(domain)
                  }
                  onChange={(event) =>
                    setDraft({
                      ...draft,
                      domains: event.target.checked
                        ? [...draft.domains, domain]
                        : draft.domains.filter((value) => value !== domain),
                    })
                  }
                />
                {copy[domain]}
              </label>
            ))}
          </div>
          <p className="text-muted-foreground text-xs leading-relaxed">
            {copy.domainsHint}
          </p>
        </fieldset>
        {error != null && (
          <ErrorNotice error={error}>
            {conflict && (
              <>
                <p>{copy.revisionHint}</p>
                <Action
                  type="button"
                  variant="outline"
                  disabled={busy}
                  onClick={() =>
                    void run(async () => {
                      const found = (
                        await client.identity("members/list", {
                          organizationId,
                        })
                      ).find((candidate) => candidate.id === base?.id)
                      if (!found) throw new BusinessError("missing")
                      setCurrent(found)
                    })
                  }
                >
                  {copy.loadCurrent}
                </Action>
              </>
            )}
          </ErrorNotice>
        )}
        {current && (
          <div className="bg-muted/40 space-y-3 rounded-xl border p-4 text-sm">
            <p className="font-medium">{copy.currentVersion}</p>
            <p>
              <bdi>{current.displayName}</bdi> · {roleLabel(current.role, copy)}{" "}
              · {copy[current.status]}
            </p>
            <p>{current.domains.map((domain) => copy[domain]).join(" · ")}</p>
            <Action
              type="button"
              variant="outline"
              disabled={current.status !== "active" || busy}
              onClick={() => {
                setBase(current)
                setCurrent(null)
                setError(null)
                setConflicted(false)
              }}
            >
              {copy.keepDraft}
            </Action>
          </div>
        )}
        {canEdit && (
          <div className="flex justify-end">
            <Action
              type="submit"
              disabled={
                busy ||
                conflict ||
                !!current ||
                !dirty ||
                !draft.displayName.trim() ||
                draft.domains.length === 0
              }
            >
              {busy ? copy.saving : copy.save}
            </Action>
          </div>
        )}
      </form>
      {base && (
        <section className="space-y-4 border-t pt-5">
          <h3 className="font-medium">{copy.credentials}</h3>
          {kind === "agent" ? (
            <p className="text-muted-foreground text-sm leading-relaxed">
              {copy.agentAccessHint}
            </p>
          ) : (
            canCredentials && (
              <>
                {credentials === null ? (
                  <Action
                    variant="outline"
                    disabled={busy}
                    onClick={() =>
                      void run(async () => {
                        setCredentials(
                          await client.identity("credentials/list", {
                            organizationId,
                            memberId: base.id,
                          })
                        )
                      })
                    }
                  >
                    {copy.credentials}
                  </Action>
                ) : (
                  <ul className="divide-y rounded-xl border">
                    {credentials.map((credential) => (
                      <li
                        key={credential.id}
                        className="flex flex-wrap items-center justify-between gap-3 p-3 text-sm"
                      >
                        <span className="min-w-0 break-words">
                          <bdi>{credential.label}</bdi> ·{" "}
                          {credential.revokedAt ? copy.revoked : copy.active}
                        </span>
                        {!credential.revokedAt && (
                          <Action
                            variant="outline"
                            disabled={busy}
                            onClick={() => setRevoke(credential)}
                          >
                            {copy.revokeCredential}
                          </Action>
                        )}
                      </li>
                    ))}
                    {credentials.length === 0 && (
                      <li className="text-muted-foreground p-4 text-sm">
                        {copy.noCredentials}
                      </li>
                    )}
                  </ul>
                )}
                {canManage && base.status === "active" && (
                  <form
                    className="flex flex-col items-stretch gap-3 sm:flex-row sm:items-end"
                    onSubmit={(event) => {
                      event.preventDefault()
                      void run(async () => {
                        const result = await client.identity(
                          "credentials/issue",
                          {
                            organizationId,
                            memberId: base.id,
                            label: label.trim(),
                          }
                        )
                        setSecret(result.token)
                        setCredentials((previous) =>
                          previous ? [...previous, result.credential] : null
                        )
                        setLabel("")
                      })
                    }}
                  >
                    <div className="min-w-0 flex-1">
                      <Field label={copy.credentialLabel}>
                        {(id) => (
                          <Input
                            id={id}
                            className="min-h-11 rounded-xl"
                            value={label}
                            onChange={(event) => setLabel(event.target.value)}
                            required
                            maxLength={120}
                            disabled={busy}
                          />
                        )}
                      </Field>
                    </div>
                    <Action type="submit" disabled={busy || !label.trim()}>
                      {copy.issueCredential}
                    </Action>
                  </form>
                )}
              </>
            )
          )}
          {canManage && !base.operatorOwner && base.status === "active" && (
            <Action
              variant="destructive"
              disabled={busy}
              onClick={() => setRevoke("member")}
            >
              {copy.revokeMember}
            </Action>
          )}
        </section>
      )}
      {secret && (
        <OneTimeCredential token={secret} onClose={() => setSecret(null)} />
      )}
      {revoke && (
        <Modal
          title={copy.confirmRevoke}
          description={copy.confirmRevokeHint}
          onClose={() => {
            if (!busy) setRevoke(null)
          }}
        >
          <div className="flex flex-wrap justify-end gap-3">
            <Action
              variant="outline"
              disabled={busy}
              onClick={() => setRevoke(null)}
            >
              {copy.cancel}
            </Action>
            <Action
              variant="destructive"
              disabled={busy}
              onClick={() =>
                void run(async () => {
                  if (revoke === "member") {
                    const saved = await client.identity("members/revoke", {
                      organizationId,
                      memberId: base!.id,
                      expectedRevision: base!.revision,
                    })
                    setBase(saved)
                    await onChanged()
                  } else {
                    const saved = await client.identity("credentials/revoke", {
                      organizationId,
                      credentialId: revoke.id,
                    })
                    setCredentials(
                      (previous) =>
                        previous?.map((item) =>
                          item.id === saved.id ? saved : item
                        ) ?? null
                    )
                  }
                  setRevoke(null)
                })
              }
            >
              {copy.revoke}
            </Action>
          </div>
        </Modal>
      )}
      {confirmDiscard && (
        <Modal
          title={copy.discardTitle}
          description={copy.discardHint}
          onClose={() => setConfirmDiscard(false)}
        >
          <div className="flex flex-wrap justify-end gap-3">
            <Action variant="outline" onClick={() => setConfirmDiscard(false)}>
              {copy.stay}
            </Action>
            <Action variant="destructive" onClick={onClose}>
              {copy.discardDraft}
            </Action>
          </div>
        </Modal>
      )}
    </Modal>
  )
}

export function OneTimeCredential({
  token,
  onClose,
}: {
  token: string
  onClose: () => void
}) {
  const copy = useBusinessCopy()
  const [revealed, setRevealed] = useState(false)
  const [copied, setCopied] = useState(false)
  const [failed, setFailed] = useState(false)
  return (
    <Modal
      title={copy.secretTitle}
      description={copy.secretHint}
      onClose={onClose}
    >
      {/* The real token is absent from the DOM until deliberate reveal. */}
      {revealed ? (
        <Input
          aria-label={copy.memberToken}
          type="text"
          dir="ltr"
          readOnly
          value={token}
          className="min-h-11 rounded-xl font-mono"
          autoComplete="off"
          spellCheck={false}
        />
      ) : (
        <p className="bg-muted rounded-xl p-4 font-mono" aria-label={copy.hide}>
          •••• •••• •••• ••••
        </p>
      )}
      <div className="flex flex-wrap gap-3">
        <Action variant="outline" onClick={() => setRevealed(!revealed)}>
          {revealed ? copy.hide : copy.reveal}
        </Action>
        <Action
          onClick={async () => {
            try {
              await navigator.clipboard.writeText(token)
              setCopied(true)
              setFailed(false)
            } catch {
              setFailed(true)
            }
          }}
        >
          {copied ? copy.copied : copy.copy}
        </Action>
      </div>
      {failed && (
        <p role="alert" className="text-sm">
          {copy.copyFailed}
        </p>
      )}
      <Action variant="outline" onClick={onClose}>
        {copy.finishToken}
      </Action>
    </Modal>
  )
}
