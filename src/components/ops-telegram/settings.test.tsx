import { fireEvent, render, screen, waitFor } from "@testing-library/react"
import { beforeEach, expect, it, vi } from "vitest"
import { telegram, type TelegramStatus } from "@/lib/ops-telegram/api"
import { TelegramSettings } from "./settings"
import { OpsSessionProvider } from "@/components/ops/session"

vi.mock("@/lib/ops-telegram/api", () => ({
  telegram: {
    status: vi.fn(),
    configure: vi.fn(),
    disable: vi.fn(),
    notify: vi.fn(),
  },
}))
const state: TelegramStatus = {
  enabled: false,
  state: "not_configured",
  configuration: null,
  channels: [{ id: 7, name: "Private operator" }],
  notices: [],
}
beforeEach(() => {
  vi.clearAllMocks()
  vi.mocked(telegram.status).mockResolvedValue(state)
})

it("defaults off and requires explicit configuration, with no mount-time dispatch (mocked facade)", async () => {
  vi.mocked(telegram.configure).mockResolvedValue(state)
  render(
    <OpsSessionProvider>
      <TelegramSettings onDirty={vi.fn()} />
    </OpsSessionProvider>
  )
  await screen.findByText(/Notifications are off/)
  expect(screen.getByRole("checkbox")).not.toBeChecked()
  expect(telegram.notify).not.toHaveBeenCalled()
  expect(screen.getByRole("button", { name: "Check queue now" })).toBeDisabled()
  fireEvent.change(screen.getByLabelText("Telegram channel"), {
    target: { value: "7" },
  })
  fireEvent.change(screen.getByLabelText("Authorized private user ID"), {
    target: { value: "123" },
  })
  fireEvent.change(screen.getByLabelText("Protected review origin"), {
    target: { value: "https://desk.example.com" },
  })
  fireEvent.click(screen.getByRole("checkbox"))
  expect(telegram.configure).not.toHaveBeenCalled()
  fireEvent.click(
    screen.getByRole("button", { name: "Save Telegram settings" })
  )
  await waitFor(() =>
    expect(telegram.configure).toHaveBeenCalledWith({
      channelId: 7,
      privateUserId: "123",
      reviewOrigin: "https://desk.example.com",
      enabled: true,
      expectedRevision: null,
    })
  )
  expect(telegram.notify).not.toHaveBeenCalled()
})

it("shows retryable preflight distinctly from ambiguous sends and surfaces configuration errors", async () => {
  vi.mocked(telegram.status).mockResolvedValue({
    ...state,
    notices: [
      {
        proposalId: 1,
        taskId: 1,
        runSeq: 1,
        status: "preflight_failed",
        updatedAt: "2026-09-08",
      },
      {
        proposalId: 2,
        taskId: 2,
        runSeq: 1,
        status: "unknown",
        updatedAt: "2026-09-08",
      },
    ],
  })
  vi.mocked(telegram.configure).mockRejectedValue(
    new Error("Recipient changed; reload settings")
  )
  render(<TelegramSettings onDirty={vi.fn()} />)
  await screen.findByText(/next queue check can retry safely/)
  expect(
    screen.getByText("Notification unconfirmed · no automatic resend")
  ).toBeVisible()
  fireEvent.change(screen.getByLabelText("Telegram channel"), {
    target: { value: "7" },
  })
  fireEvent.change(screen.getByLabelText("Authorized private user ID"), {
    target: { value: "123" },
  })
  fireEvent.change(screen.getByLabelText("Protected review origin"), {
    target: { value: "https://desk.example.com" },
  })
  fireEvent.click(
    screen.getByRole("button", { name: "Save Telegram settings" })
  )
  expect(await screen.findByRole("alert")).toHaveTextContent(
    "Recipient changed; reload settings"
  )
  expect(telegram.notify).not.toHaveBeenCalled()
})
