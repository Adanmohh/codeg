import { afterEach, beforeEach, describe, expect, it, vi } from "vitest"
import { createBusinessClient, type BusinessClient } from "./client"

const native = vi.hoisted(() => ({ invoke: vi.fn(), isTauri: vi.fn() }))
vi.mock("@tauri-apps/api/core", () => native)
const fetcher = vi.fn<typeof fetch>()
const clients: BusinessClient[] = []
const connection = {
  kind: "http" as const,
  address: "http://127.0.0.1:4359",
  token: "synthetic-business-credential",
}
function client(onUnauthorized = vi.fn()) {
  const value = createBusinessClient(connection, onUnauthorized)
  clients.push(value)
  return value
}
function deferred() {
  let resolve!: (response: Response) => void
  const promise = new Promise<Response>((done) => {
    resolve = done
  })
  return { promise, resolve }
}
beforeEach(() => {
  vi.resetAllMocks()
  native.isTauri.mockReturnValue(false)
  vi.stubGlobal("fetch", fetcher)
  localStorage.clear()
})
afterEach(() => {
  for (const value of clients.splice(0)) value.close()
  vi.unstubAllGlobals()
})

describe("business-owned execution scopes", () => {
  it("creates scopes without traffic and uses only the owned business credential for selected files", async () => {
    localStorage.setItem("codeg_token", "synthetic-other-host-token")
    const business = client()
    const scope = business.execution()
    expect(fetcher).not.toHaveBeenCalled()
    fetcher.mockResolvedValueOnce(Response.json({}))
    const selection = {
      taskId: "synthetic-task",
      deliverableId: "synthetic-deliverable",
      assetId: "synthetic-asset",
      versionId: "synthetic-version",
    }
    await scope.publishedAsset(selection)
    expect(fetcher).toHaveBeenCalledWith(
      "http://127.0.0.1:4359/api/business/tasks/deliverables/assets/get",
      expect.objectContaining({
        method: "POST",
        body: JSON.stringify({ input: selection }),
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer synthetic-business-credential",
        },
        credentials: "omit",
        redirect: "error",
        cache: "no-store",
        referrerPolicy: "no-referrer",
      })
    )
    expect(localStorage.getItem("codeg_token")).toBe(
      "synthetic-other-host-token"
    )
    expect(JSON.stringify(localStorage)).not.toContain(connection.token)
    expect(JSON.stringify(scope)).not.toContain(connection.token)
    expect(native.invoke).not.toHaveBeenCalled()
  })

  it("closes every child and discards held responses when its connection ends", async () => {
    const held = deferred()
    fetcher.mockReturnValue(held.promise)
    const business = client()
    const first = business.execution()
    const second = business.execution()
    const one = first
      .json("profiles/list", { taskId: "synthetic-task" })
      .catch((error: unknown) => error)
    const two = second
      .json("sessions/get", { sessionId: "synthetic-session" })
      .catch((error: unknown) => error)
    business.close()
    held.resolve(Response.json({ privateValue: "synthetic stale data" }))
    expect(await one).toMatchObject({ kind: "closed" })
    expect(await two).toMatchObject({ kind: "closed" })
    for (const call of fetcher.mock.calls)
      expect(call[1]?.signal?.aborted).toBe(true)
    expect(() => business.execution()).toThrow("transport_unavailable")
    expect(fetcher).toHaveBeenCalledTimes(2)
  })

  it("allows a new pane scope after closing another without closing the connection", async () => {
    fetcher.mockImplementation(async () =>
      Response.json({ profiles: [], unavailableReason: null })
    )
    const business = client()
    const old = business.execution()
    old.close()
    old.close()
    await expect(
      old.json("profiles/list", { taskId: "synthetic-task" })
    ).rejects.toMatchObject({ kind: "closed" })
    await expect(
      business.execution().json("profiles/list", { taskId: "synthetic-task" })
    ).resolves.toMatchObject({ profiles: [] })
    expect(fetcher).toHaveBeenCalledOnce()
    await business.identity("context", {})
    expect(fetcher).toHaveBeenCalledTimes(2)
  })

  it("invalidates execution children when another business operation returns 401", async () => {
    const held = deferred()
    fetcher.mockReturnValueOnce(held.promise)
    fetcher.mockResolvedValueOnce(new Response(null, { status: 401 }))
    const invalidated = vi.fn()
    const business = client(invalidated)
    const pending = business
      .execution()
      .json("sessions/get", { sessionId: "synthetic-session" })
      .catch((error: unknown) => error)
    await expect(business.identity("context", {})).rejects.toMatchObject({
      kind: "unauthorized",
    })
    held.resolve(
      Response.json({ session: { title: "inaccessible synthetic title" } })
    )
    expect(await pending).toMatchObject({ kind: "closed" })
    expect(invalidated).toHaveBeenCalledOnce()
    expect(fetcher.mock.calls[0][1]?.signal?.aborted).toBe(true)
  })

  it("closes siblings and the business connection on an execution 401", async () => {
    const held = deferred()
    fetcher.mockReturnValueOnce(held.promise)
    fetcher.mockResolvedValueOnce(new Response(null, { status: 401 }))
    const invalidated = vi.fn()
    const business = client(invalidated)
    const pending = business
      .execution()
      .json("sessions/get", { sessionId: "synthetic-session" })
      .catch((error: unknown) => error)
    await expect(
      business.execution().json("profiles/list", { taskId: "synthetic-task" })
    ).rejects.toMatchObject({ reason: "unauthorized" })
    held.resolve(Response.json({}))
    expect(await pending).toMatchObject({ kind: "closed" })
    await expect(business.identity("context", {})).rejects.toMatchObject({
      kind: "closed",
    })
    expect(invalidated).toHaveBeenCalledOnce()
    expect(fetcher).toHaveBeenCalledTimes(2)
  })

  it("refuses an unwired native E1 scope without host or HTTP fallback", () => {
    const business = createBusinessClient({ kind: "native" })
    clients.push(business)
    expect(() => business.execution()).toThrow("unavailable")
    expect(fetcher).not.toHaveBeenCalled()
    expect(native.invoke).not.toHaveBeenCalled()
  })
})
