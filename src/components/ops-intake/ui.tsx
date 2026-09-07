import { type ComponentProps, type ReactNode, useId } from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { cn } from "@/lib/utils"

export function Action({ className, ...props }: ComponentProps<typeof Button>) {
  return (
    <Button className={cn("min-h-11 rounded-lg px-4", className)} {...props} />
  )
}
export function Field({
  label,
  hint,
  ...props
}: ComponentProps<typeof Input> & { label: string; hint?: string }) {
  const id = useId()
  return (
    <div className="grid min-w-0 gap-2 text-sm font-medium">
      <label htmlFor={id}>{label}</label>
      <Input
        id={id}
        className="h-11 rounded-lg text-base md:text-base"
        aria-describedby={hint ? `${id}-hint` : undefined}
        {...props}
      />
      {hint && (
        <span
          id={`${id}-hint`}
          className="text-muted-foreground text-sm font-normal"
        >
          {hint}
        </span>
      )}
    </div>
  )
}
export function TextField({
  label,
  hint,
  ...props
}: ComponentProps<typeof Textarea> & { label: string; hint?: string }) {
  const id = useId()
  return (
    <div className="grid min-w-0 gap-2 text-sm font-medium">
      <label htmlFor={id}>{label}</label>
      <Textarea
        id={id}
        className="min-h-28 rounded-lg text-base md:text-base"
        aria-describedby={hint ? `${id}-hint` : undefined}
        {...props}
      />
      {hint && (
        <span
          id={`${id}-hint`}
          className="text-muted-foreground text-sm font-normal"
        >
          {hint}
        </span>
      )}
    </div>
  )
}
export function Choice({
  label,
  children,
  ...props
}: ComponentProps<"select"> & { label: string; children: ReactNode }) {
  const id = useId()
  return (
    <label htmlFor={id} className="grid min-w-0 gap-2 text-sm font-medium">
      {label}
      <select
        id={id}
        className="border-input bg-background text-foreground focus-visible:ring-ring h-11 min-w-0 rounded-lg border px-3 text-base focus-visible:ring-2 focus-visible:outline-none"
        {...props}
      >
        {children}
      </select>
    </label>
  )
}
export function Notice({
  children,
  error = false,
}: {
  children: ReactNode
  error?: boolean
}) {
  return (
    <div
      role={error ? "alert" : "status"}
      className="border-border bg-muted/50 rounded-lg border p-4 text-sm leading-relaxed"
    >
      {children}
    </div>
  )
}
export function Panel({
  title,
  children,
}: {
  title: string
  children: ReactNode
}) {
  return (
    <section className="border-border min-w-0 space-y-4 rounded-xl border p-4 sm:p-6">
      <h2 className="text-base font-semibold">{title}</h2>
      {children}
    </section>
  )
}
