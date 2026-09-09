// @vitest-environment node
import { afterEach, describe, expect, it, vi } from "vitest"
import { createExecutionHttpClient, type ExecutionClient } from "./client"
import type { PublishedAsset, PublishedSelection } from "./types"

const selection: PublishedSelection = {
  taskId: "synthetic-task",
  deliverableId: "synthetic-deliverable",
  assetId: "synthetic-asset",
  versionId: "synthetic-version",
}
const published: PublishedAsset = {
  version: {
    assetId: selection.assetId,
    versionId: selection.versionId,
    title: "Synthetic reviewed brief",
    mediaType: "text/markdown",
    byteSize: 12,
    sha256: "a".repeat(64),
  },
  createdAt: "2026-09-09T00:00:00Z",
  producer: { clientId: "synthetic-client", model: null },
  publication: {
    deliverableId: selection.deliverableId,
    taskRevision: 4,
    submittedBy: {
      memberId: "synthetic-author",
      displayName: "Synthetic Author",
      authorityKind: "operator",
    },
  },
}
const clients: ExecutionClient[] = []
function setup(post = vi.fn(async () => Response.json(published))) {
  const client = createExecutionHttpClient({ post, unauthorized: vi.fn() })
  clients.push(client)
  return { client, post }
}
afterEach(() => {
  clients.splice(0).forEach((client) => client.close())
})

describe("task-selected public asset projection", () => {
  it("returns only the selected public version through the task route", async () => {
    const { client, post } = setup()
    await expect(client.publishedAsset(selection)).resolves.toEqual(published)
    expect(post).toHaveBeenCalledOnce()
    expect(post).toHaveBeenCalledWith(
      "tasks/deliverables/assets/get",
      selection,
      expect.any(AbortSignal)
    )
  })

  it.each(["assetId", "versionId", "deliverableId"] as const)(
    "withholds a response for another %s",
    async (field) => {
      const value = structuredClone(published)
      if (field === "deliverableId") value.publication[field] = "foreign"
      else value.version[field] = "foreign"
      const { client } = setup(vi.fn(async () => Response.json(value)))
      await expect(client.publishedAsset(selection)).rejects.toMatchObject({
        reason: "content_changed",
      })
    }
  )

  it.each([
    { ...published, producer: { ...published.producer, sessionId: "private" } },
    {
      ...published,
      version: { ...published.version, producer: { turnId: "private" } },
    },
    { ...published, profileId: "private" },
    {
      ...published,
      publication: { ...published.publication, inputText: "private" },
    },
  ])(
    "rejects extra private fields rather than forwarding a broader DTO",
    async (value) => {
      const { client } = setup(vi.fn(async () => Response.json(value)))
      await expect(client.publishedAsset(selection)).rejects.toMatchObject({
        reason: "transport_unavailable",
        message: "transport_unavailable",
      })
    }
  )

  it.each([
    { byteSize: -1 },
    { byteSize: 50 * 1024 * 1024 + 1 },
    { sha256: "not-a-content-hash" },
  ])(
    "rejects metadata that cannot identify bounded immutable content",
    async (change) => {
      const { client } = setup(
        vi.fn(async () =>
          Response.json({
            ...published,
            version: { ...published.version, ...change },
          })
        )
      )
      await expect(client.publishedAsset(selection)).rejects.toMatchObject({
        reason: "transport_unavailable",
      })
    }
  )

  it("captures its selection before a caller changes it during a held response", async () => {
    let resolve!: (response: Response) => void
    const pending = new Promise<Response>((done) => {
      resolve = done
    })
    const { client, post } = setup(vi.fn(() => pending))
    const input = { ...selection }
    const result = client.publishedAsset(input)
    input.versionId = "new-selection"
    input.deliverableId = "new-deliverable"
    resolve(Response.json(published))
    await expect(result).resolves.toEqual(published)
    expect(post).toHaveBeenCalledWith(
      "tasks/deliverables/assets/get",
      selection,
      expect.any(AbortSignal)
    )
  })
})
