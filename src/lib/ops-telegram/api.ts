import { getTransport } from "@/lib/transport"
import type { Proposal } from "@/lib/ops/api"

export interface TelegramConfiguration {
  channelId: number
  privateUserId: string
  reviewOrigin: string
  revision: string
}
export interface TelegramStatus {
  enabled: boolean
  state:
    | "not_configured"
    | "disabled"
    | "recipient_changed"
    | "missing_token"
    | "ready"
  configuration: TelegramConfiguration | null
  channels: { id: number; name: string }[]
  notices: {
    proposalId: number
    taskId: number
    runSeq: number
    status: "preflight_failed" | "sent" | "failed" | "unknown" | "obsolete"
    updatedAt: string
  }[]
}
export interface TelegramConfigureInput {
  channelId: number
  privateUserId: string
  reviewOrigin: string
  enabled: boolean
  expectedRevision: string | null
}
export interface ReviewResolution {
  state: "ready" | "unavailable"
  proposal: Proposal | null
}
const call = <T>(command: string, input?: unknown) =>
  getTransport().call<T>(command, input === undefined ? {} : { input })
export const telegram = {
  status: () => call<TelegramStatus>("ops_telegram_status"),
  configure: (input: TelegramConfigureInput) =>
    call<TelegramStatus>("ops_telegram_configure", input),
  disable: () => call<TelegramStatus>("ops_telegram_disable"),
  notify: () => call<TelegramStatus>("ops_telegram_notify"),
  resolve: (notice: string) =>
    call<ReviewResolution>("ops_telegram_resolve", { notice }),
}
