import { isTauri } from "@tauri-apps/api/core"
import { extractAppCommandError } from "@/lib/app-error"
import type { IdentityOperations } from "./identity"
import type { TaskOperations } from "./tasks"
import {
  intakeCommands,
  intakeReason,
  type IntakeOperations,
  type IntakeReason,
} from "./intake"

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
  constructor(
    readonly kind: BusinessErrorKind,
    readonly intakeReason?: IntakeReason
  ) {
    super(kind)
    this.name = "BusinessError"
  }
}

function safeError(
  error: unknown,
  status?: number,
  intake = false
): BusinessError {
  if (error instanceof BusinessError) return error
  const parsed = extractAppCommandError(error)
  const reason = intake ? intakeReason(parsed?.i18n_key) : undefined
  if (status === 401 || parsed?.code === "authentication_failed")
    return new BusinessError("unauthorized")
  if (status === 403 || parsed?.code === "permission_denied")
    return new BusinessError("forbidden", reason)
  if (status === 404 || parsed?.code === "not_found")
    return new BusinessError("missing", reason)
  if (status === 409 || parsed?.code === "already_exists")
    return new BusinessError("conflict", reason)
  if (status === 400 || parsed?.code === "invalid_input")
    return new BusinessError("invalid", reason)
  if (parsed?.i18n_key === "business.bootstrapRequired")
    return new BusinessError("bootstrap")
  // Never expose SQL, provider bodies, arbitrary server messages or credentials.
  return new BusinessError("offline", reason)
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
  "settings/get": "business_settings_get",
  "settings/update": "business_settings_update",
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
  "tasks/entrust-execution": "business_tasks_entrust_execution",
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
  // The native host has operator capabilities. Personal HTTP sessions belong
  // in a browser until the backend can create a restricted tenant window.
  // Reject before retaining a bearer or making any request, even if an old
  // hydrated form submits. Native isolation remains a backend responsibility.
  if (connection.kind === "http" && isTauri())
    throw new BusinessError("forbidden")
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
    path: keyof typeof commands | `intake/${keyof IntakeOperations}`,
    input: object
  ): Promise<T> {
    if (closed) throw new BusinessError("closed")
    const intake = path.startsWith("intake/")
    const command = intake
      ? intakeCommands[path.slice(7) as keyof IntakeOperations]
      : commands[path as keyof typeof commands]
    if (
      !command ||
      !Object.prototype.hasOwnProperty.call(
        intake ? intakeCommands : commands,
        intake ? path.slice(7) : path
      )
    )
      throw new BusinessError("invalid")
    const controller = new AbortController()
    active.add(controller)
    const timer = window.setTimeout(() => controller.abort(), 20000)
    try {
      let result: T
      if (native) {
        const { invoke } = await import("@tauri-apps/api/core")
        result = await invoke<T>(command, { input })
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
            response.status,
            intake
          )
        result = await response.json()
      }
      if (closed) throw new BusinessError("closed")
      return result
    } catch (error) {
      if (closed) throw new BusinessError("closed")
      const safe = safeError(error, undefined, intake)
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
    intake<K extends keyof IntakeOperations>(
      operation: K,
      input: IntakeOperations[K]["input"]
    ): Promise<IntakeOperations[K]["result"]> {
      return request(`intake/${operation}`, input)
    },
  }
}
export type BusinessClient = ReturnType<typeof createBusinessClient>
