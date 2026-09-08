import { extractAppCommandError } from "@/lib/app-error"
import type { IdentityOperations } from "./identity"
import type { TaskOperations } from "./tasks"

export type BusinessErrorKind =
  | "unauthorized"
  | "forbidden"
  | "missing"
  | "conflict"
  | "invalid"
  | "bootstrap"
  | "offline"
  | "closed"
export class BusinessError extends Error {
  constructor(readonly kind: BusinessErrorKind) {
    super(kind)
    this.name = "BusinessError"
  }
}

function safeError(error: unknown, status?: number): BusinessError {
  if (error instanceof BusinessError) return error
  const parsed = extractAppCommandError(error)
  if (status === 401 || parsed?.code === "authentication_failed")
    return new BusinessError("unauthorized")
  if (status === 403 || parsed?.code === "permission_denied")
    return new BusinessError("forbidden")
  if (status === 404 || parsed?.code === "not_found")
    return new BusinessError("missing")
  if (status === 409 || parsed?.code === "already_exists")
    return new BusinessError("conflict")
  if (status === 400 || parsed?.code === "invalid_input")
    return new BusinessError("invalid")
  if (parsed?.i18n_key === "business.bootstrapRequired")
    return new BusinessError("bootstrap")
  // Never expose SQL, provider bodies, arbitrary server messages or credentials.
  return new BusinessError("offline")
}

export function workspaceOrigin(address: string): string {
  let url: URL
  try {
    url = new URL(address.trim())
  } catch {
    throw new BusinessError("invalid")
  }
  const loopback = ["localhost", "127.0.0.1", "[::1]"].includes(url.hostname)
  if (
    (url.protocol !== "https:" && !(loopback && url.protocol === "http:")) ||
    url.username ||
    url.password ||
    url.search ||
    url.hash ||
    url.pathname !== "/"
  ) {
    throw new BusinessError("invalid")
  }
  return url.origin
}

const identityCommands = {
  context: "business_context",
  bootstrap: "business_bootstrap",
  "members/list": "business_members_list",
  "members/create": "business_members_create",
  "members/update": "business_members_update",
  "members/revoke": "business_members_revoke",
  "credentials/issue": "business_credentials_issue",
  "credentials/list": "business_credentials_list",
  "credentials/revoke": "business_credentials_revoke",
} as const satisfies Record<keyof IdentityOperations, string>

const commands = {
  ...identityCommands,
  "tasks/list": "business_tasks_list",
  "tasks/get": "business_tasks_get",
  "tasks/create": "business_tasks_create",
  "tasks/update": "business_tasks_update",
  "tasks/assign": "business_tasks_assign",
  "tasks/progress": "business_tasks_progress",
  "tasks/note": "business_tasks_note",
  "tasks/submit": "business_tasks_submit",
  "tasks/review": "business_tasks_review",
  "tasks/cancel": "business_tasks_cancel",
  "tasks/archive": "business_tasks_archive",
  "tasks/link-execution": "business_tasks_link_execution",
} as const satisfies Record<
  keyof IdentityOperations | `tasks/${keyof TaskOperations}`,
  string
>

export type BusinessConnection =
  | { kind: "native" }
  | { kind: "http"; address: string; token: string }

// Only this client owns the personal bearer. It never consults or writes the
// operator transport, CODEG_TOKEN, localStorage, cookies or URL credentials.
export function createBusinessClient(
  connection: BusinessConnection,
  onUnauthorized: () => void = () => {}
) {
  const native = connection.kind === "native"
  const origin =
    connection.kind === "http" ? workspaceOrigin(connection.address) : ""
  let token = connection.kind === "http" ? connection.token.trim() : ""
  if (connection.kind === "http" && (!token || /[\r\n]/.test(token)))
    throw new BusinessError("invalid")
  let closed = false
  const active = new Set<AbortController>()
  function close() {
    closed = true
    token = ""
    for (const controller of active) controller.abort()
    active.clear()
  }
  async function request<T>(
    path: keyof typeof commands,
    input: object
  ): Promise<T> {
    if (closed) throw new BusinessError("closed")
    const controller = new AbortController()
    active.add(controller)
    const timer = window.setTimeout(() => controller.abort(), 20000)
    try {
      let result: T
      if (native) {
        const { invoke } = await import("@tauri-apps/api/core")
        result = await invoke<T>(commands[path], { input })
      } else {
        const response = await fetch(`${origin}/api/business/${path}`, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${token}`,
          },
          body: JSON.stringify({ input }),
          cache: "no-store",
          credentials: "omit",
          redirect: "error",
          referrerPolicy: "no-referrer",
          signal: controller.signal,
        })
        if (!response.ok)
          throw safeError(
            await response.json().catch(() => null),
            response.status
          )
        result = await response.json()
      }
      if (closed) throw new BusinessError("closed")
      return result
    } catch (error) {
      if (closed) throw new BusinessError("closed")
      const safe = safeError(error)
      if (safe.kind === "unauthorized") {
        close()
        onUnauthorized()
      }
      throw safe
    } finally {
      window.clearTimeout(timer)
      active.delete(controller)
    }
  }
  return {
    label: native ? "local" : origin,
    native,
    close,
    identity<K extends keyof IdentityOperations>(
      operation: K,
      input: IdentityOperations[K]["input"]
    ): Promise<IdentityOperations[K]["result"]> {
      return request(operation, input)
    },
    tasks<K extends keyof TaskOperations>(
      operation: K,
      input: TaskOperations[K]["input"]
    ): Promise<TaskOperations[K]["result"]> {
      return request(`tasks/${operation}`, input)
    },
  }
}
export type BusinessClient = ReturnType<typeof createBusinessClient>
