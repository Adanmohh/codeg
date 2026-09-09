import {
  fireEvent,
  render,
  screen,
  waitFor,
  within,
} from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { beforeEach, describe, expect, it, vi } from "vitest"
import type { BusinessClient } from "@/lib/business/client"
import type { TenantSettingsView } from "@/lib/business/settings"
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
const identity = vi.fn()
let savedSettings: TenantSettingsView
const client: BusinessClient = {
  native: false,
  label: "http://127.0.0.1:4350",
  close: vi.fn(),
  execution: () => {
    throw new Error("Execution is outside this fixture")
  },
  identity,
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
  savedSettings = {
    organizationId: member.organizationId,
    revision: 1,
    settings: {
      displayName: context.organization!.name,
      palette: "neutral",
      workspaceLayout: "split",
      defaultWorkArea: "tasks",
    },
  }
  identity.mockImplementation(async (operation, input) => {
    if (operation === "context") return context
    if (operation === "settings/get") return savedSettings
    if (operation === "settings/update") {
      savedSettings = {
        ...savedSettings,
        revision: savedSettings.revision + 1,
        settings: input.settings,
      }
      return savedSettings
    }
    return [member]
  })
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
  it("applies saved tenant appearance and stacked panes without replacing an open task draft or global theme", async () => {
    const rootTheme = document.documentElement.getAttribute("data-theme")
    const view = render(workspace())
    await openFirst()
    const brief = screen.getByRole("textbox", { name: "Brief" })
    fireEvent.change(brief, {
      target: { value: "Synthetic preserved scoped draft" },
    })
    fireEvent.click(
      screen.getByRole("button", { name: "Workspace appearance" })
    )
    fireEvent.change(await screen.findByLabelText("Workspace display name"), {
      target: { value: "Synthetic renamed workspace" },
    })
    fireEvent.change(screen.getByLabelText("Workspace colors"), {
      target: { value: "violet" },
    })
    fireEvent.change(screen.getByLabelText("Preferred pane layout"), {
      target: { value: "stacked" },
    })
    fireEvent.click(
      screen.getByRole("button", { name: "Save workspace defaults" })
    )
    await screen.findByText("Workspace defaults saved.")
    expect(identity).toHaveBeenCalledWith("settings/update", {
      expectedRevision: 1,
      settings: {
        displayName: "Synthetic renamed workspace",
        palette: "violet",
        workspaceLayout: "stacked",
        defaultWorkArea: "tasks",
      },
    })
    expect(
      view.container.querySelector("[data-business-appearance]")
    ).toHaveAttribute("data-theme", "violet")
    expect(document.documentElement.getAttribute("data-theme")).toBe(rootTheme)
    fireEvent.click(screen.getByRole("tab", { name: first.task.title }))
    expect(screen.getByRole("textbox", { name: "Brief" })).toBe(brief)
    fireEvent.click(screen.getByRole("button", { name: "Show stacked panes" }))
    const pane = screen.getByRole("separator", { name: "Resize work panes" })
    expect(pane).toHaveAttribute("data-panel-group-direction", "vertical")
    fireEvent.click(
      screen.getByRole("button", { name: "Arrange panes side by side" })
    )
    expect(screen.getByRole("separator")).toHaveAttribute(
      "data-panel-group-direction",
      "horizontal"
    )
    expect(brief).toHaveValue("Synthetic preserved scoped draft")
    fireEvent.click(
      screen.getByRole("button", { name: `Close tab: ${first.task.title}` })
    )
    expect(screen.getByRole("dialog")).toHaveAttribute("data-theme", "violet")
    fireEvent.click(screen.getByRole("button", { name: "Keep editing" }))
    expect(brief).toHaveValue("Synthetic preserved scoped draft")
  })

  it("asks before leaving the private workspace and keeps all drafts when cancelled", async () => {
    render(workspace())
    await openFirst()
    const brief = screen.getByRole("textbox", { name: "Brief" })
    fireEvent.change(brief, {
      target: { value: "Synthetic cancelled switch draft" },
    })
    fireEvent.click(screen.getByRole("button", { name: "Disconnect" }))
    const dialog = screen.getByRole("dialog", { name: "Leave this workspace?" })
    fireEvent.click(
      within(dialog).getByRole("button", { name: "Stay in this workspace" })
    )
    expect(screen.getByRole("textbox", { name: "Brief" })).toBe(brief)
    expect(brief).toHaveValue("Synthetic cancelled switch draft")
    expect(client.close).not.toHaveBeenCalled()
  })
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
