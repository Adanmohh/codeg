"use client"

import { useCallback, useEffect, useRef, useState } from "react"
import { ArrowLeft } from "lucide-react"
import { Action, Modal, controlClass } from "@/components/business/ui"
import type { BusinessClient } from "@/lib/business/client"
import { useBusinessCopy } from "@/lib/business/copy"
import { useIntakeCopy } from "@/lib/business/intake-copy"
import type { Member } from "@/lib/business/identity"
import {
  validPassages,
  sourceIsFresh,
  type BindingSummary,
  type Candidate,
  type CandidateDetail,
  type CandidateState,
  type Page,
  type SourceDetail,
} from "@/lib/business/intake"
import { CandidateReview } from "./candidate"
import { ImportPanel } from "./imports"
import {
  IntakeError,
  Passages,
  PrivateNotice,
  accessWasLost,
  outcomeIsUncertain,
  useSourceFreshness,
} from "./ui"

export function SourceReview({
  client,
  binding,
  sourceId,
  actor,
  members,
  active,
  onBack,
  onDirty,
  onTask,
  onSetup,
}: {
  client: BusinessClient
  binding: BindingSummary
  sourceId: string
  actor: Member
  members: Member[]
  active: boolean
  onBack: () => void
  onDirty: (dirty: boolean) => void
  onTask: (id: string) => void
  onSetup?: () => void
}) {
  const copy = useIntakeCopy()
  const common = useBusinessCopy()
  const [source, setSource] = useState<SourceDetail | null>(null)
  const [page, setPage] = useState<Page<Candidate> | null>(null)
  const [state, setState] = useState<CandidateState>("pending")
  const [selected, setSelected] = useState<string[]>([])
  const [candidate, setCandidate] = useState<CandidateDetail | null>(null)
  const [candidateDirty, setCandidateDirty] = useState(false)
  const [pendingCandidate, setPendingCandidate] = useState<string | null>(null)
  const [error, setError] = useState<unknown>(null)
  const [busy, setBusy] = useState(false)
  const [verified, setVerified] = useState(false)
  const [unknown, setUnknown] = useState(false)
  const [refresh, setRefresh] = useState(0)
  const serial = useRef(0)
  const invalidateReads = useCallback(() => {
    serial.current++
  }, [])
  const alive = useRef(true)
  const working = useRef(false)
  const replay = useRef<(() => Promise<Candidate>) | null>(null)
  const fresh = useSourceFreshness(source?.source ?? null)
  const visible = active && verified && fresh && source?.disclosure === "fresh"
  useEffect(() => {
    alive.current = true
    return () => {
      alive.current = false
      invalidateReads()
      replay.current = null
    }
  }, [invalidateReads])
  useEffect(() => {
    onDirty(
      candidateDirty || (!candidate && selected.length > 0) || busy || unknown
    )
    return () => onDirty(false)
  }, [candidateDirty, candidate, selected.length, busy, unknown, onDirty])
  const load = useCallback(async () => {
    const attempt = ++serial.current
    setVerified(false)
    try {
      const [detail, candidates] = await Promise.all([
        client.intake("sources/get", { sourceId }),
        client.intake("candidates/list", { sourceId, state }),
      ])
      if (!alive.current || attempt !== serial.current) return
      setSource(
        sourceIsFresh(detail.source)
          ? detail
          : { ...detail, disclosure: "metadata_only", passages: [] }
      )
      setPage(candidates)
      setVerified(true)
      setError(null)
      if (detail.disclosure !== "fresh") setSelected([])
    } catch (caught) {
      if (!alive.current || attempt !== serial.current) return
      setError(caught)
      if (accessWasLost(caught)) {
        setSource(null)
        setPage(null)
        setCandidate(null)
        setSelected([])
      }
    }
  }, [client, sourceId, state])
  useEffect(() => {
    if (!active) {
      setVerified(false)
      serial.current++
      return
    }
    void load()
    const check = () => {
      if (document.visibilityState === "visible" && !working.current)
        void load()
    }
    window.addEventListener("focus", check)
    return () => {
      window.removeEventListener("focus", check)
      invalidateReads()
    }
  }, [active, load, refresh, binding.accessEpoch, invalidateReads])
  const refreshCandidates = useCallback(() => {
    client
      .intake("candidates/list", { sourceId, state })
      .then((result) => {
        if (alive.current) setPage(result)
      })
      .catch((caught) => {
        if (alive.current) {
          setError(caught)
          if (accessWasLost(caught)) {
            setSource(null)
            setCandidate(null)
            setPage(null)
            setSelected([])
          }
        }
      })
  }, [client, sourceId, state])
  async function openCandidate(id: string) {
    if (working.current) return
    working.current = true
    setBusy(true)
    setError(null)
    try {
      const result = await client.intake("candidates/get", { candidateId: id })
      if (alive.current) {
        setCandidate(result)
        setSelected([])
        setCandidateDirty(false)
      }
    } catch (caught) {
      if (alive.current) {
        setError(caught)
        if (accessWasLost(caught)) {
          setCandidate(null)
          setSource(null)
        }
      }
    } finally {
      working.current = false
      if (alive.current) setBusy(false)
    }
  }
  async function create(operation: () => Promise<Candidate>) {
    if (working.current || !active) return
    working.current = true
    replay.current = operation
    setBusy(true)
    setError(null)
    try {
      const result = await operation()
      if (!alive.current) return
      // The selection is durable even if the following detail read fails.
      replay.current = null
      setUnknown(false)
      setSelected([])
      const detail = await client.intake("candidates/get", {
        candidateId: result.id,
      })
      if (alive.current) {
        setCandidate(detail)
        refreshCandidates()
      }
    } catch (caught) {
      if (!alive.current) return
      setError(caught)
      setUnknown(!!replay.current && outcomeIsUncertain(caught))
      if (!outcomeIsUncertain(caught)) replay.current = null
      if (accessWasLost(caught)) {
        setSource(null)
        setCandidate(null)
        setSelected([])
      }
      refreshCandidates()
    } finally {
      working.current = false
      if (alive.current) setBusy(false)
    }
  }
  const canCreate =
    visible &&
    binding.capabilities.triage &&
    !busy &&
    !unknown &&
    !!source &&
    validPassages(source.passages, selected, source.source.revision)
  return (
    <div className="space-y-6">
      <Action variant="ghost" className="-ms-3" onClick={onBack}>
        <ArrowLeft className="size-4 rtl:rotate-180" aria-hidden />
        {copy.sources}
      </Action>
      <PrivateNotice />
      {error != null && (
        <IntakeError error={error}>
          <Action variant="outline" onClick={() => void load()}>
            {common.retry}
          </Action>
        </IntakeError>
      )}
      <header className="space-y-3">
        <p className="text-primary text-xs font-semibold">
          {copy[binding.kind]}
        </p>
        <h1
          dir="auto"
          className="break-words text-2xl font-semibold tracking-tight sm:text-3xl [overflow-wrap:anywhere]"
        >
          {source?.source.title ?? copy.sources}
        </h1>
        <p className="text-muted-foreground text-sm">
          {copy.sourceRevision}: {source?.source.revision ?? copy.noRevision}
        </p>
        {onSetup && (
          <Action variant="outline" onClick={onSetup}>
            {copy.reviewSetup}
          </Action>
        )}
      </header>
      {!visible && (
        <section
          role="status"
          className="bg-muted/50 space-y-2 rounded-2xl p-5"
        >
          <h2 className="font-semibold">{copy.unavailable}</h2>
          <p className="text-muted-foreground text-sm leading-relaxed">
            {copy.unavailableHint}
          </p>
        </section>
      )}
      <div
        className={
          candidate
            ? "grid min-w-0 gap-8 xl:grid-cols-[minmax(0,0.9fr)_minmax(0,1.1fr)]"
            : "grid min-w-0 gap-8 xl:grid-cols-[minmax(0,1.35fr)_minmax(18rem,0.65fr)]"
        }
      >
        <div className="min-w-0 space-y-6">
          {visible && source && (
            <section className="space-y-4" aria-label={copy.exactPassage}>
              <h2 className="font-semibold">{copy.exactPassage}</h2>
              <p className="text-muted-foreground text-sm leading-relaxed">
                {copy.selectHint}
              </p>
              {source.source.summary === "empty" && (
                <p className="text-muted-foreground text-sm">
                  {copy.summaryEmpty}
                </p>
              )}
              {source.source.summary === "missing" && (
                <p className="text-muted-foreground text-sm">
                  {copy.summaryMissing}
                </p>
              )}
              {source.passages.length === 0 ? (
                <p className="text-sm">{copy.noPassages}</p>
              ) : (
                <Passages
                  passages={source.passages}
                  selected={selected}
                  onSelect={
                    !candidate && binding.capabilities.triage
                      ? setSelected
                      : undefined
                  }
                  disabled={busy || unknown}
                />
              )}
              {!candidate &&
                binding.capabilities.triage &&
                source.passages.length > 0 && (
                  <>
                    <p className="text-muted-foreground text-sm tabular-nums">
                      {copy.selected}: {selected.length}
                    </p>
                    {selected.length > 0 &&
                      !validPassages(
                        source.passages,
                        selected,
                        source.source.revision
                      ) && (
                        <p role="status" className="text-sm">
                          {copy.selectedInvalid}
                        </p>
                      )}
                    <Action
                      disabled={!canCreate}
                      onClick={() => {
                        if (!canCreate || source.source.revision === null)
                          return
                        const input = {
                          operationId: crypto.randomUUID(),
                          sourceId,
                          expectedSourceRevision: source.source.revision,
                          passageIds: selected,
                        }
                        void create(() =>
                          client.intake("candidates/create", input)
                        )
                      }}
                    >
                      {copy.prepare}
                    </Action>
                  </>
                )}
            </section>
          )}
          {unknown && (
            <div role="status" className="space-y-3 text-sm">
              <p>{copy.unknown}</p>
              <Action
                variant="outline"
                disabled={busy || !replay.current}
                onClick={() => {
                  if (replay.current) void create(replay.current)
                }}
              >
                {copy.retryExact}
              </Action>
            </div>
          )}
          <details
            className="border-border rounded-2xl border p-4"
            open={!visible}
          >
            <summary className="focus-visible:outline-ring cursor-pointer py-2 text-sm font-medium focus-visible:outline-2">
              {copy.sourceRefresh}
            </summary>
            <ImportPanel
              client={client}
              binding={binding}
              active={active}
              refreshSourceId={sourceId}
              onRead={() => setRefresh((value) => value + 1)}
            />
          </details>
        </div>
        <div className="min-w-0 space-y-6">
          {candidate && source && (
            <CandidateReview
              key={candidate.candidate.id}
              initial={candidate}
              source={source}
              client={client}
              actor={actor}
              members={members}
              active={active && verified}
              refresh={refresh}
              onDirty={setCandidateDirty}
              onChanged={refreshCandidates}
              onTask={onTask}
            />
          )}
          <section
            className="border-border space-y-4 rounded-2xl border p-4 sm:p-5"
            aria-label={copy.candidates}
          >
            <h2 className="font-semibold">{copy.candidates}</h2>
            <select
              aria-label={copy.candidates}
              value={state}
              className={controlClass}
              onChange={(event) =>
                setState(event.target.value as CandidateState)
              }
            >
              {(["pending", "accepted", "linked", "discarded"] as const).map(
                (value) => (
                  <option value={value} key={value}>
                    {copy[value]}
                  </option>
                )
              )}
            </select>
            {page?.items.length === 0 && (
              <p className="text-muted-foreground text-sm">
                {copy.noCandidates}
              </p>
            )}
            <ul className="divide-border divide-y">
              {page?.items.map((item, index) => (
                <li key={item.id} className="py-2">
                  <button
                    type="button"
                    className="hover:bg-muted focus-visible:outline-ring w-full min-w-0 space-y-2 rounded-lg p-3 text-start focus-visible:outline-2"
                    aria-current={
                      candidate?.candidate.id === item.id ? "true" : undefined
                    }
                    disabled={busy || unknown}
                    onClick={() => {
                      if (candidateDirty) setPendingCandidate(item.id)
                      else void openCandidate(item.id)
                    }}
                  >
                    <span
                      dir="auto"
                      className="block break-words text-sm font-medium"
                    >
                      {visible && item.disclosure === "fresh" && item.draft
                        ? item.draft.title
                        : `${copy[item.state]} ${index + 1}`}
                    </span>
                    <span className="text-muted-foreground block text-xs">
                      {copy[item.state]} · {common.sourceVersion}{" "}
                      {item.revision}
                    </span>
                    {item.hasPreparedDraft && !item.draft && (
                      <span className="text-muted-foreground block text-xs leading-relaxed">
                        {copy.withheldDraft}
                      </span>
                    )}
                  </button>
                </li>
              ))}
            </ul>
            {page?.hasMore && (
              <Action
                variant="outline"
                onClick={async () => {
                  try {
                    const result = await client.intake("candidates/list", {
                      sourceId,
                      state,
                      page: page.page + 1,
                    })
                    if (alive.current)
                      setPage((prior) =>
                        prior
                          ? {
                              ...result,
                              items: [...prior.items, ...result.items],
                            }
                          : result
                      )
                  } catch (caught) {
                    if (alive.current) setError(caught)
                  }
                }}
              >
                {common.more}
              </Action>
            )}
          </section>
        </div>
      </div>
      {pendingCandidate && (
        <Modal
          title={copy.leaveDraft}
          onClose={() => setPendingCandidate(null)}
        >
          <p className="text-sm leading-relaxed">{copy.leaveHint}</p>
          <div className="flex flex-wrap gap-2">
            <Action variant="outline" onClick={() => setPendingCandidate(null)}>
              {common.stay}
            </Action>
            <Action
              variant="destructive"
              onClick={() => {
                void openCandidate(pendingCandidate)
                setPendingCandidate(null)
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
