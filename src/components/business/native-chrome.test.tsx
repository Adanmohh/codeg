import { fireEvent, render, screen, waitFor } from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest"
import BusinessLayout from "@/app/business/layout"
import type { BusinessClient } from "@/lib/business/client"
import en from "@/i18n/messages/en.json"
import ar from "@/i18n/messages/ar.json"
import { BootstrapWorkspace } from "./bootstrap"
import { ConnectWorkspace } from "./connect"
import { BusinessWorkspace } from "./workspace"
import { context, detail, member } from "./test-fixtures"

const windowApi = vi.hoisted(() => ({
  getCurrentWindow: vi.fn(),
  isMaximized: vi.fn(),
  onResized: vi.fn(),
  minimize: vi.fn(),
  toggleMaximize: vi.fn(),
  close: vi.fn(),
  unlisten: vi.fn(),
}))
vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: windowApi.getCurrentWindow,
}))
vi.mock("@/components/i18n-provider", () => ({
  useAppI18n: () => ({ appLocale: "en", setLanguageSettings: vi.fn() }),
}))
vi.mock("next-themes", () => ({
  useTheme: () => ({ theme: "light", setTheme: vi.fn() }),
}))

const identity = vi.fn()
const tasks = vi.fn()
const connect = vi.fn()
const client: BusinessClient = {
  native: true,
  label: "local",
  close: vi.fn(),
  intake: vi.fn(),
  identity,
  tasks,
}
const states = ["connect", "bootstrap", "workspace"] as const
type State = (typeof states)[number]

function surface(state: State, locale: "en" | "ar" = "en") {
  const scene =
    state === "connect" ? (
      <ConnectWorkspace connect={connect} busy={false} error={null} />
    ) : state === "bootstrap" ? (
      <BootstrapWorkspace
        client={client}
        onReady={vi.fn()}
        disconnect={vi.fn()}
      />
    ) : (
      <BusinessWorkspace
        client={client}
        context={context}
        onContext={vi.fn()}
        disconnect={vi.fn()}
      />
    )
  return (
    <NextIntlClientProvider
      locale={locale}
      messages={locale === "ar" ? ar : en}
      timeZone="UTC"
    >
      <BusinessLayout>{scene}</BusinessLayout>
    </NextIntlClientProvider>
  )
}

beforeEach(() => {
  vi.clearAllMocks()
  localStorage.clear()
  vi.stubGlobal("isTauri", true)
  vi.stubGlobal("__TAURI_INTERNALS__", {})
  vi.spyOn(navigator, "platform", "get").mockReturnValue("MacIntel")
  vi.spyOn(navigator, "userAgent", "get").mockReturnValue("Mac OS X")
  windowApi.getCurrentWindow.mockReturnValue(windowApi)
  windowApi.isMaximized.mockResolvedValue(false)
  windowApi.onResized.mockResolvedValue(windowApi.unlisten)
  windowApi.minimize.mockResolvedValue(undefined)
  windowApi.toggleMaximize.mockResolvedValue(undefined)
  windowApi.close.mockResolvedValue(undefined)
  tasks.mockImplementation(async (operation) =>
    operation === "list"
      ? { tasks: [detail().task], page: 0, hasMore: false, canCreate: true }
      : detail()
  )
  identity.mockImplementation(async (operation) =>
    operation === "context" ? context : [member]
  )
})
afterEach(() => {
  vi.unstubAllGlobals()
  vi.restoreAllMocks()
  document.documentElement.dir = ""
})

describe("business native window chrome", () => {
  it.each(states)(
    "reserves macOS controls and a drag strip in %s",
    async (state) => {
      const { container } = render(surface(state))
      const chrome = container.querySelector("[data-business-native-chrome]")!
      await waitFor(() =>
        expect(chrome.querySelector(".pl-\\[92px\\]")).not.toBeNull()
      )
      expect(chrome.firstElementChild).toHaveClass("h-10", "shrink-0")
      expect(chrome.querySelectorAll("[data-tauri-drag-region]")).toHaveLength(
        2
      )
      expect(chrome).toHaveAttribute("dir", "ltr")
      expect(screen.queryByRole("button", { name: "Close window" })).toBeNull()
      expect(windowApi.getCurrentWindow).not.toHaveBeenCalled()
    }
  )

  it.each(states)(
    "adds no chrome or native window API to web %s",
    async (state) => {
      vi.stubGlobal("isTauri", false)
      vi.unstubAllGlobals()
      const { container } = render(surface(state))
      if (state === "workspace")
        await screen.findByText("Synthetic launch brief")
      expect(
        container.querySelector("[data-business-native-chrome]")
      ).toBeNull()
      expect(container.querySelector("[data-tauri-drag-region]")).toBeNull()
      expect(windowApi.getCurrentWindow).not.toHaveBeenCalled()
    }
  )

  it.each(
    ["Win32", "Linux x86_64"].flatMap((platform) =>
      states.map((state) => [platform, state] as const)
    )
  )(
    "retains existing %s controls in %s (mocked OS API)",
    async (platform, state) => {
      vi.spyOn(navigator, "platform", "get").mockReturnValue(platform)
      vi.spyOn(navigator, "userAgent", "get").mockReturnValue(platform)
      document.documentElement.dir = "rtl"
      const { container, unmount } = render(surface(state, "ar"))
      const minimize = await screen.findByRole("button", {
        name: "تصغير النافذة",
      })
      const maximize = screen.getByRole("button", { name: "تكبير النافذة" })
      const close = screen.getByRole("button", { name: "إغلاق النافذة" })
      await waitFor(() => expect(windowApi.isMaximized).toHaveBeenCalled())
      for (const button of [minimize, maximize, close]) {
        expect(button).not.toHaveAttribute("data-tauri-drag-region")
        fireEvent.click(button)
      }
      expect(windowApi.minimize).toHaveBeenCalledOnce()
      expect(windowApi.toggleMaximize).toHaveBeenCalledOnce()
      expect(windowApi.close).toHaveBeenCalledOnce()
      const chrome = container.querySelector("[data-business-native-chrome]")!
      expect(chrome).toHaveAttribute("dir", "ltr")
      expect(chrome.querySelector(".pr-\\[138px\\]")).not.toBeNull()
      expect(document.documentElement.dir).toBe("rtl")
      unmount()
      expect(windowApi.unlisten).toHaveBeenCalledOnce()
    }
  )

  it("offers only the local operator path in a native host, preserving chrome across locale rerenders", async () => {
    const { rerender, container } = render(surface("connect"))
    expect(screen.queryByLabelText("Personal access token")).toBeNull()
    expect(screen.queryByLabelText("Workspace address")).toBeNull()
    expect(container.querySelector('input[type="password"]')).toBeNull()
    expect(screen.getByText(/Personal sign-in is not available/)).toBeVisible()
    const local = screen.getByRole("button", {
      name: "Use this desktop’s local workspace",
    })
    const chrome = container.querySelector("[data-business-native-chrome]")
    rerender(surface("connect", "ar"))
    expect(
      screen.getByRole("button", { name: "استخدام مساحة سطح المكتب المحلية" })
    ).toBe(local)
    expect(container.querySelector('input[type="password"]')).toBeNull()
    expect(container.querySelector("[data-business-native-chrome]")).toBe(
      chrome
    )
    expect(connect).not.toHaveBeenCalled()
    expect(localStorage.length).toBe(0)
    fireEvent.click(local)
    await waitFor(() =>
      expect(connect).toHaveBeenCalledWith({ kind: "native" })
    )
  })

  it("keeps the personal HTTP draft mounted across browser locale rerenders", () => {
    vi.unstubAllGlobals()
    const { rerender } = render(surface("connect"))
    const token = screen.getByLabelText("Personal access token")
    fireEvent.change(token, { target: { value: "synthetic-private-draft" } })
    rerender(surface("connect", "ar"))
    expect(screen.getByDisplayValue("synthetic-private-draft")).toBe(token)
    expect(connect).not.toHaveBeenCalled()
    expect(localStorage.length).toBe(0)
  })
})
