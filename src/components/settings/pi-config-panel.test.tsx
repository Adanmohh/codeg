import { fireEvent, render, screen, waitFor } from "@testing-library/react"
import { NextIntlClientProvider } from "next-intl"
import { beforeEach, describe, expect, it, vi } from "vitest"
import enMessages from "@/i18n/messages/en.json"
import type { AcpAgentInfo } from "@/lib/types"
import { acpInstallPiBinary, acpUpdatePiConfig, loadPiConfig } from "@/lib/api"
import { PiConfigPanel } from "./pi-config-panel"

vi.mock("@/lib/api", () => ({
  loadPiConfig: vi.fn(),
  acpUpdatePiConfig: vi.fn().mockResolvedValue(undefined),
  acpValidatePiCommand: vi.fn().mockResolvedValue({
    found: true,
    resolvedPath: "/fixture/pi",
    version: "0.85.1",
  }),
  acpPiListTrustEntries: vi.fn().mockResolvedValue([]),
  acpPiSetProjectTrust: vi.fn(),
  acpInstallPiBinary: vi.fn(),
  acpUninstallPiBinary: vi.fn(),
}))
vi.mock("@/hooks/use-agent-install-stream", () => {
  const stream = { status: "idle", logs: [], start: vi.fn(), reset: vi.fn() }
  return { useAgentInstallStream: () => stream }
})

const copy = enMessages.AcpAgentSettings.pi
const agent: AcpAgentInfo = {
  agent_type: "pi",
  skills_capable: true,
  registry_id: "pi",
  registry_version: "0.0.33",
  supports_custom_version: false,
  name: "Pi",
  description: "Fixture",
  available: true,
  distribution_type: "npx",
  is_acp_adapter: true,
  custom_source: null,
  enabled: true,
  sort_order: 0,
  installed_version: "0.0.33",
  env: {},
  host_tools_agent_mode: false,
  config_json: null,
  config_file_path: null,
  opencode_auth_json: null,
  codex_auth_json: null,
  codex_config_toml: null,
  codex_model_catalog: null,
  codex_sandbox_settings: null,
  cline_secrets_json: null,
  hermes_config_yaml: null,
  grok_config_toml: null,
  grok_settings: null,
  cursor_cli_config_json: null,
  cursor_settings: null,
  model_provider_id: null,
  icon_url: null,
}

function renderPanel() {
  const onSaveEnv = vi.fn()
  render(
    <NextIntlClientProvider locale="en" messages={enMessages}>
      <PiConfigPanel
        agent={agent}
        saving={false}
        onSaveEnv={onSaveEnv}
        onSaved={vi.fn().mockResolvedValue(undefined)}
      />
    </NextIntlClientProvider>
  )
  return { onSaveEnv }
}

describe("Pi configuration accessibility and unchanged persistence", () => {
  beforeEach(() => {
    vi.clearAllMocks()
    vi.mocked(loadPiConfig).mockResolvedValue({
      defaultProvider: "openai",
      defaultModel: "existing-model",
      defaultThinkingLevel: "high",
      authProviders: ["openai"],
      customProviders: [],
    })
  })

  it("names configuration fields and preserves saved values until explicit save", async () => {
    const { onSaveEnv } = renderPanel()
    const model = await screen.findByRole("textbox", { name: copy.modelLabel })
    await waitFor(() => expect(model).toHaveValue("existing-model"))
    expect(
      screen.getByRole("combobox", { name: copy.providerLabel })
    ).toHaveTextContent("OpenAI")
    expect(
      screen.getByRole("combobox", { name: copy.thinkingLabel })
    ).toHaveTextContent(copy.thinking.high)
    expect(screen.getByLabelText(copy.apiKeyLabel)).toHaveValue("")
    expect(model).toHaveAttribute("placeholder", "gpt-6-astra")
    expect(acpUpdatePiConfig).not.toHaveBeenCalled()
    expect(acpInstallPiBinary).not.toHaveBeenCalled()
    expect(onSaveEnv).not.toHaveBeenCalled()

    fireEvent.click(screen.getByRole("button", { name: copy.saveConfig }))
    await waitFor(() => expect(acpUpdatePiConfig).toHaveBeenCalledOnce())
    expect(acpUpdatePiConfig).toHaveBeenCalledWith({
      provider: "openai",
      model: "existing-model",
      thinkingLevel: "high",
      apiKey: undefined,
      customBaseUrl: undefined,
      customApi: undefined,
      modelReasoning: undefined,
    })
  })

  it("names custom-provider and reasoning controls without writing config", async () => {
    vi.mocked(loadPiConfig).mockResolvedValue({
      defaultProvider: "fixture-provider",
      defaultModel: "gpt-6-astra",
      defaultThinkingLevel: "high",
      authProviders: [],
      customProviders: [
        {
          id: "fixture-provider",
          baseUrl: "https://provider.invalid/v1",
          api: "openai-responses",
          models: [
            { id: "gpt-6-astra", reasoning: true, thinkingLevelMap: {} },
          ],
        },
      ],
    })
    const { onSaveEnv } = renderPanel()
    const provider = await screen.findByRole("textbox", {
      name: copy.providerIdLabel,
    })
    expect(provider).toHaveValue("fixture-provider")
    expect(
      screen.getByRole("combobox", { name: copy.apiProtocolLabel })
    ).toHaveTextContent("openai-responses")
    expect(
      screen.getByRole("textbox", { name: copy.baseUrlLabel })
    ).toHaveValue("https://provider.invalid/v1")
    expect(
      screen.getByRole("switch", { name: copy.reasoningEnableLabel })
    ).toBeChecked()
    expect(screen.getByRole("group", { name: copy.levelsLabel })).toBeVisible()
    fireEvent.click(screen.getByRole("button", { name: copy.wireValuesTitle }))
    expect(
      screen.getByRole("textbox", { name: `${copy.wireValuesTitle}: high` })
    ).toBeVisible()
    expect(acpUpdatePiConfig).not.toHaveBeenCalled()
    expect(acpInstallPiBinary).not.toHaveBeenCalled()
    expect(onSaveEnv).not.toHaveBeenCalled()
  })
})
