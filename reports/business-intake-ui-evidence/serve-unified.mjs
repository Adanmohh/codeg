// Synthetic B browser fixture only; Node24.19.0 / installed @types/node25.2.2.
// Reuses reports/business-intake-ui-evidence/serve.mjs at Codeg
// 1974d97f44b69db96051ace1c68717cc8c27b6e9 (Apache-2.0); see NOTICE.
// Dedicated origin4354 preserves the old4350 export and its private sessions.
import { createServer, request } from "node:http"
import { readFile } from "node:fs/promises"
import { resolve, extname, sep } from "node:path"

const root = resolve(process.argv[2])
const backend = 4351
const workspaceBackend = backend
const port = 4354
if (
  process.argv[3] !== "--backend=4351" ||
  process.argv[4] !== "--synthetic-intake-fixture" ||
  process.argv.length !== 5
)
  throw Error("Requires the owned guarded synthetic B fixture on loopback4351")
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
  "settings/get",
  "settings/update",
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
  ].map((op) => `tasks/${op}`),
  ...[
    "bindings/list",
    "bindings/status",
    "bindings/create",
    "bindings/update",
    "bindings/disable",
    "grants/list",
    "grants/upsert",
    "grants/revoke",
    "sources/list",
    "sources/get",
    "imports/start",
    "imports/capture",
    "imports/list",
    "imports/get",
    "imports/advance",
    "imports/cancel",
    "candidates/list",
    "candidates/get",
    "candidates/create",
    "candidates/select",
    "candidates/edit",
    "candidates/accept",
    "candidates/link",
    "candidates/discard",
    "tasks/sources",
  ].map((op) => `intake/${op}`),
])
const platformOperations = new Set([
  "context",
  "tenants/list",
  "tenants/create",
  "tenants/status",
  "tenants/reissue-owner-credential",
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
  const url = new URL(req.url, `http://127.0.0.1:${port}`)
  const path = url.pathname
  if (path === "/__business_intake_fixture" && req.method === "GET") {
    finish(
      res,
      200,
      JSON.stringify({
        synthetic: true,
        backendPort: backend,
        workspaceBackendPort: workspaceBackend,
        export: process.argv[2],
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
      ((path.startsWith("/api/business/") && operations.has(path.slice(14))) ||
        (path.startsWith("/api/platform/business/") &&
          platformOperations.has(path.slice(23))))
    const target = path.startsWith("/api/business/intake/")
      ? backend
      : workspaceBackend
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
        port: target,
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
    const staticPath =
      path.endsWith("/") && path !== "/" ? path.slice(0, -1) : path
    let file = resolve(root, "." + decodeURIComponent(staticPath))
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
server.listen(port, "127.0.0.1", () =>
  console.log(
    JSON.stringify({
      synthetic: true,
      port,
      pid: process.pid,
      backendPort: backend,
      workspaceBackendPort: workspaceBackend,
      export: process.argv[2],
      credentialsLogged: false,
    })
  )
)
process.on("SIGTERM", () => {
  for (const socket of sockets) socket.destroy()
  server.close(() => process.exit(0))
})
