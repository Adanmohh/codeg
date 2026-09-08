import { afterEach, beforeEach, describe, expect, it, vi } from "vitest"
import { createBusinessClient, workspaceOrigin } from "./client"

const fetcher = vi.fn()
const native = vi.hoisted(() => ({ invoke: vi.fn(), isTauri: vi.fn() }))
vi.mock("@tauri-apps/api/core", () => native)
beforeEach(() => {
  vi.clearAllMocks()
  native.isTauri.mockReturnValue(false)
  vi.stubGlobal("fetch", fetcher)
  localStorage.clear()
})
afterEach(() => vi.unstubAllGlobals())
const memberConnection = {
  kind: "http" as const,
  address: "http://127.0.0.1:4340",
  token: "bdm_synthetic_member",
}
const response = (body: unknown, status = 200) =>
  new Response(JSON.stringify(body), { status })

describe("separate business credential client", () => {
  it("rejects personal HTTP access in an operator native host before any transport call", () => {
    native.isTauri.mockReturnValue(true)
    localStorage.setItem("codeg_token", "synthetic_original_operator")
    expect(() => createBusinessClient(memberConnection)).toThrow("forbidden")
    expect(fetcher).not.toHaveBeenCalled()
    expect(native.invoke).not.toHaveBeenCalled()
    expect(localStorage.getItem("codeg_token")).toBe(
      "synthetic_original_operator"
    )
    expect(JSON.stringify(localStorage)).not.toContain(memberConnection.token)
  })
  it("uses the isolated intake envelope and dedicated native command without legacy transport", async () => {
    fetcher.mockResolvedValueOnce(
      response({ items: [], canManageSetup: false, page: 0, hasMore: false })
    )
    const client = createBusinessClient(memberConnection)
    await client.intake("bindings/list", {})
    expect(fetcher).toHaveBeenCalledWith(
      "http://127.0.0.1:4340/api/business/intake/bindings/list",
      expect.objectContaining({
        body: '{"input":{}}',
        cache: "no-store",
        credentials: "omit",
      })
    )
    native.invoke.mockResolvedValueOnce({})
    const desktop = createBusinessClient({ kind: "native" })
    await desktop.intake("candidates/select", {
      operationId: "op",
      candidateId: "candidate",
      expectedRevision: 4,
      expectedSourceRevision: 2,
      passageIds: ["passage"],
    })
    expect(native.invoke).toHaveBeenCalledWith(
      "business_intake_candidates_select",
      {
        input: {
          operationId: "op",
          candidateId: "candidate",
          expectedRevision: 4,
          expectedSourceRevision: 2,
          passageIds: ["passage"],
        },
      }
    )
    client.close()
    desktop.close()
  })
  it("allows safe intake reasons only on intake, discards raw content, and never logs out on provider failures", async () => {
    const privateError = {
      code: "network_error",
      message: "private provider body",
      detail: "private key",
      i18n_key: "business.intake.provider_unavailable",
      i18n_params: { secret: "private" },
    }
    fetcher.mockResolvedValueOnce(response(privateError, 500))
    fetcher.mockResolvedValueOnce(response(privateError, 500))
    fetcher.mockResolvedValueOnce(
      response(
        { ...privateError, i18n_key: "business.intake.untrusted_secret" },
        500
      )
    )
    const unauthorized = vi.fn()
    const client = createBusinessClient(memberConnection, unauthorized)
    await expect(
      client.intake("sources/get", { sourceId: "source" })
    ).rejects.toMatchObject({
      kind: "offline",
      intakeReason: "provider_unavailable",
      message: "offline",
    })
    await expect(client.tasks("get", { taskId: "task" })).rejects.toMatchObject(
      { intakeReason: undefined }
    )
    await expect(
      client.intake("sources/get", { sourceId: "source" })
    ).rejects.toMatchObject({ intakeReason: undefined })
    expect(unauthorized).not.toHaveBeenCalled()
    expect(JSON.stringify(client)).not.toContain("private")
    client.close()
  })
  it("uses its own bearer and input envelope without reading/replacing the ambient operator token", async () => {
    localStorage.setItem("codeg_token", "synthetic_original_operator")
    fetcher.mockResolvedValueOnce(response({ needsBootstrap: false }))
    const client = createBusinessClient(memberConnection)
    await client.identity("context", {})
    expect(fetcher).toHaveBeenCalledWith(
      "http://127.0.0.1:4340/api/business/context",
      expect.objectContaining({
        method: "POST",
        body: '{"input":{}}',
        credentials: "omit",
        redirect: "error",
        cache: "no-store",
        headers: {
          "Content-Type": "application/json",
          Authorization: "Bearer bdm_synthetic_member",
        },
      })
    )
    expect(localStorage.getItem("codeg_token")).toBe(
      "synthetic_original_operator"
    )
    expect(JSON.stringify(localStorage)).not.toContain(memberConnection.token)
    expect(JSON.stringify(client)).not.toContain(memberConnection.token)
    client.close()
  })
  it.each([
    "http://example.com",
    "https://user:password@example.com",
    "https://example.com/?token=x",
    "https://example.com/#secret",
    "https://example.com/api",
    "javascript:alert(1)",
  ])("refuses unsafe/ambiguous destination %s before a request", (address) => {
    expect(() => workspaceOrigin(address)).toThrow("invalid")
    expect(fetcher).not.toHaveBeenCalled()
  })
  it("closes the session on rejected credentials without surfacing the server message", async () => {
    fetcher.mockResolvedValueOnce(
      response(
        {
          code: "authentication_failed",
          message: "synthetic private internal detail",
        },
        401
      )
    )
    const unauthorized = vi.fn()
    const client = createBusinessClient(memberConnection, unauthorized)
    await expect(client.identity("context", {})).rejects.toMatchObject({
      kind: "unauthorized",
      message: "unauthorized",
    })
    expect(unauthorized).toHaveBeenCalledOnce()
    await expect(client.identity("context", {})).rejects.toMatchObject({
      kind: "closed",
    })
    expect(fetcher).toHaveBeenCalledOnce()
  })
  it("discards a late response after connection disposal", async () => {
    let complete!: (value: Response) => void
    fetcher.mockReturnValueOnce(
      new Promise<Response>((resolve) => {
        complete = resolve
      })
    )
    const client = createBusinessClient(memberConnection)
    const pending = client.identity("members/list", {
      organizationId: "synthetic-org",
    })
    client.close()
    complete(
      response([{ displayName: "Previous organization private member" }])
    )
    await expect(pending).rejects.toMatchObject({ kind: "closed" })
    expect(fetcher.mock.calls[0][1].signal.aborted).toBe(true)
  })
  it("keeps the session usable after a network failure and maps a revision conflict safely", async () => {
    fetcher.mockRejectedValueOnce(new Error("private network detail"))
    fetcher.mockResolvedValueOnce(
      response({ code: "already_exists", message: "private row" }, 409)
    )
    const unauthorized = vi.fn()
    const client = createBusinessClient(memberConnection, unauthorized)
    await expect(client.identity("context", {})).rejects.toMatchObject({
      kind: "offline",
      message: "offline",
    })
    await expect(
      client.identity("members/revoke", {
        organizationId: "org",
        memberId: "member",
        expectedRevision: 2,
      })
    ).rejects.toMatchObject({ kind: "conflict", message: "conflict" })
    expect(unauthorized).not.toHaveBeenCalled()
    client.close()
  })
  it("native mode invokes only the dedicated operator command and carries no member bearer", async () => {
    native.invoke.mockResolvedValueOnce({ needsBootstrap: true })
    const client = createBusinessClient({ kind: "native" })
    await client.identity("context", {})
    expect(native.invoke).toHaveBeenCalledWith("business_context", {
      input: {},
    })
    expect(fetcher).not.toHaveBeenCalled()
    await client.tasks("entrust-execution", {
      taskId: "synthetic-task",
      expectedRevision: 4,
      workTaskId: 17,
    })
    expect(native.invoke).toHaveBeenLastCalledWith(
      "business_tasks_entrust_execution",
      {
        input: {
          taskId: "synthetic-task",
          expectedRevision: 4,
          workTaskId: 17,
        },
      }
    )
    client.close()
  })
})
