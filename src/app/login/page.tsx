"use client"

import { useEffect, useState } from "react"
import { useRouter } from "next/navigation"
import { useTranslations } from "next-intl"
import { isDesktop } from "@/lib/platform"
import { afterLoginPath } from "@/lib/ops-telegram/locator"

export default function LoginPage() {
  const router = useRouter()
  const t = useTranslations("LoginPage")
  const [token, setToken] = useState("")
  const [error, setError] = useState("")
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    document.title = t("documentTitle")
  }, [t])

  // Desktop users skip login entirely
  if (isDesktop()) {
    router.replace("/workspace")
    return null
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault()
    setError("")
    setLoading(true)

    try {
      // Validate token by calling a lightweight API endpoint
      const res = await fetch("/api/health", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: "{}",
      })

      if (res.ok) {
        localStorage.setItem("codeg_token", token)
        router.replace(afterLoginPath(window.location.search))
      } else if (res.status === 401) {
        setError(t("invalidToken"))
      } else {
        setError(t("connectionFailed", { status: res.status }))
      }
    } catch {
      setError(t("networkError"))
    } finally {
      setLoading(false)
    }
  }

  return (
    <main className="flex min-h-screen items-center justify-center bg-background">
      <div className="w-full max-w-sm space-y-6 px-4">
        <div className="space-y-2 text-center">
          <h1 className="text-2xl font-bold tracking-tight">{t("brand")}</h1>
          <p className="text-sm text-muted-foreground">{t("subtitle")}</p>
        </div>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <label htmlFor="operator-token" className="text-sm font-medium">
              {t("tokenPlaceholder")}
            </label>
            <input
              id="operator-token"
              type="password"
              value={token}
              onChange={(e) => setToken(e.target.value)}
              placeholder={t("tokenPlaceholder")}
              autoFocus
              aria-invalid={!!error}
              aria-describedby={error ? "login-error" : undefined}
              className="flex min-h-11 w-full rounded-md border border-input bg-background px-3 py-2 text-base md:text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            />
          </div>

          {error && (
            <p
              id="login-error"
              role="alert"
              className="text-sm text-foreground"
            >
              {error}
            </p>
          )}

          <button
            type="submit"
            disabled={!token || loading}
            className="inline-flex min-h-11 w-full items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground ring-offset-background transition-colors hover:bg-primary/90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50"
          >
            {loading ? t("connecting") : t("connect")}
          </button>
        </form>

        <p className="text-center text-xs text-muted-foreground">
          {t("helpText")}
        </p>
      </div>
    </main>
  )
}
