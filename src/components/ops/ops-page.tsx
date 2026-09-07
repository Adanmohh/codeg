"use client"

import { useCallback, useEffect, useState } from "react"
import {
  Inbox as InboxIcon,
  RefreshCw,
  Sunrise,
  ShieldCheck,
} from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { useWorkbenchRoute } from "@/contexts/workbench-route-context"
import { useRemoteConnection } from "@/contexts/remote-connection-context"
import { ops, type OpsContext, type ThreadKey } from "@/lib/ops/api"
import { InboxView } from "./inbox-view"
import { ApprovalsView } from "./proposals-view"
import { MorningView } from "./morning-view"
import { opsError, useOpsResource } from "./use-ops-resource"
import { LoadError, Loading, Notice, touchButton } from "./ui"

export function OpsPageTitle() {
  return <span className="text-sm font-medium">Ops desk</span>
}
export function OpsPage() {
  const remote = useRemoteConnection()
  // Each backend gets its own React subtree and local drafts/requests.
  return <OpsWorkspace key={remote?.connection?.id ?? "local"} />
}

type View = "inbox" | "approvals" | "morning"
function OpsWorkspace() {
  const { setRoute, registerLeaveGuard } = useWorkbenchRoute()
  const context = useOpsResource("context", ops.context)
  const [view, setView] = useState<View>("inbox")
  const [selected, setSelected] = useState<ThreadKey | null>(null)
  const [proposalId, setProposalId] = useState<number | null>(null)
  const [epoch, setEpoch] = useState(0)
  const [dirty, setDirty] = useState(false)
  useEffect(
    () =>
      registerLeaveGuard(
        (next) =>
          next === "ops" ||
          !dirty ||
          window.confirm(
            "Discard unsaved Ops changes? Save your draft or private note before leaving to keep it."
          )
      ),
    [dirty, registerLeaveGuard]
  )
  useEffect(() => {
    const warn = (event: BeforeUnloadEvent) => {
      event.preventDefault()
      event.returnValue = ""
    }
    if (dirty) window.addEventListener("beforeunload", warn)
    return () => window.removeEventListener("beforeunload", warn)
  }, [dirty])
  const navigate = (action: () => void) => {
    if (
      dirty &&
      !window.confirm(
        "Discard unsaved changes? Save your draft or private note before leaving to keep it."
      )
    )
      return
    action()
  }
  const openThread = (key: ThreadKey) =>
    navigate(() => {
      setSelected(key)
      setView("inbox")
    })
  const nav = [
    { id: "inbox" as const, label: "Inbox", icon: InboxIcon },
    { id: "approvals" as const, label: "Approvals", icon: ShieldCheck },
    { id: "morning" as const, label: "Morning", icon: Sunrise },
  ]
  return (
    <section
      aria-label="Ops desk"
      className="flex h-full min-h-0 min-w-0 flex-col bg-background pb-[env(safe-area-inset-bottom)]"
    >
      <header className="flex shrink-0 flex-wrap items-center justify-between gap-3 border-b px-4 py-3 sm:px-6">
        <nav aria-label="Ops views" className="flex gap-1">
          {nav.map(({ id, label, icon: Icon }) => (
            <Button
              key={id}
              className={touchButton}
              variant={view === id ? "secondary" : "ghost"}
              aria-current={view === id ? "page" : undefined}
              onClick={() => navigate(() => setView(id))}
            >
              <Icon aria-hidden="true" />
              {label}
            </Button>
          ))}
        </nav>
        <Button
          className={touchButton}
          variant="ghost"
          onClick={() =>
            navigate(() => {
              context.reload()
              setEpoch((n) => n + 1)
            })
          }
        >
          <RefreshCw aria-hidden="true" />
          <span>Refresh</span>
        </Button>
      </header>
      {context.loading ? (
        <Loading label="Loading your inboxes…" />
      ) : context.error ? (
        <LoadError error={context.error} retry={context.reload} />
      ) : (
        context.data && (
          <div className="flex min-h-0 flex-1 flex-col">
            <div className="shrink-0 border-b bg-muted/25 px-4 py-2 text-xs leading-relaxed text-muted-foreground sm:px-6">
              Account {context.data.accountId} · {context.data.transportMessage}
            </div>
            {view === "inbox" &&
              (context.data.inboxes.length === 0 ? (
                <CreateInbox onCreated={context.reload} />
              ) : (
                <InboxView
                  key={epoch}
                  context={context.data}
                  selected={selected}
                  onSelect={(key) => navigate(() => setSelected(key))}
                  onDirty={setDirty}
                />
              ))}
            {view === "approvals" && (
              <ApprovalsView
                key={epoch}
                context={context.data}
                selectedId={proposalId}
                onSelect={(id) => navigate(() => setProposalId(id))}
                onDirty={setDirty}
                onThread={openThread}
              />
            )}
            {view === "morning" && (
              <MorningView
                key={epoch}
                onThread={openThread}
                onProposal={(id) =>
                  navigate(() => {
                    setProposalId(id)
                    setView("approvals")
                  })
                }
                onTasks={() => navigate(() => setRoute("tasks"))}
              />
            )}
          </div>
        )
      )}
    </section>
  )
}

function CreateInbox({ onCreated }: { onCreated: () => void }) {
  const [name, setName] = useState("")
  const [email, setEmail] = useState("")
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState("")
  const submit = useCallback(async () => {
    if (busy) return
    setBusy(true)
    setError("")
    try {
      await ops.createInbox({ name, email })
      onCreated()
    } catch (e) {
      setError(opsError(e))
      setBusy(false)
    }
  }, [busy, email, name, onCreated])
  return (
    <div className="overflow-y-auto p-6">
      <div className="mx-auto max-w-lg space-y-5 py-8">
        <InboxIcon className="size-8 text-primary" aria-hidden="true" />
        <div className="space-y-2">
          <h1 className="text-xl font-semibold">
            A place for your correspondence
          </h1>
          <p className="text-sm leading-relaxed text-muted-foreground">
            Add a local inbox to organize real ticket threads, write private
            notes and prepare replies. Connecting email delivery is a separate
            step.
          </p>
        </div>
        <form
          className="space-y-4"
          onSubmit={(e) => {
            e.preventDefault()
            void submit()
          }}
        >
          <div className="space-y-1.5">
            <label htmlFor="ops-inbox-name" className="text-sm font-medium">
              Inbox name
            </label>
            <Input
              id="ops-inbox-name"
              className={touchButton}
              autoComplete="off"
              required
              maxLength={120}
              value={name}
              onChange={(e) => setName(e.target.value)}
              disabled={busy}
            />
          </div>
          <div className="space-y-1.5">
            <label htmlFor="ops-inbox-email" className="text-sm font-medium">
              Inbox email address
            </label>
            <Input
              id="ops-inbox-email"
              className={touchButton}
              type="email"
              required
              autoComplete="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              disabled={busy}
            />
          </div>
          {error && <Notice error>{error}</Notice>}
          <Button
            type="submit"
            className={touchButton}
            disabled={busy || !name.trim() || !email.trim()}
          >
            {busy ? "Adding inbox…" : "Add local inbox"}
          </Button>
        </form>
      </div>
    </div>
  )
}

export type { OpsContext }
