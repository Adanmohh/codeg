import { type ReactNode } from "react"
import {
  act,
  fireEvent,
  render,
  screen,
  waitFor,
  within,
} from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import userEvent from "@testing-library/user-event"
import { beforeEach, describe, expect, it, vi } from "vitest"
import { BusinessError, type BusinessClient } from "@/lib/business/client"
import { CreateTask } from "./task-form"
import { TaskDetailDialog } from "./task-detail"
import { OneTimeCredential, People } from "./people"
import { BusinessWorkspace } from "./workspace"
import { ActivityList } from "./activity"
import { context, detail, member } from "./test-fixtures"

vi.mock("@/components/i18n-provider", () => ({
  useAppI18n: () => ({ appLocale: "en", setLanguageSettings: vi.fn() }),
}))
vi.mock("next-themes", () => ({
  useTheme: () => ({ theme: "light", setTheme: vi.fn() }),
}))
const tasks = vi.fn()
const identity = vi.fn()
const client: BusinessClient = {
  native: false,
  label: "http://127.0.0.1:4340",
  close: vi.fn(),
  execution: () => {
    throw new Error("Execution is outside this fixture")
  },
  identity,
  tasks,
  intake: vi.fn(),
}
function wrapper(children: ReactNode, locale: "en" | "ar" = "en") {
  return (
    <NextIntlClientProvider locale={locale} messages={{}} timeZone="UTC">
      {children}
    </NextIntlClientProvider>
  )
}
beforeEach(() => {
  vi.clearAllMocks()
  localStorage.clear()
  tasks.mockImplementation(async (operation) =>
    operation === "list"
      ? { tasks: [detail().task], page: 0, hasMore: false, canCreate: true }
      : detail()
  )
  identity.mockImplementation(async (operation) =>
    operation === "context" ? context : [member]
  )
})

describe("visual business task workflow", () => {
  it.each([
    ["en", "Find a task", "Refresh"],
    ["ar", "البحث عن مهمة", "تحديث"],
  ] as const)(
    "%s search submits with Enter and tabs to the visible refresh control",
    async (locale, searchName, refreshName) => {
      const user = userEvent.setup()
      render(
        wrapper(
          <BusinessWorkspace
            client={client}
            context={context}
            onContext={vi.fn()}
            disconnect={vi.fn()}
          />,
          locale
        )
      )
      await screen.findByText("Synthetic launch brief")
      const search = screen.getByRole("textbox", { name: searchName })
      const refresh = screen.getByRole("button", { name: refreshName })
      await waitFor(() => expect(refresh).not.toBeDisabled())
      await user.click(search)
      await user.tab()
      expect(refresh).toHaveFocus()
      await user.click(search)
      await user.type(search, "  welcome  ")
      await user.keyboard("{Enter}")
      await waitFor(() =>
        expect(tasks).toHaveBeenLastCalledWith(
          "list",
          expect.objectContaining({ query: "welcome", view: "mine" })
        )
      )
    }
  )
  it.each([false, true])(
    "source entrustment requires legacyOperator=%s and linking stays a separate revisioned action",
    async (legacyOperator) => {
      const agent = {
        ...member,
        id: "44444444-4444-4444-8444-444444444444",
        kind: "agent" as const,
        role: "member" as const,
        displayName: "Synthetic brief agent",
      }
      const task = detail()
      task.task.assigneeId = agent.id
      task.task.capabilities.linkExecution = true
      tasks.mockImplementation(async (operation) => ({
        ...task,
        task: {
          ...task.task,
          revision: operation === "entrust-execution" ? 2 : 3,
        },
      }))
      render(
        wrapper(
          <TaskDetailDialog
            taskId={task.task.id}
            initial={task}
            client={client}
            actor={member}
            members={[member, agent]}
            legacyOperator={legacyOperator}
            onClose={vi.fn()}
            onChanged={vi.fn()}
          />
        )
      )
      fireEvent.click(screen.getByText("Engineering detail"))
      expect(
        screen.queryByText("Authorize this execution source") !== null
      ).toBe(legacyOperator)
      if (!legacyOperator) {
        expect(tasks).not.toHaveBeenCalled()
        return
      }
      const entrust = screen.getByRole("button", {
        name: "Authorize this execution source",
      })
      expect(entrust).toBeDisabled()
      fireEvent.change(
        screen.getByLabelText("Existing engineering task number"),
        { target: { value: "17" } }
      )
      fireEvent.click(screen.getByLabelText(/I confirm this execution belongs/))
      fireEvent.click(entrust)
      await waitFor(() =>
        expect(tasks).toHaveBeenCalledWith("entrust-execution", {
          taskId: task.task.id,
          expectedRevision: 1,
          workTaskId: 17,
        })
      )
      expect(tasks).toHaveBeenCalledTimes(1)
      expect(entrust).toBeDisabled()
      fireEvent.click(
        screen.getByRole("button", { name: "Link existing engineering work" })
      )
      await waitFor(() =>
        expect(tasks).toHaveBeenCalledWith("link-execution", {
          taskId: task.task.id,
          expectedRevision: 2,
          workTaskId: 17,
        })
      )
      expect(tasks).toHaveBeenCalledTimes(2)
    }
  )
  it("shows public note/review content as text and ignores unknown activity payload fields", () => {
    const saved = detail()
    const actor = {
      id: member.id,
      displayName: member.displayName,
      kind: member.kind,
    }
    saved.activity = [
      {
        id: "synthetic-note",
        revision: 2,
        kind: "note",
        actor,
        payload: {
          body: "<script>Synthetic public note</script>",
          providerToken: "unrecognized-private-field",
        },
        createdAt: member.createdAt,
      },
      {
        id: "synthetic-review",
        revision: 3,
        kind: "reviewed",
        actor,
        payload: {
          decision: "return",
          comment: "Please clarify the audience.",
        },
        createdAt: member.createdAt,
      },
    ]
    const { container } = render(
      wrapper(<ActivityList detail={saved} members={[member]} />)
    )
    expect(
      screen.getByText("<script>Synthetic public note</script>")
    ).toBeVisible()
    expect(screen.getByText("Please clarify the audience.")).toBeVisible()
    expect(container.querySelector("script")).toBeNull()
    expect(container.textContent).not.toContain("unrecognized-private-field")
  })
  it("creates human-only work with an exact calendar date and explicit null optional roles", async () => {
    const created = vi.fn()
    render(
      wrapper(
        <CreateTask
          client={client}
          actor={member}
          members={[member]}
          onClose={vi.fn()}
          onCreated={created}
        />
      )
    )
    fireEvent.change(screen.getByLabelText("Task title"), {
      target: { value: "Confirm the launch brief" },
    })
    fireEvent.change(screen.getByLabelText("Due date"), {
      target: { value: "2026-01-01" },
    })
    fireEvent.click(screen.getByRole("button", { name: "Create task" }))
    await waitFor(() => expect(created).toHaveBeenCalledOnce())
    expect(tasks).toHaveBeenCalledWith("create", {
      title: "Confirm the launch brief",
      notes: "",
      domain: "marketing",
      priority: "normal",
      dueDate: "2026-01-01",
      ownerId: member.id,
      assigneeId: null,
      reviewerId: null,
    })
    expect(tasks.mock.calls).toHaveLength(1)
    expect(identity).not.toHaveBeenCalled()
  })
  it("keeps a creation draft through locale changes and protects it on dismissal", () => {
    const close = vi.fn()
    const element = (
      <CreateTask
        client={client}
        actor={member}
        members={[member]}
        onClose={close}
        onCreated={vi.fn()}
      />
    )
    const { rerender } = render(wrapper(element))
    fireEvent.change(screen.getByLabelText("Brief"), {
      target: { value: "Private unsubmitted business brief" },
    })
    rerender(wrapper(element, "ar"))
    expect(screen.getByLabelText("وصف العمل")).toHaveValue(
      "Private unsubmitted business brief"
    )
    fireEvent.click(screen.getByRole("button", { name: "إغلاق" }))
    expect(
      screen.getByRole("dialog", { name: "تجاهل هذه التغييرات؟" })
    ).toBeVisible()
    expect(close).not.toHaveBeenCalled()
    expect(JSON.stringify(localStorage)).not.toContain("Private unsubmitted")
  })
  it("locks a stale metadata draft until the user loads and explicitly adopts the current revision", async () => {
    const original = detail()
    const current = detail()
    current.task.revision = 3
    current.task.status = "review"
    current.task.title = "Changed by another person"
    tasks
      .mockRejectedValueOnce(new BusinessError("conflict"))
      .mockResolvedValueOnce(current)
      .mockImplementation(async (_operation, input) => ({
        ...current,
        task: { ...current.task, ...input, revision: 4 },
      }))
    render(
      wrapper(
        <TaskDetailDialog
          taskId={original.task.id}
          initial={original}
          client={client}
          actor={member}
          members={[member]}
          onClose={vi.fn()}
          onChanged={vi.fn()}
        />
      )
    )
    fireEvent.click(screen.getByRole("button", { name: "Edit task" }))
    fireEvent.change(screen.getByLabelText("Task title"), {
      target: { value: "My pending title" },
    })
    fireEvent.click(screen.getByRole("button", { name: "Save changes" }))
    await screen.findByText("This task changed while you were editing.")
    expect(screen.getByRole("button", { name: "Save changes" })).toBeDisabled()
    expect(screen.getByLabelText("Task title")).toHaveValue("My pending title")
    fireEvent.click(screen.getByRole("button", { name: "Load current task" }))
    await screen.findByText("Changed by another person")
    const base = screen.getByRole("group", {
      name: "Your draft's base version",
    })
    expect(within(base).getByText("To do")).toBeVisible()
    expect(within(base).getByText("Revision 1")).toBeVisible()
    const saved = screen.getByRole("region", {
      name: "Current saved version",
    })
    expect(within(saved).getByText("Review")).toBeVisible()
    expect(within(saved).getByText("Revision 3")).toBeVisible()
    expect(screen.getByRole("button", { name: "Save changes" })).toBeDisabled()
    fireEvent.click(screen.getByRole("button", { name: "Save changes" }))
    expect(tasks).toHaveBeenCalledTimes(2)
    fireEvent.click(
      screen.getByRole("button", { name: "Use my draft with this version" })
    )
    expect(
      screen.queryByRole("group", { name: "Your draft's base version" })
    ).toBeNull()
    expect(screen.getByText("Revision 3")).toBeVisible()
    fireEvent.click(screen.getByRole("button", { name: "Save changes" }))
    await waitFor(() =>
      expect(tasks).toHaveBeenLastCalledWith(
        "update",
        expect.objectContaining({
          expectedRevision: 3,
          title: "My pending title",
          dueDate: "2026-09-08",
        })
      )
    )
  })
  it("requires a new human confirmation after comparing and adopting a changed review", async () => {
    const original = detail()
    original.task.status = "review"
    original.task.revision = 7
    original.task.capabilities.review = true
    const current = {
      ...original,
      task: { ...original.task, revision: 9 },
    }
    tasks
      .mockRejectedValueOnce(new BusinessError("conflict"))
      .mockResolvedValueOnce(current)
      .mockResolvedValueOnce({
        ...current,
        task: { ...current.task, status: "done", revision: 10 },
      })
    render(
      wrapper(
        <TaskDetailDialog
          taskId={original.task.id}
          initial={original}
          client={client}
          actor={member}
          members={[member]}
          onClose={vi.fn()}
          onChanged={vi.fn()}
        />
      )
    )
    const confirmation = screen.getByLabelText(
      "I have reviewed this task and its current deliverable."
    )
    fireEvent.click(confirmation)
    fireEvent.click(screen.getByRole("button", { name: "Accept work" }))
    await screen.findByText("This task changed while you were editing.")
    fireEvent.click(screen.getByRole("button", { name: "Load current task" }))
    const saved = await screen.findByRole("region", {
      name: "Current saved version",
    })
    expect(within(saved).getByText("Revision 9")).toBeVisible()
    expect(screen.getByRole("button", { name: "Accept work" })).toBeDisabled()
    fireEvent.click(
      screen.getByRole("button", { name: "Use my draft with this version" })
    )
    expect(confirmation).not.toBeChecked()
    expect(screen.getByRole("button", { name: "Accept work" })).toBeDisabled()
    expect(tasks).toHaveBeenCalledTimes(2)
    fireEvent.click(confirmation)
    fireEvent.click(screen.getByRole("button", { name: "Accept work" }))
    await waitFor(() =>
      expect(tasks).toHaveBeenLastCalledWith("review", {
        taskId: original.task.id,
        expectedRevision: 9,
        decision: "accept",
        comment: "",
      })
    )
    expect(tasks).toHaveBeenCalledTimes(3)
  })
  it("completion requires the separate human review operation on the displayed revision", async () => {
    const review = detail()
    review.task.status = "review"
    review.task.revision = 7
    review.task.capabilities.review = true
    render(
      wrapper(
        <TaskDetailDialog
          taskId={review.task.id}
          initial={review}
          client={client}
          actor={member}
          members={[member]}
          onClose={vi.fn()}
          onChanged={vi.fn()}
        />
      )
    )
    expect(screen.getByRole("button", { name: "Accept work" })).toBeDisabled()
    fireEvent.click(
      screen.getByLabelText(
        "I have reviewed this task and its current deliverable."
      )
    )
    fireEvent.click(screen.getByRole("button", { name: "Accept work" }))
    await waitFor(() =>
      expect(tasks).toHaveBeenCalledWith("review", {
        taskId: review.task.id,
        expectedRevision: 7,
        decision: "accept",
        comment: "",
      })
    )
    expect(
      tasks.mock.calls.some(
        ([operation, input]) =>
          operation === "progress" && input.status === "done"
      )
    ).toBe(false)
  })
  it.each([false, true])(
    "engineering entry uses only server legacyOperator=%s, even for an owner with engineering access",
    async (legacyOperator) => {
      render(
        wrapper(
          <BusinessWorkspace
            client={client}
            context={{
              ...context,
              capabilities: { ...context.capabilities, legacyOperator },
            }}
            onContext={vi.fn()}
            disconnect={vi.fn()}
          />
        )
      )
      await screen.findByText("Synthetic launch brief")
      expect(
        screen.queryByRole("link", { name: "Open engineering workspace" }) !==
          null
      ).toBe(legacyOperator)
    }
  )
  it("viewer task detail remains readable with no contribution or review controls", () => {
    const readOnly = detail()
    for (const key of Object.keys(
      readOnly.task.capabilities
    ) as (keyof typeof readOnly.task.capabilities)[])
      readOnly.task.capabilities[key] = false
    render(
      wrapper(
        <TaskDetailDialog
          taskId={readOnly.task.id}
          initial={readOnly}
          client={client}
          actor={{ ...member, role: "viewer" }}
          members={[member]}
          onClose={vi.fn()}
          onChanged={vi.fn()}
        />
      )
    )
    expect(
      screen.getByText("Confirm the audience and success criteria.")
    ).toBeVisible()
    for (const name of [
      "Edit task",
      "Accept work",
      "Add note",
      "Start work",
      "Cancel task",
    ])
      expect(screen.queryByRole("button", { name })).not.toBeInTheDocument()
    expect(tasks).not.toHaveBeenCalled()
  })
  it("keeps a one-time credential out of the DOM and storage until deliberate reveal/copy", async () => {
    const token = "synthetic-unit-test-secret"
    const writeText = vi.fn().mockResolvedValue(undefined)
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText },
    })
    const { container } = render(
      wrapper(<OneTimeCredential token={token} onClose={vi.fn()} />)
    )
    expect(container.ownerDocument.body.innerHTML).not.toContain(token)
    expect(JSON.stringify(localStorage)).not.toContain(token)
    await act(async () =>
      fireEvent.click(screen.getByRole("button", { name: "Copy token" }))
    )
    expect(writeText).toHaveBeenCalledWith(token)
    expect(container.ownerDocument.body.innerHTML).not.toContain(token)
    fireEvent.click(screen.getByRole("button", { name: "Reveal token" }))
    expect(screen.getByLabelText("Personal access token")).toHaveValue(token)
  })
  it("protects the original owner from member edits/revocation while allowing owner credential issuance", () => {
    const owner = { ...member, operatorOwner: true }
    render(
      wrapper(
        <People
          client={client}
          context={{ ...context, member: owner }}
          members={[owner]}
          reload={vi.fn()}
        />
      )
    )
    fireEvent.click(screen.getByRole("button", { name: /Synthetic owner/ }))
    const dialog = screen.getByRole("dialog")
    expect(within(dialog).getByLabelText("Display name")).toBeDisabled()
    expect(
      within(dialog).queryByRole("button", { name: "Revoke member access" })
    ).toBeNull()
    expect(
      within(dialog).getByRole("button", {
        name: "Create personal access token",
      })
    ).toBeVisible()
  })
})
