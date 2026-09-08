import { act, fireEvent, render, screen, waitFor } from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest"
import BusinessPage from "@/app/business/page"
import { context, detail, member } from "./test-fixtures"

vi.mock("@/components/i18n-provider", () => ({
  useAppI18n: () => ({ appLocale: "en", setLanguageSettings: vi.fn() }),
}))
vi.mock("next-themes", () => ({
  useTheme: () => ({ theme: "light", setTheme: vi.fn() }),
}))
let rejected = false
let revised = false
const requests: string[] = []
const fetcher = vi.fn(async (url: string) => {
  requests.push(url)
  const currentMember = revised
    ? { ...member, revision: 2, role: "viewer" as const }
    : member
  let value: unknown
  if (url.endsWith("/context")) {
    if (rejected)
      return new Response(
        JSON.stringify({
          code: "authentication_failed",
          message: "synthetic internal detail",
        }),
        { status: 401 }
      )
    value = { ...context, member: currentMember }
  } else if (url.endsWith("/members/list")) value = [currentMember]
  else if (url.endsWith("/tasks/list"))
    value = {
      tasks: [detail().task],
      page: 0,
      hasMore: false,
      canCreate: !revised,
    }
  else if (url.endsWith("/tasks/get")) value = detail()
  else throw new Error("Unexpected business test operation")
  return new Response(JSON.stringify(value))
})
beforeEach(() => {
  rejected = false
  revised = false
  requests.length = 0
  vi.clearAllMocks()
  localStorage.clear()
  vi.stubGlobal("fetch", fetcher)
})
afterEach(() => vi.unstubAllGlobals())
async function openDraft() {
  render(
    <NextIntlClientProvider locale="en" messages={{}} timeZone="UTC">
      <BusinessPage />
    </NextIntlClientProvider>
  )
  fireEvent.change(screen.getByLabelText("Workspace address"), {
    target: { value: "http://127.0.0.1:4340" },
  })
  fireEvent.change(screen.getByLabelText("Personal access token"), {
    target: { value: "bdm_synthetic_session" },
  })
  fireEvent.click(screen.getByRole("button", { name: "Connect" }))
  await screen.findByText("Synthetic launch brief")
  fireEvent.click(
    screen.getByRole("button", { name: /Synthetic launch brief/ })
  )
  fireEvent.click(await screen.findByRole("button", { name: "Edit task" }))
  fireEvent.change(screen.getByLabelText("Task title"), {
    target: { value: "Private unsubmitted session draft" },
  })
}
describe("business private session boundary", () => {
  it("clears private edits and bearer on credential rejection, keeping an unrelated ambient operator slot intact", async () => {
    localStorage.setItem("codeg_token", "synthetic-ambient-operator")
    await openDraft()
    rejected = true
    await act(async () => {
      window.dispatchEvent(new Event("focus"))
    })
    await screen.findByLabelText("Personal access token")
    expect(
      screen.queryByDisplayValue("Private unsubmitted session draft")
    ).toBeNull()
    expect(screen.getByLabelText("Personal access token")).toHaveValue("")
    expect(localStorage.getItem("codeg_token")).toBe(
      "synthetic-ambient-operator"
    )
    expect(JSON.stringify(localStorage)).not.toContain("bdm_synthetic_session")
    expect(
      requests.every((url) =>
        url.startsWith("http://127.0.0.1:4340/api/business/")
      )
    ).toBe(true)
  })
  it("membership revision change resets private editing state and reflects new read-only access", async () => {
    await openDraft()
    revised = true
    await act(async () => {
      window.dispatchEvent(new Event("focus"))
    })
    await waitFor(() =>
      expect(
        screen.queryByDisplayValue("Private unsubmitted session draft")
      ).toBeNull()
    )
    await screen.findByText(
      "You can read this work. Your current access does not allow changes."
    )
    expect(screen.queryByRole("button", { name: "Create task" })).toBeNull()
    expect(
      screen.queryByRole("link", { name: "Open engineering workspace" })
    ).toBeNull()
  })
})
