"use client"

import { Button } from "@/components/ui/button"
import { groupTasksByColumn } from "@/components/tasks/board-columns"
import { StatusChip } from "@/components/tasks/task-card"
import { TASK_LIST_LINE } from "@/components/tasks/task-row"
import { ops, type ThreadKey } from "@/lib/ops/api"
import { LoadError, Loading, touchButton } from "./ui"
import { useOpsResource } from "./use-ops-resource"

export function MorningView({
  onThread,
  onProposal,
  onTasks,
}: {
  onThread: (key: ThreadKey) => void
  onProposal: (id: number) => void
  onTasks: () => void
}) {
  const resource = useOpsResource("morning", ops.morning)
  if (resource.loading)
    return <Loading label="Reading tasks and inbox queues…" />
  if (resource.error)
    return <LoadError error={resource.error} retry={resource.reload} />
  if (!resource.data) return null
  const data = resource.data
  const tasks = groupTasksByColumn(data.tasks, false)
  const pending = data.proposals.filter((p) => p.status === "pending")
  return (
    <div className="overflow-y-auto p-4 sm:p-6">
      <div className="mx-auto max-w-4xl space-y-8">
        <header className="space-y-2">
          <p className="text-xs font-medium uppercase tracking-wider text-muted-foreground">
            Morning desk
          </p>
          <h1 className="text-2xl font-semibold">What needs your attention</h1>
          <p className="text-sm text-muted-foreground">
            Current task runs and local inbox queues. Refresh to pick up new
            activity.
          </p>
        </header>
        <section className="space-y-3">
          <h2 className="text-base font-semibold">Replies to review</h2>
          {pending.length ? (
            <ul className="divide-y rounded-xl border">
              {pending.map((p) => (
                <li key={p.id}>
                  <button
                    className="min-h-14 w-full space-y-1 p-4 text-start hover:bg-muted/50 focus-visible:outline-2 focus-visible:outline-ring"
                    onClick={() => onProposal(p.id)}
                  >
                    <span className="block break-words text-sm font-medium">
                      {p.payload?.reply.subject ?? `Proposal #${p.id}`}
                    </span>
                    <span className="text-xs text-muted-foreground">
                      Task #{p.taskId} ·{" "}
                      {p.stale ? "Stale review" : "Human review required"}
                    </span>
                  </button>
                </li>
              ))}
            </ul>
          ) : (
            <p className="text-sm text-muted-foreground">
              No pending reply proposals.
            </p>
          )}
        </section>
        {(["attention", "inProgress", "todo"] as const).map((group) => (
          <section className="space-y-3" key={group}>
            <div className="flex items-center justify-between gap-3">
              <h2 className="text-base font-semibold">
                {
                  {
                    attention: "Tasks awaiting attention",
                    inProgress: "In progress",
                    todo: "Up next",
                  }[group]
                }
              </h2>
              {group === "attention" && (
                <Button
                  className={touchButton}
                  variant="outline"
                  onClick={onTasks}
                >
                  Open task workspace
                </Button>
              )}
            </div>
            {tasks[group].length ? (
              <ul className="divide-y rounded-xl border">
                {tasks[group].map((task) => (
                  <li
                    className={`${TASK_LIST_LINE} min-h-16 py-3`}
                    key={task.id}
                  >
                    <div className="min-w-0 flex-1">
                      <p className="break-words text-sm font-medium">
                        {task.title}
                      </p>
                      <p className="truncate text-xs text-muted-foreground">
                        {task.last_error ??
                          task.latest_progress ??
                          `Task #${task.id}`}
                      </p>
                    </div>
                    <StatusChip task={task} className="text-xs" />
                  </li>
                ))}
              </ul>
            ) : (
              <p className="text-sm text-muted-foreground">
                No tasks in this queue.
              </p>
            )}
          </section>
        ))}
        <section className="space-y-3">
          <h2 className="text-base font-semibold">Open correspondence</h2>
          {data.tickets.length ? (
            <ul className="divide-y rounded-xl border">
              {data.tickets.map((ticket) => (
                <li key={ticket.id}>
                  <button
                    className="min-h-14 w-full space-y-1 p-4 text-start hover:bg-muted/50 focus-visible:outline-2 focus-visible:outline-ring"
                    onClick={() =>
                      onThread({
                        inboxId: ticket.inboxId,
                        conversationId: ticket.id,
                      })
                    }
                  >
                    <span className="block break-words text-sm font-medium">
                      {ticket.subject}
                    </span>
                    <span className="text-xs text-muted-foreground">
                      {ticket.contact} · Thread #{ticket.id}
                    </span>
                  </button>
                </li>
              ))}
            </ul>
          ) : (
            <p className="text-sm text-muted-foreground">
              No open correspondence. New email will appear after intake is
              connected.
            </p>
          )}
        </section>
      </div>
    </div>
  )
}
