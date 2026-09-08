import {
  act,
  fireEvent,
  render,
  screen,
  waitFor,
  within,
} from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { describe, expect, it, vi } from "vitest"
import { BusinessError } from "@/lib/business/client"
import type {
  TenantSettingsAccess,
  TenantSettingsView,
} from "@/lib/business/settings"
import { SettingsEditor } from "./settings-editor"

const initial: TenantSettingsView = {
  organizationId: "11111111-1111-4111-8111-111111111111",
  revision: 1,
  settings: {
    displayName: "Synthetic team",
    palette: "neutral",
    workspaceLayout: "split",
    defaultWorkArea: "tasks",
  },
}
function wrapper(
  access: TenantSettingsAccess,
  onApplied = vi.fn(),
  canManage = true,
  locale: "en" | "ar" = "en"
) {
  return (
    <NextIntlClientProvider locale={locale} messages={{}} timeZone="UTC">
      <SettingsEditor
        initial={initial}
        access={access}
        canManage={canManage}
        onApplied={onApplied}
      />
    </NextIntlClientProvider>
  )
}

describe("closed tenant presentation editor", () => {
  it("saves only the exact settings payload with the displayed base revision", async () => {
    const update = vi.fn(async (input) => ({
      ...initial,
      revision: 2,
      settings: input.settings,
    }))
    const applied = vi.fn()
    render(wrapper({ get: vi.fn(), update }, applied))
    fireEvent.change(screen.getByLabelText("Workspace display name"), {
      target: { value: "  Synthetic revised name  " },
    })
    fireEvent.change(screen.getByLabelText("Workspace colors"), {
      target: { value: "blue" },
    })
    fireEvent.change(screen.getByLabelText("Preferred pane layout"), {
      target: { value: "stacked" },
    })
    fireEvent.click(
      screen.getByRole("button", { name: "Save workspace defaults" })
    )
    await screen.findByText("Workspace defaults saved.")
    expect(update).toHaveBeenCalledWith({
      expectedRevision: 1,
      settings: {
        ...initial.settings,
        displayName: "Synthetic revised name",
        palette: "blue",
        workspaceLayout: "stacked",
      },
    })
    expect(applied).toHaveBeenCalledOnce()
    expect(applied.mock.calls[0][0].revision).toBe(2)
  })

  it("read-only projected capability disables the fields and rejects a forced form submit", () => {
    const access = { get: vi.fn(), update: vi.fn() }
    const { container } = render(wrapper(access, vi.fn(), false))
    expect(screen.getByLabelText("Workspace display name")).toBeDisabled()
    expect(
      screen.queryByRole("button", { name: "Save workspace defaults" })
    ).toBeNull()
    fireEvent.submit(container.querySelector("form")!)
    expect(access.update).not.toHaveBeenCalled()
  })

  it.each(["conflict", "offline"] as const)(
    "%s holds the draft until explicit current-version comparison and adoption",
    async (kind) => {
      const saved: TenantSettingsView = {
        ...initial,
        revision: 2,
        settings: {
          ...initial.settings,
          displayName: "Synthetic other manager's saved name",
          palette: "violet",
        },
      }
      const update = vi
        .fn()
        .mockRejectedValueOnce(new BusinessError(kind))
        .mockImplementation(async (input) => ({
          ...initial,
          revision: 3,
          settings: input.settings,
        }))
      const access = { update, get: vi.fn(async () => saved) }
      const applied = vi.fn()
      const view = render(wrapper(access, applied))
      const input = screen.getByLabelText("Workspace display name")
      fireEvent.change(input, { target: { value: "Synthetic retained draft" } })
      fireEvent.click(
        screen.getByRole("button", { name: "Save workspace defaults" })
      )
      fireEvent.click(
        await screen.findByRole("button", { name: "Load current defaults" })
      )
      const current = await screen.findByRole("region", {
        name: "Current saved defaults",
      })
      expect(within(current).getByText("Revision 2")).toBeVisible()
      expect(
        within(current).getByText(saved.settings.displayName)
      ).toBeVisible()
      expect(input).toHaveValue("Synthetic retained draft")
      expect(
        screen.getByRole("button", { name: "Save workspace defaults" })
      ).toBeDisabled()
      expect(applied).not.toHaveBeenCalled()
      view.rerender(wrapper(access, applied, true, "ar"))
      expect(screen.getByLabelText("اسم العرض لمساحة العمل")).toBe(input)
      view.rerender(wrapper(access, applied))
      fireEvent.click(
        screen.getByRole("button", { name: "Use my draft with this version" })
      )
      fireEvent.click(
        screen.getByRole("button", { name: "Save workspace defaults" })
      )
      await screen.findByText("Workspace defaults saved.")
      expect(update).toHaveBeenLastCalledWith({
        expectedRevision: 2,
        settings: {
          ...initial.settings,
          displayName: "Synthetic retained draft",
        },
      })
    }
  )

  it("ignores a successful old-scope response after the editor unmounts", async () => {
    let finish!: (value: TenantSettingsView) => void
    const access = {
      get: vi.fn(),
      update: vi.fn(
        () =>
          new Promise<TenantSettingsView>((resolve) => {
            finish = resolve
          })
      ),
    }
    const applied = vi.fn()
    const view = render(wrapper(access, applied))
    fireEvent.change(screen.getByLabelText("Workspace display name"), {
      target: { value: "Synthetic old scope" },
    })
    fireEvent.click(
      screen.getByRole("button", { name: "Save workspace defaults" })
    )
    view.unmount()
    await act(async () => finish({ ...initial, revision: 2 }))
    expect(applied).not.toHaveBeenCalled()
  })

  it("does not apply or display a response belonging to another organization", async () => {
    const foreign = {
      ...initial,
      organizationId: "22222222-2222-4222-8222-222222222222",
      settings: {
        ...initial.settings,
        displayName: "Synthetic foreign private name",
      },
    }
    const access = { get: vi.fn(), update: vi.fn(async () => foreign) }
    const applied = vi.fn()
    render(wrapper(access, applied))
    fireEvent.change(screen.getByLabelText("Workspace display name"), {
      target: { value: "Synthetic own name" },
    })
    fireEvent.click(
      screen.getByRole("button", { name: "Save workspace defaults" })
    )
    await waitFor(() =>
      expect(screen.getByRole("alert")).toHaveTextContent(
        "Your access does not allow this action."
      )
    )
    expect(screen.queryByText(foreign.settings.displayName)).toBeNull()
    expect(applied).not.toHaveBeenCalled()
  })
})
