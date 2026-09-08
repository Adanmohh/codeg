"use client"

import { useEffect, useRef, useState } from "react"
import { Input } from "@/components/ui/input"
import {
  Action,
  Field,
  StatusBadge,
  controlClass,
} from "@/components/business/ui"
import { BusinessError, type BusinessClient } from "@/lib/business/client"
import { useBusinessCopy } from "@/lib/business/copy"
import { useIntakeCopy } from "@/lib/business/intake-copy"
import type { BusinessDomain } from "@/lib/business/presentation"
import type { TaskDetail, TaskPage } from "@/lib/business/tasks"
import { IntakeError } from "./ui"

export function TaskTarget({
  client,
  domains,
  disabled,
  onSelected,
}: {
  client: BusinessClient
  domains: BusinessDomain[]
  disabled: boolean
  onSelected: (value: TaskDetail | null) => void
}) {
  const copy = useIntakeCopy()
  const common = useBusinessCopy()
  const [domain, setDomain] = useState<BusinessDomain>(domains[0])
  const [query, setQuery] = useState("")
  const [page, setPage] = useState<TaskPage | null>(null)
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<unknown>(null)
  const serial = useRef(0)
  useEffect(() => {
    if (disabled) {
      serial.current++
      setPage(null)
      setBusy(false)
    }
  }, [disabled])
  useEffect(
    () => () => {
      serial.current++
    },
    []
  )
  async function search(next = 0) {
    if (disabled || !domains.includes(domain)) return
    const attempt = ++serial.current
    setBusy(true)
    setError(null)
    onSelected(null)
    try {
      const result = await client.tasks("list", {
        view: "shared",
        domain,
        query,
        page: next,
      })
      if (attempt === serial.current)
        setPage((old) =>
          next && old
            ? {
                ...result,
                tasks: [
                  ...old.tasks,
                  ...result.tasks.filter(
                    (task) => !old.tasks.some((entry) => entry.id === task.id)
                  ),
                ],
              }
            : result
        )
    } catch (caught) {
      if (attempt === serial.current) {
        setError(caught)
        setPage(null)
      }
    } finally {
      if (attempt === serial.current) setBusy(false)
    }
  }
  async function select(taskId: string) {
    if (busy || disabled) return
    const attempt = ++serial.current
    setBusy(true)
    setError(null)
    onSelected(null)
    try {
      const result = await client.tasks("get", { taskId })
      if (
        !result.task.capabilities.edit ||
        result.task.domain !== domain ||
        !domains.includes(result.task.domain) ||
        result.task.archivedAt ||
        ["done", "cancelled"].includes(result.task.status)
      )
        throw new BusinessError("forbidden")
      if (attempt === serial.current) onSelected(result)
    } catch (caught) {
      if (attempt === serial.current) setError(caught)
    } finally {
      if (attempt === serial.current) setBusy(false)
    }
  }
  const tasks =
    page?.tasks.filter(
      (task) =>
        task.capabilities.edit &&
        !task.archivedAt &&
        !["done", "cancelled"].includes(task.status)
    ) ?? []
  return (
    <section className="space-y-4" aria-label={copy.findTarget}>
      <form
        onSubmit={(event) => {
          event.preventDefault()
          void search()
        }}
        className="space-y-4"
      >
        <div className="grid gap-4 sm:grid-cols-2">
          <Field label={copy.audience}>
            {(id) => (
              <select
                id={id}
                value={domain}
                disabled={busy || disabled}
                className={controlClass}
                onChange={(event) => {
                  serial.current++
                  setDomain(event.target.value as BusinessDomain)
                  setPage(null)
                  onSelected(null)
                }}
              >
                {domains.map((value) => (
                  <option key={value} value={value}>
                    {common[value]}
                  </option>
                ))}
              </select>
            )}
          </Field>
          <Field label={copy.findTarget}>
            {(id) => (
              <Input
                id={id}
                value={query}
                onChange={(event) => {
                  setQuery(event.target.value)
                  onSelected(null)
                }}
                maxLength={240}
                className="min-h-11 rounded-xl"
                disabled={busy || disabled}
              />
            )}
          </Field>
        </div>
        <Action
          type="submit"
          variant="outline"
          disabled={busy || disabled || !domains.includes(domain)}
        >
          {common.search}
        </Action>
      </form>
      {error != null && <IntakeError error={error} />}
      {page && tasks.length === 0 && (
        <p className="text-muted-foreground text-sm">{copy.noTargets}</p>
      )}
      <ul className="divide-border divide-y">
        {tasks.map((task) => (
          <li key={task.id} className="space-y-2 py-3">
            <p dir="auto" className="break-words text-sm font-medium">
              {task.title}
            </p>
            <div className="flex flex-wrap items-center gap-3">
              <StatusBadge status={task.status} />
              <span className="text-muted-foreground text-xs">
                {common.sourceVersion} {task.revision}
              </span>
              <Action
                variant="outline"
                disabled={busy || disabled}
                onClick={() => void select(task.id)}
              >
                {copy.chooseTarget}
              </Action>
            </div>
          </li>
        ))}
      </ul>
      {page?.hasMore && (
        <Action
          variant="outline"
          disabled={busy || disabled}
          onClick={() => void search(page.page + 1)}
        >
          {common.more}
        </Action>
      )}
    </section>
  )
}
