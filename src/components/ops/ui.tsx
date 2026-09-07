import { Button } from "@/components/ui/button"
import { cn } from "@/lib/utils"

export const touchButton = "min-h-11 rounded-lg"
// Keep the shared destructive tint and focus ring; use the paired foreground
// token so enabled small text meets contrast on both light and dark surfaces.
export const destructiveButton = `${touchButton} text-foreground`
export function Notice({
  children,
  error = false,
}: {
  children: React.ReactNode
  error?: boolean
}) {
  return (
    <p
      role={error ? "alert" : "status"}
      className={cn(
        "rounded-lg border bg-muted/30 p-3 text-sm leading-relaxed",
        error && "border-destructive/40 text-destructive"
      )}
    >
      {children}
    </p>
  )
}
export function Loading({ label }: { label: string }) {
  return (
    <p role="status" className="p-6 text-sm text-muted-foreground">
      {label}
    </p>
  )
}
export function LoadError({
  error,
  retry,
}: {
  error: string
  retry: () => void
}) {
  return (
    <div className="space-y-3 p-4">
      <Notice error>{error}</Notice>
      <Button className={touchButton} variant="outline" onClick={retry}>
        Try again
      </Button>
    </div>
  )
}
