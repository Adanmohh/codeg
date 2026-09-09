import { useCallback, useState } from "react"
import { act, fireEvent, render, screen, waitFor } from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { beforeEach, describe, expect, it, vi } from "vitest"
import { BusinessError, type BusinessClient } from "@/lib/business/client"
import type { BindingView, CandidateDetail } from "@/lib/business/intake"
import { member } from "@/components/business/test-fixtures"
import { SourcesWorkspace, type SourceEntry } from "./workspace"
import { binding, candidate, source } from "./test-fixtures"

const intake = vi.fn()
const readBinding = vi.fn()
const consumed = vi.fn()
const client: BusinessClient = {
  native: false,
  label: "http://127.0.0.1:4350",
  close: vi.fn(),
  identity: vi.fn(),
  tasks: vi.fn(),
  intake,
}
const first = source()
const second = source()
second.source.id = "99999999-9999-4999-8999-999999999999"
second.source.title = "Synthetic source B"
const third = source()
third.source.id = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa"
third.source.title = "Synthetic source C"
const records = [first, second, third]
let prepared: CandidateDetail
let write: (input: {
  task: CandidateDetail["candidate"]["draft"]
}) => Promise<CandidateDetail>
function LinkedSources() {
  const [entry, setEntry] = useState<SourceEntry | null>({
    sourceId: first.source.id,
    bindingId: binding.binding.id,
  })
  const consume = useCallback(() => {
    consumed()
    setEntry(null)
  }, [])
  return (
    <>
      {records.map((record, index) => (
        <button
          key={record.source.id}
          onClick={() =>
            setEntry({
              sourceId: record.source.id,
              bindingId: binding.binding.id,
            })
          }
        >
          Task source {index + 1}
        </button>
      ))}
      <SourcesWorkspace
        client={client}
        actor={member}
        members={[member]}
        active
        entry={entry}
        onEntryRead={consume}
        onTask={vi.fn()}
      />
    </>
  )
}
async function openDraft() {
  render(
    <NextIntlClientProvider locale="en" messages={{}} timeZone="UTC">
      <LinkedSources />
    </NextIntlClientProvider>
  )
  fireEvent.click(
    await screen.findByRole("button", { name: /Synthetic customer brief/ })
  )
  fireEvent.change(await screen.findByRole("textbox", { name: "Brief" }), {
    target: {
      value: "Private A brief retained through pending source navigation",
    },
  })
}
beforeEach(() => {
  vi.resetAllMocks()
  // Each test starts with a fresh server observation, not a reused clock deadline.
  first.source.accessValidUntil = new Date(Date.now() + 300000).toISOString()
  prepared = candidate(first)
  readBinding.mockResolvedValue(binding)
  write = async (input) => {
    prepared = {
      ...prepared,
      candidate: { ...prepared.candidate, revision: 4, draft: input.task },
    }
    return prepared
  }
  intake.mockImplementation(async (operation, input) => {
    if (operation === "bindings/list")
      return {
        canManageSetup: false,
        setupKinds: [],
        setupDomains: [],
        items: [binding],
        page: 0,
        hasMore: false,
      }
    if (operation === "bindings/status") return readBinding(input)
    if (operation === "sources/get")
      return records.find((record) => record.source.id === input.sourceId)
    if (operation === "sources/list")
      return {
        items: records.map((record) => record.source),
        page: 0,
        hasMore: false,
      }
    if (operation === "candidates/list")
      return {
        items: input.sourceId === first.source.id ? [prepared.candidate] : [],
        page: 0,
        hasMore: false,
      }
    if (operation === "candidates/get") return prepared
    if (operation === "candidates/edit") return write(input)
    if (operation === "imports/list")
      return { items: [], page: 0, hasMore: false }
    throw new Error("Unexpected synthetic operation")
  })
})

describe("linked source pending request and operation recovery", () => {
  it.each(["success", "failure"] as const)(
    "ignores an old binding %s when the current source is explicitly requested again",
    async (outcome) => {
      await openDraft()
      let finish!: (value: BindingView) => void
      let fail!: (reason: unknown) => void
      readBinding.mockImplementationOnce(
        () =>
          new Promise<BindingView>((resolve, reject) => {
            finish = resolve
            fail = reject
          })
      )
      fireEvent.click(screen.getByRole("button", { name: "Task source 2" }))
      fireEvent.click(screen.getByRole("button", { name: "Discard my draft" }))
      await waitFor(() => expect(readBinding).toHaveBeenCalledTimes(2))
      fireEvent.click(screen.getByRole("button", { name: "Task source 1" }))
      await act(async () => {
        if (outcome === "success") finish(binding)
        else fail(new BusinessError("forbidden"))
      })
      expect(await screen.findByRole("textbox", { name: "Brief" })).toHaveValue(
        "Private A brief retained through pending source navigation"
      )
      expect(intake).not.toHaveBeenCalledWith("sources/get", {
        sourceId: second.source.id,
      })
      expect(screen.queryByRole("dialog")).not.toBeInTheDocument()
      expect(screen.queryByRole("alert")).not.toBeInTheDocument()
      expect(consumed).toHaveBeenCalledTimes(3)
    }
  )

  it("keeps the original editor while a newer task-source request awaits confirmation", async () => {
    await openDraft()
    let finish!: (value: BindingView) => void
    readBinding.mockImplementationOnce(
      () =>
        new Promise<BindingView>((resolve) => {
          finish = resolve
        })
    )
    fireEvent.click(screen.getByRole("button", { name: "Task source 2" }))
    fireEvent.click(screen.getByRole("button", { name: "Discard my draft" }))
    await waitFor(() => expect(readBinding).toHaveBeenCalledTimes(2))
    fireEvent.click(screen.getByRole("button", { name: "Task source 3" }))
    await screen.findByRole("dialog")
    await act(async () => finish(binding))
    fireEvent.click(screen.getByRole("button", { name: "Keep editing" }))
    expect(await screen.findByRole("textbox", { name: "Brief" })).toHaveValue(
      "Private A brief retained through pending source navigation"
    )
    expect(intake).not.toHaveBeenCalledWith("sources/get", {
      sourceId: second.source.id,
    })
    expect(intake).not.toHaveBeenCalledWith("sources/get", {
      sourceId: third.source.id,
    })
    await act(async () => window.dispatchEvent(new Event("focus")))
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument()
    expect(consumed).toHaveBeenCalledTimes(3)
  })

  it("keeps a busy or unknown write until the exact operation is recovered before leaving", async () => {
    await openDraft()
    let fail!: (reason: unknown) => void
    const save = write
    write = () =>
      new Promise<CandidateDetail>((_resolve, reject) => {
        fail = reject
      })
    fireEvent.click(screen.getByRole("button", { name: "Save private draft" }))
    await waitFor(() =>
      expect(intake).toHaveBeenCalledWith("candidates/edit", expect.anything())
    )
    fireEvent.click(screen.getByRole("button", { name: "Task source 2" }))
    expect(
      screen.getByRole("button", { name: "Discard my draft" })
    ).toBeDisabled()
    fireEvent.click(screen.getByRole("button", { name: "Keep editing" }))
    await act(async () => fail(new BusinessError("offline")))
    await screen.findByText(/The decision could not be confirmed/)
    fireEvent.click(screen.getByRole("button", { name: "Task source 2" }))
    expect(
      screen.getByRole("button", { name: "Discard my draft" })
    ).toBeDisabled()
    fireEvent.click(screen.getByRole("button", { name: "Keep editing" }))
    expect(intake).not.toHaveBeenCalledWith("sources/get", {
      sourceId: second.source.id,
    })
    write = save
    fireEvent.click(
      screen.getByRole("button", { name: "Retry this exact operation" })
    )
    await waitFor(() =>
      expect(
        intake.mock.calls.filter(
          ([operation]) => operation === "candidates/edit"
        )
      ).toHaveLength(2)
    )
    await waitFor(() =>
      expect(
        screen.queryByText(/The decision could not be confirmed/)
      ).not.toBeInTheDocument()
    )
    await screen.findByRole("button", { name: "Accept into shared work" })
    const inputs = intake.mock.calls
      .filter(([operation]) => operation === "candidates/edit")
      .map(([, input]) => input)
    expect(inputs[1]).toEqual(inputs[0])
    fireEvent.click(screen.getByRole("button", { name: "Task source 2" }))
    await screen.findByRole("heading", { name: second.source.title })
    expect(consumed).toHaveBeenCalledTimes(4)
    expect(client.tasks).not.toHaveBeenCalled()
  })
})
