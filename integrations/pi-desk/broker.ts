// Adapted from pi-mcp-adapter v2.32.1 types.ts/tool-approval.ts and its tests.
// Copyright (c) 2026 Nico Bailon. MIT; full notice in NOTICE and LICENSE.
import { HAFIDH_READ_TOOLS, isDeskContext, snapshotCall } from "./protocol.ts"
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

export { HAFIDH_READ_TOOLS } from "./protocol.ts"

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
      const read = HAFIDH_READ_TOOLS.find(
        (name) => name === request.originalToolName
      )
      if (!read) return "deny"
      snapshotCall(read, request.args)
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
      snapshotCall(read, request.args)
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
