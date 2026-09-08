// Closed frontend projection of approvals' settings.rs at
// f3b408dae5c724f354763961d79a17a7ae5c86f8 / contract75164616.
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
