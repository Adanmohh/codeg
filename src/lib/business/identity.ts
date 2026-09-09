import type { BusinessDomain } from "./presentation"
import type { SettingsOperations } from "./settings"

// approvals/docs/contracts/business-identity.md, f3c36dc6. UUIDs stay strings.
export const MEMBER_ROLES = [
  "owner",
  "admin",
  "manager",
  "member",
  "viewer",
] as const
export type MemberRole = (typeof MEMBER_ROLES)[number]
export interface Member {
  id: string
  organizationId: string
  displayName: string
  kind: "human" | "agent"
  role: MemberRole
  domains: BusinessDomain[]
  status: "active" | "revoked"
  revision: number
  operatorOwner: boolean
  createdAt: string
  updatedAt: string
}
export interface BusinessContext {
  needsBootstrap: boolean
  organization: Organization | null
  member: Member | null
  operator: boolean
  capabilities: {
    manageMembers: boolean
    manageTenantSettings: boolean
    legacyOperator: boolean
  }
}
// Exact tenancy projection at 29774b50. Presentation revision is separate.
export interface Organization {
  id: string
  name: string
  status: "active" | "suspended"
  revision: number
  authorizationEpoch: number
}
export interface Credential {
  id: string
  organizationId: string
  memberId: string
  label: string
  createdAt: string
  revokedAt: string | null
}
export interface MemberFields {
  displayName: string
  role: MemberRole
  domains: BusinessDomain[]
}
export interface IdentityOperations extends SettingsOperations {
  context: { input: Record<string, never>; result: BusinessContext }
  bootstrap: {
    input: { organizationName: string; ownerName: string }
    result: BusinessContext
  }
  "members/list": {
    input: { organizationId: string; domain?: BusinessDomain }
    result: Member[]
  }
  "members/create": {
    input: MemberFields & { organizationId: string; kind: Member["kind"] }
    result: Member
  }
  "members/update": {
    input: MemberFields & {
      organizationId: string
      memberId: string
      expectedRevision: number
    }
    result: Member
  }
  "members/revoke": {
    input: {
      organizationId: string
      memberId: string
      expectedRevision: number
    }
    result: Member
  }
  "credentials/issue": {
    input: { organizationId: string; memberId: string; label: string }
    result: { credential: Credential; token: string }
  }
  "credentials/list": {
    input: { organizationId: string; memberId: string }
    result: Credential[]
  }
  "credentials/revoke": {
    input: { organizationId: string; credentialId: string }
    result: Credential
  }
}
