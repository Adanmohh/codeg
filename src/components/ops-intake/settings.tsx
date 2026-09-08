"use client"

import { useState } from "react"
import { intake, intakeError } from "@/lib/ops-intake/api"
import type { Product, Status } from "@/lib/ops-intake/types"
import { Action, Choice, Field, Notice, Panel, TextField } from "./ui"

export function IntakeSettings({
  status,
  product,
  onSaved,
}: {
  status: Status
  product?: Product
  onSaved: (status: Status) => void
}) {
  const [binding, setBinding] = useState(
    product?.binding ?? {
      product_id: "hafidh",
      folder_id: 0,
      app_id: "",
      installation_id: 0,
      repository_id: 0,
      full_name: "",
      enabled: true,
    }
  )
  const [origin, setOrigin] = useState(product?.origin ?? "")
  // Credentials stay only in this mounted write-only form; never session/disk state.
  const [bearer, setBearer] = useState("")
  const [key, setKey] = useState("")
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<string | null>(null)
  return (
    <Panel title="Product and repository settings">
      <p className="text-muted-foreground text-sm leading-relaxed">
        Connect the existing Hafidh admin read access and GitHub App
        installation. Saving here does not install an App or create an issue.
      </p>
      {!status.adapter_installed && (
        <Notice>
          The host needs the pinned Hafidh intake package in its own Python
          environment. Set CODEG_INTAKE_PYTHON to that environment’s Python
          executable.
        </Notice>
      )}
      <form
        className="grid gap-4"
        onSubmit={async (event) => {
          event.preventDefault()
          if (busy) return
          setBusy(true)
          setError(null)
          try {
            const saved = await intake.configure({
              binding,
              origin,
              intake_bearer: bearer || null,
              app_private_key: key || null,
            })
            setBearer("")
            setKey("")
            onSaved(saved)
          } catch (error) {
            setError(intakeError(error))
          } finally {
            setBusy(false)
          }
        }}
      >
        <fieldset disabled={busy} className="grid min-w-0 gap-4 sm:grid-cols-2">
          <Field
            label="Product ID"
            value={binding.product_id}
            readOnly={!!product}
            required
            onChange={(e) =>
              setBinding({ ...binding, product_id: e.target.value })
            }
          />
          <Choice
            label="Project folder"
            required
            value={binding.folder_id || ""}
            onChange={(e) =>
              setBinding({ ...binding, folder_id: Number(e.target.value) })
            }
          >
            <option value="">Choose an existing project</option>
            {status.folders.map((f) => (
              <option key={f.id} value={f.id}>
                {f.name}
              </option>
            ))}
          </Choice>
          <Field
            label="Hafidh origin"
            type="url"
            value={origin}
            required
            onChange={(e) => setOrigin(e.target.value)}
            hint="HTTPS origin only, without /api/v1."
          />
          <Field
            label="GitHub repository"
            value={binding.full_name}
            required
            onChange={(e) =>
              setBinding({ ...binding, full_name: e.target.value })
            }
            hint="Exact owner/repository from the existing installation."
          />
          <Field
            label="GitHub App ID"
            value={binding.app_id}
            required
            onChange={(e) => setBinding({ ...binding, app_id: e.target.value })}
          />
          <Field
            label="Installation ID"
            type="number"
            min="1"
            value={binding.installation_id || ""}
            required
            onChange={(e) =>
              setBinding({
                ...binding,
                installation_id: Number(e.target.value),
              })
            }
          />
          <Field
            label="Repository ID"
            type="number"
            min="1"
            value={binding.repository_id || ""}
            required
            onChange={(e) =>
              setBinding({ ...binding, repository_id: Number(e.target.value) })
            }
          />
          <Choice
            label="Product access"
            value={binding.enabled ? "enabled" : "disabled"}
            onChange={(e) =>
              setBinding({ ...binding, enabled: e.target.value === "enabled" })
            }
          >
            <option value="enabled">Enabled</option>
            <option value="disabled">Disabled</option>
          </Choice>
        </fieldset>
        <fieldset disabled={busy} className="grid min-w-0 gap-4">
          <Field
            label="Hafidh admin bearer"
            type="password"
            autoComplete="off"
            value={bearer}
            onChange={(e) => setBearer(e.target.value)}
            hint={
              product?.intake_credential_present
                ? "Stored. Leave blank to keep it; enter a value to rotate it."
                : "Required for TestFlight reads. Stored separately from product data."
            }
          />
          <TextField
            label="GitHub App private key (PEM)"
            autoComplete="off"
            spellCheck={false}
            value={key}
            onChange={(e) => setKey(e.target.value)}
            hint={
              product?.app_key_present
                ? "Stored. Leave blank to keep it; enter a value to rotate it."
                : "Required to file. Use the App’s RSA private key, never a personal access token."
            }
          />
        </fieldset>
        {error && <Notice error>{error}</Notice>}
        <div className="flex flex-wrap items-center gap-3">
          <Action type="submit" disabled={busy}>
            {busy ? "Saving settings…" : "Save settings"}
          </Action>
          <span className="text-muted-foreground text-sm">
            After saving, refresh the source before reviewing evidence.
          </span>
        </div>
      </form>
    </Panel>
  )
}
