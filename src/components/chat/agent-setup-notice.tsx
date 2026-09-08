"use client"

import { useTranslations } from "next-intl"
import { Button } from "@/components/ui/button"
import { cn } from "@/lib/utils"

/** Shared presentation for the welcome and existing-conversation composers. */
export function AgentSetupNotice({
  message,
  onOpenSettings,
  onDiagnose,
  className,
}: {
  message: string
  onOpenSettings: () => void
  onDiagnose?: () => void
  className?: string
}) {
  const t = useTranslations("Folder.chat.agentSelector")
  const tDiag = useTranslations("DiagnosticsSettings")

  return (
    <div
      className={cn(
        "w-full min-w-0 space-y-2 rounded-lg border border-destructive/30 bg-destructive/5 px-3 py-2 text-sm text-foreground",
        className
      )}
    >
      <p role="alert" className="whitespace-pre-wrap wrap-anywhere">
        {message}
      </p>
      <div className="flex flex-wrap gap-2">
        <Button
          type="button"
          variant="outline"
          size="sm"
          className="min-h-11 whitespace-normal"
          onClick={onOpenSettings}
        >
          {t("openAgentsSettings")}
        </Button>
        {onDiagnose && (
          <Button
            type="button"
            variant="outline"
            size="sm"
            className="min-h-11 whitespace-normal"
            onClick={onDiagnose}
          >
            {tDiag("button")}
          </Button>
        )}
      </div>
    </div>
  )
}
