import { useState } from "react"
import { act, fireEvent, render, screen, waitFor } from "@testing-library/react"
import { beforeEach, describe, expect, it, vi } from "vitest"
import { AppI18nProvider, useAppI18n } from "@/components/i18n-provider"
import { AppearanceProvider } from "@/components/appearance-provider"
import { WebConnectionGuard } from "@/components/connection/web-connection-guard"
import { LANGUAGE_SETTINGS_STORAGE_KEY } from "@/lib/i18n"
import { STORAGE_KEY_WORKSPACE_BG_ENABLED } from "@/lib/appearance-script"
import en from "@/i18n/messages/en.json"

const boundary = vi.hoisted(() => ({
  pathname: "/business",
  systemLanguage: vi.fn(),
  readBackground: vi.fn(),
  connection: vi.fn(() => "connected"),
  subscribe: vi.fn(() => () => {}),
}))
vi.mock("next/navigation", () => ({
  usePathname: () => boundary.pathname,
}))
vi.mock("@/lib/api", () => ({
  getSystemLanguageSettings: boundary.systemLanguage,
}))
vi.mock("@/lib/workspace-background", async (original) => ({
  ...(await original<typeof import("@/lib/workspace-background")>()),
  readWorkspaceBackground: boundary.readBackground,
}))
vi.mock("@/lib/transport/web-connection-store", () => ({
  getWebConnectionSnapshot: boundary.connection,
  getWebConnectionServerSnapshot: () => "connected",
  subscribeWebConnection: boundary.subscribe,
  reconnectWebNow: vi.fn(),
}))

function Draft() {
  const [text, setText] = useState("")
  const { appLocale } = useAppI18n()
  return (
    <>
      <label>
        Private draft
        <input value={text} onChange={(event) => setText(event.target.value)} />
      </label>
      <output>{appLocale}</output>
    </>
  )
}

function Surface() {
  return (
    <AppI18nProvider initialLocale="en" initialMessages={en}>
      <AppearanceProvider>
        <WebConnectionGuard />
        <Draft />
      </AppearanceProvider>
    </AppI18nProvider>
  )
}

beforeEach(() => {
  vi.clearAllMocks()
  localStorage.clear()
  boundary.pathname = "/business"
  boundary.systemLanguage.mockResolvedValue({ mode: "manual", language: "en" })
  boundary.readBackground.mockResolvedValue(null)
})

describe("business route provider isolation", () => {
  it.each(["/business", "/business/", "/business.html"])(
    "isolates %s with an ambient operator token and wallpaper enabled",
    async (pathname) => {
      boundary.pathname = pathname
      localStorage.setItem("codeg_token", "synthetic-old-operator-token")
      localStorage.setItem(STORAGE_KEY_WORKSPACE_BG_ENABLED, "1")
      render(<Surface />)
      await screen.findByLabelText("Private draft")
      expect(boundary.systemLanguage).not.toHaveBeenCalled()
      expect(boundary.readBackground).not.toHaveBeenCalled()
      expect(boundary.connection).not.toHaveBeenCalled()
      expect(boundary.subscribe).not.toHaveBeenCalled()
    }
  )

  it("keeps private edits mounted while the local language changes across tabs", async () => {
    render(<Surface />)
    const input = await screen.findByLabelText("Private draft")
    fireEvent.change(input, { target: { value: "Unsubmitted business note" } })
    await act(async () => {
      window.dispatchEvent(
        new StorageEvent("storage", {
          key: LANGUAGE_SETTINGS_STORAGE_KEY,
          newValue: JSON.stringify({ mode: "manual", language: "ar" }),
        })
      )
    })
    await waitFor(() => expect(document.documentElement.dir).toBe("rtl"))
    expect(screen.getByLabelText("Private draft")).toBe(input)
    expect(input).toHaveValue("Unsubmitted business note")
    expect(JSON.stringify(localStorage)).not.toContain(
      "Unsubmitted business note"
    )
    expect(boundary.systemLanguage).not.toHaveBeenCalled()
    expect(boundary.connection).not.toHaveBeenCalled()
  })

  it.each(["/workspace", "/business-other"])(
    "retains operator providers on %s",
    async (pathname) => {
      boundary.pathname = pathname
      localStorage.setItem(STORAGE_KEY_WORKSPACE_BG_ENABLED, "1")
      render(<Surface />)
      await screen.findByLabelText("Private draft")
      expect(boundary.systemLanguage).toHaveBeenCalledOnce()
      expect(boundary.readBackground).toHaveBeenCalled()
      expect(boundary.subscribe).toHaveBeenCalled()
    }
  )
})
