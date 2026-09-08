import { fireEvent, render, screen, waitFor } from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { beforeEach, describe, expect, it, vi } from "vitest"
import type { BusinessClient } from "@/lib/business/client"
import { BusinessWorkspace } from "./workspace"
import { context, detail, member } from "./test-fixtures"

let wide = true
vi.mock("@/hooks/use-media-query", () => ({
  useMediaQuery: () => wide,
}))
vi.mock("@/components/i18n-provider", () => ({
  useAppI18n: () => ({ appLocale: "en", setLanguageSettings: vi.fn() }),
}))
vi.mock("next-themes", () => ({
  useTheme: () => ({ theme: "light", setTheme: vi.fn() }),
}))

const first = detail()
const second = detail()
second.task = {
  ...second.task,
  id: "44444444-4444-4444-8444-444444444444",
  title: "Synthetic second brief",
  dueDate: "2028-02-29",
}
const tasks = vi.fn()
const client: BusinessClient = {
  native: false,
  label: "http://127.0.0.1:4350",
  close: vi.fn(),
  identity: vi.fn(async (operation) =>
    operation === "context" ? context : [member]
  ) as BusinessClient["identity"],
  tasks,
  intake: vi.fn(),
}
function workspace(locale: "en" | "ar" = "en") {
  return (
    <NextIntlClientProvider locale={locale} messages={{}} timeZone="UTC">
      <BusinessWorkspace
        client={client}
        context={context}
        onContext={vi.fn()}
        disconnect={vi.fn()}
      />
    </NextIntlClientProvider>
  )
}
async function openFirst() {
  fireEvent.click(
    await screen.findByRole("button", { name: /Synthetic launch brief/ })
  )
  fireEvent.click(await screen.findByRole("button", { name: "Edit task" }))
}
beforeEach(() => {
  vi.clearAllMocks()
  wide = true
  tasks.mockImplementation(async (operation, input) => {
    if (operation === "list")
      return {
        tasks: [first.task, second.task],
        page: 0,
        hasMore: false,
        canCreate: true,
      }
    return input.taskId === second.task.id ? second : first
  })
})

describe("business workbench uses real task seams without legacy shell providers", () => {
  it("keeps two independent drafts and DOM identity across tabs, split, locale and narrow layout", async () => {
    const view = render(workspace())
    await openFirst()
    const original = screen.getByRole("textbox", { name: "Brief" })
    fireEvent.change(original, {
      target: { value: "Synthetic private first draft" },
    })
    fireEvent.click(screen.getByRole("tab", { name: "My work" }))
    expect(original).not.toBeVisible()
    fireEvent.click(
      screen.getByRole("button", { name: /Synthetic second brief/ })
    )
    fireEvent.click(await screen.findByRole("button", { name: "Edit task" }))
    fireEvent.change(screen.getByRole("textbox", { name: "Brief" }), {
      target: { value: "Synthetic private second draft" },
    })
    fireEvent.click(screen.getByRole("tab", { name: first.task.title }))
    expect(screen.getByRole("textbox", { name: "Brief" })).toBe(original)
    expect(original).toHaveValue("Synthetic private first draft")
    fireEvent.click(screen.getByRole("button", { name: "Show side by side" }))
    expect(
      screen.getByRole("separator", { name: "Resize work panes" })
    ).toBeVisible()
    expect(screen.getByRole("textbox", { name: "Brief" })).toBe(original)
    wide = false
    view.rerender(workspace("ar"))
    expect(screen.queryByRole("separator")).toBeNull()
    expect(screen.getByRole("textbox", { name: "وصف العمل" })).toBe(original)
    expect(original).toHaveValue("Synthetic private first draft")
    wide = true
    view.rerender(workspace())
    expect(screen.getByRole("textbox", { name: "Brief" })).toBe(original)
    fireEvent.click(screen.getByRole("tab", { name: second.task.title }))
    expect(screen.getByRole("textbox", { name: "Brief" })).toHaveValue(
      "Synthetic private second draft"
    )
    expect(
      tasks.mock.calls.every(([operation]) =>
        ["list", "get"].includes(operation)
      )
    ).toBe(true)
    expect(client.intake).not.toHaveBeenCalled()
    expect(Object.keys(localStorage)).not.toContain("workspace:tab-groups:v1")
  })

  it("tab close invokes the editor's discard guard; cancellation preserves the draft", async () => {
    render(workspace())
    await openFirst()
    fireEvent.change(screen.getByRole("textbox", { name: "Brief" }), {
      target: { value: "Synthetic unsaved close draft" },
    })
    fireEvent.click(
      screen.getByRole("button", { name: `Close tab: ${first.task.title}` })
    )
    expect(
      screen.getByRole("dialog", { name: "Discard these changes?" })
    ).toBeVisible()
    fireEvent.click(screen.getByRole("button", { name: "Keep editing" }))
    expect(screen.getByRole("textbox", { name: "Brief" })).toHaveValue(
      "Synthetic unsaved close draft"
    )
    const tab = screen.getByRole("tab", { name: first.task.title })
    fireEvent.keyDown(tab, { key: "Delete" })
    fireEvent.click(screen.getByRole("button", { name: "Discard my draft" }))
    await waitFor(() =>
      expect(screen.queryByRole("tab", { name: first.task.title })).toBeNull()
    )
    expect(screen.getByRole("tab", { name: "My work" })).toHaveAttribute(
      "aria-selected",
      "true"
    )
    await waitFor(() =>
      expect(screen.getByRole("tab", { name: "My work" })).toHaveFocus()
    )
    expect(
      tasks.mock.calls.every(([operation]) =>
        ["list", "get"].includes(operation)
      )
    ).toBe(true)
  })

  it.each(["en", "ar"] as const)(
    "%s tabs have manual arrow activation and linked panels",
    async (locale) => {
      render(workspace(locale))
      fireEvent.click(
        await screen.findByRole("button", { name: /Synthetic launch brief/ })
      )
      await screen.findByRole("heading", { name: first.task.title })
      const tab = screen.getByRole("tab", { name: first.task.title })
      tab.focus()
      fireEvent.keyDown(tab, {
        key: locale === "ar" ? "ArrowRight" : "ArrowLeft",
      })
      const collection = screen.getByRole("tab", {
        name: locale === "ar" ? "عملي" : "My work",
      })
      expect(collection).toHaveFocus()
      expect(tab).toHaveAttribute("aria-selected", "true")
      fireEvent.click(collection)
      expect(collection).toHaveAttribute("aria-selected", "true")
      const panel = document.getElementById(
        collection.getAttribute("aria-controls")!
      )
      expect(panel).toHaveAttribute("role", "tabpanel")
      expect(panel).toHaveAttribute("aria-labelledby", collection.id)
      expect(panel).toBeVisible()
    }
  )

  it("renders the same authorized page as a semantic table with the exact calendar date", async () => {
    render(workspace())
    await screen.findByRole("button", { name: /Synthetic launch brief/ })
    fireEvent.click(screen.getByRole("button", { name: "Table" }))
    const table = screen.getByRole("table", { name: "Tasks" })
    expect(table.querySelectorAll("th[scope=col]")).toHaveLength(6)
    expect(
      table.querySelector('time[datetime="2028-02-29"]')
    ).toHaveTextContent("2028-02-29")
    fireEvent.click(screen.getByRole("button", { name: second.task.title }))
    expect(
      await screen.findByRole("heading", { name: second.task.title })
    ).toBeVisible()
    expect(tasks).toHaveBeenCalledWith("get", { taskId: second.task.id })
  })
})
