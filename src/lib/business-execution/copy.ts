"use client"

import { useLocale } from "next-intl"
import { ExecutionError } from "./protocol"

const english = {
  conversation: "AI conversation",
  prompt: "Prompt",
  promptHint: "Describe the next step. Shift + Enter adds a line.",
  send: "Send prompt",
  sending: "Waiting for the receipt…",
  checking: "Checking the receipt…",
  checkReceipt: "Check receipt",
  uncertain:
    "The outcome is not confirmed. Your exact prompt is kept here. Check its receipt before sending another.",
  accepted: "Prompt accepted. The response will appear in the conversation.",
  refused:
    "The request was not accepted. Review the prompt before submitting again.",
  reviewPrompt: "Review prompt",
  context: "Include with this prompt",
  noContext: "Only the prompt text is sent unless you select context below.",
  contextChanged:
    "A selected reference changed. Review the current selection before sending.",
  clearContext: "Clear selection",
  tooLong: "Use at most 32,000 characters and 16 context references.",
  unavailable: "This action is not available for this session.",
  denied:
    "You no longer have access to this work. Refresh the workspace to check your current access.",
  conflict: "This work changed. Refresh it before making another change.",
  offline:
    "The session could not be reached. No request will be repeated automatically.",
  busy: "Another action is still in progress.",
  setup: "This session needs a configured, available AI client.",
  receipt: "Request receipt",
  detach: "Close this pane",
  detachHint:
    "Closing a pane detaches the view. It does not stop the AI session.",
  submittedFiles: "Submitted files",
  selectedFile: "Submitted file",
  submittedVersion: "The exact version submitted for review",
  readFile: "Read file",
  download: "Prepare download",
  saveFile: "Save file",
  checkingFile: "Checking this submitted version…",
  loadingFile: "Loading verified file content…",
  retry: "Check access again",
  bytes: "bytes",
  previewUnavailable:
    "Preview is not available for this format. You can download the submitted version.",
  previewTooLarge:
    "This file is too large for the text reader. Download it to read the full content.",
  fileUnavailable: "This submitted file is currently unavailable.",
  fileInformation: "File information",
  submittedBy: "Submitted by",
  taskRevision: "Task revision at submission",
  createdWith: "Created with",
  modelUnavailable: "Model not recorded",
  verifiedDownload: "The selected version is verified and ready to save.",
}
export type ExecutionCopy = typeof english
const arabic: ExecutionCopy = {
  conversation: "محادثة الذكاء الاصطناعي",
  prompt: "الطلب",
  promptHint: "صف الخطوة التالية. اضغط Shift + Enter لإضافة سطر.",
  send: "إرسال الطلب",
  sending: "بانتظار تأكيد الاستلام…",
  checking: "جارٍ التحقق من الاستلام…",
  checkReceipt: "التحقق من الاستلام",
  uncertain:
    "لم تتأكد النتيجة بعد. طلبك محفوظ هنا كما كتبته. تحقق من سجل الاستلام قبل إرسال طلب آخر.",
  accepted: "تم استلام الطلب. ستظهر الإجابة في المحادثة.",
  refused: "لم يُقبل الطلب. راجعه قبل إرساله مجددًا.",
  reviewPrompt: "مراجعة الطلب",
  context: "إرفاق سياق مع الطلب",
  noContext: "سيُرسل نص الطلب فقط ما لم تختَر سياقًا أدناه.",
  contextChanged:
    "تغيّر أحد المراجع المختارة. راجع الاختيار الحالي قبل الإرسال.",
  clearContext: "مسح الاختيار",
  tooLong: "الحد الأقصى ٣٢٬٠٠٠ حرف و١٦ مرجعًا للسياق.",
  unavailable: "هذا الإجراء غير متاح لهذه الجلسة.",
  denied:
    "لم تعد لديك صلاحية الوصول إلى هذا العمل. حدّث مساحة العمل للتحقق من صلاحياتك الحالية.",
  conflict: "تغيّر هذا العمل. حدّثه قبل إجراء تغيير آخر.",
  offline: "تعذّر الوصول إلى الجلسة. لن يُعاد أي طلب تلقائيًا.",
  busy: "ما زال إجراء آخر قيد التنفيذ.",
  setup: "تحتاج هذه الجلسة إلى أداة ذكاء اصطناعي مُعدّة ومتاحة.",
  receipt: "سجل استلام الطلب",
  detach: "إغلاق هذه اللوحة",
  detachHint: "إغلاق اللوحة يفصل العرض ولا يوقف جلسة الذكاء الاصطناعي.",
  submittedFiles: "الملفات المقدّمة",
  selectedFile: "الملف المقدّم",
  submittedVersion: "النسخة نفسها التي قُدّمت للمراجعة",
  readFile: "قراءة الملف",
  download: "تجهيز التنزيل",
  saveFile: "حفظ الملف",
  checkingFile: "جارٍ التحقق من النسخة المقدّمة…",
  loadingFile: "جارٍ تحميل محتوى الملف والتحقق منه…",
  retry: "التحقق من الوصول مجددًا",
  bytes: "بايت",
  previewUnavailable:
    "معاينة هذا التنسيق غير متاحة. يمكنك تنزيل النسخة المقدّمة.",
  previewTooLarge:
    "هذا الملف أكبر من حدّ قارئ النصوص. نزّله لقراءة المحتوى كاملًا.",
  fileUnavailable: "الملف المقدّم غير متاح حاليًا.",
  fileInformation: "معلومات الملف",
  submittedBy: "قدّمه",
  taskRevision: "مراجعة المهمة عند التقديم",
  createdWith: "أُنشئ باستخدام",
  modelUnavailable: "النموذج غير مسجّل",
  verifiedDownload: "تم التحقق من النسخة المختارة وهي جاهزة للحفظ.",
}
export function useExecutionCopy(): ExecutionCopy {
  return useLocale() === "ar" ? arabic : english
}
export function executionErrorMessage(error: unknown, copy: ExecutionCopy) {
  if (!(error instanceof ExecutionError)) return copy.offline
  switch (error.reason) {
    case "unauthorized":
    case "forbidden":
    case "authority_changed":
      return copy.denied
    case "missing":
    case "unavailable":
    case "cancelled":
    case "content_unavailable":
      return copy.unavailable
    case "conflict":
    case "content_changed":
      return copy.conflict
    case "busy":
    case "rate_limited":
      return copy.busy
    case "setup_required":
      return copy.setup
    default:
      return copy.offline
  }
}
