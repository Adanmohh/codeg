// Synthetic browser fixture only; Node24.19.0 / @types/node25.2.2.
// Adapts the Apache Codeg report server at accepted 4ec04d7282a50529335d724438d42b99a53385a2:
// reports/design-reduced-motion-evidence/server.mjs. No production server changes.
import { createServer, request } from "node:http"
import { readFile } from "node:fs/promises"
import { resolve, extname, sep } from "node:path"

const root = resolve(process.argv[2])
const backend = process.argv[3] === "--backend=4342" ? 4342 : null
if (process.argv[3] && !backend)
  throw Error("Only guarded loopback4342 is allowed")
const operations = new Set([
  "context",
  "bootstrap",
  "members/list",
  "members/create",
  "members/update",
  "members/revoke",
  "credentials/issue",
  "credentials/list",
  "credentials/revoke",
  ...[
    "list",
    "get",
    "create",
    "update",
    "assign",
    "progress",
    "note",
    "submit",
    "review",
    "cancel",
    "archive",
    "link-execution",
    "entrust-execution",
  ].map((op) => `tasks/${op}`),
])
const calls = []
const sockets = new Set()
const mime = {
  ".html": "text/html",
  ".js": "text/javascript",
  ".css": "text/css",
  ".json": "application/json",
  ".txt": "text/plain",
  ".svg": "image/svg+xml",
  ".png": "image/png",
  ".woff2": "font/woff2",
  ".ico": "image/x-icon",
}
function finish(res, status, body, contentType = "text/plain") {
  res.writeHead(status, {
    "content-type": contentType,
    "cache-control": "no-store",
  })
  res.end(body)
}
const server = createServer(async (req, res) => {
  const url = new URL(req.url, "http://127.0.0.1:4340")
  const path = url.pathname
  if (path === "/__business_fixture" && req.method === "GET") {
    finish(
      res,
      200,
      JSON.stringify({
        synthetic: true,
        backendPort: backend,
        export: "out-business-workspace",
        calls,
      }),
      "application/json"
    )
    return
  }
  if (path.startsWith("/api/") || path.startsWith("/ws")) {
    const allowed =
      req.method === "POST" &&
      !url.search &&
      path.startsWith("/api/business/") &&
      operations.has(path.slice(14))
    const entry = { method: req.method, path, status: null }
    calls.push(entry)
    if (!allowed || !backend) {
      entry.status = allowed ? 503 : 403
      finish(
        res,
        entry.status,
        allowed
          ? "Synthetic fixture backend is not connected"
          : "Synthetic fixture guard"
      )
      return
    }
    const upstream = request(
      {
        hostname: "127.0.0.1",
        port: backend,
        path,
        method: "POST",
        headers: {
          "content-type": "application/json",
          authorization: req.headers.authorization || "",
        },
      },
      (response) => {
        entry.status = response.statusCode
        res.writeHead(response.statusCode, {
          "content-type":
            response.headers["content-type"] || "application/json",
          "cache-control": "no-store",
        })
        response.pipe(res)
      }
    )
    upstream.setTimeout(19000, () => upstream.destroy())
    upstream.on("error", () => {
      entry.status = 502
      if (!res.headersSent)
        finish(res, 502, "Synthetic fixture backend unavailable")
      else res.destroy()
    })
    req.pipe(upstream)
    return
  }
  if (req.method !== "GET" && req.method !== "HEAD") {
    finish(res, 405, "Read-only export")
    return
  }
  try {
    let file = resolve(root, "." + decodeURIComponent(path))
    if (file !== root && !file.startsWith(root + sep)) throw Error("path")
    if (file === root) file += "/index.html"
    else if (!extname(file)) file += ".html"
    const body = await readFile(file)
    finish(
      res,
      200,
      req.method === "HEAD" ? "" : body,
      mime[extname(file)] || "application/octet-stream"
    )
  } catch {
    finish(res, 404, "Fixture static file missing")
  }
})
server.on("connection", (socket) => {
  sockets.add(socket)
  socket.on("close", () => sockets.delete(socket))
})
server.on("upgrade", (_req, socket) => socket.destroy())
server.listen(4340, "127.0.0.1", () =>
  console.log(
    JSON.stringify({
      synthetic: true,
      port: 4340,
      pid: process.pid,
      backendPort: backend,
      export: "out-business-workspace",
      credentialsLogged: false,
    })
  )
)
process.on("SIGTERM", () => {
  for (const socket of sockets) socket.destroy()
  server.close(() => process.exit(0))
})
