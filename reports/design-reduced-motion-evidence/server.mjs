// Local report fixture only. Node 24.19.0 / @types/node 25.2.2 HTTP/FS APIs.
// Serves an isolated production export and proxies the accepted no-engine
// pi_desk_browser_fixture on 4324. No credentials are read or logged.
import { createServer, request } from "node:http"
import { createConnection } from "node:net"
import { readFile } from "node:fs/promises"
import { resolve, extname, sep } from "node:path"

const root = resolve(process.argv[2])
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
const server = createServer(async (req, res) => {
  const path = new URL(req.url, "http://127.0.0.1:4325").pathname
  if (path.startsWith("/api/")) {
    const upstream = request({
      hostname: "127.0.0.1", port: 4324, path: req.url,
      method: req.method, headers: req.headers,
    }, response => {
      res.writeHead(response.statusCode, response.headers)
      response.pipe(res)
    })
    upstream.on("error", () => { res.writeHead(502); res.end() })
    req.pipe(upstream)
    return
  }
  try {
    let file = resolve(root, "." + decodeURIComponent(path))
    if (file !== root && !file.startsWith(root + sep)) throw Error("path")
    if (file === root) file += "/index.html"
    else if (!extname(file)) file += ".html"
    const body = await readFile(file)
    res.writeHead(200, { "content-type": mime[extname(file)] || "application/octet-stream", "cache-control": "no-store" })
    res.end(body)
  } catch { res.writeHead(404); res.end("Fixture static file missing") }
})
server.on("connection", socket => {
  sockets.add(socket)
  socket.on("close", () => sockets.delete(socket))
})
server.on("upgrade", (req, socket, head) => {
  const upstream = createConnection(4324, "127.0.0.1", () => {
    upstream.write(`${req.method} ${req.url} HTTP/${req.httpVersion}\r\n`)
    for (let i = 0; i < req.rawHeaders.length; i += 2) {
      upstream.write(`${req.rawHeaders[i]}: ${req.rawHeaders[i + 1]}\r\n`)
    }
    upstream.write("\r\n")
    upstream.write(head)
    socket.pipe(upstream).pipe(socket)
  })
  upstream.on("error", () => socket.destroy())
  socket.on("close", () => upstream.destroy())
})
server.listen(4325, "127.0.0.1", () => console.log("Owned export on 4325; API fixture on 4324", root))
process.on("SIGTERM", () => {
  for (const socket of sockets) socket.destroy()
  server.close(() => { console.log("Owned export listener closed"); process.exit(0) })
})
