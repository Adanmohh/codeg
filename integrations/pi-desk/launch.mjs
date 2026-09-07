#!/usr/bin/env node
// pi-acp 0.0.33 process/command pattern; Pi 0.85.1 explicit extension loader.
// Full attributions in NOTICE and LICENSE. No shell, installation or login.
import { spawn, spawnSync } from "node:child_process"
import { readFileSync, realpathSync } from "node:fs"
import { createRequire } from "node:module"
import { dirname, join, resolve } from "node:path"
import { fileURLToPath } from "node:url"
import { createInterface } from "node:readline"
import { randomUUID } from "node:crypto"

const SETUP = "Desk setup required: install pi 0.85.1 and pi-mcp-adapter 2.32.1, and configure gpt-6-astra with max reasoning in the existing Pi client. No fallback was selected."
const here = dirname(fileURLToPath(import.meta.url))

function packageRoot(entry, name, version) {
  let dir = dirname(realpathSync(entry))
  while (true) {
    try {
      const pkg = JSON.parse(readFileSync(join(dir, "package.json"), "utf8"))
      if (pkg.name === name) {
        if (pkg.version !== version) throw new Error(SETUP)
        return dir
      }
    } catch (error) { if (error.message === SETUP) throw error }
    const parent = dirname(dir)
    if (parent === dir) throw new Error(SETUP)
    dir = parent
  }
}

function runtime() {
  const pi = realpathSync(process.env.CODEG_DESK_REAL_PI ?? "")
  packageRoot(pi, "@earendil-works/pi-coding-agent", "0.85.1")
  const require = createRequire(pi)
  const adapter = require.resolve("pi-mcp-adapter")
  packageRoot(adapter, "pi-mcp-adapter", "2.32.1")
  return { pi, adapter }
}

export function rpcRefusal(command, provider) {
  if (!command || typeof command !== "object" || typeof command.type !== "string") return "Invalid pi RPC input"
  if (command.type === "set_model" && (command.modelId !== "gpt-6-astra" || command.provider !== provider)) return "Desk requires its configured gpt-6-astra provider; relaunch to change it"
  if (command.type === "set_thinking_level" && command.level !== "max") return "Desk requires max reasoning; reasoning change refused"
  if (["cycle_model", "cycle_thinking_level", "new_session", "switch_session", "fork", "clone"].includes(command.type)) return "Desk model and session are bound to this launch; start a new task run to change them"
  return undefined
}

function main() {
  const { pi, adapter } = runtime()
  if (process.argv.includes("--probe")) {
    // Offline catalogue metadata only; no auth export or inference request.
    const result = spawnSync(pi, ["--offline", "--no-extensions", "--no-skills", "--no-prompt-templates",
      "--no-themes", "--no-context-files", "--no-approve", "--list-models", "gpt-6-astra"],
    { encoding: "utf8", timeout: 10000, maxBuffer: 1024 * 1024, env: { ...process.env, PI_OFFLINE: "1" } })
    const providers = result.status === 0 ? (result.stdout ?? "").split("\n")
      .map(line => line.trim().split(/\s+/)).filter(cells => cells[1] === "gpt-6-astra").map(cells => cells[0]) : []
    process.stdout.write(JSON.stringify({ adapter, modelAvailable: providers.length > 0 }) + "\n")
    return
  }
  if (!process.env.CODEG_DESK_TOKEN || !process.env.CODEG_DESK_SOCKET) throw new Error("Desk bridge is unavailable; launch from a live Desk task")
  const supplied = process.argv.slice(2)
  // pi-acp's immutable launch contract: --mode rpc --no-themes [--session path].
  let session
  for (let i = 0; i < supplied.length; i++) {
    if (supplied[i] === "--mode" && supplied[i + 1] === "rpc") { i++; continue }
    if (supplied[i] === "--no-themes") continue
    if (supplied[i] === "--session" && supplied[i + 1]) { session = supplied[++i]; continue }
    throw new Error("Unsupported pi-acp launch arguments")
  }
  const args = ["--offline", "--mode", "rpc", "--no-extensions", "--no-skills", "--no-prompt-templates",
    "--no-themes", "--no-context-files", "--no-approve", "-e", join(here, "index.ts"),
    "--model", "gpt-6-astra", "--models", "*/gpt-6-astra", "--thinking", "max"]
  if (session) args.push("--session", session)
  const env = { ...process.env, CODEG_DESK_ADAPTER: adapter, PI_OFFLINE: "1" }
  delete env.CODEG_TOKEN
  delete env.CODEG_PI_DESK_LAUNCH
  const child = spawn(pi, args, { stdio: "pipe", env })
  let ready = false
  let failed = false
  let inputClosed = false
  let provider
  const queued = []
  let queuedBytes = 0
  const probe = `desk-discovery-${randomUUID()}`
  const stateProbe = `desk-state-${randomUUID()}`
  const timer = setTimeout(() => fail(SETUP), 10000)
  const emit = data => process.stdout.write(JSON.stringify(data) + "\n")
  const fail = message => {
    if (failed) return
    failed = true; clearTimeout(timer); process.stderr.write(message + "\n"); child.kill(); process.exitCode = 1
  }
  const forward = line => {
    if (failed) return
    const message = JSON.parse(line)
    const refusal = rpcRefusal(message, provider)
    if (refusal) emit({ type: "response", id: message?.id, command: message?.type, success: false, error: refusal })
    else child.stdin.write(line + "\n")
  }
  child.once("error", () => fail(SETUP))
  child.once("exit", code => { clearTimeout(timer); process.exit(failed ? 1 : (code ?? 1)) })
  child.stdin.on("error", () => fail("Desk RPC process disconnected"))
  // Third-party diagnostics can contain custom configuration. Only expose our
  // bounded, credential-free setup error; never echo provider config/errors.
  child.stderr.on("data", () => {})
  createInterface({ input: child.stdout }).on("line", line => {
    let message
    try { message = JSON.parse(line) } catch { return }
    if (message.id === probe) {
      const names = message.data?.commands?.map(command => command.name) ?? []
      if (!message.success || !names.includes("desk-status") || !names.includes("mcp")) { fail("Desk extension discovery failed; prompts are blocked"); return }
      child.stdin.write(JSON.stringify({ type: "get_state", id: stateProbe }) + "\n")
      return
    }
    if (message.id === stateProbe) {
      if (!message.success || message.data?.model?.id !== "gpt-6-astra" || message.data?.thinkingLevel !== "max") { fail(SETUP); return }
      provider = message.data.model.provider
      clearTimeout(timer); ready = true
      for (const request of queued.splice(0)) forward(request)
      if (inputClosed) child.stdin.end()
      return
    }
    emit(message)
  })
  child.stdin.write(JSON.stringify({ type: "get_commands", id: probe }) + "\n")
  createInterface({ input: process.stdin }).on("line", line => {
    if (failed) return
    try { JSON.parse(line) } catch { fail("Invalid pi RPC input"); return }
    if (ready) forward(line)
    else {
      queuedBytes += Buffer.byteLength(line)
      if (queuedBytes > 1024 * 1024) { fail("Desk startup input exceeded its limit"); return }
      queued.push(line)
    }
  }).on("close", () => { inputClosed = true; if (ready) child.stdin.end() })
  for (const signal of ["SIGTERM", "SIGINT"]) process.on(signal, () => { child.kill(signal); process.exit(0) })
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try { main() } catch { process.stderr.write(SETUP + "\n"); process.exitCode = 1 }
}
