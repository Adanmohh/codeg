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
  attaching: "Connecting to this saved session…",
  attached: "Connected to this session",
  disconnected:
    "The view is disconnected. Check the session to reconnect. No prompt is repeated.",
  reconnect: "Check session and reconnect",
  emptyConversation:
    "This session has no saved messages yet. Send a prompt to begin.",
  recentOnly:
    "Showing recent messages. Browse the saved conversation for older work.",
  savedConversation: "Browse saved conversation",
  returnLive: "Return to conversation",
  olderMessages: "Older messages",
  newerMessages: "Newer messages",
  loadingHistory: "Loading saved messages…",
  historyEmpty: "No saved messages on this page.",
  you: "You",
  assistant: "AI assistant",
  activity: "Session activity",
  starting: "Starting",
  idle: "Ready",
  running: "Working",
  awaiting_input: "Waiting for input",
  stopped: "Stopped",
  failed: "Failed",
  interrupted: "Interrupted",
  revoked: "Access ended",
  closed: "Closed",
  completed: "Completed",
  currentAccessRequired:
    "Current session access is required. Your task and submitted files are separate from this private conversation.",
  aiWorkspace: "AI workspace",
  aiHint:
    "Work with AI beside this task. Conversations stay private; submit selected files separately for human review.",
  operatorOnly:
    "AI sessions currently require the original operator's browser connection. Submitted task files remain available through your own task access.",
  loadingSessions: "Checking available clients and saved sessions…",
  chooseProfile: "Choose an AI profile",
  profilePrompt: "Select an available profile",
  startConversation: "Start conversation",
  openConversation: "Open conversation",
  continueSession: "Continue session",
  savedSessions: "Your saved sessions",
  noSessions: "No saved sessions for this task yet.",
  noProfiles:
    "No AI profile is available for this task. Configure an installed client with the original operator's engineering tools, then check again.",
  managedOutputUnavailable:
    "This profile does not report managed output support.",
  terminalPending: "This connection cannot display this terminal session yet.",
  savedTaskRevision: "Saved task revision",
  moreSessions: "More saved sessions",
  requestNotConfirmed:
    "This request is not confirmed. Check the original receipt before starting or continuing again.",
  requestFailed:
    "This request was refused. Review your selection before making another request.",
  reviewRequest: "Review selection",
  pendingPane:
    "Resolve the pending request here before closing this pane. No request will be repeated automatically.",
  taskSubmitRequired:
    "Current task contribution permission is required to start a session.",
  profileReady: "Ready",
  profileBlocked: "Needs setup",
  documents: "Documents & assets",
  documentsHint:
    "Keep exact file versions with this task. Importing a file does not share it or complete the work.",
  loadingAssets: "Checking saved files…",
  findFiles: "Find a saved file",
  searchFiles: "Search files",
  noFiles: "No retained files for this task yet.",
  noMatchingFiles: "No saved files match this search.",
  fileVersions: "Saved versions",
  version: "Version",
  privateVersion: "Private version",
  sharedVersion: "Previously submitted to this task",
  privatePreviewUnavailable:
    "This version has no supported text preview. A download is offered only when current file access permits it.",
  selectVersion: "Select this version for review",
  removeVersion: "Remove from selection",
  selectedVersions: "Selected file versions",
  sessionOutput: "Files from this session",
  outputHint:
    "Inspect files from the session's own workspace, then retain a selected file. No file is imported automatically.",
  checkOutput: "Check session files",
  noOutputs: "No available files were returned for this session.",
  outputChanged: "Changed — check files again",
  outputUnsupported: "Cannot retain this file",
  retainOutput: "Retain file",
  importTitle: "Retain this session file",
  retainedTitle: "File title",
  importDestination: "Save as",
  newAsset: "A new saved file",
  addVersion: "Add a version to",
  importHint:
    "The service copies and verifies this exact output revision. The retained version stays private until a separate submission.",
  moreFiles: "More saved files",
  moreVersions: "More versions",
  moreOutputs: "More session files",
  submission: "Submit selected files",
  submissionBody: "Note for the reviewer",
  reviewSubmission: "Review files and audience",
  submissionAudience: "Destination audience",
  audienceHint:
    "These exact versions and your note will be visible to everyone with access to this task in this area of work. The private conversation and unselected versions are not included.",
  confirmSubmission:
    "I have reviewed these exact file versions, the note and this task's audience.",
  submitFiles: "Submit files for human review",
  submitted:
    "The selected versions were submitted for human review. They are not approved or marked done.",
  openTask: "Open task",
  recheckSelection: "Recheck current files and task",
  selectionLimit:
    "Select 1–16 exact versions and use at most 20,000 characters in the note.",
  selectionChanged:
    "The selection or note changed. Review the exact files and current task audience again.",
  waitingImport:
    "This import is not confirmed. Keep this selection and check the original receipt; no file will be imported again automatically.",
  waitingSubmission:
    "This submission is not confirmed. Its exact versions, note and task revision are kept here. Check the original receipt before another submission.",
  assetUnavailable:
    "This version is no longer available under your current access. Check the files again.",
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
  attaching: "جارٍ الاتصال بهذه الجلسة المحفوظة…",
  attached: "متصل بهذه الجلسة",
  disconnected:
    "انقطع اتصال العرض. تحقق من الجلسة لإعادة الاتصال. لن يُعاد إرسال أي طلب.",
  reconnect: "التحقق من الجلسة وإعادة الاتصال",
  emptyConversation:
    "لا توجد رسائل محفوظة في هذه الجلسة بعد. أرسل طلبًا للبدء.",
  recentOnly:
    "تُعرض الرسائل الحديثة. تصفّح المحادثة المحفوظة للاطلاع على العمل السابق.",
  savedConversation: "تصفّح المحادثة المحفوظة",
  returnLive: "العودة إلى المحادثة",
  olderMessages: "رسائل أقدم",
  newerMessages: "رسائل أحدث",
  loadingHistory: "جارٍ تحميل الرسائل المحفوظة…",
  historyEmpty: "لا توجد رسائل محفوظة في هذه الصفحة.",
  you: "أنت",
  assistant: "مساعد الذكاء الاصطناعي",
  activity: "نشاط الجلسة",
  starting: "جارٍ البدء",
  idle: "جاهز",
  running: "يعمل الآن",
  awaiting_input: "بانتظار إدخال",
  stopped: "متوقف",
  failed: "فشل",
  interrupted: "انقطعت الجلسة",
  revoked: "انتهى الوصول",
  closed: "مغلقة",
  completed: "مكتمل",
  currentAccessRequired:
    "يلزم وصول سارٍ إلى الجلسة. المهمة وملفاتها المقدّمة منفصلة عن هذه المحادثة الخاصة.",
  aiWorkspace: "مساحة الذكاء الاصطناعي",
  aiHint:
    "اعمل مع الذكاء الاصطناعي بجانب هذه المهمة. تبقى المحادثات خاصة، وتُقدَّم الملفات المختارة للمراجعة البشرية بشكل منفصل.",
  operatorOnly:
    "تتطلب جلسات الذكاء الاصطناعي حاليًا اتصال المشغّل الأصلي عبر المتصفح. تبقى ملفات المهام المقدّمة متاحة وفق صلاحياتك الخاصة.",
  loadingSessions: "جارٍ التحقق من الأدوات المتاحة والجلسات المحفوظة…",
  chooseProfile: "اختيار ملف إعداد للذكاء الاصطناعي",
  profilePrompt: "اختر ملف إعداد متاحًا",
  startConversation: "بدء محادثة",
  openConversation: "فتح المحادثة",
  continueSession: "متابعة الجلسة",
  savedSessions: "جلساتك المحفوظة",
  noSessions: "لا توجد جلسات محفوظة لهذه المهمة بعد.",
  noProfiles:
    "لا يوجد ملف إعداد متاح لهذه المهمة. أعدّ أداة مثبّتة من أدوات الهندسة بحساب المشغّل الأصلي، ثم تحقق مجددًا.",
  managedOutputUnavailable: "لا يوفّر ملف الإعداد هذا دعمًا للمخرجات المُدارة.",
  terminalPending: "لا يمكن لهذا الاتصال عرض جلسة الطرفية هذه بعد.",
  savedTaskRevision: "مراجعة المهمة المحفوظة",
  moreSessions: "المزيد من الجلسات المحفوظة",
  requestNotConfirmed:
    "لم يتأكد هذا الطلب بعد. تحقق من سجل الاستلام الأصلي قبل البدء أو المتابعة مجددًا.",
  requestFailed: "رُفض هذا الطلب. راجع اختيارك قبل تقديم طلب آخر.",
  reviewRequest: "مراجعة الاختيار",
  pendingPane:
    "تحقق من نتيجة الطلب المعلّق هنا قبل إغلاق اللوحة. لن يُعاد أي طلب تلقائيًا.",
  taskSubmitRequired:
    "يلزم امتلاك صلاحية المساهمة الحالية في المهمة لبدء جلسة.",
  profileReady: "جاهز",
  profileBlocked: "يحتاج إلى إعداد",
  documents: "المستندات والملفات",
  documentsHint:
    "احتفظ بنسخ محددة من الملفات مع هذه المهمة. الاحتفاظ بملف لا يشاركه ولا يُكمل العمل.",
  loadingAssets: "جارٍ التحقق من الملفات المحفوظة…",
  findFiles: "البحث عن ملف محفوظ",
  searchFiles: "البحث في الملفات",
  noFiles: "لا توجد ملفات محتفظ بها لهذه المهمة بعد.",
  noMatchingFiles: "لا توجد ملفات محفوظة تطابق هذا البحث.",
  fileVersions: "النسخ المحفوظة",
  version: "النسخة",
  privateVersion: "نسخة خاصة",
  sharedVersion: "قُدّمت سابقًا إلى هذه المهمة",
  privatePreviewUnavailable:
    "لا تتوفر معاينة نصية مدعومة لهذه النسخة. يتاح التنزيل فقط عندما تسمح صلاحيات الوصول الحالية.",
  selectVersion: "اختيار هذه النسخة للمراجعة",
  removeVersion: "إزالة من الاختيار",
  selectedVersions: "نسخ الملفات المختارة",
  sessionOutput: "ملفات هذه الجلسة",
  outputHint:
    "اطّلع على ملفات مساحة هذه الجلسة ثم احتفظ بملف تختاره. لا يُحتفظ بأي ملف تلقائيًا.",
  checkOutput: "التحقق من ملفات الجلسة",
  noOutputs: "لم تُرجع هذه الجلسة ملفات متاحة.",
  outputChanged: "تغيّر — تحقق من الملفات مجددًا",
  outputUnsupported: "لا يمكن الاحتفاظ بهذا الملف",
  retainOutput: "الاحتفاظ بالملف",
  importTitle: "الاحتفاظ بملف الجلسة هذا",
  retainedTitle: "عنوان الملف",
  importDestination: "الحفظ كـ",
  newAsset: "ملف محفوظ جديد",
  addVersion: "إضافة نسخة إلى",
  importHint:
    "تنسخ الخدمة مراجعة المخرج المحددة وتتحقق منها. تبقى النسخة المحتفظ بها خاصة حتى تُقدَّم بشكل منفصل.",
  moreFiles: "المزيد من الملفات المحفوظة",
  moreVersions: "المزيد من النسخ",
  moreOutputs: "المزيد من ملفات الجلسة",
  submission: "تقديم الملفات المختارة",
  submissionBody: "ملاحظة للمراجع",
  reviewSubmission: "مراجعة الملفات والجمهور",
  submissionAudience: "الجمهور المستهدف",
  audienceHint:
    "ستظهر هذه النسخ المحددة وملاحظتك لكل من لديه وصول إلى هذه المهمة في مجال العمل هذا. لا تشمل المشاركة المحادثة الخاصة أو النسخ غير المختارة.",
  confirmSubmission: "راجعت نسخ الملفات المحددة والملاحظة وجمهور هذه المهمة.",
  submitFiles: "تقديم الملفات للمراجعة البشرية",
  submitted:
    "قُدّمت النسخ المختارة للمراجعة البشرية. لم تُعتمد ولم تُعلَّم كمكتملة.",
  openTask: "فتح المهمة",
  recheckSelection: "التحقق من الملفات والمهمة الحالية",
  selectionLimit:
    "اختر من نسخة إلى ١٦ نسخة محددة، واكتب ملاحظة لا تتجاوز ٢٠٬٠٠٠ حرف.",
  selectionChanged:
    "تغيّر الاختيار أو الملاحظة. راجع الملفات المحددة وجمهور المهمة الحالي مجددًا.",
  waitingImport:
    "لم يتأكد الاحتفاظ بالملف بعد. أبقِ هذا الاختيار وتحقق من سجل الاستلام الأصلي. لن يُعاد الاحتفاظ بالملف تلقائيًا.",
  waitingSubmission:
    "لم يتأكد هذا التقديم بعد. حُفظت نسخه المحددة وملاحظته ومراجعة المهمة هنا. تحقق من سجل الاستلام الأصلي قبل تقديم آخر.",
  assetUnavailable:
    "لم تعد هذه النسخة متاحة بصلاحياتك الحالية. تحقق من الملفات مجددًا.",
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
