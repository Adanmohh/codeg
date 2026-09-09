// Independent synthetic regression at PR29 191f69b3 / product c1e618de.
// Fixture composition follows this pin's workflow.test.tsx/test-fixtures.ts
// (Codeg Apache-2.0); real SourcesWorkspace/SourceReview/CandidateReview render.
import { useCallback, useState } from "react"
import { fireEvent, render, screen, waitFor } from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { describe, expect, it, vi } from "vitest"
import type { BusinessClient } from "@/lib/business/client"
import { member } from "@/components/business/test-fixtures"
import { SourcesWorkspace, type SourceEntry } from "./workspace"
import { binding, candidate, source } from "./test-fixtures"

vi.mock("next-themes", () => ({
  useTheme: () => ({ theme: "light" }),
}))

describe("independent source-entry draft ownership", () => {
  it("guards a task source link before replacing another review's unsaved draft", async () => {
    const a = source()
    const b = source()
    b.source.id = "99999999-9999-4999-8999-999999999999"
    b.source.title = "Synthetic second source"
    const reviewA = candidate(a)
    const reviewB = candidate(b)
    reviewB.candidate.id = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa"
    const intake = vi.fn(async (operation: string, input: Record<string, unknown>) => {
      if (operation === "bindings/list")
        return { canManageSetup: false, items: [binding], page: 0, hasMore: false }
      if (operation === "bindings/status") return binding
      if (operation === "sources/get") return input.sourceId === b.source.id ? b : a
      if (operation === "candidates/list")
        return {
          items: [(input.sourceId === b.source.id ? reviewB : reviewA).candidate],
          page: 0,
          hasMore: false,
        }
      if (operation === "candidates/get")
        return input.candidateId === reviewB.candidate.id ? reviewB : reviewA
      if (operation === "imports/list") return { items: [], page: 0, hasMore: false }
      throw new Error(`Unexpected operation: ${operation}`)
    })
    const client = {
      native: false,
      label: "http://127.0.0.1:4350",
      close: vi.fn(),
      identity: vi.fn(),
      tasks: vi.fn(),
      intake,
    } as unknown as BusinessClient
    function Harness() {
      const [entry, setEntry] = useState<SourceEntry | null>({
        bindingId: binding.binding.id,
        sourceId: a.source.id,
      })
      const consumed = useCallback(() => setEntry(null), [])
      return (
        <NextIntlClientProvider locale="en" messages={{}} timeZone="UTC">
          <button onClick={() => setEntry({ bindingId: binding.binding.id, sourceId: b.source.id })}>
            Open task-linked second source
          </button>
          <SourcesWorkspace
            client={client}
            actor={member}
            members={[member]}
            active
            entry={entry}
            onEntryRead={consumed}
            onTask={vi.fn()}
          />
        </NextIntlClientProvider>
      )
    }
    render(<Harness />)
    fireEvent.click(await screen.findByRole("button", { name: /Synthetic customer brief/ }))
    const brief = await screen.findByRole("textbox", { name: "Brief" })
    fireEvent.change(brief, { target: { value: "Synthetic unsaved source A sentinel" } })
    await waitFor(() => expect(brief).toHaveValue("Synthetic unsaved source A sentinel"))
    fireEvent.click(screen.getByRole("button", { name: "Open task-linked second source" }))
    // This is the same entry seam used by BusinessWorkspace's task onSource.
    // The destination editor must own the discard decision before its key changes.
    expect(await screen.findByRole("dialog", { name: "Leave this source review?" })).toBeInTheDocument()
    expect(screen.getByRole("textbox", { name: "Brief" })).toHaveValue("Synthetic unsaved source A sentinel")
    expect(intake.mock.calls.filter(([name]) => /candidates\/(edit|select|accept|link|discard)$/.test(name))).toHaveLength(0)
  })
})
