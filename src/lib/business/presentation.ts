// Vocabulary from tickets' business-tasks contract (cb2e184f). These are view
// properties, not wire/auth DTOs. The adapter retains backend IDs and revisions.
export const BUSINESS_STATUSES = [
  "todo",
  "in_progress",
  "review",
  "done",
  "cancelled",
] as const
export type BusinessStatus = (typeof BUSINESS_STATUSES)[number]
export const BUSINESS_DOMAINS = [
  "marketing",
  "channels",
  "ads",
  "website",
  "feedback",
  "engineering",
] as const
export type BusinessDomain = (typeof BUSINESS_DOMAINS)[number]
export const BUSINESS_PRIORITIES = ["low", "normal", "high", "urgent"] as const
export type BusinessPriority = (typeof BUSINESS_PRIORITIES)[number]

export interface PersonLabel {
  name: string
  kind: "human" | "agent" | "unknown"
}
export interface TaskPreview {
  key: string
  title: string
  domain: BusinessDomain
  status: BusinessStatus
  priority: BusinessPriority
  dueDate: string | null
  owner: PersonLabel | null
  assignee: PersonLabel | null
  reviewer: PersonLabel | null
}
