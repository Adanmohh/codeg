import { Blob as NodeBlob } from "node:buffer"
import { webcrypto } from "node:crypto"
import {
  act,
  cleanup,
  fireEvent,
  render,
  screen,
  waitFor,
} from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest"
import {
  createBusinessClient,
  type BusinessClient,
} from "@/lib/business/client"
import type { PublishedAsset } from "@/lib/business-execution/types"
import { BusinessWorkspace } from "@/components/business/workspace"
import { context, detail, member } from "@/components/business/test-fixtures"
import { PublishedAssetPane, type PublishedFileIntent } from "./published-asset"

vi.mock("@tauri-apps/api/core", () => ({
  isTauri: () => false,
  invoke: vi.fn(),
}))
vi.mock("@/hooks/use-media-query", () => ({ useMediaQuery: () => true }))
vi.mock("@/components/i18n-provider", () => ({
  useAppI18n: () => ({ appLocale: "en", setLanguageSettings: vi.fn() }),
}))
vi.mock("next-themes", () => ({
  useTheme: () => ({ theme: "light", setTheme: vi.fn() }),
}))

const clients: BusinessClient[] = []
const fetcher = vi.fn<typeof fetch>()
const createURL = vi.fn<(blob: Blob) => string>(
  () => "blob:synthetic-selected-version"
)
const revokeURL = vi.fn()
let file: PublishedAsset
let intent: PublishedFileIntent
let content =
  "# Synthetic reviewed brief\n<script>window.leak = true</script>\n![image](https://invalid.example/private)\nمرحبا"
let contentResponse: (init?: RequestInit) => Promise<Response>

function business() {
  const client = createBusinessClient(
    {
      kind: "http",
      address: "http://127.0.0.1:4359",
      token: "synthetic-personal-reader",
    },
    vi.fn()
  )
  clients.push(client)
  return client
}
function page(
  client: BusinessClient,
  locale: "en" | "ar" = "en",
  visible = true,
  selected = intent
) {
  return (
    <NextIntlClientProvider locale={locale} messages={{}} timeZone="UTC">
      <div style={visible ? undefined : { display: "none" }}>
        <PublishedAssetPane client={client} intent={selected} />
      </div>
    </NextIntlClientProvider>
  )
}
function workspace(client: BusinessClient) {
  return (
    <NextIntlClientProvider locale="en" messages={{}} timeZone="UTC">
      <BusinessWorkspace
        client={client}
        context={context}
        onContext={vi.fn()}
        disconnect={vi.fn()}
      />
    </NextIntlClientProvider>
  )
}
async function configure(
  mediaType = "text/markdown",
  versionId = "synthetic-version"
) {
  const bytes = new TextEncoder().encode(content)
  const digest = await webcrypto.subtle.digest("SHA-256", bytes)
  const sha256 = Array.from(new Uint8Array(digest), (byte) =>
    byte.toString(16).padStart(2, "0")
  ).join("")
  file = {
    version: {
      assetId: "synthetic-asset",
      versionId,
      title: "Synthetic selected brief",
      mediaType,
      byteSize: bytes.byteLength,
      sha256,
    },
    createdAt: "2026-09-09T00:00:00Z",
    producer: { clientId: "synthetic-client", model: null },
    publication: {
      deliverableId: "synthetic-deliverable",
      taskRevision: 4,
      submittedBy: {
        memberId: member.id,
        displayName: "Synthetic Author",
        authorityKind: "operator",
      },
    },
  }
  intent = {
    selection: {
      taskId: detail().task.id,
      deliverableId: file.publication.deliverableId,
      assetId: file.version.assetId,
      versionId,
    },
    expected: { ...file.version },
  }
  const captured = { file: structuredClone(file), text: content }
  contentResponse = async (init) => {
    const { input } = JSON.parse(String(init?.body)) as {
      input: { disposition: string }
    }
    return new Response(captured.text, {
      headers: {
        "content-type": captured.file.version.mediaType,
        "content-length": String(captured.file.version.byteSize),
        etag: `"sha256-${captured.file.version.sha256}"`,
        "content-disposition":
          input.disposition === "preview" ? "inline" : "attachment",
        "cache-control": "no-store",
        "x-content-type-options": "nosniff",
      },
    })
  }
}
beforeEach(async () => {
  vi.clearAllMocks()
  vi.stubGlobal("crypto", webcrypto)
  vi.stubGlobal("Blob", NodeBlob)
  vi.stubGlobal(
    "URL",
    class extends URL {
      static createObjectURL = createURL
      static revokeObjectURL = revokeURL
    }
  )
  vi.stubGlobal("fetch", fetcher)
  localStorage.clear()
  content =
    "# Synthetic reviewed brief\n<script>window.leak = true</script>\n![image](https://invalid.example/private)\nمرحبا"
  await configure()
  fetcher.mockImplementation(async (url, init) => {
    const path = new URL(String(url)).pathname
    if (path.endsWith("/tasks/deliverables/assets/get"))
      return Response.json(file)
    if (path.endsWith("/tasks/deliverables/assets/content"))
      return contentResponse(init)
    if (path.endsWith("/settings/get"))
      return Response.json({
        organizationId: member.organizationId,
        revision: 1,
        settings: {
          displayName: context.organization!.name,
          palette: "neutral",
          workspaceLayout: "split",
          defaultWorkArea: "tasks",
        },
      })
    if (path.endsWith("/context")) return Response.json(context)
    if (path.endsWith("/members/list")) return Response.json([member])
    const task = detail()
    task.task.currentDeliverableId = file.publication.deliverableId
    task.task.status = "review"
    task.task.revision = 4
    task.deliverables = [
      {
        id: file.publication.deliverableId,
        revision: 4,
        author: {
          id: member.id,
          displayName: member.displayName,
          kind: "human",
        },
        body: "Synthetic human review request",
        createdAt: file.createdAt,
        assets: [file.version],
      },
    ]
    if (path.endsWith("/tasks/list"))
      return Response.json({
        tasks: [task.task],
        page: 0,
        hasMore: false,
        canCreate: true,
      })
    if (path.endsWith("/tasks/get")) return Response.json(task)
    throw new Error("Unexpected synthetic test route")
  })
})
afterEach(() => {
  cleanup()
  clients.splice(0).forEach((client) => client.close())
  vi.restoreAllMocks()
  vi.unstubAllGlobals()
})

describe("submitted file in the existing business workbench", () => {
  it("reads selected metadata first and escapes active content without private execution or external requests", async () => {
    render(page(business()))
    const read = await screen.findByRole("button", { name: "Read file" })
    expect(fetcher).toHaveBeenCalledOnce()
    fireEvent.click(read)
    await waitFor(() =>
      expect(document.querySelector("pre")?.textContent).toBe(content)
    )
    expect(document.querySelector("script,iframe,img")).toBeNull()
    expect(fetcher).toHaveBeenCalledTimes(2)
    expect(
      fetcher.mock.calls.every(([url]) =>
        String(url).startsWith(
          "http://127.0.0.1:4359/api/business/tasks/deliverables/assets/"
        )
      )
    ).toBe(true)
    expect(createURL).toHaveBeenCalledOnce()
    expect(revokeURL).toHaveBeenCalledWith("blob:synthetic-selected-version")
    expect(JSON.stringify(localStorage)).not.toContain(
      "Synthetic reviewed brief"
    )
  })

  it("keeps loaded content and the mounted reader through locale and hidden pane changes without new reads", async () => {
    const client = business()
    const view = render(page(client))
    fireEvent.click(await screen.findByRole("button", { name: "Read file" }))
    await waitFor(() =>
      expect(document.querySelector("pre")?.textContent).toBe(content)
    )
    const reader = document.querySelector("pre")
    view.rerender(page(client, "ar", false))
    expect(reader?.isConnected).toBe(true)
    view.rerender(page(client, "en"))
    expect(document.querySelector("pre")).toBe(reader)
    expect(fetcher).toHaveBeenCalledTimes(2)
  })

  it("offers an inert exact-version download with an honest unsupported-format preview gap", async () => {
    await configure(
      "application/vnd.openxmlformats-officedocument.presentationml.presentation"
    )
    const view = render(page(business()))
    fireEvent.click(
      await screen.findByRole("button", { name: "Prepare download" })
    )
    const save = await screen.findByRole("link", { name: "Save file" })
    expect(save).toHaveAttribute("href", "blob:synthetic-selected-version")
    expect(save).toHaveAttribute("download", file.version.title)
    expect(createURL.mock.calls[0][0].type).toBe("application/octet-stream")
    expect(screen.queryByRole("button", { name: "Read file" })).toBeNull()
    expect(
      screen.getByText(
        "Preview is not available for this format. You can download the submitted version."
      )
    ).toBeVisible()
    expect(fetcher).toHaveBeenCalledTimes(2)
    view.unmount()
    expect(revokeURL).toHaveBeenCalledWith("blob:synthetic-selected-version")
  })

  it("refuses a same-ID metadata response whose immutable manifest differs from the selected submission", async () => {
    file.version.sha256 = "0".repeat(64)
    render(page(business()))
    await screen.findByRole("alert")
    expect(screen.queryByRole("button", { name: "Read file" })).toBeNull()
    expect(
      screen.queryByRole("button", { name: "Prepare download" })
    ).toBeNull()
    expect(fetcher).toHaveBeenCalledOnce()
  })

  it("withdraws a loaded preview on denied fresh content access without exposing the denial body", async () => {
    render(page(business()))
    fireEvent.click(await screen.findByRole("button", { name: "Read file" }))
    await waitFor(() =>
      expect(document.querySelector("pre")?.textContent).toBe(content)
    )
    contentResponse = async () =>
      new Response("Synthetic private diagnostic", { status: 403 })
    fireEvent.click(screen.getByRole("button", { name: "Prepare download" }))
    await screen.findByRole("alert")
    expect(document.querySelector("pre")).toBeNull()
    expect(screen.queryByText("Synthetic Author")).toBeNull()
    expect(screen.queryByText("Synthetic private diagnostic")).toBeNull()
    expect(screen.queryByRole("link", { name: "Save file" })).toBeNull()
    expect(fetcher).toHaveBeenCalledTimes(3)
  })

  it("discards a held old-version response on an explicit version change", async () => {
    const oldResponse = contentResponse
    let finish!: (response: Response) => void
    contentResponse = () =>
      new Promise((resolve) => {
        finish = resolve
      })
    const client = business()
    const view = render(page(client))
    fireEvent.click(await screen.findByRole("button", { name: "Read file" }))
    const heldInit = fetcher.mock.calls[1][1]
    content = "New synthetic reviewed version"
    await configure("text/markdown", "synthetic-new-version")
    view.rerender(page(client))
    await screen.findByRole("button", { name: "Read file" })
    await act(async () => {
      finish(await oldResponse(heldInit))
    })
    expect(heldInit?.signal?.aborted).toBe(true)
    expect(document.querySelector("pre")).toBeNull()
    fireEvent.click(screen.getByRole("button", { name: "Read file" }))
    await waitFor(() =>
      expect(document.querySelector("pre")?.textContent).toBe(content)
    )
    expect(fetcher).toHaveBeenCalledTimes(4)
  })

  it("opens the reloaded task's submitted version in the accepted tabs and keeps the human draft intact", async () => {
    render(workspace(business()))
    fireEvent.click(
      await screen.findByRole("button", { name: /Synthetic launch brief/ })
    )
    fireEvent.click(
      await screen.findByRole("button", { name: /Synthetic selected brief/ })
    )
    await screen.findByRole("button", { name: "Read file" })
    const fileTab = screen.getByRole("tab", { name: file.version.title })
    expect(fileTab).toHaveAttribute("aria-selected", "true")
    fireEvent.click(screen.getByRole("tab", { name: "Synthetic launch brief" }))
    fireEvent.click(screen.getByRole("button", { name: "Edit task" }))
    const brief = screen.getByRole("textbox", { name: "Brief" })
    fireEvent.change(brief, {
      target: { value: "Unsent synthetic human changes" },
    })
    fireEvent.click(fileTab)
    fireEvent.click(screen.getByRole("button", { name: "Read file" }))
    await waitFor(() =>
      expect(document.querySelector("pre")?.textContent).toBe(content)
    )
    fireEvent.click(screen.getByRole("tab", { name: "Synthetic launch brief" }))
    expect(screen.getByRole("textbox", { name: "Brief" })).toBe(brief)
    expect(brief).toHaveValue("Unsent synthetic human changes")
    expect(
      fetcher.mock.calls.some(([url]) => String(url).includes("/execution/"))
    ).toBe(false)
    expect(
      fetcher.mock.calls.some(([url]) => String(url).includes("/tasks/update"))
    ).toBe(false)
    expect(screen.queryByText("Engineering workspace")).toBeNull()
  })
})
