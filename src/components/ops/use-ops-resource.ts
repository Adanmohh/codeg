"use client"

import { useEffect, useState } from "react"
import { extractAppCommandError } from "@/lib/app-error"

export function opsError(error: unknown): string {
  return (
    extractAppCommandError(error)?.message ??
    (error instanceof Error
      ? error.message
      : "The change was not confirmed. Reload and try again.")
  )
}

// Codeg cancellation pattern, with a render-time key check as well: a previous
// inbox's data must not paint even once while the next effect is starting.
export function useOpsResource<T>(key: string, load: () => Promise<T>) {
  const [attempt, setAttempt] = useState(0)
  const requestKey = `${key}:${attempt}`
  const [state, setState] = useState<{ key: string; data?: T; error?: string }>(
    { key: "" }
  )
  useEffect(() => {
    let cancelled = false
    void load().then(
      (data) => {
        if (!cancelled) setState({ key: requestKey, data })
      },
      (error) => {
        if (!cancelled) setState({ key: requestKey, error: opsError(error) })
      }
    )
    return () => {
      cancelled = true
    }
  }, [requestKey, load])
  const current = state.key === requestKey ? state : undefined
  return {
    data: current?.data,
    error: current?.error,
    loading: !current,
    reload: () => setAttempt((n) => n + 1),
  }
}
