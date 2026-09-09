import { readAssetContent, type ContentMetadata } from "./content"
import { ExecutionError, safeExecutionError } from "./protocol"
import {
  assertActive,
  readBoundedJson,
  readSessionStream,
  requireSuccess,
} from "./reader"
import type {
  AssetSelection,
  Disposition,
  EventsInput,
  ExecutionInputs,
  ExecutionResults,
  JsonOperation,
  PublishedAsset,
  PublishedSelection,
  SessionFrame,
} from "./types"

const JSON_OPERATIONS = [
  "operations/get",
  "profiles/list",
  "sessions/list",
  "sessions/start",
  "sessions/get",
  "sessions/continue",
  "sessions/prompt",
  "sessions/history",
  "outputs/list",
  "assets/import-output",
  "assets/list",
  "assets/versions",
  "assets/get",
  "assets/submit",
] as const satisfies readonly JsonOperation[]

export type ExecutionPath =
  | `execution/${JsonOperation}`
  | "execution/sessions/events"
  | "execution/assets/content"
  | "tasks/deliverables/assets/get"
  | "tasks/deliverables/assets/content"

// Supplied ONLY by the existing business HTTP credential closure at integration.
// It must POST {input}, omit cookies, deny redirects and retain the inner auth.
// There is deliberately no token/config getter, native invoke or global fetch.
export interface ExecutionHttpTransport {
  post(
    path: ExecutionPath,
    input: object,
    signal: AbortSignal
  ): Promise<Response>
  unauthorized(): void
}
export interface AssetContentHandle extends ContentMetadata {
  blob: Blob
  url: string
  dispose(): void
}
export function createExecutionHttpClient(host: ExecutionHttpTransport) {
  let closed = false
  const active = new Set<AbortController>()
  const content = new Set<AssetContentHandle>()
  function close() {
    closed = true
    for (const controller of active) controller.abort()
    active.clear()
    for (const handle of content) handle.dispose()
    content.clear()
  }
  async function request<T>(
    path: ExecutionPath,
    input: object,
    signal: AbortSignal | undefined,
    consume: (response: Response, signal: AbortSignal) => Promise<T>,
    stream = false
  ): Promise<T> {
    if (closed) throw new ExecutionError("cancelled")
    const controller = new AbortController()
    const abort = () => controller.abort()
    if (signal?.aborted) abort()
    signal?.addEventListener("abort", abort, { once: true })
    active.add(controller)
    // Same existing business request budget. An attached event stream is held
    // open after its headers; scope disposal and explicit detach still abort it.
    const timer = globalThis.setTimeout(abort, 20000)
    let response: Response | undefined
    try {
      assertActive(controller.signal)
      response = await host.post(path, input, controller.signal)
      if (stream) globalThis.clearTimeout(timer)
      if (closed || controller.signal.aborted) {
        void response.body?.cancel().catch(() => {})
        throw new ExecutionError("cancelled")
      }
      const result = await consume(response, controller.signal)
      assertActive(controller.signal)
      if (closed) throw new ExecutionError("cancelled")
      return result
    } catch (error) {
      const safe = controller.signal.aborted
        ? new ExecutionError("cancelled")
        : safeExecutionError(error)
      if (safe.reason === "unauthorized") {
        close()
        host.unauthorized()
      }
      throw safe
    } finally {
      if (response?.body && !response.bodyUsed)
        void response.body.cancel().catch(() => {})
      globalThis.clearTimeout(timer)
      signal?.removeEventListener("abort", abort)
      active.delete(controller)
    }
  }
  async function json<T>(response: Response, signal: AbortSignal): Promise<T> {
    await requireSuccess(response, signal)
    return (await readBoundedJson(response, signal)) as T
  }
  async function assetContent(
    path: "execution/assets/content" | "tasks/deliverables/assets/content",
    selection: AssetSelection | PublishedSelection,
    expected: ContentMetadata,
    disposition: Disposition,
    signal?: AbortSignal
  ): Promise<AssetContentHandle> {
    const blob = await request(
      path,
      { ...selection, disposition },
      signal,
      (response, scopeSignal) =>
        readAssetContent(response, expected, disposition, scopeSignal)
    )
    // A close/abort may occur in the microtask between the inner read and here.
    if (closed || signal?.aborted) throw new ExecutionError("cancelled")
    const url = URL.createObjectURL(blob)
    let disposed = false
    const handle: AssetContentHandle = {
      ...expected,
      blob,
      url,
      dispose() {
        if (disposed) return
        disposed = true
        URL.revokeObjectURL(url)
        content.delete(handle)
      },
    }
    content.add(handle)
    return handle
  }
  return {
    close,
    json<K extends JsonOperation>(
      operation: K,
      input: ExecutionInputs[K],
      signal?: AbortSignal
    ): Promise<ExecutionResults[K]> {
      if (!JSON_OPERATIONS.some((allowed) => allowed === operation))
        return Promise.reject(new ExecutionError("invalid"))
      if (
        operation === "sessions/prompt" &&
        "inputs" in input &&
        input.inputs.some((reference) => reference.kind === "account_snapshot")
      )
        return Promise.reject(new ExecutionError("unavailable"))
      return request<ExecutionResults[K]>(
        `execution/${operation}`,
        input,
        signal,
        json
      )
    },
    events(
      input: EventsInput,
      onFrame: (frame: SessionFrame) => void,
      signal?: AbortSignal
    ) {
      return request(
        "execution/sessions/events",
        input,
        signal,
        (response, scopeSignal) =>
          readSessionStream(response, input, scopeSignal, onFrame),
        true
      )
    },
    publishedAsset(selection: PublishedSelection, signal?: AbortSignal) {
      return request<PublishedAsset>(
        "tasks/deliverables/assets/get",
        selection,
        signal,
        json
      )
    },
    assetContent(
      selection: AssetSelection,
      expected: ContentMetadata,
      disposition: Disposition,
      signal?: AbortSignal
    ) {
      return assetContent(
        "execution/assets/content",
        selection,
        expected,
        disposition,
        signal
      )
    },
    publishedContent(
      selection: PublishedSelection,
      expected: ContentMetadata,
      disposition: Disposition,
      signal?: AbortSignal
    ) {
      return assetContent(
        "tasks/deliverables/assets/content",
        selection,
        expected,
        disposition,
        signal
      )
    },
  }
}
export type ExecutionClient = ReturnType<typeof createExecutionHttpClient>
