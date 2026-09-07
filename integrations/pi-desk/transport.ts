// Codeg v0.30.4 delegation/transport.rs length-prefixed one-shot IPC pattern.
import { createConnection } from "node:net"
import { isAbsolute } from "node:path"
import { parseResponse, type DeskCall, type DeskResponse } from "./protocol.ts"

export const MAX_FRAME_BYTES = 1024 * 1024
export interface DeskTransport {
  call(request: DeskCall, signal?: AbortSignal): Promise<DeskResponse>
}

/** Credentials are captured, never returned by tools or included in errors. */
export function socketTransport(socketPath: string, token: string, timeoutMs = 10000): DeskTransport {
  if ((!isAbsolute(socketPath) && !socketPath.startsWith("\\\\.\\pipe\\")) || !token || token.length > 256) {
    throw new Error("Desk bridge is unavailable")
  }
  return {
    call(request, signal) {
      if (signal?.aborted) return Promise.reject(new Error("Desk request aborted"))
      const body = Buffer.from(JSON.stringify({ kind: "desk", token, request }))
      if (body.length > MAX_FRAME_BYTES) return Promise.reject(new Error("Desk request is too large"))
      const prefix = Buffer.alloc(4)
      prefix.writeUInt32LE(body.length)
      return new Promise((resolve, reject) => {
        let done = false
        let received = Buffer.alloc(0)
        const socket = createConnection(socketPath)
        const finish = (error?: string, response?: DeskResponse) => {
          if (done) return
          done = true
          clearTimeout(timer)
          signal?.removeEventListener("abort", abort)
          socket.destroy()
          if (error) reject(new Error(error))
          else resolve(response!)
        }
        const abort = () => finish("Desk request aborted")
        // Absolute deadline, rather than an idle timer a trickling peer can reset.
        const timer = setTimeout(() => finish("Desk request timed out"), timeoutMs)
        signal?.addEventListener("abort", abort, { once: true })
        if (signal?.aborted) { abort(); return }
        socket.on("error", () => finish("Desk bridge is unavailable"))
        socket.on("close", () => finish("Desk connection closed"))
        socket.on("connect", () => socket.write(Buffer.concat([prefix, body])))
        socket.on("data", (chunk: Buffer) => {
          if (done) return
          if (received.length + chunk.length > MAX_FRAME_BYTES + 4) { finish("Invalid Desk response"); return }
          received = Buffer.concat([received, chunk])
          if (received.length < 4) return
          const length = received.readUInt32LE(0)
          if (length > MAX_FRAME_BYTES) { finish("Invalid Desk response"); return }
          if (received.length < length + 4) return
          if (received.length !== length + 4) { finish("Invalid Desk response"); return }
          try { finish(undefined, parseResponse(JSON.parse(received.subarray(4).toString("utf8")))) }
          catch { finish("Invalid Desk response") }
        })
      })
    },
  }
}
