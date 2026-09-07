import { afterEach, expect, it } from "vitest"
import { spawn, type ChildProcessWithoutNullStreams } from "node:child_process"
import { createInterface } from "node:readline"
import { mkdir, rm } from "node:fs/promises"
import { resolve } from "node:path"
import { createRequire } from "node:module"
import { createServer, type Server, type Socket } from "node:net"
import { DESK_TOOLS } from "./protocol.ts"

const require = createRequire(import.meta.url)
const scratch = resolve("integrations/pi-desk/process-fixtures.log")
const children: ChildProcessWithoutNullStreams[] = []
const servers: Server[] = []
const sockets: Socket[] = []
afterEach(async () => {
  await Promise.all(
    children.splice(0).map(
      (child) =>
        new Promise<void>((done) => {
          if (child.exitCode !== null || child.signalCode !== null) {
            done()
            return
          }
          child.once("exit", () => done())
          child.kill("SIGTERM")
        })
    )
  )
  sockets.splice(0).forEach((socket) => socket.destroy())
  await Promise.all(
    servers
      .splice(0)
      .map((server) => new Promise<void>((done) => server.close(() => done())))
  )
  await rm(scratch, { recursive: true, force: true })
})

it("real codeg-mcp exposes only Desk tools and cancels a parked native request", async () => {
  const path = resolve(`.pi-mcp-${process.pid}.log`)
  await rm(path, { force: true })
  const requests: Array<Record<string, unknown>> = []
  let parked!: () => void
  let closed!: () => void
  const parkedRequest = new Promise<void>((done) => {
    parked = done
  })
  const closedRequest = new Promise<void>((done) => {
    closed = done
  })
  const server = createServer((socket) => {
    sockets.push(socket)
    let bytes = Buffer.alloc(0)
    socket.on("data", (chunk) => {
      bytes = Buffer.concat([bytes, Buffer.from(chunk)])
      if (bytes.length < 4 || bytes.length < bytes.readUInt32LE(0) + 4) return
      const request = JSON.parse(bytes.subarray(4).toString())
      requests.push(request)
      if (request.request.tool === "desk_thread") {
        socket.once("close", closed)
        parked()
        return
      }
      const body = Buffer.from(
        JSON.stringify({ outcome: { ok: false, code: "stale" } })
      )
      const prefix = Buffer.alloc(4)
      prefix.writeUInt32LE(body.length)
      socket.end(Buffer.concat([prefix, body]))
    })
  })
  servers.push(server)
  await new Promise<void>((done, reject) => {
    server.once("error", reject)
    server.listen(path, done)
  })
  const child = spawn(
    resolve("src-tauri/target/debug/codeg-mcp"),
    [
      "--features",
      "desk",
      "--parent-connection-id",
      "untrusted-command-label",
      "--socket-path",
      path,
      "--token",
      "fixture-launch-token",
    ],
    { stdio: "pipe", env: { NODE_ENV: "test", PATH: process.env.PATH } }
  )
  children.push(child)
  const replies = new Map<number, Record<string, unknown>>()
  const waiters = new Map<number, (reply: Record<string, unknown>) => void>()
  createInterface({ input: child.stdout }).on("line", (line) => {
    const reply = JSON.parse(line)
    replies.set(reply.id, reply)
    waiters.get(reply.id)?.(reply)
  })
  function send(
    id: number,
    method: string,
    params?: unknown
  ): Promise<Record<string, unknown>> {
    return new Promise((done, reject) => {
      const timer = setTimeout(
        () => reject(new Error("companion RPC timed out")),
        5000
      )
      waiters.set(id, (reply) => {
        clearTimeout(timer)
        done(reply)
      })
      child.stdin.write(
        JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n"
      )
    })
  }
  expect((await send(1, "initialize")).error).toBeUndefined()
  const list = (await send(2, "tools/list")).result as {
    tools: Array<{ name: string }>
  }
  expect(list.tools.map((tool) => tool.name).sort()).toEqual(
    [...DESK_TOOLS].sort()
  )
  expect(
    (await send(3, "tools/call", { name: "approve", arguments: {} })).error
  ).toBeDefined()
  const result = (
    await send(4, "tools/call", { name: "desk_context", arguments: {} })
  ).result as { isError: boolean; content: Array<{ text: string }> }
  expect(result.isError).toBe(true)
  expect(JSON.parse(result.content[0].text)).toEqual({
    ok: false,
    code: "stale",
  })
  expect(requests[0]).toEqual({
    kind: "desk",
    token: "fixture-launch-token",
    request: { tool: "desk_context", input: {} },
  })
  child.stdin.write(
    JSON.stringify({
      jsonrpc: "2.0",
      id: 20,
      method: "tools/call",
      params: {
        name: "desk_thread",
        arguments: { inboxId: 1, conversationId: 2 },
      },
    }) + "\n"
  )
  await parkedRequest
  child.stdin.write(
    JSON.stringify({
      jsonrpc: "2.0",
      method: "notifications/cancelled",
      params: { requestId: 20 },
    }) + "\n"
  )
  await closedRequest
  await send(21, "tools/list")
  expect(replies.has(20)).toBe(false)
})

it("real pi 0.85.1 RPC discovers Desk and the isolated installed adapter without a model call", async () => {
  await mkdir(scratch, { recursive: true })
  const child = spawn(
    process.env.PI_DESK_PI_BIN ?? "pi",
    [
      "--offline",
      "--mode",
      "rpc",
      "--no-session",
      "--no-extensions",
      "--no-skills",
      "--no-prompt-templates",
      "--no-themes",
      "--no-context-files",
      "--no-approve",
      "-e",
      resolve("integrations/pi-desk/index.ts"),
    ],
    {
      cwd: scratch,
      stdio: "pipe",
      env: {
        NODE_ENV: "test",
        PATH: process.env.PATH,
        PI_CODING_AGENT_DIR: resolve(scratch, "agent"),
        PI_OFFLINE: "1",
        CODEG_DESK_ADAPTER: require.resolve("pi-mcp-adapter"),
        // Deliberately no provider credentials, bridge token or MCP server.
      },
    }
  )
  children.push(child)
  const output = createInterface({ input: child.stdout })
  const response = new Promise<Record<string, unknown>>((done, reject) => {
    const timer = setTimeout(
      () => reject(new Error("pi discovery timed out")),
      12000
    )
    child.once("error", (error) => {
      clearTimeout(timer)
      reject(error)
    })
    child.once("exit", () => {
      clearTimeout(timer)
      reject(new Error("pi exited before discovery"))
    })
    output.on("line", (line) => {
      let data: Record<string, unknown>
      try {
        data = JSON.parse(line)
      } catch {
        return
      }
      if (data.type === "response" && data.id === "discovery") {
        clearTimeout(timer)
        done(data)
      }
      expect(["message_start", "tool_execution_start"]).not.toContain(data.type)
    })
  })
  let stderr = ""
  child.stderr.on("data", (data) => {
    stderr += data.toString()
  })
  child.stdin.write(
    JSON.stringify({ id: "discovery", type: "get_commands" }) + "\n"
  )
  const result = await response
  expect(result.success, stderr).toBe(true)
  const commands = (
    result.data as { commands: Array<{ name: string; source: string }> }
  ).commands
  expect(
    commands.some((c) => c.name === "desk-status" && c.source === "extension"),
    stderr
  ).toBe(true)
  expect(
    commands.some((c) => c.name === "mcp" && c.source === "extension"),
    stderr
  ).toBe(true)
})
