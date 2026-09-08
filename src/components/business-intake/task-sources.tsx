"use client"

import { useEffect, useRef, useState } from "react"
import { Action } from "@/components/business/ui"
import type { BusinessClient } from "@/lib/business/client"
import { useBusinessCopy } from "@/lib/business/copy"
import { useIntakeCopy } from "@/lib/business/intake-copy"
import type { SourceLinkView, SourceSummary } from "@/lib/business/intake"
import { IntakeError } from "./ui"

export function TaskSources({
  taskId,
  client,
  onSource,
}: {
  taskId: string
  client: BusinessClient
  onSource: (source: SourceSummary) => void
}) {
  const copy = useIntakeCopy()
  const common = useBusinessCopy()
  const [links, setLinks] = useState<SourceLinkView[] | null>(null)
  const [error, setError] = useState<unknown>(null)
  const [busy, setBusy] = useState(false)
  const serial = useRef(0)
  useEffect(
    () => () => {
      serial.current++
    },
    []
  )
  async function load() {
    const attempt = ++serial.current
    setBusy(true)
    setLinks(null)
    setError(null)
    try {
      const result = await client.intake("tasks/sources", { taskId })
      if (attempt === serial.current) setLinks(result.links)
    } catch (caught) {
      if (attempt === serial.current) setError(caught)
    } finally {
      if (attempt === serial.current) setBusy(false)
    }
  }
  return (
    <details
      className="border-border rounded-xl border p-4"
      onToggle={(event) => {
        if (event.currentTarget.open) void load()
        else {
          serial.current++
          setLinks(null)
          setError(null)
          setBusy(false)
        }
      }}
    >
      <summary className="focus-visible:outline-ring cursor-pointer py-2 text-sm font-medium focus-visible:outline-2">
        {copy.taskSources}
      </summary>
      <div className="space-y-3 pt-3 text-sm">
        {busy && <p role="status">{common.loading}</p>}
        {error != null && (
          <IntakeError error={error}>
            <Action
              variant="outline"
              disabled={busy}
              onClick={() => void load()}
            >
              {common.retry}
            </Action>
          </IntakeError>
        )}
        {links?.length === 0 && (
          <p className="text-muted-foreground">{copy.noTaskSources}</p>
        )}
        <ul className="space-y-3">
          {links?.map((link) => (
            <li key={link.linkId}>
              {link.accessible && link.source ? (
                <Action
                  variant="outline"
                  className="h-auto min-h-11 max-w-full whitespace-normal text-start"
                  onClick={() => {
                    if (link.source) onSource(link.source)
                  }}
                >
                  <span dir="auto" className="break-words">
                    {link.source.title}
                  </span>
                  <span className="sr-only">{copy.openSource}</span>
                </Action>
              ) : (
                <p>{copy.privateReference}</p>
              )}
            </li>
          ))}
        </ul>
      </div>
    </details>
  )
}
