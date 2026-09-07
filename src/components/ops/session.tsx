"use client"

import {
  createContext,
  useCallback,
  useContext,
  useState,
  useSyncExternalStore,
  type Dispatch,
  type ReactNode,
  type SetStateAction,
} from "react"
import { useRemoteConnection } from "@/contexts/remote-connection-context"

// Like the workbench's lifted route/automation state, this lives ABOVE the
// desktop/mobile shells. Memory only: never localStorage, disk, or a singleton.
function createSession() {
  const values = new Map<string, unknown>()
  const listeners = new Set<() => void>()
  const emit = () => listeners.forEach((listener) => listener())
  return {
    get: <T,>(key: string, fallback: T): T =>
      values.has(key) ? (values.get(key) as T) : fallback,
    set: <T,>(key: string, fallback: T, action: SetStateAction<T>) => {
      const previous = values.has(key) ? (values.get(key) as T) : fallback
      const next =
        typeof action === "function"
          ? (action as (previous: T) => T)(previous)
          : action
      values.set(key, next)
      emit()
    },
    subscribe: (listener: () => void) => {
      listeners.add(listener)
      return () => {
        listeners.delete(listener)
      }
    },
    discardEdits: () => {
      for (const key of values.keys())
        if (key.startsWith("edit:")) values.delete(key)
      emit()
    },
  }
}
const SessionContext = createContext<ReturnType<typeof createSession> | null>(
  null
)
export function OpsSessionProvider({ children }: { children: ReactNode }) {
  const [session] = useState(createSession)
  return (
    <SessionContext.Provider value={session}>
      {children}
    </SessionContext.Provider>
  )
}
export function OpsSessionBoundary({ children }: { children: ReactNode }) {
  const remote = useRemoteConnection()
  const connection = remote?.connection
  const backendKey = connection
    ? `${connection.id}:${connection.base_url}`
    : "local"
  return <OpsSessionProvider key={backendKey}>{children}</OpsSessionProvider>
}
export function useOpsSession() {
  const shared = useContext(SessionContext)
  // Standalone review surfaces/tests have their own isolated lifetime.
  const [local] = useState(createSession)
  return shared ?? local
}
export function useOpsSessionState<T>(
  key: string,
  initial: T | (() => T)
): [T, Dispatch<SetStateAction<T>>] {
  const session = useOpsSession()
  const [fallback] = useState(initial)
  const snapshot = useCallback(
    () => session.get(key, fallback),
    [session, key, fallback]
  )
  const value = useSyncExternalStore(session.subscribe, snapshot, snapshot)
  const set = useCallback(
    (action: SetStateAction<T>) => session.set(key, fallback, action),
    [session, key, fallback]
  )
  return [value, set]
}
