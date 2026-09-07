"use client"

import { useId } from "react"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import type { Reply } from "@/lib/ops/api"

export interface ReplyFields {
  from: string
  to: string
  cc: string
  bcc: string
  subject: string
  text: string
  inReplyTo: string
  references: string
}
export function replyFields(reply: Reply): ReplyFields {
  return {
    ...reply,
    to: reply.to.join(", "),
    cc: reply.cc.join(", "),
    bcc: reply.bcc.join(", "),
    inReplyTo: reply.inReplyTo ?? "",
    references: reply.references.join("\n"),
  }
}
export function completeReply(binding: Reply, fields: ReplyFields): Reply {
  const addresses = (value: string) =>
    value
      .split(",")
      .map((s) => s.trim())
      .filter(Boolean)
  return {
    ...binding,
    ...fields,
    to: addresses(fields.to),
    cc: addresses(fields.cc),
    bcc: addresses(fields.bcc),
    inReplyTo: fields.inReplyTo || null,
    references: fields.references
      .split("\n")
      .map((s) => s.trim())
      .filter(Boolean),
  }
}

export function ReplyEditor({
  fields,
  onChange,
  disabled = false,
}: {
  fields: ReplyFields
  onChange: (fields: ReplyFields) => void
  disabled?: boolean
}) {
  const prefix = useId()
  const input = (key: keyof ReplyFields, label: string, readOnly = false) => (
    <div className="space-y-1.5">
      <label htmlFor={`${prefix}-${key}`} className="text-sm font-medium">
        {label}
      </label>
      <Input
        id={`${prefix}-${key}`}
        value={fields[key]}
        readOnly={readOnly}
        disabled={disabled}
        className="min-h-11 rounded-lg"
        spellCheck={key === "subject"}
        onChange={(e) => onChange({ ...fields, [key]: e.target.value })}
      />
    </div>
  )
  return (
    <div className="space-y-3">
      {input("from", "From · inbox identity", true)}
      {input("to", "To")}
      <div className="grid gap-3 sm:grid-cols-2">
        {input("cc", "Cc")}
        {input("bcc", "Bcc · hidden from other recipients")}
      </div>
      <p className="text-xs text-muted-foreground">
        Use plain email addresses, separated by commas. All recipients above are
        part of the reviewed reply.
      </p>
      {input("subject", "Subject")}
      <div className="space-y-1.5">
        <label htmlFor={`${prefix}-text`} className="text-sm font-medium">
          Reply message
        </label>
        <Textarea
          id={`${prefix}-text`}
          rows={7}
          className="min-h-40 rounded-lg"
          value={fields.text}
          disabled={disabled}
          onChange={(e) => onChange({ ...fields, text: e.target.value })}
        />
      </div>
      <details className="rounded-lg border p-3">
        <summary className="min-h-11 cursor-pointer text-sm font-medium focus-visible:outline-2 focus-visible:outline-ring">
          Thread headers · part of the complete payload
        </summary>
        <div className="space-y-3 pt-3">
          {input("inReplyTo", "In-Reply-To · message ID")}
          <label
            htmlFor={`${prefix}-references`}
            className="block text-sm font-medium"
          >
            References · one message ID per line
          </label>
          <Textarea
            id={`${prefix}-references`}
            value={fields.references}
            disabled={disabled}
            rows={3}
            className="rounded-lg"
            onChange={(e) =>
              onChange({ ...fields, references: e.target.value })
            }
          />
        </div>
      </details>
      <p className="text-xs text-muted-foreground">
        Plain text only. Attachments are not supported in this draft.
      </p>
    </div>
  )
}
