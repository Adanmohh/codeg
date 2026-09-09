"use client"

import { useCallback, useEffect, useRef, useState } from "react"
import {
  BusinessError,
  createBusinessClient,
  type BusinessClient,
  type BusinessConnection,
} from "@/lib/business/client"
import type { BusinessContext } from "@/lib/business/identity"
import { ConnectWorkspace } from "@/components/business/connect"
import { BootstrapWorkspace } from "@/components/business/bootstrap"
import { BusinessWorkspace } from "@/components/business/workspace"

interface Session {
  serial: number
  client: BusinessClient
  context: BusinessContext
}

export default function BusinessPage() {
  const [session, setSession] = useState<Session | null>(null)
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<unknown>(null)
  const active = useRef<BusinessClient | null>(null)
  const generation = useRef(0)
  const disconnect = useCallback(() => {
    generation.current++
    active.current?.close()
    active.current = null
    setSession(null)
    setError(null)
    setBusy(false)
  }, [])
  useEffect(
    () => () => {
      generation.current++
      active.current?.close()
    },
    []
  )
  async function connect(connection: BusinessConnection): Promise<boolean> {
    const attempt = ++generation.current
    active.current?.close()
    setSession(null)
    setBusy(true)
    setError(null)
    let client: BusinessClient | null = null
    try {
      client = createBusinessClient(connection, () => {
        if (generation.current !== attempt) return
        active.current = null
        setSession(null)
        setError(new BusinessError("unauthorized"))
      })
      active.current = client
      const context = await client.identity("context", {})
      if (attempt !== generation.current) {
        client.close()
        return false
      }
      setSession({ serial: attempt, client, context })
      return true
    } catch (caught) {
      client?.close()
      if (attempt === generation.current) {
        active.current = null
        setError(caught)
      }
      return false
    } finally {
      if (attempt === generation.current) setBusy(false)
    }
  }
  function updateContext(client: BusinessClient, context: BusinessContext) {
    if (active.current !== client) return
    const previous = session?.context
    if (
      previous?.organization &&
      (previous.organization.id !== context.organization?.id ||
        previous.organization.authorizationEpoch !==
          context.organization?.authorizationEpoch ||
        previous.member?.id !== context.member?.id)
    ) {
      // New login may authenticate after a suspension/resume, but it cannot
      // bless the previous epoch's private drafts, reads or pending responses.
      generation.current++
      client.close()
      active.current = null
      setSession(null)
      setError(new BusinessError("unauthorized"))
      return
    }
    setSession((current) =>
      current?.client === client ? { ...current, context } : current
    )
  }
  if (!session)
    return <ConnectWorkspace connect={connect} busy={busy} error={error} />
  if (session.context.needsBootstrap)
    return (
      <BootstrapWorkspace
        client={session.client}
        onReady={(context) => updateContext(session.client, context)}
        disconnect={disconnect}
      />
    )
  if (!session.context.organization || !session.context.member)
    return (
      <ConnectWorkspace
        connect={connect}
        busy={busy}
        error={new BusinessError("unauthorized")}
      />
    )
  // One private editing lifetime per connection/principal and membership
  // revision. Epoch drift closes the connection above; locale/appearance and
  // viewport changes never replace this lifetime.
  const key = `${session.serial}:${session.context.organization.id}:${session.context.member.id}:${session.context.member.revision}`
  return (
    <BusinessWorkspace
      key={key}
      client={session.client}
      context={session.context}
      onContext={(context) => updateContext(session.client, context)}
      disconnect={disconnect}
    />
  )
}
