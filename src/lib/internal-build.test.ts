import { beforeEach, describe, expect, it, vi } from "vitest"

let desktop = true
let remote = false
const call = vi.fn(async () => ({ currentVersion: "0.30.4" }))
const check = vi.fn()

vi.mock("@/lib/transport", () => ({
  getTransport: () => ({ call }),
  isDesktop: () => desktop,
  isRemoteDesktopMode: () => remote,
}))
vi.mock("@tauri-apps/api/app", () => ({
  getVersion: async () => "0.30.4",
}))
vi.mock("@tauri-apps/plugin-updater", () => ({ check }))

import {
  checkAppUpdateInfo,
  restartApp,
  rollbackServer,
  startAppUpdate,
} from "./updater"

describe("internal build application updates", () => {
  beforeEach(() => {
    call.mockClear()
    check.mockClear()
  })

  it.each([
    { mode: "desktop", isDesktop: true, isRemote: false },
    { mode: "server", isDesktop: false, isRemote: false },
    { mode: "remote desktop", isDesktop: true, isRemote: true },
  ])("does not contact a release source in $mode mode", async (runtime) => {
    desktop = runtime.isDesktop
    remote = runtime.isRemote

    await expect(checkAppUpdateInfo()).resolves.toEqual({
      currentVersion: "0.30.4",
      update: null,
    })
    expect(check).not.toHaveBeenCalled()
    // Version display may read local status; it must not invoke an update check.
    expect(call.mock.calls).toEqual(
      desktop && !remote ? [] : [["app_update_status"]]
    )

    call.mockClear()
    await expect(startAppUpdate()).rejects.toThrow("updates are disabled")
    await expect(restartApp()).rejects.toThrow("updates are disabled")
    await expect(rollbackServer()).rejects.toThrow("updates are disabled")
    expect(call).not.toHaveBeenCalled()
  })
})
