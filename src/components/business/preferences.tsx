"use client"

import { useTheme } from "next-themes"
import { useAppI18n } from "@/components/i18n-provider"
import { useBusinessCopy } from "@/lib/business/copy"
import { isAppLocale } from "@/lib/i18n"
import { controlClass } from "./ui"

const languages = {
  en: "English",
  zh_cn: "简体中文",
  zh_tw: "繁體中文",
  ja: "日本語",
  ko: "한국어",
  es: "Español",
  de: "Deutsch",
  fr: "Français",
  pt: "Português",
  ar: "العربية",
}
export function BusinessPreferences() {
  const copy = useBusinessCopy()
  const { appLocale, setLanguageSettings } = useAppI18n()
  const { theme, setTheme } = useTheme()
  return (
    <div className="flex min-w-0 flex-wrap gap-2">
      <select
        aria-label={copy.language}
        value={appLocale}
        className={controlClass + " !w-auto max-w-full"}
        onChange={(event) => {
          if (isAppLocale(event.target.value))
            setLanguageSettings({
              mode: "manual",
              language: event.target.value,
            })
        }}
      >
        {Object.entries(languages).map(([value, label]) => (
          <option key={value} value={value}>
            {label}
          </option>
        ))}
      </select>
      <select
        aria-label={copy.appearance}
        value={theme ?? "system"}
        className={controlClass + " !w-auto max-w-full"}
        onChange={(event) => setTheme(event.target.value)}
      >
        <option value="system">
          {appLocale === "ar" ? "مظهر الجهاز" : "Device appearance"}
        </option>
        <option value="light">{copy.light}</option>
        <option value="dark">{copy.dark}</option>
      </select>
    </div>
  )
}
