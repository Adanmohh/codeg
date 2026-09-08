import { fireEvent, render, screen, waitFor } from "@testing-library/react"
import { beforeEach, expect, it, vi } from "vitest"
import { BugWorkflowPage } from "./bug-workflow-page"
import { WorkbenchRouteProvider } from "@/contexts/workbench-route-context"
import { useRemoteConnection } from "@/contexts/remote-connection-context"
import { useTasksView } from "@/contexts/tasks-view-context"
import { intake } from "@/lib/ops-intake/api"
import type {
  Detail,
  EvidenceField,
  PreparedIssue,
  Product,
} from "@/lib/ops-intake/types"

vi.mock("@/contexts/remote-connection-context", () => ({
  useRemoteConnection: vi.fn(() => null),
}))
vi.mock("@/contexts/tasks-view-context", () => ({ useTasksView: vi.fn() }))
vi.mock("@/lib/ops-intake/api", () => ({
  intake: {
    status: vi.fn(),
    list: vi.fn(),
    detail: vi.fn(),
    refresh: vi.fn(),
    save: vi.fn(),
    attach: vi.fn(),
    prepare: vi.fn(),
    approve: vi.fn(),
    deny: vi.fn(),
    reconcile: vi.fn(),
    fix: vi.fn(),
  },
  intakeError: () => "The request could not be confirmed. Reload status.",
}))
const product: Product = {
  binding: {
    product_id: "synthetic-hafidh",
    folder_id: 1,
    app_id: "fixture-app",
    installation_id: 7,
    repository_id: 11,
    full_name: "owner/repo",
    enabled: true,
  },
  origin: "https://synthetic.invalid",
  intake_credential_present: true,
  app_key_present: true,
}
const source = {
  product_id: product.binding.product_id,
  ulid: "01ARZ3NDEKTSV4RRFFQ69G5FAV",
}
function detail(): Detail {
  return {
    snapshot: {
      record: {
        source_ref: { ...source, source: "testflight", external_id: "asc-1" },
        source_revision: "a".repeat(64),
        fetched_at: new Date().toISOString(),
        title: "Synthetic audio report",
        description: "Synthetic report <script>window.unsafe = true</script>",
        source_status: "pending",
        submitted_at: null,
        source_updated_at: new Date().toISOString(),
        device: "iPhone",
        os_version: "18",
        app_version: null,
        build_number: "42",
        platform: "IOS",
        locale: "en",
        screenshots: [],
        triage: {
          seeded_tags: ["audio"],
          seeded_severity: "medium",
          source_tags: [],
          source_severity: "medium",
        },
      },
      verified_at: Math.floor(Date.now() / 1000),
      error: null,
    },
    draft: {
      id: "synthetic-draft",
      revision: 1,
      source_revision: "a".repeat(64),
      title: "Synthetic audio report",
      summary: "Synthetic playback stops",
      labels: [],
      confirmed_severity: null,
      proofs: {},
      prepared: null,
    },
    tasks: [{ id: 7, title: "Synthetic triage", run_seq: 1 }],
    proposals: [],
    receipt: null,
    handoff_unknown: false,
    fix_task_id: null,
    fix_task_conflict: false,
  }
}
function prepared(d: Detail): Detail {
  const proof = {
    artifact_id: "synthetic-proof",
    sha256: "b".repeat(64),
    value: "Synthetic reviewed content",
  }
  const evidence = Object.fromEntries(
    ["build", "screen", "reciter", "log"].map((f) => [f, proof])
  ) as Record<EvidenceField, typeof proof>
  d.draft.proofs = Object.fromEntries(
    Object.entries(evidence).map(([f, proof]) => [
      f,
      { proof, captured_at: null, session_ulid: null },
    ])
  )
  d.draft.confirmed_severity = "medium"
  const p: PreparedIssue = {
    draft: {
      schema_version: 1,
      template_version: "hafidh-bug-v1",
      task_id: 7,
      run_seq: 1,
      source_ref: { ...source, source: "testflight" },
      source_revision: d.draft.source_revision,
      title: d.draft.title,
      summary: d.draft.summary,
      labels: ["audio"],
      evidence,
    },
    repository_id: 11,
    repository: "owner/repo",
    binding_digest: "c".repeat(64),
    outgoing: {
      title: "Exact reviewed title",
      body: "Exact complete issue body including reviewed evidence",
      labels: ["audio"],
    },
  }
  d.draft.prepared = p
  d.proposals = [{ id: 9, status: "pending", payload: p, stale: false }]
  return d
}
beforeEach(() => {
  vi.clearAllMocks()
  vi.mocked(useRemoteConnection).mockReturnValue(null)
  vi.mocked(useTasksView).mockReturnValue({
    tasks: [],
    attentionCount: 0,
    loading: false,
    refetch: vi.fn().mockResolvedValue(undefined),
    viewMode: "board",
    setViewMode: vi.fn(),
  })
  const d = detail()
  vi.mocked(intake.status).mockResolvedValue({
    products: [product],
    folders: [{ id: 1, name: "Synthetic project" }],
    adapter_installed: true,
    in_app_available: false,
  })
  vi.mocked(intake.list).mockResolvedValue({
    records: [d.snapshot],
    next_cursor: null,
    scan_complete: true,
  })
  vi.mocked(intake.detail).mockResolvedValue(d)
})
function Shell({ mobile = false }: { mobile?: boolean }) {
  return (
    <WorkbenchRouteProvider>
      {mobile ? (
        <section>
          <BugWorkflowPage />
        </section>
      ) : (
        <div>
          <BugWorkflowPage />
        </div>
      )}
    </WorkbenchRouteProvider>
  )
}
async function select() {
  fireEvent.click(
    await screen.findByRole("button", { name: "Read TestFlight" })
  )
  fireEvent.click(
    await screen.findByRole("button", { name: /Synthetic audio report/ })
  )
  await screen.findByRole("heading", { name: "Required evidence" })
}

it("missing configuration is actionable and never pretends in-app intake exists", async () => {
  vi.mocked(intake.status).mockResolvedValue({
    products: [],
    folders: [],
    adapter_installed: false,
    in_app_available: false,
  })
  render(<Shell />)
  await screen.findByText(/No product is connected/)
  expect(
    screen.getByText(/In-app feedback is unavailable because/)
  ).toBeInTheDocument()
  expect(
    screen.queryByRole("button", { name: "Read TestFlight" })
  ).not.toBeInTheDocument()
  expect(intake.list).not.toHaveBeenCalled()
})
it("requires all evidence and confirmation, displays absent metadata honestly and escapes source markup", async () => {
  render(<Shell />)
  await select()
  fireEvent.change(screen.getByLabelText("Active triage task"), {
    target: { value: "7" },
  })
  expect(
    screen.getByRole("button", { name: "Prepare exact issue" })
  ).toBeDisabled()
  expect(
    screen.getByText(/Cannot prepare or file: add build, screen, reciter, log/)
  ).toBeInTheDocument()
  expect(screen.getAllByText("Not provided").length).toBeGreaterThan(0)
  expect(
    document.querySelector("[data-testid='bug-workflow'] script")
  ).toBeNull()
  expect(intake.prepare).not.toHaveBeenCalled()
})
it("keeps evidence and issue edits across shell remounts in memory without automatic writes", async () => {
  const storage = vi.spyOn(Storage.prototype, "setItem")
  const { rerender } = render(<Shell />)
  await select()
  fireEvent.change(screen.getByLabelText(/Build evidence summary/), {
    target: { value: "42" },
  })
  fireEvent.change(screen.getByLabelText(/Sanitized proof content/), {
    target: { value: "Proof sentinel 42" },
  })
  fireEvent.change(screen.getByLabelText(/Issue title/), {
    target: { value: "Edited sentinel title" },
  })
  rerender(<Shell mobile />)
  expect(await screen.findByLabelText(/Sanitized proof content/)).toHaveValue(
    "Proof sentinel 42"
  )
  expect(screen.getByLabelText(/Issue title/)).toHaveValue(
    "Edited sentinel title"
  )
  expect(intake.attach).not.toHaveBeenCalled()
  expect(intake.save).not.toHaveBeenCalled()
  expect(
    storage.mock.calls.some((call) =>
      call.some((v) => String(v).includes("sentinel"))
    )
  ).toBe(false)
})
it("drops previous backend evidence before painting a new connection", async () => {
  const { rerender } = render(<Shell />)
  await select()
  fireEvent.change(screen.getByLabelText(/Sanitized proof content/), {
    target: { value: "Private backend A edit" },
  })
  vi.mocked(useRemoteConnection).mockReturnValue({
    connection: {
      id: 2,
      name: "Synthetic second host",
      base_url: "https://second.invalid",
      token: "synthetic",
      headers: [],
      sort_order: 0,
      created_at: "",
      updated_at: "",
    },
    expired: false,
    markExpired: vi.fn(),
  })
  rerender(<Shell mobile />)
  expect(
    screen.queryByDisplayValue("Private backend A edit")
  ).not.toBeInTheDocument()
  await select()
  expect(screen.getByLabelText(/Sanitized proof content/)).toHaveValue("")
})
it("shows the whole issue and hands off exactly the confirmed payload once", async () => {
  const d = prepared(detail())
  vi.mocked(intake.detail).mockResolvedValue(d)
  vi.mocked(intake.approve).mockImplementation(() => new Promise(() => {}))
  render(<Shell />)
  await select()
  expect(screen.getByLabelText("Exact issue body")).toHaveTextContent(
    d.draft.prepared!.outgoing.body
  )
  const approve = screen.getByRole("button", { name: "Approve and file issue" })
  expect(approve).toBeDisabled()
  fireEvent.click(screen.getByRole("checkbox"))
  fireEvent.click(approve)
  fireEvent.click(approve)
  await waitFor(() => expect(intake.approve).toHaveBeenCalledTimes(1))
  expect(intake.approve).toHaveBeenCalledWith(
    source,
    9,
    d.draft.prepared,
    d.draft.prepared
  )
})
it("stale and unknown receipts block filing while preserving a read-only reconciliation", async () => {
  const d = prepared(detail())
  d.proposals[0].stale = true
  d.receipt = {
    attempt_id: 1,
    proposal_id: 9,
    state: "unknown",
    issue: null,
    error_code: null,
    retry_after: null,
  }
  vi.mocked(intake.detail).mockResolvedValue(d)
  render(<Shell />)
  await select()
  expect(
    screen.getByRole("button", { name: "Approve and file issue" })
  ).toBeDisabled()
  expect(
    screen.getByRole("button", { name: "Check existing issue" })
  ).toBeEnabled()
  expect(screen.getByText(/Outcome unknown/)).toBeInTheDocument()
  expect(
    screen.queryByRole("button", { name: "Create held fix plan" })
  ).not.toBeInTheDocument()
})
it("conflicted fix links never offer an unrelated task or a replacement creation", async () => {
  const d = detail()
  d.fix_task_conflict = true
  vi.mocked(intake.detail).mockResolvedValue(d)
  render(<Shell />)
  await select()
  expect(screen.getByText(/saved fix link conflicts/)).toBeInTheDocument()
  expect(
    screen.queryByRole("button", { name: "Review in Tasks" })
  ).not.toBeInTheDocument()
  expect(
    screen.queryByRole("button", { name: "Create held fix plan" })
  ).not.toBeInTheDocument()
})

it("keeps stale pending review deniable after source refresh clears prepared evidence", async () => {
  const d = prepared(detail())
  d.draft.prepared = null
  d.draft.proofs = {}
  d.proposals[0].stale = true
  vi.mocked(intake.detail).mockResolvedValue(d)
  vi.mocked(intake.deny).mockResolvedValue({ ...d, proposals: [] })
  render(<Shell />)
  await select()
  expect(screen.getByLabelText("Exact issue body")).toHaveTextContent(
    d.proposals[0].payload!.outgoing.body
  )
  expect(
    screen.getByRole("button", { name: "Approve and file issue" })
  ).toBeDisabled()
  fireEvent.click(screen.getByRole("button", { name: "Deny proposal" }))
  await waitFor(() =>
    expect(intake.deny).toHaveBeenCalledWith(source, 9, d.proposals[0].payload)
  )
})
