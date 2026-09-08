import type {
  BusinessDomain,
  BusinessPriority,
  BusinessStatus,
  PersonLabel,
  TaskPreview,
} from "./presentation"
import type { Member } from "./identity"

// Mirrors tickets' fixed business-tasks.md and business_tasks/types.rs.
export interface TaskFields {
  title: string
  notes: string
  domain: BusinessDomain
  priority: BusinessPriority
  dueDate: string | null
}
export interface Assignment {
  ownerId: string
  assigneeId: string | null
  reviewerId: string | null
}
export interface Task extends TaskFields, Assignment {
  id: string
  organizationId: string
  creatorId: string
  status: BusinessStatus
  revision: number
  currentDeliverableId: string | null
  createdAt: string
  updatedAt: string
  archivedAt: string | null
  capabilities: {
    edit: boolean
    assign: boolean
    progress: boolean
    review: boolean
    comment: boolean
    submit: boolean
    cancel: boolean
    archive: boolean
    linkExecution: boolean
  }
}
export interface TaskActor {
  id: string
  displayName: string
  kind: "human" | "agent"
}
export interface Activity {
  id: string
  revision: number
  kind: string
  actor: TaskActor
  payload: unknown
  createdAt: string
}
export interface Deliverable {
  id: string
  revision: number
  author: TaskActor
  body: string
  createdAt: string
}
export interface TaskDetail {
  task: Task
  activity: Activity[]
  deliverables: Deliverable[]
  execution: {
    workTaskId: number
    runSeq: number
    agentMemberId: string
    active: boolean
  } | null
}
export interface TaskPage {
  tasks: Task[]
  page: number
  hasMore: boolean
  canCreate: boolean
}
export interface ListInput {
  view?: "shared" | "mine"
  domain?: BusinessDomain
  status?: BusinessStatus
  query?: string
  page?: number
  archived?: boolean
}
export interface RevisionInput {
  taskId: string
  expectedRevision: number
}
type DetailOperation<T> = { input: T; result: TaskDetail }
export interface TaskOperations {
  list: { input: ListInput; result: TaskPage }
  get: DetailOperation<{ taskId: string }>
  create: DetailOperation<TaskFields & Partial<Assignment>>
  update: DetailOperation<RevisionInput & TaskFields>
  assign: DetailOperation<RevisionInput & Assignment>
  progress: DetailOperation<
    RevisionInput & { status: "todo" | "in_progress" | "review" }
  >
  note: DetailOperation<RevisionInput & { body: string }>
  submit: DetailOperation<RevisionInput & { body: string }>
  review: DetailOperation<
    RevisionInput & { decision: "accept" | "return"; comment?: string }
  >
  cancel: DetailOperation<RevisionInput>
  archive: DetailOperation<RevisionInput & { archived: boolean }>
  "link-execution": DetailOperation<RevisionInput & { workTaskId: number }>
  "entrust-execution": DetailOperation<RevisionInput & { workTaskId: number }>
}
export function personFor(
  id: string | null,
  members: Member[],
  unavailable: string
): PersonLabel | null {
  if (!id) return null
  const member = members.find((candidate) => candidate.id === id)
  return {
    name: member?.displayName ?? unavailable,
    kind: member?.kind ?? "unknown",
  }
}
export function taskPreview(
  task: Task,
  members: Member[],
  unavailable: string
): TaskPreview {
  return {
    key: task.id,
    title: task.title,
    domain: task.domain,
    status: task.status,
    priority: task.priority,
    dueDate: task.dueDate,
    owner: personFor(task.ownerId, members, unavailable),
    assignee: personFor(task.assigneeId, members, unavailable),
    reviewer: personFor(task.reviewerId, members, unavailable),
  }
}
