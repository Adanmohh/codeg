// Adapted from pi-mcp-adapter v2.32.1 types.ts/tool-approval.ts and its tests.
// Copyright (c) 2026 Nico Bailon. MIT; full notice in NOTICE and LICENSE.
import { isDeskContext, snapshotCall } from "./protocol.ts"
import type { DeskTransport } from "./transport.ts"

export const APPROVAL_EVENT = "pi-mcp-adapter:tool-approval-request"
export type ApprovalDecision =
  | "allow_once"
  | "allow_for_session"
  | "deny"
  | "abstain"
export interface ApprovalRequest {
  requestId: string
  serverName: string
  originalToolName: string
  prefixedToolName: string
  args: Record<string, unknown>
  origin: "proxy" | "direct" | "script" | "resource" | "iframe"
  signal?: AbortSignal
  claim(handler: () => ApprovalDecision | Promise<ApprovalDecision>): boolean
}

export const HAFIDH_READ_TOOLS = [
  "hafidh_feedback_list",
  "hafidh_feedback_get",
  "hafidh_intake_status",
] as const

/** Mutations are always denied; edits are submitted through the Desk draft tool. */
export function claimApproval(
  request: ApprovalRequest,
  transport: DeskTransport | undefined,
  lifecycle: AbortSignal
): void {
  // claim MUST happen synchronously during emit. No IO or await before this call.
  request.claim(async () => {
    if (
      !transport ||
      lifecycle.aborted ||
      request.signal?.aborted ||
      request.serverName !== "hafidh" ||
      !HAFIDH_READ_TOOLS.some((name) => name === request.originalToolName) ||
      !["proxy", "direct"].includes(request.origin)
    )
      return "deny"
    const snapshot = JSON.stringify(request.args)
    const server = request.serverName
    const tool = request.originalToolName
    const signal = request.signal
      ? AbortSignal.any([lifecycle, request.signal])
      : lifecycle
    try {
      const response = await transport.call(
        snapshotCall("desk_context", {}),
        signal
      )
      if (
        signal.aborted ||
        !response.ok ||
        !isDeskContext(response.value) ||
        server !== request.serverName ||
        tool !== request.originalToolName ||
        snapshot !== JSON.stringify(request.args)
      )
        return "deny"
      // Freeze the actual execution argument tree: no post-guard substitution.
      freezeJson(request.args)
      return "allow_once"
    } catch {
      return "deny"
    }
  })
}

function freezeJson(value: unknown): void {
  if (value === null || typeof value !== "object" || Object.isFrozen(value))
    return
  for (const child of Object.values(value)) freezeJson(child)
  Object.freeze(value)
}
