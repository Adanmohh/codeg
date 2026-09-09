"use client"

import { useLocale } from "next-intl"

const en = {
  title: "Workspace appearance",
  hint: "Shared defaults for this organization. Your personal language and light or dark mode stay separate.",
  name: "Workspace display name",
  palette: "Workspace colors",
  neutral: "Hafidh",
  blue: "Blue",
  violet: "Violet",
  layout: "Preferred pane layout",
  split: "Side by side",
  stacked: "Stacked",
  defaultArea: "Start in",
  tasks: "Tasks",
  conversations: "Conversations",
  conversationHint:
    "This preference does not enable AI tools. Tasks remain available when this connection has no authorized conversation surface.",
  saved: "Workspace defaults saved.",
  save: "Save workspace defaults",
  saving: "Saving…",
  readOnly:
    "You can see these defaults. A workspace owner or administrator can change them.",
  conflict:
    "The saved defaults may have changed. Your draft is kept here; compare the current version before saving again.",
  load: "Load current defaults",
  current: "Current saved defaults",
  base: "Your draft's base revision",
  revision: "Revision",
  keep: "Use my draft with this version",
  useSaved: "Use the saved defaults",
  failed: "The workspace defaults could not be loaded or saved.",
  forbidden: "Your access does not allow this action.",
  staleScope: "These defaults are unavailable in this workspace.",
  invalid: "Check the display name and selected defaults.",
}
type Copy = { [K in keyof typeof en]: string }
const ar: Copy = {
  title: "مظهر مساحة العمل",
  hint: "إعدادات افتراضية مشتركة لهذه المؤسسة. تبقى لغتك الشخصية واختيار المظهر الفاتح أو الداكن منفصلين.",
  name: "اسم العرض لمساحة العمل",
  palette: "ألوان مساحة العمل",
  neutral: "حافظ",
  blue: "أزرق",
  violet: "بنفسجي",
  layout: "ترتيب مساحات العمل المفضل",
  split: "جنبًا إلى جنب",
  stacked: "فوق بعضها",
  defaultArea: "ابدأ في",
  tasks: "المهام",
  conversations: "المحادثات",
  conversationHint:
    "هذا التفضيل لا يفعّل أدوات الذكاء الاصطناعي. تبقى المهام متاحة عندما لا يتيح هذا الاتصال محادثات مصرّحًا بها.",
  saved: "حُفظت الإعدادات الافتراضية لمساحة العمل.",
  save: "حفظ الإعدادات الافتراضية",
  saving: "جارٍ الحفظ…",
  readOnly:
    "يمكنك الاطلاع على هذه الإعدادات. يستطيع مالك مساحة العمل أو مسؤولها تغييرها.",
  conflict:
    "ربما تغيّرت الإعدادات المحفوظة. مسودتك محفوظة هنا؛ قارن النسخة الحالية قبل الحفظ مجددًا.",
  load: "تحميل الإعدادات الحالية",
  current: "الإعدادات المحفوظة الحالية",
  base: "رقم النسخة التي تستند إليها مسودتك",
  revision: "النسخة",
  keep: "استخدام مسودتي مع هذه النسخة",
  useSaved: "استخدام الإعدادات المحفوظة",
  failed: "تعذّر تحميل الإعدادات الافتراضية أو حفظها.",
  forbidden: "صلاحياتك لا تسمح بهذا الإجراء.",
  staleScope: "هذه الإعدادات غير متاحة في مساحة العمل هذه.",
  invalid: "راجع اسم العرض والإعدادات المختارة.",
}
export function useSettingsCopy(): Copy {
  return useLocale() === "ar" ? ar : en
}
