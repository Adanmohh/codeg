// Synthetic unit-test data only; never imported by production components.
import type { BusinessContext, Member } from "@/lib/business/identity"
import type { TaskDetail } from "@/lib/business/tasks"

export const member: Member = {
  id: "11111111-1111-4111-8111-111111111111",
  organizationId: "22222222-2222-4222-8222-222222222222",
  displayName: "Synthetic owner",
  kind: "human",
  role: "owner",
  domains: ["marketing", "engineering"],
  status: "active",
  revision: 1,
  operatorOwner: false,
  createdAt: "2026-09-08T10:00:00Z",
  updatedAt: "2026-09-08T10:00:00Z",
}
export const context: BusinessContext = {
  needsBootstrap: false,
  organization: {
    id: member.organizationId,
    name: "Synthetic organization",
    status: "active",
    revision: 1,
    authorizationEpoch: 1,
  },
  member,
  operator: false,
  capabilities: {
    manageMembers: true,
    manageTenantSettings: true,
    legacyOperator: false,
  },
}
export function detail(): TaskDetail {
  return {
    task: {
      id: "33333333-3333-4333-8333-333333333333",
      organizationId: member.organizationId,
      title: "Synthetic launch brief",
      notes: "Confirm the audience and success criteria.",
      domain: "marketing",
      status: "todo",
      priority: "normal",
      dueDate: "2026-09-08",
      ownerId: member.id,
      assigneeId: null,
      creatorId: member.id,
      reviewerId: null,
      revision: 1,
      currentDeliverableId: null,
      createdAt: "2026-09-08T10:00:00Z",
      updatedAt: "2026-09-08T10:00:00Z",
      archivedAt: null,
      capabilities: {
        edit: true,
        assign: true,
        progress: true,
        review: false,
        comment: true,
        submit: true,
        cancel: true,
        archive: false,
        linkExecution: false,
      },
    },
    activity: [],
    deliverables: [],
    execution: null,
  }
}
