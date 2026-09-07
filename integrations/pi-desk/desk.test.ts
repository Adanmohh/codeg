import { afterEach, describe, expect, it } from "vitest"
import { createServer, type Server, type Socket } from "node:net"
import { mkdir, rm } from "node:fs/promises"
import { resolve } from "node:path"
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent"
import {
  claimApproval,
  type ApprovalDecision,
  type ApprovalRequest,
} from "./broker.ts"
import { installDesk } from "./index.ts"
import { snapshotCall, type DeskCall, type DeskResponse } from "./protocol.ts"
import {
  MAX_FRAME_BYTES,
  socketTransport,
  type DeskTransport,
} from "./transport.ts"
import { rpcRefusal } from "./launch.mjs"

const context: DeskResponse = {
  ok: true,
  value: { taskId: 1, runSeq: 2, accountId: 3, inboxes: [] },
}
const ready: DeskTransport = {
  async call() {
    return context
  },
  async health() {
    return true
  },
}

describe("headless launch and lifecycle", () => {
  function hooks(transport?: DeskTransport) {
    const handlers = new Map<string, (...args: unknown[]) => unknown>()
    const pi = {
      events: { on() {} },
      on(name: string, handler: (...args: unknown[]) => unknown) {
        handlers.set(name, handler)
      },
      registerCommand() {},
      registerTool() {},
    }
    installDesk(pi as unknown as ExtensionAPI, transport)
    return handlers
  }
  const ctx = {
    model: { id: "gpt-6-astra" },
    thinkingLevel: "max",
    hasUI: false,
    ui: { notify() {} },
  }

  it("blocks headless prompt and compaction for missing/failed bridge or cheaper settings", async () => {
    for (const transport of [
      undefined,
      {
        ...ready,
        async health() {
          return false
        },
      },
    ]) {
      const handler = hooks(transport)
      expect(await handler.get("input")!({}, ctx)).toEqual({
        action: "handled",
      })
      expect(await handler.get("session_before_compact")!({}, ctx)).toEqual({
        cancel: true,
      })
    }
    const handler = hooks(ready)
    expect(
      await handler.get("input")!({}, { ...ctx, thinkingLevel: "high" })
    ).toEqual({ action: "handled" })
    expect(
      await handler.get("input")!({}, { ...ctx, model: { id: "cheap" } })
    ).toEqual({ action: "handled" })
    expect(await handler.get("input")!({}, ctx)).toEqual({ action: "continue" })
  })

  it("aborts a pending prompt preflight when the session generation changes", async () => {
    let release!: (value: boolean) => void
    const handler = hooks({
      ...ready,
      health: () =>
        new Promise((resolve) => {
          release = resolve
        }),
    })
    const pending = handler.get("input")!({}, ctx)
    handler.get("session_start")!()
    release(true)
    expect(await pending).toEqual({ action: "handled" })
  })

  it("refuses RPC model/reasoning cycling and session replacement while allowing abort", () => {
    for (const type of [
      "cycle_model",
      "cycle_thinking_level",
      "new_session",
      "switch_session",
      "fork",
      "clone",
    ]) {
      expect(rpcRefusal({ type }, "configured")).toBeTypeOf("string")
    }
    expect(
      rpcRefusal(
        { type: "set_model", provider: "unconfigured", modelId: "gpt-6-astra" },
        "configured"
      )
    ).toBeTypeOf("string")
    expect(
      rpcRefusal(
        { type: "set_model", provider: "configured", modelId: "gpt-6-astra" },
        "configured"
      )
    ).toBeUndefined()
    expect(
      rpcRefusal({ type: "set_thinking_level", level: "max" }, "configured")
    ).toBeUndefined()
    expect(
      rpcRefusal({ type: "set_thinking_level", level: "high" }, "configured")
    ).toBeTypeOf("string")
    expect(rpcRefusal({ type: "abort" }, "configured")).toBeUndefined()
    expect(rpcRefusal(null, "configured")).toBeTypeOf("string")
  })
})

function request(overrides: Partial<ApprovalRequest> = {}) {
  let handler: (() => ApprovalDecision | Promise<ApprovalDecision>) | undefined
  const req: ApprovalRequest = {
    requestId: "request-1",
    serverName: "hafidh",
    originalToolName: "hafidh_feedback_get",
    prefixedToolName: "hafidh_hafidh_feedback_get",
    args: { source_ref: "fixture" },
    origin: "proxy",
    claim(candidate) {
      if (handler) return false
      handler = candidate
      return true
    },
    ...overrides,
  }
  return {
    req,
    decide: () => {
      expect(handler).toBeTypeOf("function")
      return handler!()
    },
  }
}

describe("adapter broker contract (pi-mcp-adapter 2.32.1 regression patterns)", () => {
  it("claims synchronously, allows only once, and checks the live bridge on every call", async () => {
    let calls = 0
    for (let i = 0; i < 2; i++) {
      const fixture = request()
      claimApproval(
        fixture.req,
        {
          async call() {
            calls++
            return context
          },
        },
        new AbortController().signal
      )
      expect(fixture.req.claim(() => "allow_for_session")).toBe(false)
      expect(await fixture.decide()).toBe("allow_once")
      expect(Object.isFrozen(fixture.req.args)).toBe(true)
    }
    expect(calls).toBe(2)
  })
  it("denies mutations, lookalike servers, arbitrary resources and scripts without transport IO", async () => {
    for (const overrides of [
      { originalToolName: "send_email" },
      { originalToolName: "approve" },
      { serverName: "github" },
      { serverName: "hafidh-attacker" },
      { origin: "resource" as const },
      { origin: "script" as const },
    ]) {
      const fixture = request(overrides)
      claimApproval(
        fixture.req,
        {
          call() {
            throw new Error("must not call")
          },
        },
        new AbortController().signal
      )
      expect(await fixture.decide()).toBe("deny")
    }
  })
  it("denies headlessly when bridge is absent, revoked, stale, malformed or unavailable", async () => {
    for (const transport of [
      undefined,
      {
        async call() {
          return { ok: false, code: "denied" } as DeskResponse
        },
      },
      {
        async call() {
          return { ok: false, code: "stale" } as DeskResponse
        },
      },
      {
        async call() {
          return { ok: true, value: {} } as DeskResponse
        },
      },
      {
        async call(): Promise<DeskResponse> {
          throw new Error("no connection")
        },
      },
    ]) {
      const fixture = request()
      claimApproval(fixture.req, transport, new AbortController().signal)
      expect(await fixture.decide()).toBe("deny")
    }
  })
  it("never lets edited args or a late response after abort authorize the original call", async () => {
    for (const abort of [false, true]) {
      let release!: (value: DeskResponse) => void
      const controller = new AbortController()
      const fixture = request({ signal: controller.signal })
      claimApproval(
        fixture.req,
        {
          call: () =>
            new Promise((r) => {
              release = r
            }),
        },
        new AbortController().signal
      )
      const decision = fixture.decide()
      if (abort) controller.abort()
      else fixture.req.args.source_ref = "changed"
      release(context)
      expect(await decision).toBe("deny")
    }
  })
  it("denies a prior session generation even if its bridge response arrives late", async () => {
    const lifecycle = new AbortController()
    const fixture = request()
    claimApproval(fixture.req, ready, lifecycle.signal)
    lifecycle.abort()
    expect(await fixture.decide()).toBe("deny")
  })
})

const scratch = resolve("integrations/pi-desk/fixtures.log")
const servers: Server[] = []
const sockets: Socket[] = []
afterEach(async () => {
  sockets.splice(0).forEach((s) => s.destroy())
  await Promise.all(
    servers.splice(0).map((s) => new Promise<void>((r) => s.close(() => r())))
  )
  await rm(scratch, { recursive: true, force: true })
})
async function serve(fn: (socket: Socket) => void): Promise<string> {
  await mkdir(scratch, { recursive: true })
  // macOS UDS paths are limited to 104 bytes: use a short repo-local symlink-free path.
  const path = resolve(`.pi-desk-${process.pid}-${servers.length}.log`)
  await rm(path, { force: true })
  const server = createServer((socket) => {
    sockets.push(socket)
    fn(socket)
  })
  servers.push(server)
  await new Promise<void>((r, j) => {
    server.once("error", j)
    server.listen(path, r)
  })
  return path
}
function frame(outcome: unknown): Buffer {
  const body = Buffer.from(JSON.stringify({ outcome }))
  const prefix = Buffer.alloc(4)
  prefix.writeUInt32LE(body.length)
  return Buffer.concat([prefix, body])
}

describe("typed local IPC", () => {
  it("uses existing Ping for availability without disclosing a token or granting a task scope", async () => {
    const seen: unknown[] = []
    const path = await serve((socket) =>
      socket.once("data", (data) => {
        const request = JSON.parse(Buffer.from(data).subarray(4).toString())
        seen.push(request)
        socket.end(
          frame(
            request.kind === "ping"
              ? { ok: true }
              : { ok: false, code: "stale" }
          )
        )
      })
    )
    const transport = socketTransport(path, "fixture-token")
    expect(await transport.health!()).toBe(true)
    expect(await transport.call(snapshotCall("desk_context", {}))).toEqual({
      ok: false,
      code: "stale",
    })
    expect(seen[0]).toEqual({ kind: "ping" })
  })
  it("normalizes fragmented framing and authenticates without caller identity", async () => {
    let seen: unknown
    const path = await serve((socket) =>
      socket.once("data", (data) => {
        seen = JSON.parse(Buffer.from(data).subarray(4).toString())
        const reply = frame(context)
        socket.write(reply.subarray(0, 2))
        setTimeout(() => socket.write(reply.subarray(2)), 5)
      })
    )
    expect(
      await socketTransport(path, "fixture-token").call(
        snapshotCall("desk_context", {})
      )
    ).toEqual(context)
    expect(seen).toEqual({
      kind: "desk",
      token: "fixture-token",
      request: { tool: "desk_context", input: {} },
    })
  })
  it("rejects oversized, malformed and truncated frames without leaking peer data", async () => {
    for (const bytes of [
      Buffer.from([255, 255, 255, 255]),
      Buffer.from([1, 0, 0, 0, 33]),
      Buffer.from([3, 0, 0, 0, 123]),
    ]) {
      const path = await serve((socket) =>
        socket.once("data", () => socket.end(bytes))
      )
      await expect(
        socketTransport(path, "fixture-secret").call(
          snapshotCall("desk_context", {})
        )
      ).rejects.toThrow(/Invalid Desk response|Desk connection closed/)
    }
    expect(MAX_FRAME_BYTES).toBeLessThanOrEqual(16 * 1024 * 1024)
  })
  it("times out or aborts a parked request and closes its socket", async () => {
    const path = await serve(() => {})
    await expect(
      socketTransport(path, "fixture", 20).call(
        snapshotCall("desk_context", {})
      )
    ).rejects.toThrow("timed out")
    const controller = new AbortController()
    const result = socketTransport(path, "fixture").call(
      snapshotCall("desk_context", {}),
      controller.signal
    )
    controller.abort()
    await expect(result).rejects.toThrow("aborted")
  })
})

describe("Pi mutable hook boundary", () => {
  it("rejects caller identity, header injection and unexpected draft fields", () => {
    expect(() => snapshotCall("desk_context", { accountId: 4 })).toThrow()
    expect(() =>
      snapshotCall("desk_propose_reply", {
        draftId: 1,
        expectedRevision: 2,
        actor: "operator",
      })
    ).toThrow()
    const input = {
      inboxId: 1,
      conversationId: 1,
      expectedRevision: 0,
      reply: {
        inboxId: 1,
        conversationId: 1,
        from: "desk@fixture.test",
        to: ["a@fixture.test\r\nBcc: b@fixture.test"],
        cc: [],
        bcc: [],
        subject: "Reply",
        text: "Draft",
        inReplyTo: null,
        references: [],
      },
    }
    expect(() => snapshotCall("desk_save_reply", input)).toThrow()
  })
  it("revalidates after a later tool_call handler edits input, and detaches accepted payload", async () => {
    const tools = new Map<
      string,
      { execute: (...args: unknown[]) => Promise<{ isError?: boolean }> }
    >()
    const pi = {
      events: { on() {} },
      on() {},
      registerCommand() {},
      registerTool(tool: { name: string }) {
        tools.set(tool.name, tool as never)
      },
    }
    const calls: DeskCall[] = []
    installDesk(pi as unknown as ExtensionAPI, {
      async call(call) {
        calls.push(call)
        return context
      },
    })
    const execute = tools.get("desk_propose_reply")!.execute
    const changed = { draftId: 1, expectedRevision: 2, actor: "operator" }
    expect((await execute("a", changed, undefined)).isError).toBe(true)
    expect(calls).toHaveLength(0)
    const input = { draftId: 1, expectedRevision: 2 }
    const pending = execute("b", input, undefined)
    input.expectedRevision = 99
    await pending
    expect(calls[0]).toEqual({
      tool: "desk_propose_reply",
      input: { draftId: 1, expectedRevision: 2 },
    })
  })
})
