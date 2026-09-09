// Synthetic unit fixtures only. Never imported by a product component.
import type {
  BindingAdmin,
  BindingView,
  CandidateDetail,
  Decision,
  SourceDetail,
} from "@/lib/business/intake"
import { member } from "@/components/business/test-fixtures"

export const binding: BindingView = {
  binding: {
    id: "44444444-4444-4444-8444-444444444444",
    kind: "fireflies",
    label: "Synthetic meeting account",
    domain: "marketing",
    revision: 1,
    accessEpoch: 1,
    enabled: true,
    credentialState: "present",
    capabilities: {
      read: true,
      import: true,
      triage: true,
      publicationDomains: ["marketing"],
    },
  },
  admin: null,
}
export const admin: BindingAdmin = {
  id: binding.binding.id,
  kind: "fireflies",
  label: binding.binding.label,
  domain: "marketing",
  revision: 1,
  accessEpoch: 1,
  enabled: false,
  credentialState: "present",
  sourceOwnerId: member.id,
  publicationDomains: ["marketing"],
  retainedTaskText: true,
  resource: {
    kind: "fireflies",
    providerUserId: "synthetic-provider-account",
    mine: true,
  },
}
export function source(): SourceDetail {
  return {
    source: {
      id: "55555555-5555-4555-8555-555555555555",
      bindingId: binding.binding.id,
      kind: "fireflies",
      title: "Synthetic customer conversation",
      revision: 2,
      observedAt: new Date().toISOString(),
      accessValidUntil: new Date(Date.now() + 300000).toISOString(),
      access: "fresh",
      content: "available",
      summary: "empty",
      providerSummaryStatus: "ready",
      requiresRefresh: false,
    },
    disclosure: "fresh",
    passages: [
      {
        id: "66666666-6666-4666-8666-666666666666",
        sourceRevision: 2,
        kind: "sentence",
        text: "Synthetic exact passage: ask the customer to review the website brief.",
        index: 0,
        start: 10,
        end: 14,
      },
    ],
    candidateCount: 1,
  }
}
export function candidate(reading = source()): CandidateDetail {
  return {
    source: reading.source,
    passages: reading.passages,
    decision: null,
    candidate: {
      id: "77777777-7777-4777-8777-777777777777",
      sourceId: reading.source.id,
      revision: 3,
      sourceRevision: 2,
      state: "pending",
      origin: "human_selection",
      requiresRebase: false,
      disclosure: "fresh",
      hasPreparedDraft: true,
      draft: {
        title: "Synthetic customer brief",
        notes: "Private prepared task text",
        domain: "marketing",
        priority: "normal",
        dueDate: "2028-02-29",
        ownerId: member.id,
        assigneeId: null,
        reviewerId: null,
      },
      ownerSuggestion: null,
      dueSuggestion: null,
      capabilities: {
        select: true,
        edit: true,
        accept: true,
        link: true,
        discard: true,
        publicationDomains: ["marketing"],
      },
    },
  }
}
export function decision(kind: Decision["kind"] = "accepted"): Decision {
  return {
    id: "88888888-8888-4888-8888-888888888888",
    candidateId: "77777777-7777-4777-8777-777777777777",
    fromRevision: 3,
    sourceRevision: 2,
    kind,
    actorId: member.id,
    createdAt: "2026-09-08T19:00:00Z",
    task:
      kind === "discarded"
        ? { state: "none" }
        : {
            state: "accessible",
            taskId: "33333333-3333-4333-8333-333333333333",
            taskRevision: 1,
          },
  }
}
