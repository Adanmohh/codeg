import { afterEach, expect, it } from "vitest"
import { spawn, type ChildProcessWithoutNullStreams } from "node:child_process"
import { createInterface } from "node:readline"
import { mkdir, rm } from "node:fs/promises"
import { resolve } from "node:path"
import { createRequire } from "node:module"

const require = createRequire(import.meta.url)
const scratch = resolve("integrations/pi-desk/process-fixtures.log")
const children: ChildProcessWithoutNullStreams[] = []
afterEach(async () => {
  await Promise.all(children.splice(0).map(child => new Promise<void>(done => {
    if (child.exitCode !== null || child.signalCode !== null) { done(); return }
    child.once("exit", () => done()); child.kill("SIGTERM")
  })))
  await rm(scratch, { recursive: true, force: true })
})

it("real pi 0.85.1 RPC discovers Desk and the isolated installed adapter without a model call", async () => {
  await mkdir(scratch, { recursive: true })
  const child = spawn(process.env.PI_DESK_PI_BIN ?? "pi", [
    "--offline", "--mode", "rpc", "--no-session", "--no-extensions",
    "--no-skills", "--no-prompt-templates", "--no-themes", "--no-context-files", "--no-approve",
    "-e", resolve("integrations/pi-desk/index.ts"),
  ], {
    cwd: scratch, stdio: "pipe",
    env: { NODE_ENV: "test", PATH: process.env.PATH, PI_CODING_AGENT_DIR: resolve(scratch, "agent"),
      PI_OFFLINE: "1", CODEG_DESK_ADAPTER: require.resolve("pi-mcp-adapter"),
      // Deliberately no provider credentials, bridge token or MCP server.
    },
  })
  children.push(child)
  const output = createInterface({ input: child.stdout })
  const response = new Promise<Record<string, unknown>>((done, reject) => {
    const timer = setTimeout(() => reject(new Error("pi discovery timed out")), 12000)
    child.once("error", error => { clearTimeout(timer); reject(error) })
    child.once("exit", () => { clearTimeout(timer); reject(new Error("pi exited before discovery")) })
    output.on("line", line => {
      let data: Record<string, unknown>
      try { data = JSON.parse(line) } catch { return }
      if (data.type === "response" && data.id === "discovery") { clearTimeout(timer); done(data) }
      expect(["message_start", "tool_execution_start"]).not.toContain(data.type)
    })
  })
  let stderr = ""
  child.stderr.on("data", data => { stderr += data.toString() })
  child.stdin.write(JSON.stringify({ id: "discovery", type: "get_commands" }) + "\n")
  const result = await response
  expect(result.success, stderr).toBe(true)
  const commands = (result.data as { commands: Array<{ name: string; source: string }> }).commands
  expect(commands.some(c => c.name === "desk-status" && c.source === "extension"), stderr).toBe(true)
  expect(commands.some(c => c.name === "mcp" && c.source === "extension"), stderr).toBe(true)
})
