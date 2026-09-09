import {
  act,
  fireEvent,
  render,
  screen,
  waitFor,
  within,
} from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { beforeEach, describe, expect, it, vi } from "vitest"
import type { BusinessClient } from "@/lib/business/client"
import type { BindingList, BindingView } from "@/lib/business/intake"
import { member } from "@/components/business/test-fixtures"
import { SourceSetupDialog } from "./setup"
import { SourcesWorkspace } from "./workspace"
import { admin, binding } from "./test-fixtures"

const intake = vi.fn()
const client: BusinessClient = {
  native: false,
  label: "http://127.0.0.1:4350",
  close: vi.fn(),
  identity: vi.fn(),
  tasks: vi.fn(),
  intake,
}
function scopes(): BindingList {
  return {
    canManageSetup: true,
    setupKinds: ["fireflies"],
    setupDomains: ["engineering"],
    items: [],
    page: 0,
    hasMore: false,
  }
}
function workspace() {
  return render(
    <NextIntlClientProvider locale="en" messages={{}} timeZone="UTC">
      <SourcesWorkspace
        client={client}
        actor={member}
        members={[member]}
        active
        onEntryRead={vi.fn()}
        onTask={vi.fn()}
      />
    </NextIntlClientProvider>
  )
}
beforeEach(() => {
  vi.resetAllMocks()
  localStorage.clear()
})

describe("source setup follows the protected response scope", () => {
  it("offers only the returned kind/domain and never infers source read from setup", async () => {
    const view: BindingView = {
      binding: {
        ...binding.binding,
        domain: "engineering",
        capabilities: {
          read: false,
          import: false,
          triage: false,
          publicationDomains: [],
        },
      },
      admin: { ...admin, domain: "engineering", publicationDomains: [] },
    }
    intake.mockImplementation(async (operation) => {
      if (operation === "bindings/list") return { ...scopes(), items: [view] }
      throw new Error("Unexpected synthetic read without a source grant")
    })
    workspace()
    fireEvent.change(
      await screen.findByRole("combobox", { name: "Source connection" }),
      {
        target: { value: view.binding.id },
      }
    )
    expect(
      await screen.findByText(
        "Ask your workspace administrator to review access and setup."
      )
    ).toBeInTheDocument()
    fireEvent.click(screen.getByRole("button", { name: "Set up a source" }))
    const dialog = within(screen.getByRole("dialog"))
    const kind = dialog.getByRole("combobox", { name: "Source type" })
    expect(within(kind).getAllByRole("option")).toHaveLength(1)
    expect(kind).toHaveValue("fireflies")
    const domain = dialog.getByRole("combobox", { name: "Area of work" })
    expect(within(domain).getAllByRole("option")).toHaveLength(1)
    expect(domain).toHaveValue("engineering")
    expect(
      dialog.getByRole("checkbox", { name: "Engineering" })
    ).toBeInTheDocument()
    expect(
      dialog.queryByRole("checkbox", { name: "Marketing" })
    ).not.toBeInTheDocument()
    expect(intake.mock.calls.map(([operation]) => operation)).toEqual([
      "bindings/list",
    ])
  })

  it("offers legacy kinds only when the server includes them", async () => {
    intake.mockResolvedValue({
      ...scopes(),
      setupKinds: ["email", "hafidh_testflight"],
    } satisfies BindingList)
    workspace()
    fireEvent.click(
      await screen.findByRole("button", { name: "Set up a source" })
    )
    const kind = screen.getByRole("combobox", { name: "Source type" })
    expect(
      within(kind)
        .getAllByRole("option")
        .map((option) => option.getAttribute("value"))
    ).toEqual(["email", "hafidh_testflight"])
    expect(kind).toHaveValue("email")
    expect(
      screen.getByRole("textbox", { name: "Existing inbox number" })
    ).toBeInTheDocument()
    expect(screen.queryByLabelText("Fireflies API key")).not.toBeInTheDocument()
    fireEvent.change(kind, { target: { value: "hafidh_testflight" } })
    expect(
      screen.getByRole("textbox", {
        name: "Existing Hafidh product identifier",
      })
    ).toBeInTheDocument()
  })

  it.each([
    { setupKinds: undefined, setupDomains: undefined },
    { setupKinds: [], setupDomains: ["engineering"] },
    { setupKinds: ["fireflies"], setupDomains: [] },
    { canManageSetup: false },
  ])(
    "keeps setup unavailable for missing or empty authority %j",
    async (restriction) => {
      intake.mockResolvedValue({ ...scopes(), ...restriction })
      workspace()
      await screen.findByText("No sources available yet.")
      expect(
        screen.queryByRole("button", { name: "Set up a source" })
      ).not.toBeInTheDocument()
    }
  )

  it("retains a key through same-scope refresh but releases it when setup scope shrinks", async () => {
    let response = {
      ...scopes(),
      setupDomains: ["engineering", "marketing"],
    } satisfies BindingList
    intake.mockImplementation(async () => response)
    workspace()
    fireEvent.click(
      await screen.findByRole("button", { name: "Set up a source" })
    )
    fireEvent.change(screen.getByLabelText("Fireflies API key"), {
      target: { value: "SYNTHETIC_PRIVATE_SETUP_KEY" },
    })
    await act(async () => window.dispatchEvent(new Event("focus")))
    expect(screen.getByLabelText("Fireflies API key")).toHaveValue(
      "SYNTHETIC_PRIVATE_SETUP_KEY"
    )
    response = { ...response, setupDomains: ["engineering"] }
    await act(async () => window.dispatchEvent(new Event("focus")))
    await waitFor(() =>
      expect(screen.queryByRole("dialog")).not.toBeInTheDocument()
    )
    fireEvent.click(screen.getByRole("button", { name: "Set up a source" }))
    expect(screen.getByLabelText("Fireflies API key")).toHaveValue("")
    expect(JSON.stringify(localStorage)).not.toContain(
      "SYNTHETIC_PRIVATE_SETUP_KEY"
    )
    expect(
      intake.mock.calls.every(([operation]) => operation === "bindings/list")
    ).toBe(true)
  })

  it("requires removal of a saved destination outside current setup authority before updating", async () => {
    intake.mockImplementation(async (operation, input) => {
      if (operation === "grants/list")
        return { items: [], page: 0, hasMore: false }
      if (operation === "bindings/update")
        return { ...admin, publicationDomains: input.publicationDomains }
      throw new Error("Unexpected synthetic setup operation")
    })
    render(
      <NextIntlClientProvider locale="en" messages={{}} timeZone="UTC">
        <SourceSetupDialog
          client={client}
          initial={{
            ...admin,
            publicationDomains: ["marketing", "engineering"],
          }}
          setupKinds={["fireflies"]}
          setupDomains={["marketing"]}
          members={[member]}
          onClose={vi.fn()}
          onChanged={vi.fn()}
        />
      </NextIntlClientProvider>
    )
    await screen.findByText("Nobody has an explicit grant yet.")
    expect(screen.getByRole("button", { name: "Save changes" })).toBeDisabled()
    expect(
      screen.getByText(
        "Remove destinations outside your current setup access before saving."
      )
    ).toBeInTheDocument()
    fireEvent.click(screen.getByRole("checkbox", { name: "Engineering" }))
    expect(
      screen.queryByRole("checkbox", { name: "Engineering" })
    ).not.toBeInTheDocument()
    fireEvent.click(screen.getByRole("button", { name: "Save changes" }))
    await waitFor(() =>
      expect(intake).toHaveBeenCalledWith(
        "bindings/update",
        expect.objectContaining({
          expectedRevision: 1,
          publicationDomains: ["marketing"],
          retainedTaskText: true,
        })
      )
    )
  })
})
