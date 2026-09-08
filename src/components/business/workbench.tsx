"use client"

import { useEffect, useId, useRef, useState, type ReactNode } from "react"
import { useLocale } from "next-intl"
import { Columns2, PanelTop, X, type LucideIcon } from "lucide-react"
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable"
import { OverlayHostHiddenProvider } from "@/components/ui/overlay-host-hidden"
import { useMediaQuery } from "@/hooks/use-media-query"
import { computeRects, singleGroupLayout } from "@/lib/tab-group-layout"
import { useBusinessCopy } from "@/lib/business/copy"
import { cn } from "@/lib/utils"

export interface WorkSurface {
  id: string
  label: string
  icon: LucideIcon
  close?: () => void
  render: (visible: boolean) => ReactNode
}

/** Controlled business data, with Codeg's stable sibling pane composition.
 * No legacy tab store, transport, storage or execution provider is mounted here.
 */
export function BusinessWorkbench({
  surfaces,
  activeId,
  onActivate,
}: {
  surfaces: WorkSurface[]
  activeId: string
  onActivate: (id: string) => void
}) {
  const copy = useBusinessCopy()
  const locale = useLocale()
  const id = useId()
  const buttons = useRef(new Map<string, HTMLButtonElement>())
  const closing = useRef<string | null>(null)
  const wide = useMediaQuery("(min-width: 1100px)")
  const [referenceId, setReferenceId] = useState<string | null>(null)
  const [sizes, setSizes] = useState([45, 55])
  const reference = surfaces.find((surface) => surface.id === referenceId)
  useEffect(() => {
    if (
      closing.current &&
      !surfaces.some((surface) => surface.id === closing.current)
    ) {
      closing.current = null
      buttons.current.get(activeId)?.focus()
    }
  }, [surfaces, activeId])
  const paired = wide && !!reference && reference.id !== activeId
  const { groups } = computeRects(
    paired
      ? {
          type: "split",
          id: "business-panes",
          orientation: "horizontal",
          children: [singleGroupLayout("reference"), singleGroupLayout("work")],
          ratios: sizes,
        }
      : singleGroupLayout("work")
  )
  function split() {
    if (reference) setReferenceId(null)
    else
      setReferenceId(
        surfaces.find((surface) => surface.id !== activeId)?.id ?? null
      )
  }
  function close(surface: WorkSurface) {
    closing.current = surface.id
    surface.close?.()
  }
  return (
    <div className="flex h-full min-h-0 min-w-0 flex-col">
      <div className="bg-sidebar border-border flex min-h-12 shrink-0 border-b">
        <div
          role="tablist"
          aria-label={copy.openWork}
          className="flex min-w-0 flex-1 items-stretch overflow-x-auto"
        >
          {surfaces.map((surface, index) => {
            const selected = surface.id === activeId
            const Icon = surface.icon
            return (
              <div
                key={surface.id}
                role="presentation"
                className={cn(
                  "border-border relative flex min-w-32 max-w-64 shrink-0 items-stretch border-e",
                  selected
                    ? "bg-background text-foreground after:bg-primary after:absolute after:inset-x-3 after:bottom-0 after:h-0.5"
                    : "text-muted-foreground hover:bg-sidebar-accent"
                )}
              >
                <button
                  ref={(node) => {
                    if (node) buttons.current.set(surface.id, node)
                    else buttons.current.delete(surface.id)
                  }}
                  type="button"
                  role="tab"
                  id={`${id}-tab-${surface.id}`}
                  aria-controls={`${id}-panel-${surface.id}`}
                  aria-selected={selected}
                  tabIndex={selected ? 0 : -1}
                  onClick={() => onActivate(surface.id)}
                  onKeyDown={(event) => {
                    let next: number | null = null
                    const forward = locale === "ar" ? "ArrowLeft" : "ArrowRight"
                    const backward =
                      locale === "ar" ? "ArrowRight" : "ArrowLeft"
                    if (event.key === forward)
                      next = (index + 1) % surfaces.length
                    if (event.key === backward)
                      next = (index + surfaces.length - 1) % surfaces.length
                    if (event.key === "Home") next = 0
                    if (event.key === "End") next = surfaces.length - 1
                    if (next !== null) {
                      event.preventDefault()
                      buttons.current.get(surfaces[next].id)?.focus()
                    } else if (event.key === "Delete" && surface.close) {
                      event.preventDefault()
                      // The editor owns pending-write and unsaved-change guards.
                      close(surface)
                    }
                  }}
                  className="focus-visible:ring-ring flex min-h-12 min-w-0 flex-1 items-center gap-2 px-3 text-start text-sm outline-none focus-visible:ring-2 focus-visible:ring-inset"
                >
                  <Icon className="size-4 shrink-0" aria-hidden="true" />
                  <bdi className="truncate">{surface.label}</bdi>
                </button>
                {surface.close && (
                  <button
                    type="button"
                    tabIndex={selected ? 0 : -1}
                    aria-label={`${copy.closeTab}: ${surface.label}`}
                    onClick={() => close(surface)}
                    className="hover:bg-muted focus-visible:ring-ring flex w-11 shrink-0 items-center justify-center outline-none focus-visible:ring-2 focus-visible:ring-inset"
                  >
                    <X className="size-3.5" aria-hidden="true" />
                  </button>
                )}
              </div>
            )
          })}
        </div>
        {wide && (
          <button
            type="button"
            onClick={split}
            disabled={surfaces.length < 2}
            aria-pressed={!!reference}
            aria-label={reference ? copy.singlePane : copy.splitView}
            title={reference ? copy.singlePane : copy.splitView}
            className="hover:bg-sidebar-accent focus-visible:ring-ring flex w-12 shrink-0 items-center justify-center outline-none focus-visible:ring-2 focus-visible:ring-inset disabled:opacity-40"
          >
            {reference ? (
              <PanelTop className="size-4" aria-hidden="true" />
            ) : (
              <Columns2 className="size-4" aria-hidden="true" />
            )}
          </button>
        )}
      </div>
      <div className="relative min-h-0 min-w-0 flex-1 overflow-hidden">
        {paired && (
          <ResizablePanelGroup
            direction="horizontal"
            dir={locale === "ar" ? "rtl" : "ltr"}
            onLayout={setSizes}
            keyboardResizeBy={5}
          >
            <ResizablePanel
              id="business-reference"
              order={1}
              defaultSize={sizes[0]}
              minSize={35}
            />
            <ResizableHandle aria-label={copy.resizeWorkPanes} />
            <ResizablePanel
              id="business-work"
              order={2}
              defaultSize={sizes[1]}
              minSize={35}
            />
          </ResizablePanelGroup>
        )}
        {surfaces.map((surface) => {
          const isReference = paired && surface.id === referenceId
          const visible = surface.id === activeId || isReference
          const rect = groups.get(isReference ? "reference" : "work")!
          return (
            <OverlayHostHiddenProvider key={surface.id} hidden={!visible}>
              <section
                id={`${id}-panel-${surface.id}`}
                role="tabpanel"
                aria-labelledby={`${id}-tab-${surface.id}`}
                tabIndex={0}
                inert={!visible || undefined}
                aria-hidden={!visible || undefined}
                className={cn(
                  "bg-background focus-visible:ring-ring absolute flex min-h-0 min-w-0 flex-col overflow-hidden outline-none focus-visible:ring-2 focus-visible:ring-inset",
                  !visible && "conversation-tab-hidden invisible"
                )}
                style={{
                  insetInlineStart: `${rect.x}%`,
                  top: `${rect.y}%`,
                  width: `${rect.w}%`,
                  height: `${rect.h}%`,
                  visibility: visible ? undefined : "hidden",
                }}
              >
                {isReference && (
                  <div className="bg-muted/35 border-border flex min-h-9 shrink-0 items-center border-b px-4 text-xs font-medium">
                    {copy.referencePane}
                  </div>
                )}
                <div className="min-h-0 min-w-0 flex-1 overflow-x-hidden overflow-y-auto">
                  {surface.render(visible)}
                </div>
              </section>
            </OverlayHostHiddenProvider>
          )
        })}
      </div>
    </div>
  )
}
