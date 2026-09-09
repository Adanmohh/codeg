"use client"

import { useCallback, useEffect, useRef, useState } from "react"
import { BookOpen, Settings2 } from "lucide-react"
import { Action, Modal, controlClass } from "@/components/business/ui"
import type { BusinessClient } from "@/lib/business/client"
import { useBusinessCopy } from "@/lib/business/copy"
import { useIntakeCopy } from "@/lib/business/intake-copy"
import type { Member } from "@/lib/business/identity"
import type {
  BindingList,
  BindingView,
  Page,
  SourceSummary,
} from "@/lib/business/intake"
import { IntakeError, PrivateNotice, accessWasLost } from "./ui"
import { ImportPanel } from "./imports"
import { SourceSetupDialog } from "./setup"
import { SourceReview } from "./source-review"

export interface SourceEntry {
  sourceId: string
  bindingId: string
}
type SetupSession = Pick<BindingList, "setupKinds" | "setupDomains"> & {
  target: "new" | "current"
}
export function SourcesWorkspace({
  client,
  actor,
  members,
  active,
  entry,
  onEntryRead,
  onTask,
}: {
  client: BusinessClient
  actor: Member
  members: Member[]
  active: boolean
  entry?: SourceEntry | null
  onEntryRead: () => void
  onTask: (id: string) => void
}) {
  const copy = useIntakeCopy()
  const common = useBusinessCopy()
  const [list, setList] = useState<BindingList | null>(null)
  const [binding, setBinding] = useState<BindingView | null>(null)
  const [sources, setSources] = useState<Page<SourceSummary> | null>(null)
  const [sourceId, setSourceId] = useState<string | null>(null)
  const [error, setError] = useState<unknown>(null)
  const [loading, setLoading] = useState(true)
  const [setup, setSetup] = useState<SetupSession | null>(null)
  const [dirty, setDirty] = useState(false)
  const [leave, setLeave] = useState<(() => void) | null>(null)
  const [acceptedEntry, setAcceptedEntry] = useState<SourceEntry | null>(null)
  const setupKinds = list?.setupKinds ?? []
  const setupDomains = list?.setupDomains ?? []
  const canSetup =
    !!list?.canManageSetup && setupKinds.length > 0 && setupDomains.length > 0
  const canReviewSetup =
    canSetup &&
    !!binding?.admin &&
    setupKinds.includes(binding.admin.kind) &&
    setupDomains.includes(binding.admin.domain)
  const setupAllowed =
    !!setup &&
    canSetup &&
    setup.setupKinds.every((kind) => setupKinds.includes(kind)) &&
    setup.setupDomains.every((domain) => setupDomains.includes(domain)) &&
    (setup.target === "new" || canReviewSetup)
  useEffect(() => {
    // A reduced setup scope closes the form and releases its write-only secret
    // and frozen request. Same-scope refreshes keep the existing editor intact.
    if (setup && !setupAllowed) setSetup(null)
  }, [setup, setupAllowed])
  const alive = useRef(true)
  const serial = useRef(0)
  const invalidateReads = useCallback(() => {
    serial.current++
  }, [])
  const bindingRef = useRef(binding)
  useEffect(() => {
    bindingRef.current = binding
  }, [binding])
  useEffect(() => {
    alive.current = true
    return () => {
      alive.current = false
      invalidateReads()
    }
  }, [invalidateReads])
  const loadSources = useCallback(
    async (view: BindingView, next = 0) => {
      if (!view.binding.capabilities.read) {
        setSources(null)
        return
      }
      const attempt = ++serial.current
      setLoading(true)
      try {
        const result = await client.intake("sources/list", {
          bindingId: view.binding.id,
          page: next,
        })
        if (!alive.current || attempt !== serial.current) return
        setSources((old) =>
          next && old
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
        setError(null)
      } catch (caught) {
        if (!alive.current || attempt !== serial.current) return
        setError(caught)
        if (accessWasLost(caught)) {
          setSources(null)
          setSourceId(null)
        }
      } finally {
        if (alive.current && attempt === serial.current) setLoading(false)
      }
    },
    [client]
  )
  const loadBindings = useCallback(
    async (next = 0) => {
      setLoading(true)
      try {
        const result = await client.intake("bindings/list", { page: next })
        if (!alive.current) return
        setList((old) =>
          next && old
            ? {
                ...result,
                items: [
                  ...old.items,
                  ...result.items.filter(
                    (item) =>
                      !old.items.some(
                        (prior) => prior.binding.id === item.binding.id
                      )
                  ),
                ],
              }
            : result
        )
        const selected = bindingRef.current
        if (selected && !next) {
          const status = await client.intake("bindings/status", {
            bindingId: selected.binding.id,
          })
          if (
            !alive.current ||
            bindingRef.current?.binding.id !== selected.binding.id
          )
            return
          setBinding(status)
          if (!status.binding.capabilities.read) {
            setSourceId(null)
            setSources(null)
          }
        }
        setError(null)
      } catch (caught) {
        if (!alive.current) return
        setError(caught)
        if (accessWasLost(caught)) {
          setList(null)
          setBinding(null)
          setSources(null)
          setSourceId(null)
          setSetup(null)
        }
      } finally {
        if (alive.current) setLoading(false)
      }
    },
    [client]
  )
  useEffect(() => {
    if (!active) return
    void loadBindings()
    const check = () => {
      if (document.visibilityState === "visible") void loadBindings()
    }
    window.addEventListener("focus", check)
    return () => window.removeEventListener("focus", check)
  }, [active, loadBindings])
  useEffect(() => {
    if (active && binding && !sourceId) void loadSources(binding)
  }, [active, binding, loadSources, sourceId])
  useEffect(() => {
    if (!active || !entry) return
    // Task references share the existing review surface. Consume the request
    // once, but keep its private editor until the same discard guard accepts it.
    if (
      entry.sourceId !== sourceId ||
      entry.bindingId !== binding?.binding.id
    ) {
      if (dirty) setLeave(() => () => setAcceptedEntry(entry))
      else setAcceptedEntry(entry)
    }
    onEntryRead()
  }, [active, entry, sourceId, binding?.binding.id, dirty, onEntryRead])
  useEffect(() => {
    if (!active || !acceptedEntry) return
    let cancelled = false
    client
      .intake("bindings/status", { bindingId: acceptedEntry.bindingId })
      .then((result) => {
        if (!cancelled) {
          setBinding(result)
          setSourceId(acceptedEntry.sourceId)
          setSources(null)
          setDirty(false)
          setAcceptedEntry(null)
        }
      })
      .catch((caught) => {
        if (!cancelled) {
          setError(caught)
          setAcceptedEntry(null)
        }
      })
    return () => {
      cancelled = true
    }
  }, [active, acceptedEntry, client])
  function navigate(action: () => void) {
    if (dirty) setLeave(() => action)
    else action()
  }
  function openSetup(target: SetupSession["target"]) {
    if (!canSetup || (target === "current" && !canReviewSetup)) return
    setSetup({ target, setupKinds, setupDomains })
  }
  function selectBinding(id: string) {
    const next = list?.items.find((item) => item.binding.id === id)
    if (next)
      navigate(() => {
        serial.current++
        setBinding(next)
        setSourceId(null)
        setSources(null)
        setDirty(false)
      })
  }
  return (
    <div
      hidden={!active}
      className="@container space-y-7"
      data-business-sources
    >
      {sourceId && binding ? (
        <SourceReview
          key={`${binding.binding.id}:${sourceId}`}
          client={client}
          binding={binding.binding}
          sourceId={sourceId}
          actor={actor}
          members={members}
          active={active && !acceptedEntry}
          onDirty={setDirty}
          onBack={() =>
            navigate(() => {
              setSourceId(null)
              setDirty(false)
            })
          }
          onTask={onTask}
          onSetup={canReviewSetup ? () => openSetup("current") : undefined}
        />
      ) : (
        <>
          <header className="flex flex-wrap items-start justify-between gap-5">
            <div className="max-w-2xl space-y-3">
              <p className="text-primary text-xs font-semibold">
                {copy.sources}
              </p>
              <h1 className="text-2xl font-semibold tracking-tight sm:text-3xl">
                {copy.heading}
              </h1>
              <p className="text-muted-foreground text-sm leading-relaxed sm:text-base">
                {copy.introduction}
              </p>
            </div>
            {canSetup && (
              <Action variant="outline" onClick={() => openSetup("new")}>
                <Settings2 className="size-4" aria-hidden />
                {copy.setup}
              </Action>
            )}
          </header>
          <PrivateNotice />
          {error != null && (
            <IntakeError error={error}>
              <Action variant="outline" onClick={() => void loadBindings()}>
                {common.retry}
              </Action>
            </IntakeError>
          )}
          {!list && loading && (
            <p role="status" className="text-muted-foreground text-sm">
              {common.loading}
            </p>
          )}
          {list?.items.length === 0 && (
            <section className="border-border flex gap-4 rounded-2xl border p-6">
              <BookOpen
                className="text-primary mt-1 size-6 shrink-0"
                aria-hidden
              />
              <div className="space-y-2">
                <h2 className="font-semibold">{copy.empty}</h2>
                <p className="text-muted-foreground max-w-xl text-sm leading-relaxed">
                  {copy.emptyHint}
                </p>
              </div>
            </section>
          )}
          {!!list?.items.length && (
            <section className="space-y-5" aria-label={copy.connections}>
              <div className="flex flex-wrap items-end justify-between gap-3">
                <label className="min-w-0 flex-1 space-y-2 text-sm">
                  <span>{copy.connection}</span>
                  <select
                    value={binding?.binding.id ?? ""}
                    onChange={(event) => selectBinding(event.target.value)}
                    className={controlClass}
                  >
                    <option value="">{copy.connection}</option>
                    {list.items.map((item) => (
                      <option key={item.binding.id} value={item.binding.id}>
                        {item.binding.label}
                      </option>
                    ))}
                  </select>
                </label>
                <Action
                  variant="outline"
                  disabled={loading}
                  onClick={() => void loadBindings()}
                >
                  {common.refresh}
                </Action>
              </div>
              {list.hasMore && (
                <Action
                  variant="outline"
                  disabled={loading}
                  onClick={() => void loadBindings(list.page + 1)}
                >
                  {common.more}
                </Action>
              )}
            </section>
          )}
          {binding && (
            <>
              <div className="border-border flex flex-wrap items-center justify-between gap-4 border-b pb-5">
                <div className="space-y-1">
                  <h2 dir="auto" className="break-words text-lg font-semibold">
                    {binding.binding.label}
                  </h2>
                  <p className="text-muted-foreground text-sm">
                    {copy[binding.binding.kind]} ·{" "}
                    {common[binding.binding.domain]} ·{" "}
                    {binding.binding.enabled ? copy.enabled : copy.disabled}
                  </p>
                </div>
                {canReviewSetup && (
                  <Action
                    variant="outline"
                    onClick={() => openSetup("current")}
                  >
                    {copy.reviewSetup}
                  </Action>
                )}
              </div>
              {!binding.binding.capabilities.read && (
                <p
                  role="status"
                  className="text-muted-foreground text-sm leading-relaxed"
                >
                  {copy.contactOperator}
                </p>
              )}
              <div className="grid min-w-0 gap-7 @[50rem]:grid-cols-[minmax(0,1.3fr)_minmax(20rem,0.7fr)]">
                <section
                  className="min-w-0 space-y-4"
                  aria-label={copy.sources}
                >
                  {sources?.items.length === 0 && (
                    <div className="border-border space-y-2 rounded-2xl border p-5">
                      <h3 className="font-medium">{copy.emptySource}</h3>
                      <p className="text-muted-foreground text-sm leading-relaxed">
                        {copy.emptySourceHint}
                      </p>
                    </div>
                  )}
                  <ul className="divide-border divide-y">
                    {sources?.items.map((item) => (
                      <li key={item.id} className="py-2">
                        <button
                          type="button"
                          onClick={() => setSourceId(item.id)}
                          className="hover:bg-muted focus-visible:outline-ring w-full space-y-2 rounded-xl p-4 text-start focus-visible:outline-2"
                        >
                          <span
                            dir="auto"
                            className="block break-words text-base font-medium [overflow-wrap:anywhere]"
                          >
                            {item.title}
                          </span>
                          <span className="text-muted-foreground block text-xs">
                            {copy.sourceRevision}:{" "}
                            {item.revision ?? copy.noRevision}
                          </span>
                          {(item.requiresRefresh ||
                            item.access !== "fresh") && (
                            <span className="text-muted-foreground block text-sm">
                              {copy.sourceRefresh}
                            </span>
                          )}
                        </button>
                      </li>
                    ))}
                  </ul>
                  {sources?.hasMore && (
                    <Action
                      variant="outline"
                      disabled={loading}
                      onClick={() =>
                        void loadSources(binding, sources.page + 1)
                      }
                    >
                      {common.more}
                    </Action>
                  )}
                </section>
                <ImportPanel
                  key={binding.binding.id}
                  client={client}
                  binding={binding.binding}
                  active={active}
                  onRead={() => void loadSources(binding)}
                />
              </div>
            </>
          )}
        </>
      )}
      {setup && setupAllowed && (
        <SourceSetupDialog
          key={setup.target === "new" ? "new" : binding!.binding.id}
          client={client}
          initial={
            setup.target === "current"
              ? (binding?.admin ?? undefined)
              : undefined
          }
          setupKinds={setup.setupKinds}
          setupDomains={setup.setupDomains}
          members={members}
          onClose={() => setSetup(null)}
          onChanged={() => void loadBindings()}
        />
      )}
      {leave && (
        <Modal title={copy.leaveDraft} onClose={() => setLeave(null)}>
          <p className="text-sm leading-relaxed">{copy.leaveHint}</p>
          <div className="flex flex-wrap gap-2">
            <Action variant="outline" onClick={() => setLeave(null)}>
              {common.stay}
            </Action>
            <Action
              variant="destructive"
              onClick={() => {
                leave()
                setLeave(null)
              }}
            >
              {common.discardDraft}
            </Action>
          </div>
        </Modal>
      )}
    </div>
  )
}
