"use client"

import { type ReactNode, useSyncExternalStore } from "react"
import { isTauri } from "@tauri-apps/api/core"
import { AppTitleBar } from "@/components/layout/app-title-bar"

// Runtime identity cannot change during this window's lifetime. Keep the
// exported HTML identical for web/desktop until the native snapshot hydrates.
const subscribeRuntime = () => () => {}
const serverRuntime = () => false

export default function BusinessLayout({ children }: { children: ReactNode }) {
  const native = useSyncExternalStore(subscribeRuntime, isTauri, serverRuntime)
  return (
    <div className="bg-background flex h-dvh flex-col overflow-hidden">
      {native && (
        <div dir="ltr" className="shrink-0" data-business-native-chrome>
          {/* Match the existing main-window traffic-light position and keep
              physical caption controls independent of the content's RTL. */}
          <AppTitleBar
            className="h-10"
            center={
              <span className="text-sm font-medium">Hafidh Ops Desk</span>
            }
          />
        </div>
      )}
      <div className="min-h-0 flex-1">{children}</div>
    </div>
  )
}
