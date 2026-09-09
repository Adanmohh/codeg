"use client"

import { createContext, useContext, type ReactNode } from "react"
import { useTheme } from "next-themes"
import type { TenantSettings } from "@/lib/business/settings"
import { cn } from "@/lib/utils"

const AppearanceContext = createContext<TenantSettings["palette"] | undefined>(
  undefined
)

/** Reuses existing paired [data-theme] selectors on this DOM subtree only.
 * Portalled business layers consume the same React context below. No root
 * attribute, global palette, font or personal preference is written.
 */
export function WorkspaceAppearance({
  palette,
  children,
}: {
  palette?: TenantSettings["palette"]
  children: ReactNode
}) {
  return (
    <AppearanceContext.Provider value={palette}>
      <AppearanceFrame>{children}</AppearanceFrame>
    </AppearanceContext.Provider>
  )
}

function AppearanceFrame({ children }: { children: ReactNode }) {
  const appearance = useWorkspaceAppearance()
  return (
    <div
      data-business-appearance
      data-theme={appearance.palette}
      className={cn("h-full min-h-0 min-w-0", appearance.dark && "dark")}
    >
      {children}
    </div>
  )
}

export function useWorkspaceAppearance() {
  const palette = useContext(AppearanceContext)
  const { resolvedTheme, theme } = useTheme()
  return {
    palette,
    dark: !!palette && (resolvedTheme ?? theme) === "dark",
  }
}
