// Pi 0.85.1 extension lifecycle, commands and tool-call pattern. See NOTICE.
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent"
import {
  APPROVAL_EVENT,
  claimApproval,
  type ApprovalRequest,
} from "./broker.ts"
import {
  DESK_TOOLS,
  HAFIDH_READ_TOOLS,
  inputSchemas,
  isDeskTool,
  snapshotCall,
} from "./protocol.ts"
import { isAbsolute } from "node:path"
import type { McpConfig } from "pi-mcp-adapter/types"
import { socketTransport, type DeskTransport } from "./transport.ts"

const descriptions = {
  desk_business_task:
    "Read the public business task assigned and linked to this live run, including current revision and deliverables. No cross-task search or private source data; missing/revoked linkage requires a human to link an existing execution.",
  desk_business_progress:
    "Update only this linked business task to todo, in_progress or review at its current expected revision. Done and cancellation are unavailable; only an authorized human can accept review. Revoked delegation or stale run/revision requires reload.",
  desk_business_note:
    "Append a public work note to this linked business task at its expected revision. The backend supplies authorship. This never approves work or acts on a provider.",
  desk_business_submit:
    "Submit an exact public deliverable for human review on this linked business task at its expected revision. This cannot accept review, complete the task or perform an external action.",
  desk_context:
    "Read this live task's Desk account and inboxes. No credentials.",
  desk_tickets: "List public tickets in a Desk inbox for this live task.",
  desk_thread:
    "Read public messages and the current versioned reply draft. Private notes are excluded.",
  desk_save_reply:
    "Save a reply draft using its expected revision (0 creates). This never sends mail.",
  desk_propose_reply:
    "Submit the exact saved draft revision for human review. This never approves or sends mail.",
  desk_propose_issue:
    "Propose an existing human-prepared issue draft/revision for human review. Use the cached Hafidh reads to choose it. Current host source freshness, evidence, severity and live task scope are required. Missing/ambiguous product bindings require operator configuration; expired sources require operator refresh. No import, evidence creation, approval or filing.",
}

export function installDesk(pi: ExtensionAPI, transport?: DeskTransport): void {
  let lifetime = new AbortController()
  pi.events.on(APPROVAL_EVENT, (data) =>
    claimApproval(data as ApprovalRequest, transport, lifetime.signal)
  )
  pi.on("session_start", () => {
    lifetime.abort()
    lifetime = new AbortController()
  })
  pi.on("session_shutdown", () => lifetime.abort())

  pi.on("tool_call", (event) => {
    if (!isDeskTool(event.toolName)) return
    if (!transport || lifetime.signal.aborted)
      return { block: true, reason: "Desk bridge is unavailable" }
    try {
      snapshotCall(event.toolName, event.input)
    } catch {
      return { block: true, reason: "Desk input is invalid" }
    }
  })

  for (const name of DESK_TOOLS) {
    pi.registerTool({
      name,
      label: name,
      description: descriptions[name],
      parameters: inputSchemas[name],
      executionMode: "sequential",
      async execute(_id, input, signal) {
        try {
          // Revalidate here: pi does not revalidate after other tool_call hooks.
          const request = snapshotCall(name, input)
          if (!transport || lifetime.signal.aborted)
            throw new Error("Desk bridge is unavailable")
          const combined = signal
            ? AbortSignal.any([signal, lifetime.signal])
            : lifetime.signal
          const outcome = await transport.call(request, combined)
          if (combined.aborted) throw new Error("Desk request aborted")
          return {
            content: [{ type: "text", text: JSON.stringify(outcome) }],
            details: outcome,
            isError: !outcome.ok,
          }
        } catch {
          return {
            content: [
              {
                type: "text",
                text: "Desk request failed or was cancelled. Reload context before retrying; a committed draft may already exist.",
              },
            ],
            details: { ok: false },
            isError: true,
          }
        }
      },
    })
  }

  pi.registerCommand("desk-status", {
    description:
      "Show Desk bridge and required Astra setup status (no model request)",
    async handler(_args, ctx) {
      const tools = pi.getAllTools().map((tool) => tool.name)
      const discovered = HAFIDH_READ_TOOLS.filter((name) =>
        tools.includes(name)
      )
      const business = DESK_TOOLS.filter(
        (name) => name.startsWith("desk_business_") && tools.includes(name)
      )
      const ready =
        transport &&
        ctx.model?.id === "gpt-6-astra" &&
        ctx.thinkingLevel === "max"
      ctx.ui.notify(
        ready
          ? `Desk bridge configured; capabilities are checked against the live task on every call. Cached intake tools discovered: ${discovered.join(", ") || "none; check the companion installation"}. Linked work tools: ${business.join(", ") || "unavailable"}.`
          : "Desk setup required: live task bridge and a configured gpt-6-astra model with max reasoning. No fallback is selected.",
        ready ? "info" : "warning"
      )
    },
  })
  async function healthyBridge(): Promise<boolean> {
    if (!transport?.health || lifetime.signal.aborted) return false
    const signal = lifetime.signal
    try {
      const alive = await transport.health(signal)
      return !signal.aborted && alive
    } catch {
      return false
    }
  }
  pi.on("input", async (_event, ctx) => {
    if (
      ctx.model?.id !== "gpt-6-astra" ||
      ctx.thinkingLevel !== "max" ||
      !(await healthyBridge())
    ) {
      ctx.ui.notify(
        "Desk setup required: running bridge and gpt-6-astra with max reasoning. Prompt blocked.",
        "warning"
      )
      return { action: "handled" }
    }
    return { action: "continue" }
  })
  // Unlike before_provider_request exceptions (which Pi catches), this pinned
  // lifecycle result actually cancels manual and automatic compaction.
  pi.on("session_before_compact", async (_event, ctx) => {
    if (
      ctx.model?.id !== "gpt-6-astra" ||
      ctx.thinkingLevel !== "max" ||
      !(await healthyBridge())
    )
      return { cancel: true }
  })
}

export default async function deskExtension(pi: ExtensionAPI): Promise<void> {
  let transport: DeskTransport | undefined
  const socket = process.env.CODEG_DESK_SOCKET
  const token = process.env.CODEG_DESK_TOKEN
  if (socket && token) {
    try {
      transport = socketTransport(socket, token)
    } catch {
      /* tools remain fail-closed */
    }
  }
  installDesk(pi, transport)
  // The launcher resolves this exact installed package and verifies its version.
  // No ambient config is imported, and no transport is created from tool input.
  const adapterPath = process.env.CODEG_DESK_ADAPTER
  if (!adapterPath) return
  const adapter = await import(adapterPath)
  const config = cachedIntakeConfig(process.env)
  await adapter.createMcpAdapter({ config })(pi)
}

/** Only prepare_at's overwritten launch values configure this fixed companion.
 * The endpoint is local host IPC; there is no Hafidh provider secret or command.
 */
export function cachedIntakeConfig(env: NodeJS.ProcessEnv): McpConfig {
  const {
    CODEG_DESK_COMPANION: companion,
    CODEG_DESK_SOCKET: socket,
    CODEG_DESK_TOKEN: token,
  } = env
  const configured = companion && isAbsolute(companion) && socket && token
  return {
    mcpServers: configured
      ? {
          hafidh: {
            command: companion,
            args: [
              "--features",
              "intake",
              "--parent-connection-id",
              "token-bound-intake",
              "--socket-path",
              socket,
              "--token",
              token,
            ],
            env: { CODEG_PI_DESK_LAUNCH: "", CODEG_TOKEN: "" },
            literalEnv: true,
            lifecycle: "eager",
            toolPrefix: "none",
            includeTools: [...HAFIDH_READ_TOOLS],
            directTools: [...HAFIDH_READ_TOOLS],
            exposeResources: false,
            approveTools: true,
            debug: false,
            requestTimeoutMs: 10000,
          },
        }
      : {},
    settings: {
      approveTools: true,
      directTools: false,
      autoAuth: false,
      sampling: false,
      elicitation: false,
    },
  }
}
