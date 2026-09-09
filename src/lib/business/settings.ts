// Closed frontend projection of approvals' settings.rs at
// 29774b50aafc29658a2f48fab1f44d366ed2c8a0 / contract75164616.
// These values describe presentation, never tenant authority or execution.
export interface TenantSettings {
  displayName: string
  palette: "neutral" | "blue" | "violet"
  workspaceLayout: "split" | "stacked"
  defaultWorkArea: "tasks" | "conversations"
}
export interface TenantSettingsView {
  organizationId: string
  revision: number
  settings: TenantSettings
}
export interface UpdateTenantSettings {
  expectedRevision: number
  settings: TenantSettings
}
export interface TenantSettingsAccess {
  get: () => Promise<TenantSettingsView>
  update: (input: UpdateTenantSettings) => Promise<TenantSettingsView>
}
export interface SettingsOperations {
  "settings/get": {
    input: Record<string, never>
    result: TenantSettingsView
  }
  "settings/update": {
    input: UpdateTenantSettings
    result: TenantSettingsView
  }
}

// Only closed, same-organization values may reach scoped CSS or visible names.
// This is response validation, never a substitute for backend authorization.
export function isTenantSettingsView(
  value: unknown,
  organizationId: string
): value is TenantSettingsView {
  if (!value || typeof value !== "object") return false
  const candidate = value as Partial<TenantSettingsView>
  const settings = candidate.settings
  return (
    candidate.organizationId === organizationId &&
    Number.isSafeInteger(candidate.revision) &&
    candidate.revision! > 0 &&
    !!settings &&
    typeof settings.displayName === "string" &&
    !!settings.displayName.trim() &&
    Array.from(settings.displayName.trim()).length <= 120 &&
    ["neutral", "blue", "violet"].includes(settings.palette) &&
    ["split", "stacked"].includes(settings.workspaceLayout) &&
    ["tasks", "conversations"].includes(settings.defaultWorkArea)
  )
}
