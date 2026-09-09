// @vitest-environment node
import { afterEach, describe, expect, it, vi } from "vitest"
import {
  createExecutionHttpClient,
  type ExecutionClient,
  type ExecutionHttpTransport,
} from "./client"
import { MAX_ASSET_BYTES } from "./content"
import type { Disposition } from "./types"

const clients: ExecutionClient[] = []
function setup(post: ExecutionHttpTransport["post"]) {
  const host = { post: vi.fn(post), unauthorized: vi.fn() }
  const client = createExecutionHttpClient(host)
  clients.push(client)
  return { client, host }
}
function deferred<T>() {
  let resolve!: (value: T) => void
  const promise = new Promise<T>((done) => {
    resolve = done
  })
  return { promise, resolve }
}
const selection = { assetId: "synthetic-asset", versionId: "synthetic-v1" }
async function artifact(
  text: string,
  mediaType = "text/plain",
  disposition: Disposition = "preview"
) {
  const bytes = new TextEncoder().encode(text)
  const digest = await crypto.subtle.digest("SHA-256", bytes)
  const sha256 = Array.from(new Uint8Array(digest), (byte) =>
    byte.toString(16).padStart(2, "0")
  ).join("")
  const metadata = { mediaType, byteSize: bytes.byteLength, sha256 }
  const headers = {
    "content-type": mediaType,
    "content-length": String(metadata.byteSize),
    etag: `"sha256-${sha256}"`,
    "content-disposition": disposition === "preview" ? "inline" : "attachment",
    "cache-control": "no-store",
    "x-content-type-options": "nosniff",
  }
  return { metadata, headers, response: () => new Response(text, { headers }) }
}

afterEach(() => {
  for (const client of clients.splice(0)) client.close()
  vi.restoreAllMocks()
})

describe("E1 version-bound content", () => {
  it.each(["preview", "download"] as const)(
    "verifies Unicode bytes before providing a %s handle",
    async (disposition) => {
      const text = "Synthetic plan — خطة 🧭"
      const file = await artifact(text, "text/markdown", disposition)
      const { client } = setup(async () => file.response())
      const handle = await client.assetContent(
        selection,
        file.metadata,
        disposition
      )
      expect(await handle.blob.text()).toBe(text)
      expect(handle).toMatchObject(file.metadata)
      expect(handle.blob.type).toBe(
        disposition === "preview" ? "text/plain" : "application/octet-stream"
      )
    }
  )

  it("keeps the selected version metadata while its caller changes selection", async () => {
    const file = await artifact("Synthetic original")
    const pending = deferred<Response>()
    const { client, host } = setup(() => pending.promise)
    const selected = { ...selection }
    const expected = { ...file.metadata }
    const result = client.assetContent(selected, expected, "preview")
    selected.versionId = "synthetic-v2"
    expected.sha256 = "0".repeat(64)
    expected.mediaType = "text/html"
    expected.byteSize = 999
    pending.resolve(file.response())
    const handle = await result
    expect(handle).toMatchObject(file.metadata)
    expect(await handle.blob.text()).toBe("Synthetic original")
    expect(host.post.mock.calls[0][1]).toEqual({
      ...selection,
      disposition: "preview",
    })
  })

  it("rejects bytes matching caller-mutated metadata instead of the original version", async () => {
    const original = await artifact("Synthetic version A")
    const changed = await artifact("Synthetic version B")
    const pending = deferred<Response>()
    const createURL = vi.spyOn(URL, "createObjectURL")
    const { client } = setup(() => pending.promise)
    const expected = { ...original.metadata }
    const result = client.assetContent(selection, expected, "preview")
    const rejected = expect(result).rejects.toMatchObject({
      reason: "content_changed",
    })
    Object.assign(expected, changed.metadata)
    pending.resolve(changed.response())
    await rejected
    expect(createURL).not.toHaveBeenCalled()
  })

  it("checks the actual SHA-256 even when headers and byte count match", async () => {
    const original = await artifact("Synthetic version A")
    const createURL = vi.spyOn(URL, "createObjectURL")
    const { client } = setup(
      async () =>
        new Response("Synthetic version B", { headers: original.headers })
    )
    await expect(
      client.assetContent(selection, original.metadata, "preview")
    ).rejects.toMatchObject({ reason: "content_changed" })
    expect(createURL).not.toHaveBeenCalled()
  })

  it.each([
    "content-length",
    "etag",
    "content-type",
    "content-disposition",
    "cache-control",
    "x-content-type-options",
  ])(
    "refuses a response with missing %s before creating a handle",
    async (field) => {
      const file = await artifact("Synthetic content")
      const headers = new Headers(file.headers)
      headers.delete(field)
      const createURL = vi.spyOn(URL, "createObjectURL")
      const { client } = setup(
        async () =>
          new Response(new TextEncoder().encode("Synthetic content"), {
            headers,
          })
      )
      await expect(
        client.assetContent(selection, file.metadata, "preview")
      ).rejects.toThrow()
      expect(createURL).not.toHaveBeenCalled()
    }
  )

  it.each(["short", "Synthetic content plus extra bytes"])(
    "rejects truncated or excess bytes (%s)",
    async (text) => {
      const file = await artifact("Synthetic content")
      const createURL = vi.spyOn(URL, "createObjectURL")
      const { client } = setup(
        async () => new Response(text, { headers: file.headers })
      )
      await expect(
        client.assetContent(selection, file.metadata, "preview")
      ).rejects.toMatchObject({ reason: "content_changed" })
      expect(createURL).not.toHaveBeenCalled()
    }
  )

  it("caps the declared allocation without allocating a large fixture", async () => {
    const file = await artifact("Synthetic content")
    const createURL = vi.spyOn(URL, "createObjectURL")
    const { client } = setup(async () => file.response())
    await expect(
      client.assetContent(
        selection,
        { ...file.metadata, byteSize: MAX_ASSET_BYTES + 1 },
        "preview"
      )
    ).rejects.toMatchObject({ reason: "transport_unavailable" })
    expect(createURL).not.toHaveBeenCalled()
  })

  it("aborts a partially read body without returning a URL", async () => {
    const file = await artifact("Synthetic content")
    const reading = deferred<void>()
    const cancel = vi.fn()
    const createURL = vi.spyOn(URL, "createObjectURL")
    const { client } = setup(
      async () =>
        new Response(
          new ReadableStream({
            start(controller) {
              controller.enqueue(new TextEncoder().encode("Synthetic"))
            },
            pull() {
              reading.resolve()
            },
            cancel,
          }),
          { headers: file.headers }
        )
    )
    const scope = new AbortController()
    const result = client.assetContent(
      selection,
      file.metadata,
      "preview",
      scope.signal
    )
    const rejected = expect(result).rejects.toMatchObject({
      reason: "transport_unavailable",
      kind: "aborted",
    })
    await reading.promise
    scope.abort()
    await rejected
    expect(cancel).toHaveBeenCalledOnce()
    expect(createURL).not.toHaveBeenCalled()
  })

  it("revokes handles once on explicit disposal, client closure and 401", async () => {
    const file = await artifact("Synthetic content")
    const revokeURL = vi.spyOn(URL, "revokeObjectURL")
    const { client, host } = setup(async () => file.response())
    const first = await client.assetContent(selection, file.metadata, "preview")
    const second = await client.assetContent(
      selection,
      file.metadata,
      "preview"
    )
    first.dispose()
    first.dispose()
    expect(revokeURL).toHaveBeenCalledOnce()
    expect(revokeURL).toHaveBeenCalledWith(first.url)
    host.post.mockResolvedValueOnce(new Response(null, { status: 401 }))
    await expect(
      client.publishedAsset({
        ...selection,
        taskId: "synthetic-task",
        deliverableId: "synthetic-deliverable",
      })
    ).rejects.toMatchObject({ reason: "unauthorized" })
    expect(revokeURL.mock.calls).toEqual([[first.url], [second.url]])
    client.close()
    second.dispose()
    expect(revokeURL).toHaveBeenCalledTimes(2)
  })

  it("refuses executable preview before transport but permits inert download", async () => {
    const text = "<script>syntheticNeverExecuted()</script>"
    const file = await artifact(text, "text/html", "download")
    const { client, host } = setup(async () => file.response())
    await expect(
      client.assetContent(selection, file.metadata, "preview")
    ).rejects.toMatchObject({ reason: "unavailable" })
    expect(host.post).not.toHaveBeenCalled()
    const handle = await client.assetContent(
      selection,
      file.metadata,
      "download"
    )
    expect(handle.blob.type).toBe("application/octet-stream")
    expect(await handle.blob.text()).toBe(text)
  })

  it("uses the task selected-version path without any private asset or session call", async () => {
    const file = await artifact("Synthetic published content")
    const { client, host } = setup(async () => file.response())
    const selected = {
      ...selection,
      taskId: "synthetic-task",
      deliverableId: "synthetic-deliverable",
    }
    const handle = await client.publishedContent(
      selected,
      file.metadata,
      "preview"
    )
    expect(await handle.blob.text()).toBe("Synthetic published content")
    expect(host.post).toHaveBeenCalledOnce()
    expect(host.post.mock.calls[0][0]).toBe("tasks/deliverables/assets/content")
    expect(host.post.mock.calls[0][1]).toEqual({
      ...selected,
      disposition: "preview",
    })
  })
})
