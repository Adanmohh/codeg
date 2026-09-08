import { fireEvent, render, screen } from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { describe, expect, it, vi } from "vitest"
import enMessages from "@/i18n/messages/en.json"
import { AgentSetupNotice } from "./agent-setup-notice"

const message =
  "Pi Desk is not installed or configured: a configured gpt-6-astra catalogue entry with max reasoning is required. Use the existing Pi client configuration."

describe("AgentSetupNotice recovery actions", () => {
  it("keeps the complete reason as an alert and opens settings only on request", () => {
    const onOpenSettings = vi.fn()
    render(
      <NextIntlClientProvider locale="en" messages={enMessages}>
        <AgentSetupNotice message={message} onOpenSettings={onOpenSettings} />
      </NextIntlClientProvider>
    )
    expect(screen.getByRole("alert")).toHaveTextContent(message)
    expect(screen.queryByRole("button", { name: message })).toBeNull()
    expect(
      screen.queryByRole("button", {
        name: enMessages.DiagnosticsSettings.button,
      })
    ).toBeNull()
    expect(onOpenSettings).not.toHaveBeenCalled()
    fireEvent.click(
      screen.getByRole("button", {
        name: enMessages.Folder.chat.agentSelector.openAgentsSettings,
      })
    )
    expect(onOpenSettings).toHaveBeenCalledOnce()
  })

  it("preserves the optional diagnostic action independently from settings", () => {
    const onOpenSettings = vi.fn()
    const onDiagnose = vi.fn()
    render(
      <NextIntlClientProvider locale="en" messages={enMessages}>
        <AgentSetupNotice
          message={message}
          onOpenSettings={onOpenSettings}
          onDiagnose={onDiagnose}
        />
      </NextIntlClientProvider>
    )
    fireEvent.click(
      screen.getByRole("button", {
        name: enMessages.DiagnosticsSettings.button,
      })
    )
    expect(onDiagnose).toHaveBeenCalledOnce()
    expect(onOpenSettings).not.toHaveBeenCalled()
  })
})
