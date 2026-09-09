"use client"

import { useLocale } from "next-intl"

const english = {
  sources: "Sources",
  heading: "Turn what you heard into clear work.",
  introduction:
    "Read the source, choose the exact passage, then decide what belongs in shared work.",
  private: "Private source review",
  privateHint:
    "Only people with explicit source access can read these passages and drafts. Task access is separate.",
  connections: "Source connections",
  connection: "Source connection",
  empty: "No sources available yet.",
  emptyHint:
    "A workspace administrator can connect a source and grant access to the people who need it.",
  emptySource: "No imported records in this connection.",
  emptySourceHint:
    "Import a bounded set of records to begin reading. Nothing is turned into a task automatically.",
  setup: "Set up a source",
  reviewSetup: "Review access and setup",
  contactOperator:
    "Ask your workspace administrator to review access and setup.",
  fireflies: "Fireflies meetings",
  email: "Email correspondence",
  hafidh_testflight: "Hafidh feedback",
  enabled: "Enabled",
  disabled: "Disabled",
  credentialPresent: "Credential stored",
  credentialMissing: "Credential missing",
  sourceType: "Source type",
  label: "Connection name",
  sourceOwner: "Source account owner",
  sourceOwnerHint:
    "Choose the active person whose account owns these records. Ownership cannot be changed after creation.",
  apiKey: "Fireflies API key",
  secretHint:
    "Write-only. Stored by the protected backend; never shown again here.",
  inboxId: "Existing inbox number",
  productId: "Existing Hafidh product identifier",
  existingSetup:
    "Use an existing operator-configured account. This screen does not create an inbox, product or provider account.",
  createConnection: "Create disabled connection",
  disabledFirst:
    "A new connection is disabled and grants nobody access, including its owner. Add explicit grants before enabling source use.",
  enable: "Enable connection",
  disable: "Disable connection",
  revalidate: "Revalidate access and setup",
  revalidateHint:
    "Recheck the same account owner and protected configuration. Source records still require a fresh read afterwards.",
  replaceCredential: "Replace stored Fireflies key (optional)",
  publicationAreas: "Allowed task destinations",
  publicationHint:
    "These areas are a ceiling. Each person also needs an explicit grant and current task permissions.",
  removeUnavailableDestinations:
    "Remove destinations outside your current setup access before saving.",
  retainedConsent:
    "I allow explicitly accepted task text to remain in the chosen work area, even if source access is later removed.",
  grants: "People with source access",
  noGrants: "Nobody has an explicit grant yet.",
  grantPerson: "Person receiving access",
  grantRead: "Read retained and newly imported source records",
  grantImport: "Import and refresh source records",
  grantTriage: "Prepare and decide on source drafts",
  grantConfirm:
    "I am granting this person access through this exact connection to all retained historical versions, including those imported before this grant, and to current and future imports.",
  grantSave: "Save explicit access",
  grantRevoke: "Revoke source access",
  grantExpires: "Expires at (UTC, optional)",
  grantExpiryHint:
    "An exact UTC instant, for example 2026-12-31T18:00:00Z. Leave empty for no expiry.",
  revoked: "Revoked",
  grantActive: "Active grant",
  setupUncertain:
    "This setup change could not be confirmed. Check current setup or retry the exact same operation; do not create a replacement connection.",
  retryExact: "Retry this exact operation",
  importRecords: "Import records",
  captureRecord: "Read an existing record",
  importHint:
    "Choose a UTC window of at most 31 days. Each step reads a bounded page; coverage and failures stay visible.",
  from: "From (UTC)",
  to: "Until (UTC)",
  utcHint:
    "UTC date and time; this is an import boundary, not a task deadline.",
  conversationId: "Existing conversation number",
  messageId: "Existing message number",
  feedbackId: "Existing feedback reference (ULID)",
  captureHint:
    "Reads only the permitted content of this existing record. No email, feedback or provider record is changed.",
  imports: "Import progress",
  importHistory: "All import attempts",
  unfinished: "Unfinished imports",
  noImports: "No import attempts in this view.",
  startImport: "Start bounded import",
  advance: "Read next step",
  cancelImport: "Cancel import",
  checkImport: "Check import status",
  discovered: "Found",
  completed: "Read",
  failedCount: "Failed",
  nextAttempt: "Next read available after",
  coverage: "Coverage",
  partialHint:
    "This import has partial coverage. Records already read remain available; completion does not imply the whole account was imported.",
  importUnknown:
    "The last read could not be confirmed. Check its status before choosing another step. A running claim cannot be retried as a new read.",
  sourceRefresh: "Refresh source access",
  sourceRefreshHint:
    "Read this record again through its connection. A cached page cannot renew access.",
  unavailable: "Source content is currently withheld.",
  unavailableHint:
    "Refresh upstream access before reading or preparing a task. Cached private text is not shown while access is stale.",
  sourceRevision: "Source version",
  noRevision: "Not yet validated",
  exactPassage: "Exact source passage",
  selectHint:
    "Choose 1–20 passages, up to 20,000 characters in total. Nothing is shortened or rewritten for you.",
  noPassages: "This record has no supported passage to select.",
  summaryEmpty: "The provider returned an empty summary.",
  summaryMissing: "No summary was supplied.",
  selected: "Selected passages",
  prepare: "Prepare private draft",
  selectedInvalid:
    "Select 1–20 passages from this source version, with at most 20,000 characters in total.",
  candidates: "Private drafts & decisions",
  noCandidates: "No drafts in this view.",
  pending: "Pending",
  accepted: "Task created",
  linked: "Linked to existing task",
  discarded: "Discarded",
  draft: "Private task draft",
  draftHint:
    "Write the task that should be shared. Saving here does not publish it or assign work.",
  unprepared: "Passages selected; task not prepared yet.",
  noDestination:
    "No task destination is available with your current source and task permissions.",
  scopeConflict:
    "This source operation changed or its revision is no longer current. Load its latest state before trying again.",
  scopeMissing:
    "This source or source operation is unavailable with your current access.",
  withheldDraft:
    "A prepared draft exists, but its contents are withheld by current source or destination access.",
  saveDraft: "Save private draft",
  ownerSuggestion: "Owner mentioned in source (optional)",
  dueSuggestion: "Deadline mentioned in source (optional)",
  suggestionHint: "Context only. This does not assign a person or set a date.",
  reviewDraft: "Review before sharing",
  audience: "Destination audience",
  audienceHint:
    "Everyone with current or future Read access to this area can read the complete task title, brief, responsibility, due date and activity. Source grants do not limit that task audience.",
  retainHint:
    "Accepted task text remains in this area even if the original source or its access is later removed. Only the source reference stays private to source-authorized readers.",
  acceptConfirm:
    "I have reviewed these exact passages, the complete task and this destination audience. I explicitly approve sharing this task here.",
  acceptTask: "Accept into shared work",
  link: "Link an existing task",
  linkHint:
    "Adds a private source reference only. No passage or draft text is copied. Linking changes task metadata and returns work in Review to In progress, requiring a new review.",
  findTarget: "Find an existing task",
  chooseTarget: "Review this task",
  noTargets: "No editable active tasks match this destination.",
  targetVersion: "Exact target task",
  linkConfirm:
    "I reviewed this exact task and destination. I approve adding the private reference without copying source text and understand that any pending task review is invalidated.",
  confirmLink: "Confirm text-free link",
  discard: "Discard candidate",
  discardHint:
    "Keep a decision record without creating or changing a task. This does not remove the source.",
  discardConfirm: "Discard this candidate without publishing a task.",
  outcome: "Decision recorded",
  openTask: "Open shared task",
  restrictedTask:
    "A task decision was recorded. The target is not accessible with your current task permissions.",
  unknown:
    "The decision could not be confirmed. Do not submit a new decision. Check the saved outcome using this candidate.",
  recover: "Check saved outcome",
  conflict:
    "This candidate or source changed. Your local draft has not been applied.",
  loadCurrent: "Load current source and candidate",
  draftBase: "Your draft is based on",
  currentSaved: "Current saved candidate",
  adopt: "Use current version and keep my draft",
  adoptSaved: "Use the current saved draft",
  rebase: "Review and reselect current passages",
  rebaseHint:
    "Content or source access changed. Compare the current version, explicitly select its passages, then save again before accepting or linking.",
  confirmSelection: "Confirm this passage selection",
  editsKept:
    "Local edits are kept in this window. Save the complete draft again before sharing.",
  leaveDraft: "Leave this source review?",
  leavePending:
    "Wait for the current operation or recover its saved outcome before leaving this review.",
  leaveHint:
    "Unsaved edits in this window will be discarded. Saved private drafts can be reopened from Sources.",
  taskSources: "Source references",
  privateReference: "Private source reference",
  openSource: "Open source review",
  noTaskSources: "No source references linked.",
  reason: {
    binding_missing: "This connection is unavailable. Review access and setup.",
    binding_disabled:
      "This connection is disabled. Ask a workspace administrator to review setup.",
    credential_unavailable:
      "The protected credential is unavailable. Ask a workspace administrator to review setup.",
    binding_unavailable:
      "Source use is paused. Review access and setup, then refresh the source.",
    source_expired:
      "Source access has expired. Refresh the record before continuing.",
    rebase_required:
      "Source content or access changed. Review and reselect the current passages.",
    import_busy:
      "Another read is already in progress. Check its status before continuing.",
    retry_later:
      "The provider asked us to wait. Use the next available read time.",
    source_denied: "The source is no longer readable with current access.",
    publication_not_allowed:
      "Current permissions do not allow this destination. Review source and task access.",
    unsupported_schema:
      "This source format is not supported. No task was prepared from it.",
    provider_unavailable:
      "The source provider could not be read. Your workspace session remains connected.",
    request_timeout:
      "The read could not finish within its time limit. Check its saved status before another step.",
  },
  importState: {
    queued: "Queued",
    running: "Reading",
    waiting: "Waiting",
    complete: "Finished",
    failed: "Failed",
    cancelled: "Cancelled",
  },
  coverageState: {
    not_started: "Not started",
    partial: "Partial",
    bounded_end: "Selected boundary reached",
    capped: "Import limit reached",
  },
}
type IntakeCopy = typeof english
const arabic: IntakeCopy = {
  sources: "المصادر",
  heading: "حوّل ما سمعته إلى عمل واضح.",
  introduction:
    "اقرأ المصدر، وحدّد المقطع الدقيق، ثم قرّر ما ينبغي مشاركته كعمل.",
  private: "مراجعة خاصة للمصدر",
  privateHint:
    "لا يقرأ هذه المقاطع والمسودات إلا من مُنح صلاحية صريحة للمصدر. صلاحية المهمة منفصلة.",
  connections: "اتصالات المصادر",
  connection: "اتصال المصدر",
  empty: "لا توجد مصادر متاحة بعد.",
  emptyHint: "يستطيع مسؤول مساحة العمل ربط مصدر ومنح الوصول لمن يحتاجه.",
  emptySource: "لا توجد سجلات مستوردة في هذا الاتصال.",
  emptySourceHint:
    "استورد مجموعة محدودة من السجلات لبدء القراءة. لا يتحوّل أي شيء إلى مهمة تلقائيًا.",
  setup: "إعداد مصدر",
  reviewSetup: "مراجعة الوصول والإعداد",
  contactOperator: "اطلب من مسؤول مساحة العمل مراجعة الوصول والإعداد.",
  fireflies: "اجتماعات Fireflies",
  email: "المراسلات البريدية",
  hafidh_testflight: "ملاحظات حافظ",
  enabled: "مفعّل",
  disabled: "معطّل",
  credentialPresent: "بيانات الاعتماد محفوظة",
  credentialMissing: "بيانات الاعتماد مفقودة",
  sourceType: "نوع المصدر",
  label: "اسم الاتصال",
  sourceOwner: "مالك حساب المصدر",
  sourceOwnerHint:
    "اختر الشخص النشط الذي يملك حساب هذه السجلات. لا يمكن تغيير المالك بعد الإنشاء.",
  apiKey: "مفتاح Fireflies API",
  secretHint: "للإدخال فقط. يحفظه الخادم المحمي ولا يُعرض هنا مجددًا.",
  inboxId: "رقم صندوق بريد قائم",
  productId: "معرّف منتج حافظ القائم",
  existingSetup:
    "استخدم حسابًا سبق أن أعدّه المسؤول. هذه الشاشة لا تنشئ صندوقًا أو منتجًا أو حسابًا لدى المزوّد.",
  createConnection: "إنشاء اتصال معطّل",
  disabledFirst:
    "يبدأ الاتصال معطّلًا دون منح أي شخص الوصول، حتى مالكه. أضف منحًا صريحة قبل التفعيل.",
  enable: "تفعيل الاتصال",
  disable: "تعطيل الاتصال",
  revalidate: "إعادة التحقق من الوصول والإعداد",
  revalidateHint:
    "أعد التحقق من المالك نفسه والإعداد المحمي. تظل السجلات بحاجة إلى قراءة جديدة بعد ذلك.",
  replaceCredential: "استبدال مفتاح Fireflies المحفوظ (اختياري)",
  publicationAreas: "وجهات المهام المسموح بها",
  publicationHint:
    "هذه المجالات حدّ أعلى. يحتاج كل شخص أيضًا إلى منحة صريحة وصلاحيات المهام الحالية.",
  removeUnavailableDestinations:
    "أزل الوجهات التي لا تشملها صلاحيات الإعداد الحالية قبل الحفظ.",
  retainedConsent:
    "أسمح ببقاء نص المهمة المقبول صراحةً في مجال العمل المختار، حتى إذا أُزيل الوصول إلى المصدر لاحقًا.",
  grants: "الأشخاص المخوّلون للمصدر",
  noGrants: "لم تُمنح صلاحية صريحة لأحد بعد.",
  grantPerson: "الشخص الذي سيُمنح الوصول",
  grantRead: "قراءة السجلات المحفوظة والمستوردة حديثًا",
  grantImport: "استيراد السجلات وتحديثها",
  grantTriage: "إعداد مسودات المصدر واتخاذ القرار بشأنها",
  grantConfirm:
    "أمنح هذا الشخص عبر هذا الاتصال تحديدًا الوصول إلى جميع النسخ التاريخية المحفوظة، بما فيها ما استُورد قبل هذه المنحة، وإلى الاستيرادات الحالية والمستقبلية.",
  grantSave: "حفظ الوصول الصريح",
  grantRevoke: "سحب صلاحية المصدر",
  grantExpires: "انتهاء الصلاحية (UTC، اختياري)",
  grantExpiryHint:
    "وقت UTC دقيق، مثل 2026-12-31T18:00:00Z. اتركه فارغًا دون انتهاء.",
  revoked: "مسحوبة",
  grantActive: "منحة نشطة",
  setupUncertain:
    "تعذّر تأكيد تعديل الإعداد. تحقّق من الإعداد الحالي أو أعد العملية نفسها؛ لا تنشئ اتصالًا بديلًا.",
  retryExact: "إعادة العملية نفسها",
  importRecords: "استيراد سجلات",
  captureRecord: "قراءة سجل قائم",
  importHint:
    "اختر فترة UTC لا تتجاوز 31 يومًا. تقرأ كل خطوة صفحة محدودة وتبقى التغطية والإخفاقات ظاهرة.",
  from: "من (UTC)",
  to: "حتى (UTC)",
  utcHint: "تاريخ ووقت UTC؛ هذا حدّ للاستيراد وليس موعدًا للمهمة.",
  conversationId: "رقم المحادثة القائمة",
  messageId: "رقم الرسالة القائمة",
  feedbackId: "مرجع الملاحظة القائم (ULID)",
  captureHint:
    "يقرأ فقط المحتوى المسموح لهذا السجل القائم. لا يغيّر البريد أو الملاحظة أو سجل المزوّد.",
  imports: "تقدّم الاستيراد",
  importHistory: "جميع محاولات الاستيراد",
  unfinished: "الاستيرادات غير المكتملة",
  noImports: "لا توجد محاولات استيراد في هذا العرض.",
  startImport: "بدء استيراد محدود",
  advance: "قراءة الخطوة التالية",
  cancelImport: "إلغاء الاستيراد",
  checkImport: "التحقّق من حالة الاستيراد",
  discovered: "عُثر عليه",
  completed: "قُرئ",
  failedCount: "تعذّرت قراءته",
  nextAttempt: "تتاح القراءة التالية بعد",
  coverage: "التغطية",
  partialHint:
    "تغطية هذا الاستيراد جزئية. تبقى السجلات المقروءة متاحة؛ واكتماله لا يعني استيراد الحساب كله.",
  importUnknown:
    "تعذّر تأكيد القراءة الأخيرة. تحقّق من حالتها قبل خطوة أخرى. لا يمكن تكرار قراءة جارية كقراءة جديدة.",
  sourceRefresh: "تحديث الوصول إلى المصدر",
  sourceRefreshHint:
    "أعد قراءة هذا السجل عبر اتصاله. الصفحة المحفوظة لا تجدّد الوصول.",
  unavailable: "محتوى المصدر محجوب حاليًا.",
  unavailableHint:
    "جدّد الوصول من المصدر قبل القراءة أو إعداد مهمة. لا يُعرض النص الخاص المحفوظ عندما يصبح الوصول قديمًا.",
  sourceRevision: "نسخة المصدر",
  noRevision: "لم يُتحقّق منه بعد",
  exactPassage: "مقطع المصدر الدقيق",
  selectHint:
    "اختر من مقطع إلى 20 مقطعًا، بمجموع لا يتجاوز 20 ألف حرف. لا يُختصر شيء أو يُعاد تحريره نيابةً عنك.",
  noPassages: "لا يحتوي هذا السجل على مقطع مدعوم للاختيار.",
  summaryEmpty: "أعاد المزوّد ملخصًا فارغًا.",
  summaryMissing: "لم يُقدّم ملخص.",
  selected: "المقاطع المختارة",
  prepare: "إعداد مسودة خاصة",
  selectedInvalid:
    "اختر 1–20 مقطعًا من نسخة المصدر هذه، بمجموع لا يتجاوز 20 ألف حرف.",
  candidates: "المسودات الخاصة والقرارات",
  noCandidates: "لا توجد مسودات في هذا العرض.",
  pending: "قيد الانتظار",
  accepted: "أُنشئت المهمة",
  linked: "رُبطت بمهمة قائمة",
  discarded: "استُبعدت",
  draft: "مسودة مهمة خاصة",
  draftHint: "اكتب المهمة المراد مشاركتها. الحفظ هنا لا ينشرها ولا يسند عملًا.",
  unprepared: "حُددت المقاطع ولم تُعد المهمة بعد.",
  noDestination: "لا تتاح وجهة للمهمة بصلاحيات المصدر والمهام الحالية.",
  scopeConflict:
    "تغيّرت عملية المصدر أو لم تعد نسختها حالية. حمّل الحالة الأحدث قبل المحاولة مجددًا.",
  scopeMissing: "المصدر أو عملية المصدر غير متاح بصلاحياتك الحالية.",
  withheldDraft:
    "توجد مسودة معدّة، لكن محتواها محجوب بصلاحية المصدر أو الوجهة الحالية.",
  saveDraft: "حفظ المسودة الخاصة",
  ownerSuggestion: "المالك المذكور في المصدر (اختياري)",
  dueSuggestion: "الموعد المذكور في المصدر (اختياري)",
  suggestionHint: "للسياق فقط. لا يسند شخصًا ولا يحدد تاريخًا.",
  reviewDraft: "المراجعة قبل المشاركة",
  audience: "جمهور الوجهة",
  audienceHint:
    "يستطيع كل من لديه صلاحية القراءة الحالية أو المستقبلية لهذا المجال قراءة عنوان المهمة الكامل ووصفها ومسؤوليتها وموعدها ونشاطها. منح المصدر لا تحدّ جمهور المهمة.",
  retainHint:
    "يبقى نص المهمة المقبول في هذا المجال حتى إذا أُزيل المصدر أو صلاحية الوصول إليه لاحقًا. يظل مرجع المصدر خاصًا بالمخوّلين له.",
  acceptConfirm:
    "راجعت هذه المقاطع الدقيقة والمهمة الكاملة وجمهور الوجهة. أوافق صراحةً على مشاركة هذه المهمة هنا.",
  acceptTask: "اعتمادها في العمل المشترك",
  link: "ربط مهمة قائمة",
  linkHint:
    "يضيف مرجع مصدر خاصًا فقط. لا يُنسخ نص مقطع أو مسودة. يغيّر الربط بيانات المهمة ويعيد العمل قيد المراجعة إلى قيد التنفيذ ليتطلب مراجعة جديدة.",
  findTarget: "البحث عن مهمة قائمة",
  chooseTarget: "مراجعة هذه المهمة",
  noTargets: "لا توجد مهام نشطة قابلة للتعديل تطابق هذه الوجهة.",
  targetVersion: "المهمة المستهدفة الدقيقة",
  linkConfirm:
    "راجعت هذه المهمة الدقيقة ووجهتها. أوافق على إضافة المرجع الخاص دون نسخ النص وأفهم أن أي مراجعة معلّقة للمهمة ستُلغى.",
  confirmLink: "تأكيد الربط دون نص",
  discard: "استبعاد المرشح",
  discardHint: "احفظ سجل قرار دون إنشاء مهمة أو تغييرها. لا يزيل ذلك المصدر.",
  discardConfirm: "استبعاد هذا المرشح دون نشر مهمة.",
  outcome: "سُجّل القرار",
  openTask: "فتح المهمة المشتركة",
  restrictedTask: "سُجّل قرار المهمة. الهدف غير متاح بصلاحيات مهامك الحالية.",
  unknown:
    "تعذّر تأكيد القرار. لا ترسل قرارًا جديدًا. تحقّق من النتيجة المحفوظة لهذا المرشح.",
  recover: "التحقّق من النتيجة المحفوظة",
  conflict: "تغيّر المرشح أو المصدر. لم تُطبّق مسودتك المحلية.",
  loadCurrent: "تحميل المصدر والمرشح الحاليين",
  draftBase: "مسودتك مبنية على",
  currentSaved: "المرشح المحفوظ الحالي",
  adopt: "استخدام النسخة الحالية مع إبقاء مسودتي",
  adoptSaved: "استخدام المسودة المحفوظة الحالية",
  rebase: "مراجعة المقاطع الحالية وإعادة اختيارها",
  rebaseHint:
    "تغيّر المحتوى أو الوصول. قارن النسخة الحالية، واختر مقاطعها صراحةً، ثم احفظ مجددًا قبل الاعتماد أو الربط.",
  confirmSelection: "تأكيد اختيار المقاطع",
  editsKept:
    "تبقى تعديلاتك المحلية في هذه النافذة. احفظ المسودة كاملةً مجددًا قبل المشاركة.",
  leaveDraft: "مغادرة مراجعة المصدر؟",
  leavePending:
    "انتظر اكتمال العملية الحالية أو تحقّق من نتيجتها المحفوظة قبل مغادرة هذه المراجعة.",
  leaveHint:
    "ستُحذف التعديلات غير المحفوظة في هذه النافذة. يمكنك فتح المسودات الخاصة المحفوظة مجددًا من المصادر.",
  taskSources: "مراجع المصادر",
  privateReference: "مرجع مصدر خاص",
  openSource: "فتح مراجعة المصدر",
  noTaskSources: "لا توجد مراجع مصادر مرتبطة.",
  reason: {
    binding_missing: "هذا الاتصال غير متاح. راجع الوصول والإعداد.",
    binding_disabled: "هذا الاتصال معطّل. اطلب من المسؤول مراجعة الإعداد.",
    credential_unavailable:
      "بيانات الاعتماد المحمية غير متاحة. اطلب من المسؤول مراجعة الإعداد.",
    binding_unavailable:
      "استخدام المصدر متوقف. راجع الوصول والإعداد ثم حدّث المصدر.",
    source_expired: "انتهت صلاحية المصدر. حدّث السجل قبل المتابعة.",
    rebase_required:
      "تغيّر محتوى المصدر أو الوصول. راجع المقاطع الحالية وأعد اختيارها.",
    import_busy: "توجد قراءة جارية. تحقّق من حالتها قبل المتابعة.",
    retry_later: "طلب المزوّد الانتظار. استخدم وقت القراءة التالي المتاح.",
    source_denied: "لم يعد المصدر مقروءًا بالصلاحيات الحالية.",
    publication_not_allowed:
      "لا تسمح الصلاحيات الحالية بهذه الوجهة. راجع وصول المصدر والمهام.",
    unsupported_schema: "صيغة المصدر غير مدعومة. لم تُعد مهمة منها.",
    provider_unavailable: "تعذّرت قراءة المزوّد. تبقى جلسة مساحة عملك متصلة.",
    request_timeout:
      "لم تكتمل القراءة ضمن المهلة. تحقّق من حالتها المحفوظة قبل خطوة أخرى.",
  },
  importState: {
    queued: "في الانتظار",
    running: "تجري القراءة",
    waiting: "انتظار المزوّد",
    complete: "انتهى",
    failed: "أخفق",
    cancelled: "أُلغي",
  },
  coverageState: {
    not_started: "لم يبدأ",
    partial: "جزئية",
    bounded_end: "بُلغ الحد المختار",
    capped: "بُلغ حد الاستيراد",
  },
}

export function useIntakeCopy(): IntakeCopy {
  return useLocale() === "ar" ? arabic : english
}
