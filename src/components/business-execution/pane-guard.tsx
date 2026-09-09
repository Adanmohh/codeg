"use client"

import { useEffect, useState, type ReactNode } from "react"
import { Action, Modal } from "@/components/business/ui"
import { useBusinessCopy } from "@/lib/business/copy"
import { useExecutionCopy } from "@/lib/business-execution/copy"
import type { PromptGuard } from "./session-prompt"

// Uses the accepted TaskFrame's registered close/discard seam. Scope teardown
// detaches the view; it never substitutes for a stop/receipt operation.
export function ExecutionPaneGuard({
  onClose,
  registerClose,
  getGuard,
  children,
}: {
  onClose: () => void
  registerClose: (request: (() => void) | null) => void
  getGuard: () => PromptGuard
  children: ReactNode
}) {
  const common = useBusinessCopy()
  const copy = useExecutionCopy()
  const [discard, setDiscard] = useState(false)
  const [pending, setPending] = useState(false)
  useEffect(() => {
    registerClose(() => {
      const guard = getGuard()
      if (guard.busy || guard.unresolved) setPending(true)
      else if (guard.dirty) setDiscard(true)
      else onClose()
    })
    return () => registerClose(null)
  }, [onClose, registerClose, getGuard])
  return (
    <>
      {pending && (
        <div
          role="alert"
          className="m-4 space-y-3 rounded-xl border p-4 text-sm"
        >
          <p>{copy.pendingPane}</p>
          <Action variant="outline" onClick={() => setPending(false)}>
            {common.stay}
          </Action>
        </div>
      )}
      {children}
      {discard && (
        <Modal
          title={common.discardTitle}
          description={common.discardHint}
          onClose={() => setDiscard(false)}
        >
          <div className="flex flex-wrap justify-end gap-3">
            <Action variant="outline" onClick={() => setDiscard(false)}>
              {common.stay}
            </Action>
            <Action
              variant="destructive"
              onClick={() => {
                const guard = getGuard()
                if (guard.busy || guard.unresolved) {
                  setDiscard(false)
                  setPending(true)
                  return
                }
                onClose()
              }}
            >
              {common.discardDraft}
            </Action>
          </div>
        </Modal>
      )}
    </>
  )
}
