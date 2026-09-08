"use client"

import { useCallback, useRef, useState } from "react"
import { ArrowLeft, LockKeyhole, FilePenLine } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"
import { Message, MessageContent } from "@/components/ai-elements/message"
import {
  ops,
  type OpsContext,
  type Thread,
  type ThreadKey,
} from "@/lib/ops/api"
import { cn } from "@/lib/utils"
import { LoadError, Loading, Notice, touchButton } from "./ui"
import { ReplyEditor, completeReply, replyFields } from "./reply-editor"
import { opsError, useOpsResource } from "./use-ops-resource"
import { EmailSettings } from "./email-settings"
import { useOpsSessionState } from "./session"

const statuses = ["Open", "Resolved", "Pending", "Snoozed"]
export function InboxView({
  context,
  selected,
  onSelect,
  onDirty,
}: {
  context: OpsContext
  selected: ThreadKey | null
  onSelect: (key: ThreadKey | null) => void
  onDirty: (dirty: boolean) => void
}) {
  const [chosenInbox, setChosenInbox] = useOpsSessionState(
    "nav:inbox",
    context.inboxes[0].id
  )
  const inboxId = selected?.inboxId ?? chosenInbox
  const [status, setStatus] = useOpsSessionState("nav:status", "")
  const [page, setPage] = useOpsSessionState("nav:page", 0)
  const loader = useCallback(
    () =>
      ops.tickets({
        inboxId,
        status: status === "" ? undefined : Number(status),
        page,
      }),
    [inboxId, status, page]
  )
  const tickets = useOpsResource(`tickets:${inboxId}:${status}:${page}`, loader)
  return (
    <div className="flex min-h-0 flex-1">
      <aside
        aria-label="Ticket list"
        className={cn(
          "flex min-h-0 w-full shrink-0 flex-col border-e sm:w-72 lg:w-80",
          selected && "hidden sm:flex"
        )}
      >
        <div className="space-y-3 overflow-y-auto border-b p-4">
          <EmailSettings
            key={inboxId}
            inboxId={inboxId}
            onPulled={tickets.reload}
          />
          <div className="space-y-1.5">
            <label
              htmlFor="ops-inbox"
              className="text-xs font-medium text-muted-foreground"
            >
              Inbox
            </label>
            <select
              id="ops-inbox"
              dir="auto"
              value={inboxId}
              className="min-h-11 w-full rounded-lg border bg-background px-3 text-base focus-visible:outline-2 focus-visible:outline-ring sm:text-sm"
              onChange={(e) => {
                setChosenInbox(Number(e.target.value))
                setPage(0)
                onSelect(null)
              }}
            >
              {context.inboxes.map((i) => (
                <option key={i.id} value={i.id}>
                  {i.name} · {i.email}
                </option>
              ))}
            </select>
          </div>
          <div className="space-y-1.5">
            <label
              htmlFor="ops-status"
              className="text-xs font-medium text-muted-foreground"
            >
              Ticket status
            </label>
            <select
              id="ops-status"
              value={status}
              className="min-h-11 w-full rounded-lg border bg-background px-3 text-base focus-visible:outline-2 focus-visible:outline-ring sm:text-sm"
              onChange={(e) => {
                setStatus(e.target.value)
                setPage(0)
              }}
            >
              <option value="">All statuses</option>
              {statuses.map((s, i) => (
                <option value={i} key={s}>
                  {s}
                </option>
              ))}
            </select>
          </div>
        </div>
        <div className="min-h-0 flex-1 overflow-y-auto">
          {tickets.loading ? (
            <Loading label="Loading ticket threads…" />
          ) : tickets.error ? (
            <LoadError error={tickets.error} retry={tickets.reload} />
          ) : (
            tickets.data &&
            (tickets.data.items.length ? (
              <ul className="divide-y">
                {tickets.data.items.map((ticket) => (
                  <li key={ticket.id}>
                    <button
                      type="button"
                      aria-current={
                        selected?.conversationId === ticket.id
                          ? "true"
                          : undefined
                      }
                      className={cn(
                        "w-full space-y-1 p-4 text-start hover:bg-muted/50 focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-ring",
                        selected?.conversationId === ticket.id && "bg-primary/8"
                      )}
                      onClick={() =>
                        onSelect({ inboxId, conversationId: ticket.id })
                      }
                    >
                      <span className="flex items-center justify-between gap-3">
                        <span
                          dir="auto"
                          className="truncate text-sm font-medium"
                        >
                          {ticket.contact}
                        </span>
                        <span
                          className={cn(
                            "text-xs",
                            selected?.conversationId === ticket.id
                              ? "text-foreground/75"
                              : "text-muted-foreground"
                          )}
                        >
                          {statuses[ticket.status]}
                        </span>
                      </span>
                      <span dir="auto" className="block truncate text-sm">
                        {ticket.subject}
                      </span>
                      <span
                        className={cn(
                          "block text-xs",
                          selected?.conversationId === ticket.id
                            ? "text-foreground/75"
                            : "text-muted-foreground"
                        )}
                      >
                        {new Date(ticket.updatedAt).toLocaleDateString()}
                      </span>
                    </button>
                  </li>
                ))}
              </ul>
            ) : (
              <div className="space-y-2 p-6">
                <h2 className="text-sm font-medium">
                  {status
                    ? "No threads with this status"
                    : "No ticket threads yet"}
                </h2>
                <p className="text-sm leading-relaxed text-muted-foreground">
                  {status
                    ? "Choose all statuses to see the rest of this inbox."
                    : "Incoming email will appear here when intake is connected. This inbox has no imported messages."}
                </p>
              </div>
            ))
          )}
        </div>
        <div className="flex justify-between border-t p-2">
          <Button
            variant="ghost"
            className={touchButton}
            disabled={page === 0 || tickets.loading}
            onClick={() => setPage((n) => n - 1)}
          >
            Previous
          </Button>
          <Button
            variant="ghost"
            className={touchButton}
            disabled={!tickets.data?.hasMore || tickets.loading}
            onClick={() => setPage((n) => n + 1)}
          >
            Next
          </Button>
        </div>
      </aside>
      {selected ? (
        <ThreadView
          key={`${selected.inboxId}:${selected.conversationId}`}
          threadKey={selected}
          onBack={() => {
            setChosenInbox(selected.inboxId)
            onSelect(null)
          }}
          onDirty={onDirty}
        />
      ) : (
        <div className="hidden flex-1 items-center justify-center p-8 sm:flex">
          <div className="max-w-sm space-y-2 text-center">
            <FilePenLine
              className="mx-auto size-7 text-primary"
              aria-hidden="true"
            />
            <h2 className="font-medium">Select a thread</h2>
            <p className="text-sm text-muted-foreground">
              Read the conversation, add a private note or prepare a reply for
              review.
            </p>
          </div>
        </div>
      )}
    </div>
  )
}

function ThreadView({
  threadKey,
  onBack,
  onDirty,
}: {
  threadKey: ThreadKey
  onBack: () => void
  onDirty: (dirty: boolean) => void
}) {
  const load = useCallback(() => ops.thread(threadKey), [threadKey])
  const resource = useOpsResource(
    `thread:${threadKey.inboxId}:${threadKey.conversationId}`,
    load
  )
  return (
    <div className="min-w-0 flex-1 overflow-y-auto">
      <div className="border-b p-2 sm:hidden">
        <Button variant="ghost" className={touchButton} onClick={onBack}>
          <ArrowLeft className="rtl:rotate-180" aria-hidden="true" />
          Back to inbox
        </Button>
      </div>
      {resource.loading ? (
        <Loading label="Loading the conversation…" />
      ) : resource.error ? (
        <LoadError error={resource.error} retry={resource.reload} />
      ) : (
        resource.data && <ThreadBody thread={resource.data} onDirty={onDirty} />
      )}
    </div>
  )
}

export function ThreadBody({
  thread,
  onDirty,
}: {
  thread: Thread
  onDirty: (dirty: boolean) => void
}) {
  const [messages, setMessages] = useState(thread.messages)
  const editorKey = `thread:${thread.ticket.inboxId}:${thread.ticket.id}`
  const [editor, setEditor] = useOpsSessionState(`edit:${editorKey}`, () => {
    const binding = thread.draft?.reply ?? thread.suggestedReply
    return {
      draft: thread.draft,
      binding,
      fields: replyFields(binding),
      savedFields: JSON.stringify(replyFields(binding)),
      note: "",
      mode: "reply" as "reply" | "note",
    }
  })
  const { draft, binding, fields, savedFields, note, mode } = editor
  const setFields = (fields: typeof editor.fields) =>
    setEditor((previous) => ({ ...previous, fields }))
  const setNote = (note: string) =>
    setEditor((previous) => ({ ...previous, note }))
  const setMode = (mode: "reply" | "note") =>
    setEditor((previous) => ({ ...previous, mode }))
  const [busy, setBusy] = useOpsSessionState(`busy:${editorKey}`, false)
  const inFlight = useRef(false)
  const [error, setError] = useState("")
  const [notice, setNotice] = useState("")
  const draftDirty = JSON.stringify(fields) !== savedFields
  const dirty = draftDirty || note.length > 0
  const key = {
    inboxId: thread.ticket.inboxId,
    conversationId: thread.ticket.id,
  }
  const save = async () => {
    if (inFlight.current || busy) return
    inFlight.current = true
    setBusy(true)
    setError("")
    setNotice("")
    try {
      if (mode === "note") {
        const message = await ops.note({ ...key, content: note })
        setMessages((current) => [...current, message])
        setNote("")
        onDirty(draftDirty)
        setNotice("Private note saved. Visible only inside this workspace.")
      } else {
        const saved = await ops.saveDraft({
          ...key,
          expectedRevision: draft?.revision ?? 0,
          reply: completeReply(binding, fields),
        })
        setEditor((previous) => ({
          ...previous,
          draft: saved,
          binding: saved.reply,
          savedFields: JSON.stringify(fields),
        }))
        onDirty(note.length > 0)
        setNotice(
          `Draft saved · revision ${saved.revision}. Saving a draft does not send email.`
        )
      }
    } catch (e) {
      setError(opsError(e))
    } finally {
      inFlight.current = false
      setBusy(false)
    }
  }
  return (
    <article className="mx-auto max-w-3xl space-y-6 p-4 sm:p-6">
      <header className="space-y-2 border-b pb-4">
        <div className="text-xs font-medium text-muted-foreground">
          {statuses[thread.ticket.status]} · Thread #{thread.ticket.id}
        </div>
        <h1 dir="auto" className="break-words text-xl font-semibold">
          {thread.ticket.subject}
        </h1>
        <p className="break-all text-sm text-muted-foreground">
          <bdi>{thread.ticket.contact}</bdi> ·{" "}
          <bdi dir="ltr">{thread.contactEmail}</bdi>
        </p>
      </header>
      <div aria-label="Conversation messages" className="space-y-5">
        {messages.map((message) => (
          <Message key={message.id} from="assistant">
            <div className="flex flex-wrap items-center gap-2 text-xs text-muted-foreground">
              {message.private ? (
                <>
                  <LockKeyhole className="size-3.5" aria-hidden="true" />
                  <strong className="text-foreground">Private note</strong>
                </>
              ) : (
                <strong className="text-foreground">
                  {message.outgoing
                    ? message.status === 3
                      ? "Delivery failed"
                      : "Recorded outgoing email"
                    : "Incoming email"}
                </strong>
              )}
              <bdi dir={message.private ? "auto" : "ltr"}>
                {message.private
                  ? message.author
                  : message.outgoing
                    ? binding.from
                    : thread.contactEmail}
              </bdi>
              <time dateTime={message.createdAt}>
                {new Date(message.createdAt).toLocaleString()}
              </time>
            </div>
            <MessageContent
              className={cn(
                "rounded-lg border p-4",
                message.private && "border-amber-500/40 bg-amber-500/5"
              )}
            >
              <p
                dir="auto"
                className="whitespace-pre-wrap break-words [overflow-wrap:anywhere]"
              >
                {message.content}
              </p>
            </MessageContent>
          </Message>
        ))}
      </div>
      <section
        aria-label="Compose"
        className="space-y-4 rounded-xl border bg-card p-4"
      >
        <div className="flex flex-wrap gap-2">
          <Button
            className={touchButton}
            variant={mode === "reply" ? "secondary" : "ghost"}
            aria-pressed={mode === "reply"}
            onClick={() => {
              setMode("reply")
              setError("")
            }}
          >
            Reply draft
          </Button>
          <Button
            className={touchButton}
            variant={mode === "note" ? "secondary" : "ghost"}
            aria-pressed={mode === "note"}
            onClick={() => {
              setMode("note")
              setError("")
            }}
          >
            <LockKeyhole aria-hidden="true" />
            Private note
          </Button>
        </div>
        {mode === "reply" ? (
          <>
            <p className="text-sm text-muted-foreground">
              {draft ? `Saved draft · revision ${draft.revision}` : "New draft"}
              . Prepare the full reply here. Sending requires a proposal, human
              review and connected delivery.
            </p>
            <ReplyEditor
              fields={fields}
              disabled={busy}
              onChange={(next) => {
                setFields(next)
                onDirty(JSON.stringify(next) !== savedFields || note.length > 0)
                setNotice("")
              }}
            />
          </>
        ) : (
          <div className="space-y-2">
            <label htmlFor="ops-private-note" className="text-sm font-medium">
              Private note · never emailed
            </label>
            <Textarea
              id="ops-private-note"
              dir="auto"
              rows={5}
              value={note}
              className="min-h-32 rounded-lg"
              disabled={busy}
              onChange={(e) => {
                setNote(e.target.value)
                onDirty(draftDirty || e.target.value.length > 0)
                setNotice("")
              }}
            />
            <p className="text-xs text-muted-foreground">
              For internal context. This will not become a public reply.
            </p>
          </div>
        )}
        {error && (
          <Notice error>
            {error} Your text is still here. Copy it before refreshing if you
            need to reconcile a newer draft.
          </Notice>
        )}
        {notice && <Notice>{notice}</Notice>}
        <div className="flex flex-wrap items-center gap-3">
          <Button
            className={touchButton}
            disabled={
              busy || (mode === "note" ? !note.trim() : !draftDirty && !!draft)
            }
            onClick={() => void save()}
          >
            {busy
              ? "Saving…"
              : mode === "note"
                ? "Save private note"
                : "Save draft"}
          </Button>
          <span className="text-xs text-muted-foreground">
            {dirty
              ? "Unsaved changes · save before leaving"
              : "No unsaved changes"}
          </span>
        </div>
      </section>
    </article>
  )
}
