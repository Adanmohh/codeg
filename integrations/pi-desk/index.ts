// Pi 0.85.1 extension lifecycle, commands and tool-call pattern. See NOTICE.
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent"
import { APPROVAL_EVENT, claimApproval, HAFIDH_READ_TOOLS, type ApprovalRequest } from "./broker.ts"
import { DESK_TOOLS, inputSchemas, isDeskTool, snapshotCall } from "./protocol.ts"
import { socketTransport, type DeskTransport } from "./transport.ts"

const descriptions = {
  desk_context: "Read this live task's Desk account and inboxes. No credentials.",
  desk_tickets: "List public tickets in a Desk inbox for this live task.",
  desk_thread: "Read public messages and the current versioned reply draft. Private notes are excluded.",
  desk_save_reply: "Save a reply draft using its expected revision (0 creates). This never sends mail.",
  desk_propose_reply: "Submit the exact saved draft revision for human review. This never approves or sends mail.",
}

export function installDesk(pi: ExtensionAPI, transport?: DeskTransport): void {
  let lifetime = new AbortController()
  pi.events.on(APPROVAL_EVENT, data => claimApproval(data as ApprovalRequest, transport, lifetime.signal))
  pi.on("session_start", () => { lifetime.abort(); lifetime = new AbortController() })
  pi.on("session_shutdown", () => lifetime.abort())

  pi.on("tool_call", event => {
    if (!isDeskTool(event.toolName)) return
    if (!transport || lifetime.signal.aborted) return { block: true, reason: "Desk bridge is unavailable" }
    try { snapshotCall(event.toolName, event.input) }
    catch { return { block: true, reason: "Desk input is invalid" } }
  })

  for (const name of DESK_TOOLS) {
    pi.registerTool({
      name, label: name, description: descriptions[name], parameters: inputSchemas[name],
      executionMode: "sequential",
      async execute(_id, input, signal) {
        try {
          // Revalidate here: pi does not revalidate after other tool_call hooks.
          const request = snapshotCall(name, input)
          if (!transport || lifetime.signal.aborted) throw new Error("Desk bridge is unavailable")
          const combined = signal ? AbortSignal.any([signal, lifetime.signal]) : lifetime.signal
          const outcome = await transport.call(request, combined)
          if (combined.aborted) throw new Error("Desk request aborted")
          return { content: [{ type: "text", text: JSON.stringify(outcome) }], details: outcome, isError: !outcome.ok }
        } catch {
          return { content: [{ type: "text", text: "Desk request failed or was cancelled. Reload context before retrying; a committed draft may already exist." }], details: { ok: false }, isError: true }
        }
      },
    })
  }

  pi.registerCommand("desk-status", {
    description: "Show Desk bridge and required Astra setup status (no model request)",
    async handler(_args, ctx) {
      const ready = transport && ctx.model?.id === "gpt-6-astra" && ctx.thinkingLevel === "max"
      ctx.ui.notify(ready ? "Desk bridge configured; capabilities are checked against the live task on every call."
        : "Desk setup required: live task bridge and a configured gpt-6-astra model with max reasoning. No fallback is selected.", ready ? "info" : "warning")
    },
  })
  pi.on("input", (_event, ctx) => {
    if (!transport || ctx.model?.id !== "gpt-6-astra" || ctx.thinkingLevel !== "max") {
      ctx.ui.notify("Desk setup required: live bridge and gpt-6-astra with max reasoning. Prompt blocked.", "warning")
      return { action: "handled" }
    }
    return { action: "continue" }
  })
}

export default async function deskExtension(pi: ExtensionAPI): Promise<void> {
  let transport: DeskTransport | undefined
  const socket = process.env.CODEG_DESK_SOCKET
  const token = process.env.CODEG_DESK_TOKEN
  if (socket && token) {
    try { transport = socketTransport(socket, token) } catch { /* tools remain fail-closed */ }
  }
  installDesk(pi, transport)
  // The launcher resolves this exact installed package and verifies its version.
  // No ambient config is imported, and no transport is created from tool input.
  const adapterPath = process.env.CODEG_DESK_ADAPTER
  if (!adapterPath) return
  const adapter = await import(adapterPath)
  const command = process.env.CODEG_DESK_HAFIDH_COMMAND
  const args = process.env.CODEG_DESK_HAFIDH_ARGS
  const config = {
    mcpServers: command ? {
      hafidh: { command, args: args ? JSON.parse(args) : [], approveTools: true,
        includeTools: [...HAFIDH_READ_TOOLS], directTools: false },
    } : {},
    settings: { approveTools: true, directTools: false },
  }
  await adapter.createMcpAdapter({ config })(pi)
}
