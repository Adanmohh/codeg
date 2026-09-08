# Business workspace Design Studio checks

Derived from the unchanged `docs/design/BRIEF.html` scan v1 tokens, with the
business section plan and scope from `docs/BUSINESS-IMPLEMENTATION.md` and
`docs/contracts/business-ui.md`. The newer business direction supersedes the
brief's older support-only scope; it does not replace inherited fonts/themes.
Sections: connection, navigation, work list/board, task editor, review, people.

<check id="BC-1" section="connection">
<text>The Hafidh mark, existing Inter UI font and primary #245e58 anchor a clearly labelled sign-in form; the paired dark primary remains #9bd4c5.</text>
<evidence>Connection and workspace desktop light/dark captures and Canvas-normalized token inventory.</evidence>
</check>
<check id="BC-2" section="connection">
<text>Personal access and administrator bootstrap are distinct choices, and failed/revoked access offers a labelled recovery without retaining a credential.</text>
<evidence>Connection errors, bootstrap, personal-owner and revoked-session captures.</evidence>
</check>
<check id="BC-3" section="navigation">
<text>My work, Shared work, Review and People &amp; agents lead the workspace; engineering appears only in the original administrator context.</text>
<evidence>Owner-personal and member desktop frames, with the operator bootstrap flow as the separate context.</evidence>
</check>
<check id="BC-4" section="navigation">
<text>At390px the labelled navigation drawer opens from the correct RTL edge, contains keyboard focus and returns focus to its menu control.</text>
<evidence>Arabic390px drawer/open/closed captures and keyboard measurements.</evidence>
</check>
<check id="BC-5" section="work list/board">
<text>Each real task presents its accountable human and separate executor by name, status by text and shape, and only the actual calendar deadline.</text>
<evidence>Populated list and board frames with human, agent and unassigned work.</evidence>
</check>
<check id="BC-6" section="work list/board">
<text>List/board and area/search filters act on real returned tasks, with bounded loaded counts and a useful Clear filters empty result.</text>
<evidence>Area-filter and no-results flow, twelve list/board viewport/theme frames.</evidence>
</check>
<check id="BC-7" section="work list/board">
<text>At390/768/1280px the page stays within its viewport, active controls remain44px and mobile filter values are readable; only the board scrolls horizontally inside its container.</text>
<evidence>Before/after filter measurements and all responsive list/board captures.</evidence>
</check>
<check id="BC-8" section="task editor">
<text>Create task provides a labelled brief, area, priority, exact date-only input and permitted owner/executor/reviewer choices without requiring a conversation.</text>
<evidence>Real owner/member creation flow and calendar roundtrip results.</evidence>
</check>
<check id="BC-9" section="task editor">
<text>Long unbroken titles and notes wrap within the mobile dialog, and saved dates retain their YYYY-MM-DD spelling and LTR direction in Arabic.</text>
<evidence>Settled358px long-content before/after and Arabic task/date capture.</evidence>
</check>
<check id="BC-10" section="task editor">
<text>A stale save preserves the private draft and requires visible comparison and explicit adoption before another save.</text>
<evidence>Real409 conflict and comparison screenshots, followed by successful CAS recovery.</evidence>
</check>
<check id="BC-11" section="review">
<text>Review shows the actual current deliverable and separate reviewer, requires explicit human confirmation and visibly records completion only after a successful response.</text>
<evidence>Member submission, independent review-ready and done frames with response outcomes.</evidence>
</check>
<check id="BC-12" section="review">
<text>Keyboard focus stays visible inside the review dialog; Arabic390px and theme changes keep the same unsaved review and fit the dialog.</text>
<evidence>Keyboard sequence, RTL review capture and cross-tab draft-retention checks.</evidence>
</check>
<check id="BC-13" section="review">
<text>Reduced motion removes the business dialog's entrance zoom and button transitions; no continuous decorative animation is introduced.</text>
<evidence>Actual computed animation/transition values before and after the reduced-motion correction.</evidence>
</check>
<check id="BC-14" section="people">
<text>People and agents have visible roles/areas; personal credentials are masked by default, and agent entries have no human credential issuer.</text>
<evidence>People, member editor and masked one-time access captures.</evidence>
</check>
<check id="BC-15" section="people">
<text>Viewer and revoked states remove unavailable actions and private editing surfaces while leaving a clear explanation.</text>
<evidence>Actual role downgrade, viewer task and revoked sign-in frames.</evidence>
</check>
<check id="BC-16" section="review">
<text>Painted text meets4.5:1 for normal text or3:1 for large text in both themes, and interactive controls have accessible names.</text>
<evidence>Canvas-composited CLI samples, Design Studio pure reports and direct checks of any flagged samples.</evidence>
</check>
