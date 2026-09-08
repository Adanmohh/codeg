// Report-only artifact acceptance harness; no product imports or builds.
// Adapted from integrations/pi-desk/process.test.ts at Codeg 294fb634b1833ebb13c4223484624e595598235b
// (blob e0d98025cb98719aaa0027b64022717c55483c2e).
// Changes: explicit artifact/hash handoff, synthetic reads/refusals, bounded
// cleanup and sanitized evidence. See the adjacent NOTICE and root LICENSE.
// SPDX-License-Identifier: Apache-2.0 AND MIT

import { spawn } from "node:child_process"
import { createHash, randomBytes } from "node:crypto"
import { constants } from "node:fs"
import { access, chmod, mkdtemp, readFile, realpath, rm, stat, writeFile } from "node:fs/promises"
import { createServer } from "node:net"
import { dirname, isAbsolute, join, resolve } from "node:path"
import { createInterface } from "node:readline"
import { fileURLToPath } from "node:url"
import { isDeepStrictEqual } from "node:util"

const evidenceDirectory = dirname(fileURLToPath(import.meta.url))
const worktree = resolve(evidenceDirectory, "../..")
const expectedWorktree = "/Users/mohamedadan/projects/_worktrees/ops-desk/tickets"
const timeoutMs = 5000
const children = []
const fixtures = []
let interrupted = false
const report = {
  status: "not_run",
  scope: "actual bundled MCP with a synthetic UDS peer; not live backend authorization",
  recipeSource: "294fb634b1833ebb13c4223484624e595598235b",
  node: process.version,
  checks: [],
  failures: [],
  children: [],
  groups: [],
  harnessActions: { modelOrPiLaunches: 0, providerRequestsIssued: 0, tcpListeners: 0 },
}

class CheckFailure extends Error {}
function check(condition, label) {
  if (!condition) throw new CheckFailure(label)
  report.checks.push(label)
}
function equal(actual, expected, label) {
  check(isDeepStrictEqual(actual, expected), label)
}
function deferred() {
  let resolvePromise
  const promise = new Promise((done) => { resolvePromise = done })
  return { promise, resolve: resolvePromise }
}
async function deadline(promise, label, duration = timeoutMs) {
  let timer
  try {
    return await Promise.race([
      promise,
      new Promise((_, reject) => {
        timer = setTimeout(() => reject(new CheckFailure(label)), duration)
      }),
    ])
  } finally {
    clearTimeout(timer)
  }
}
function failure(error) {
  // Never serialize AssertionError payloads, request JSON, tokens or raw IO errors.
  report.failures.push(error instanceof CheckFailure ? error.message : "harness_io_error")
}
function options() {
  const keys = ["--binary", "--sha256", "--source", "--package-version", "--stable-window"]
  const values = {}
  const argv = process.argv.slice(2)
  check(argv.length === keys.length * 2, "complete_explicit_artifact_handoff")
  for (let i = 0; i < argv.length; i += 2) {
    check(keys.includes(argv[i]) && !Object.hasOwn(values, argv[i]), "known_unique_handoff_option")
    values[argv[i]] = argv[i + 1]
  }
  check(keys.every((key) => typeof values[key] === "string" && values[key].length > 0), "all_handoff_values_present")
  check(isAbsolute(values["--binary"]), "absolute_artifact_path")
  check(/^[a-f0-9]{64}$/.test(values["--sha256"]), "full_sha256")
  check(/^[a-f0-9]{40}$/.test(values["--source"]), "immutable_source_sha")
  check(/^\d+\.\d+\.\d+(?:-[a-zA-Z0-9.-]+)?$/.test(values["--package-version"]), "explicit_package_version")
  check(/^[a-zA-Z0-9._:-]{1,120}$/.test(values["--stable-window"]), "explicit_stable_window_reference")
  return values
}
async function identity(path) {
  const resolved = await realpath(path)
  check(resolved.endsWith("/Contents/MacOS/codeg-mcp"), "actual_bundle_companion_path")
  const metadata = await stat(resolved)
  check(metadata.isFile() && metadata.size > 0, "nonempty_regular_artifact")
  await access(resolved, constants.X_OK)
  return {
    path: resolved,
    bytes: metadata.size,
    sha256: createHash("sha256").update(await readFile(resolved)).digest("hex"),
  }
}
function launch(binary, args, cwd, label) {
  check(!interrupted, "no_launch_after_interrupt")
  const child = spawn(binary, args, {
    cwd, shell: false, stdio: "pipe",
    // No inherited HOME, NODE_OPTIONS, CODEG_PI_DESK_LAUNCH or credentials.
    env: { NODE_ENV: "test", PATH: "/usr/bin:/bin" },
  })
  const closed = deferred()
  const entry = {
    label, pid: child.pid ?? null, exitCode: null, signal: null,
    spawnError: false, stdinError: false, stderrBytes: 0, forcedTermination: false,
  }
  child.once("error", () => { entry.spawnError = true })
  child.stdin.on("error", () => { entry.stdinError = true })
  child.stderr.on("data", (bytes) => { entry.stderrBytes += bytes.length })
  child.once("close", (code, signal) => {
    entry.exitCode = code
    entry.signal = signal
    closed.resolve()
  })
  const handle = { child, entry, closed: closed.promise }
  children.push(handle)
  report.children.push(entry)
  return handle
}
async function help(binary, scratch) {
  const handle = launch(binary, ["--help"], scratch, "help")
  let output = ""
  handle.child.stdout.on("data", (bytes) => {
    if (output.length < 65536) output += bytes.toString()
  })
  handle.child.stdin.end()
  await deadline(handle.closed, "help_exit_deadline")
  check(!handle.entry.spawnError && handle.entry.exitCode === 0 && handle.entry.signal === null, "help_clean_exit")
  check(output.includes("--socket-path") && output.includes("--features") && output.includes("--token"), "actual_cli_help_matches_recipe")
}
function rpc(handle, label) {
  const pending = new Map()
  const seen = new Set()
  const forbidden = new Set()
  let nextId = 1
  let fault = null
  let outputBytes = 0
  const fail = (name) => {
    fault ??= name
    for (const waiter of pending.values()) {
      clearTimeout(waiter.timer)
      waiter.reject(new CheckFailure(label + ":" + name))
    }
    pending.clear()
  }
  handle.child.stdout.on("data", (bytes) => {
    outputBytes += bytes.length
    if (outputBytes > 4 * 1024 * 1024) fail("stdout_budget")
  })
  const lines = createInterface({ input: handle.child.stdout })
  lines.on("line", (line) => {
    let message
    try { message = JSON.parse(line) } catch { fail("stdout_not_json_rpc"); return }
    if (message?.jsonrpc !== "2.0" || !Number.isSafeInteger(message.id)) {
      fail("invalid_response_envelope"); return
    }
    if (seen.has(message.id) || forbidden.has(message.id)) {
      fail("duplicate_or_cancelled_response"); return
    }
    seen.add(message.id)
    const waiter = pending.get(message.id)
    if (!waiter) { fail("uncorrelated_response"); return }
    pending.delete(message.id)
    clearTimeout(waiter.timer)
    waiter.resolve(message)
  })
  handle.child.once("error", () => fail("spawn_error"))
  handle.child.stdin.on("error", () => fail("stdin_error"))
  handle.closed.then(() => { if (pending.size) fail("exit_before_response") })
  function healthy() { check(fault === null, label + ":" + (fault ?? "rpc_healthy")) }
  function write(message) {
    healthy()
    handle.child.stdin.write(JSON.stringify(message) + "\n")
  }
  return {
    healthy,
    seen,
    call(method, params) {
      healthy()
      const id = nextId++
      return new Promise((done, reject) => {
        const timer = setTimeout(() => {
          pending.delete(id)
          fault = "rpc_deadline"
          reject(new CheckFailure(label + ":rpc_deadline"))
        }, timeoutMs)
        pending.set(id, { resolve: done, reject, timer })
        write({ jsonrpc: "2.0", id, method, ...(params === undefined ? {} : { params }) })
      })
    },
    notify(method, params) {
      write({ jsonrpc: "2.0", method, ...(params === undefined ? {} : { params }) })
    },
    parked(tool, input) {
      const id = nextId++
      forbidden.add(id)
      write({ jsonrpc: "2.0", id, method: "tools/call", params: { name: tool, arguments: input } })
      return id
    },
  }
}
async function listener(path, token, label) {
  let planned = null
  let count = 0
  let fault = null
  let closing = null
  const sockets = new Set()
  const server = createServer((socket) => {
    sockets.add(socket)
    socket.on("error", () => { fault ??= "socket_error" })
    socket.once("close", () => sockets.delete(socket))
    let buffer = Buffer.alloc(0)
    let handled = false
    socket.on("data", (chunk) => {
      if (handled) { fault ??= "extra_frame"; socket.destroy(); return }
      buffer = Buffer.concat([buffer, chunk])
      if (buffer.length < 4) return
      const size = buffer.readUInt32LE(0)
      if (size > 1024 * 1024 || buffer.length > size + 4) {
        fault ??= "frame_budget_or_trailing_bytes"; socket.destroy(); return
      }
      if (buffer.length < size + 4) return
      handled = true
      count++
      let request
      try { request = JSON.parse(buffer.subarray(4).toString()) }
      catch { fault ??= "invalid_frame_json"; socket.destroy(); return }
      const current = planned
      planned = null
      if (!current || !isDeepStrictEqual(request, {
        kind: "desk", token, request: { tool: current.tool, input: current.input },
      })) {
        fault ??= "unexpected_forwarded_request"; socket.destroy(); return
      }
      socket.once("close", current.closed.resolve)
      current.arrived.resolve()
      if (current.park) return
      const body = Buffer.from(JSON.stringify({ outcome: current.outcome }))
      const prefix = Buffer.alloc(4)
      prefix.writeUInt32LE(body.length)
      socket.end(Buffer.concat([prefix, body]))
    })
  })
  const fixture = {
    healthy() { check(fault === null, label + ":" + (fault ?? "uds_healthy")) },
    count: () => count,
    plan(tool, input, outcome, park = false) {
      check(planned === null, label + ":one_expected_request")
      const arrived = deferred()
      const closed = deferred()
      planned = { tool, input, outcome, park, arrived, closed }
      return { arrived: arrived.promise, closed: closed.promise }
    },
    close() {
      if (!closing) {
        closing = new Promise((done) => {
          for (const socket of sockets) socket.destroy()
          server.close(() => done())
        })
      }
      return closing
    },
    destroySockets() { for (const socket of sockets) socket.destroy() },
  }
  fixtures.push(fixture)
  await deadline(new Promise((done, reject) => {
    server.once("error", () => reject(new CheckFailure(label + ":listener_bind")))
    server.listen(path, done)
  }), label + ":listener_deadline")
  return fixture
}

const intakeTools = ["hafidh_feedback_list", "hafidh_feedback_get", "hafidh_intake_status"]
const deskTools = [
  "desk_context", "desk_tickets", "desk_thread", "desk_save_reply",
  "desk_propose_reply", "desk_propose_issue", "desk_business_task",
  "desk_business_progress", "desk_business_note", "desk_business_submit",
]
const forbiddenTools = [
  "approve", "deny", "execute", "send_email", "fetch", "private_notes",
  "delegate_to_agent", "task_complete",
]
function outcome(reply, expected, label) {
  check(reply.error === undefined, label + ":no_rpc_error")
  equal(Object.keys(reply.result ?? {}).sort(), ["content", "isError"], label + ":closed_result")
  equal(reply.result.isError, expected.ok !== true, label + ":error_flag")
  equal(reply.result.content?.length, 1, label + ":one_content_block")
  equal(reply.result.content[0].type, "text", label + ":text_content")
  let value
  try { value = JSON.parse(reply.result.content[0].text) }
  catch { throw new CheckFailure(label + ":invalid_outcome_json") }
  equal(value, expected, label + ":exact_public_outcome")
}
function rpcError(reply, code, label) {
  equal(reply.error?.code, code, label + ":error_code")
  check(reply.result === undefined, label + ":no_success_result")
}
async function group(binary, scratch, features, version) {
  const label = features
  const path = join(scratch, features + ".sock")
  const token = randomBytes(24).toString("hex")
  const fixture = await listener(path, token, label)
  const handle = launch(binary, [
    "--features", features, "--parent-connection-id", "fixture-untrusted-label",
    "--socket-path", path, "--token", token, "--parent-pid", String(process.pid),
  ], scratch, label)
  const peer = rpc(handle, label)
  const initialized = await peer.call("initialize")
  equal(initialized.result, {
    protocolVersion: "2024-11-05",
    serverInfo: { name: "codeg-mcp", version },
    capabilities: { tools: {} },
  }, label + ":initialize")
  check(initialized.error === undefined, label + ":initialize_no_error")
  peer.notify("notifications/initialized")
  const discovery = await peer.call("tools/list")
  check(discovery.error === undefined && Array.isArray(discovery.result?.tools), label + ":discovery")
  const expected = features === "intake" ? intakeTools : deskTools
  const tools = discovery.result.tools
  equal(tools.map((tool) => tool.name).sort(), [...expected].sort(), label + ":exact_tools")
  for (const tool of tools) {
    equal(tool.inputSchema?.additionalProperties, false, label + ":" + tool.name + ":closed_schema")
    if (features === "intake") check(tool.description?.toLowerCase().includes("cached"), label + ":" + tool.name + ":cached_description")
  }
  if (features === "desk") {
    const progress = tools.find((tool) => tool.name === "desk_business_progress")
    equal(progress.inputSchema.properties.status.anyOf.map((value) => value.const).sort(),
      ["in_progress", "review", "todo"], "desk:nonterminal_progress_schema")
  }
  const contextTool = features === "intake" ? "hafidh_intake_status" : "desk_business_task"
  for (const name of [...forbiddenTools, ...(features === "intake" ? deskTools : [])]) {
    rpcError(await peer.call("tools/call", { name, arguments: {} }), -32602, label + ":" + name)
  }
  for (const argumentsValue of [null, []]) {
    rpcError(await peer.call("tools/call", { name: contextTool, arguments: argumentsValue }), -32602, label + ":nonobject_input")
  }
  equal(fixture.count(), 0, label + ":refusals_never_forwarded")
  fixture.healthy()
  const publicOutcome = {
    ok: true,
    value: { synthetic: true, title: "Public fixture record", revision: 1 },
  }
  const success = fixture.plan(contextTool, {}, publicOutcome)
  outcome(await peer.call("tools/call", { name: contextTool, arguments: {} }), publicOutcome, label + ":public_read")
  await deadline(success.arrived, label + ":public_read_arrival")
  fixture.healthy()
  const recordTool = features === "intake" ? "hafidh_feedback_get" : "desk_thread"
  const input = features === "intake"
    ? { ulid: "01ARZ3NDEKTSV4RRFFQ69G5FAV" }
    : { inboxId: 1, conversationId: 2 }
  const wrongRecord = features === "intake"
    ? { ulid: "01ARZ3NDEKTSV4RRFFQ69G5FAW" }
    : { inboxId: 1, conversationId: 999 }
  for (const [name, argumentsValue, code] of [
    [recordTool, wrongRecord, "denied"],
    [contextTool, {}, "stale"],
  ]) {
    const expectedOutcome = { ok: false, code }
    const refused = fixture.plan(name, argumentsValue, expectedOutcome)
    outcome(await peer.call("tools/call", { name, arguments: argumentsValue }), expectedOutcome, label + ":" + code + "_relay")
    await deadline(refused.arrived, label + ":" + code + "_arrival")
    fixture.healthy()
  }
  const parked = fixture.plan(recordTool, input, null, true)
  const cancelledId = peer.parked(recordTool, input)
  await deadline(parked.arrived, label + ":parked_arrival")
  peer.notify("notifications/cancelled", { requestId: cancelledId })
  await deadline(parked.closed, label + ":cancel_closes_socket")
  check((await peer.call("tools/list")).error === undefined, label + ":responsive_after_cancel")
  check(!peer.seen.has(cancelledId), label + ":cancelled_result_suppressed")
  await deadline(fixture.close(), label + ":listener_close")
  rpcError(await peer.call("tools/call", { name: contextTool, arguments: {} }), -32603, label + ":missing_listener")
  fixture.healthy()
  equal(fixture.count(), 4, label + ":exact_forwarded_reads")
  handle.child.stdin.end()
  await deadline(handle.closed, label + ":stdio_eof_exit")
  check(handle.entry.exitCode === 0 && handle.entry.signal === null && !handle.entry.spawnError, label + ":clean_exit")
  peer.healthy()
  report.groups.push({
    features, tools: [...expected], forwardedReads: fixture.count(),
    syntheticPublicRead: publicOutcome, refusalCodes: ["denied", "stale"],
    cancelledReadSuppressed: true, missingListenerError: -32603,
  })
}
async function cleanup() {
  for (const fixture of fixtures) {
    try { await deadline(fixture.close(), "cleanup_listener_deadline", 2000) }
    catch (error) { failure(error) }
  }
  for (const handle of children) {
    if (handle.child.exitCode === null && handle.child.signalCode === null) {
      handle.entry.forcedTermination = true
      handle.child.kill("SIGTERM")
    }
    try { await deadline(handle.closed, "cleanup_child_deadline", 2000) }
    catch {
      handle.child.kill("SIGKILL")
      try { await deadline(handle.closed, "cleanup_kill_deadline", 2000) }
      catch (error) { failure(error) }
    }
  }
}
for (const signal of ["SIGINT", "SIGTERM"]) {
  process.once(signal, () => {
    interrupted = true
    for (const fixture of fixtures) fixture.destroySockets()
    for (const handle of children) {
      if (handle.child.exitCode === null && handle.child.signalCode === null) {
        handle.entry.forcedTermination = true
        handle.child.kill("SIGTERM")
      }
    }
  })
}

let scratch
let supplied
let ownedOutput = false
try {
  equal(await realpath(worktree), expectedWorktree, "owned_worktree_only")
  ownedOutput = true
  supplied = options()
  report.handoff = {
    source: supplied["--source"], expectedSha256: supplied["--sha256"],
    packageVersion: supplied["--package-version"], stableWindow: supplied["--stable-window"],
  }
  report.before = await identity(supplied["--binary"])
  equal(report.before.sha256, supplied["--sha256"], "artifact_hash_before")
  scratch = await mkdtemp(join(worktree, ".bpc-"))
  await chmod(scratch, 0o700)
  report.scratch = scratch
  await help(report.before.path, scratch)
  for (const features of ["intake", "desk"]) {
    await group(report.before.path, scratch, features, supplied["--package-version"])
  }
} catch (error) {
  failure(error)
} finally {
  await cleanup()
  if (report.before) {
    try {
      report.after = await identity(supplied["--binary"])
      equal(report.after, report.before, "artifact_identity_unchanged_after")
    } catch (error) { failure(error) }
  }
  if (scratch) {
    try {
      check(scratch.startsWith(join(worktree, ".bpc-")), "owned_cleanup_path")
      await rm(scratch, { recursive: true })
      const remains = await stat(scratch).then(() => true, (error) => {
        if (error.code === "ENOENT") return false
        throw error
      })
      check(!remains, "owned_sockets_and_directory_removed")
      report.scratchRemoved = true
    } catch (error) { failure(error) }
  }
  if (interrupted) report.failures.push("interrupted")
  if (report.children.some((entry) => entry.forcedTermination)) {
    report.failures.push("forced_child_cleanup_not_clean_acceptance")
  }
  report.status = report.failures.length ? "failed" : "passed"
  const output = join(evidenceDirectory, "result-" + Date.now() + "-" + process.pid + ".json")
  try {
    if (!ownedOutput) throw new CheckFailure("refuse_output_outside_owned_worktree")
    await writeFile(output, JSON.stringify(report, null, 2) + "\n", { flag: "wx", mode: 0o600 })
    console.log(JSON.stringify({ status: report.status, report: output, checks: report.checks.length }))
  } catch {
    console.error("Could not persist sanitized companion evidence")
    report.status = "failed"
  }
  process.exitCode = report.status === "passed" ? 0 : 1
}
