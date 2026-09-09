"use client"

import { useEffect, useRef, useState } from "react"
import { Input } from "@/components/ui/input"
import { BusinessError } from "@/lib/business/client"
import type {
  TenantSettings,
  TenantSettingsAccess,
  TenantSettingsView,
} from "@/lib/business/settings"
import { isTenantSettingsView } from "@/lib/business/settings"
import { useSettingsCopy } from "@/lib/business/settings-copy"
import { Action, controlClass, Field } from "./ui"

/** Task-editor CAS/recovery glue. The host supplies the authenticated settings
 * seam and projected capability; no credential, actor or tenant selector here.
 */
export function SettingsEditor({
  initial,
  access,
  canManage,
  onApplied,
}: {
  initial: TenantSettingsView
  access: TenantSettingsAccess
  canManage: boolean
  onApplied: (value: TenantSettingsView) => void
}) {
  const copy = useSettingsCopy()
  const [base, setBase] = useState(initial)
  const [draft, setDraft] = useState(initial.settings)
  const [current, setCurrent] = useState<TenantSettingsView | null>(null)
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<unknown>(null)
  const [uncertain, setUncertain] = useState(false)
  const [saved, setSaved] = useState(false)
  const generation = useRef(0)
  useEffect(() => {
    const version = ++generation.current
    return () => {
      generation.current = version + 1
    }
  }, [access, initial.organizationId])
  const locked = !canManage || busy || uncertain || !!current
  const dirty = JSON.stringify(draft) !== JSON.stringify(base.settings)
  const validName =
    !!draft.displayName.trim() &&
    Array.from(draft.displayName.trim()).length <= 120
  function verified(value: TenantSettingsView): TenantSettingsView {
    if (!isTenantSettingsView(value, initial.organizationId))
      throw new BusinessError("forbidden")
    return value
  }
  async function save() {
    if (locked || !dirty || !validName) return
    const version = generation.current
    setBusy(true)
    setSaved(false)
    setError(null)
    try {
      const value = verified(
        await access.update({
          expectedRevision: base.revision,
          settings: { ...draft, displayName: draft.displayName.trim() },
        })
      )
      if (version !== generation.current) return
      setBase(value)
      setDraft(value.settings)
      setSaved(true)
      onApplied(value)
    } catch (caught) {
      if (version !== generation.current) return
      setError(caught)
      // A lost response may have committed. Read and explicitly adopt the
      // current revision; never invent a successful save or blind new revision.
      if (
        !(caught instanceof BusinessError) ||
        ["offline", "conflict"].includes(caught.kind)
      )
        setUncertain(true)
    } finally {
      if (version === generation.current) setBusy(false)
    }
  }
  async function loadCurrent() {
    if (busy) return
    const version = generation.current
    setBusy(true)
    setError(null)
    try {
      const value = verified(await access.get())
      if (version === generation.current) setCurrent(value)
    } catch (caught) {
      if (version === generation.current) setError(caught)
    } finally {
      if (version === generation.current) setBusy(false)
    }
  }
  function adopt(keepDraft: boolean) {
    if (!current || busy) return
    setBase(current)
    if (!keepDraft) setDraft(current.settings)
    onApplied(current)
    setCurrent(null)
    setError(null)
    setUncertain(false)
    setSaved(false)
  }
  return (
    <section className="mx-auto max-w-2xl space-y-6 p-5 sm:p-6">
      <header className="space-y-2">
        <h1 className="text-2xl font-semibold tracking-tight">{copy.title}</h1>
        <p className="text-muted-foreground text-sm leading-relaxed">
          {copy.hint}
        </p>
      </header>
      {!canManage && (
        <p className="bg-muted/40 rounded-xl border p-4 text-sm">
          {copy.readOnly}
        </p>
      )}
      <p className="text-muted-foreground text-xs tabular-nums">
        {copy.base} {base.revision}
      </p>
      {error != null && (
        <p
          role="alert"
          className="border-destructive/25 bg-destructive/5 rounded-xl border p-4 text-sm"
        >
          {error instanceof BusinessError && error.kind === "forbidden"
            ? copy.forbidden
            : error instanceof BusinessError && error.kind === "invalid"
              ? copy.invalid
              : copy.failed}
        </p>
      )}
      {uncertain && (
        <div className="space-y-3 rounded-xl border p-4">
          <p className="text-sm leading-relaxed">{copy.conflict}</p>
          {!current && (
            <Action
              variant="outline"
              disabled={busy}
              onClick={() => void loadCurrent()}
            >
              {copy.load}
            </Action>
          )}
        </div>
      )}
      {current && (
        <section
          aria-label={copy.current}
          className="bg-muted/30 space-y-4 rounded-xl border p-4"
        >
          <h2 className="font-semibold">{copy.current}</h2>
          <p className="text-muted-foreground text-xs tabular-nums">
            {copy.revision} {current.revision}
          </p>
          <SettingsValues value={current.settings} />
          <div className="flex flex-wrap gap-3">
            <Action disabled={!canManage || busy} onClick={() => adopt(true)}>
              {copy.keep}
            </Action>
            <Action
              variant="outline"
              disabled={busy}
              onClick={() => adopt(false)}
            >
              {copy.useSaved}
            </Action>
          </div>
        </section>
      )}
      <form
        className="space-y-5"
        onSubmit={(event) => {
          event.preventDefault()
          void save()
        }}
      >
        <fieldset disabled={locked} className="min-w-0 space-y-5">
          <Field label={copy.name}>
            {(id) => (
              <Input
                id={id}
                className="min-h-11"
                value={draft.displayName}
                maxLength={240}
                required
                dir="auto"
                onChange={(event) => {
                  setSaved(false)
                  setDraft({ ...draft, displayName: event.target.value })
                }}
              />
            )}
          </Field>
          <Field label={copy.palette}>
            {(id) => (
              <select
                id={id}
                className={controlClass}
                value={draft.palette}
                onChange={(event) => {
                  const palette = (["neutral", "blue", "violet"] as const).find(
                    (value) => value === event.target.value
                  )
                  if (palette) {
                    setSaved(false)
                    setDraft({ ...draft, palette })
                  }
                }}
              >
                {(["neutral", "blue", "violet"] as const).map((value) => (
                  <option key={value} value={value}>
                    {copy[value]}
                  </option>
                ))}
              </select>
            )}
          </Field>
          <Field label={copy.layout}>
            {(id) => (
              <select
                id={id}
                className={controlClass}
                value={draft.workspaceLayout}
                onChange={(event) => {
                  const workspaceLayout = (["split", "stacked"] as const).find(
                    (value) => value === event.target.value
                  )
                  if (workspaceLayout) {
                    setSaved(false)
                    setDraft({ ...draft, workspaceLayout })
                  }
                }}
              >
                {(["split", "stacked"] as const).map((value) => (
                  <option key={value} value={value}>
                    {copy[value]}
                  </option>
                ))}
              </select>
            )}
          </Field>
          <Field label={copy.defaultArea} hint={copy.conversationHint}>
            {(id) => (
              <select
                id={id}
                className={controlClass}
                value={draft.defaultWorkArea}
                onChange={(event) => {
                  const defaultWorkArea = (
                    ["tasks", "conversations"] as const
                  ).find((value) => value === event.target.value)
                  if (defaultWorkArea) {
                    setSaved(false)
                    setDraft({ ...draft, defaultWorkArea })
                  }
                }}
              >
                {(["tasks", "conversations"] as const).map((value) => (
                  <option key={value} value={value}>
                    {copy[value]}
                  </option>
                ))}
              </select>
            )}
          </Field>
        </fieldset>
        {saved && (
          <p role="status" className="text-sm">
            {copy.saved}
          </p>
        )}
        {canManage && (
          <div className="flex justify-end">
            <Action type="submit" disabled={locked || !dirty || !validName}>
              {busy ? copy.saving : copy.save}
            </Action>
          </div>
        )}
      </form>
    </section>
  )
}

function SettingsValues({ value }: { value: TenantSettings }) {
  const copy = useSettingsCopy()
  return (
    <dl className="grid gap-3 text-sm">
      <div>
        <dt className="text-muted-foreground text-xs">{copy.name}</dt>
        <dd className="mt-1 break-words">
          <bdi>{value.displayName}</bdi>
        </dd>
      </div>
      <div>
        <dt className="text-muted-foreground text-xs">{copy.palette}</dt>
        <dd className="mt-1">{copy[value.palette]}</dd>
      </div>
      <div>
        <dt className="text-muted-foreground text-xs">{copy.layout}</dt>
        <dd className="mt-1">{copy[value.workspaceLayout]}</dd>
      </div>
      <div>
        <dt className="text-muted-foreground text-xs">{copy.defaultArea}</dt>
        <dd className="mt-1">{copy[value.defaultWorkArea]}</dd>
      </div>
    </dl>
  )
}
