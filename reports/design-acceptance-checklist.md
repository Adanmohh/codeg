# Phase 1 Design Studio acceptance checklist

Derived once from `docs/design/BRIEF.html` (schema 1, scan mode), 2026-09-08.
These are acceptance checks, not passing results. Final judges must cite the
stable IDs and actual Playwright CLI evidence. Page sections: connection and
navigation; inbox and thread; composer; review and delivery; morning; bug intake.
The existing light/dark token pairs and user font choices take precedence over
generic aesthetic templates. Provider fixtures must be identified separately.

<check id="BC-1" section="connection and navigation">
<text>The connection screen identifies Hafidh Ops Desk and a rejected token shows a visible error beside the connection form, with an available retry action.</text>
<evidence>390px connection screenshots before submission, after invalid token, and after successful fixture login.</evidence>
</check>

<check id="BC-2" section="connection and navigation">
<text>The active Ops destination and keyboard focus remain distinguishable in both the light #245e58 and paired dark #9bd4c5 accent modes.</text>
<evidence>Desktop light/dark navigation crops and a keyboard-focused destination screenshot.</evidence>
</check>

<check id="BC-3" section="connection and navigation">
<text>At 390px, navigation opens and closes without clipping the main content, and Arabic direction places the navigation and back affordances consistently.</text>
<evidence>Mobile drawer open/closed and Arabic-direction page screenshots.</evidence>
</check>

<check id="BC-4" section="inbox and thread">
<text>The selected ticket is visually distinct, and choosing a different ticket replaces the thread heading and message history with that ticket's content.</text>
<evidence>Two populated ticket selections on desktop and mobile.</evidence>
</check>

<check id="BC-5" section="inbox and thread">
<text>A private note is labelled as private before saving and remains visibly separate from public correspondence after saving.</text>
<evidence>Note composer before submission and resulting thread entry.</evidence>
</check>

<check id="BC-6" section="inbox and thread">
<text>Empty and failed inbox loads explain the state and provide the actual available next action without invented customer records.</text>
<evidence>Empty inbox and failed-load/retry screenshots from identified fixtures.</evidence>
</check>

<check id="BC-7" section="composer">
<text>From, To, Cc, Bcc, subject and reply text have persistent visible labels, and the full plain-text reply can be read at 390px without horizontal scrolling.</text>
<evidence>Desktop composer and mobile top/bottom crops with all fields populated.</evidence>
</check>

<check id="BC-8" section="composer">
<text>Saving a draft visibly confirms the saved state; invalid content shows an actionable error while retaining the unsaved text.</text>
<evidence>Saved draft state and validation-failure state for the same composer.</evidence>
</check>

<check id="BC-9" section="composer">
<text>Leaving an edited draft presents the supported unsaved-change choice, and cancelling that choice returns to the preserved draft with visible focus.</text>
<evidence>Navigation confirmation and returned composer screenshots.</evidence>
</check>

<check id="BC-10" section="review and delivery">
<text>The pending review presents all recipients and complete content together, and its send action explicitly says that approval sends the reply.</text>
<evidence>Full populated pending-review capture with action controls.</evidence>
</check>

<check id="BC-11" section="review and delivery">
<text>A newer draft or task run displays a stale-review explanation and makes the old send action unavailable.</text>
<evidence>Stale-review screenshot after the controlled revision change.</evidence>
</check>

<check id="BC-12" section="review and delivery">
<text>Denied, provider-accepted, failed and unknown delivery states are visibly different, and unknown delivery warns against resending.</text>
<evidence>Four separately identified terminal-state fixtures.</evidence>
</check>

<check id="BC-13" section="review and delivery">
<text>Missing Resend configuration directs the operator to the inbox connection action and does not present the proposal as already sent.</text>
<evidence>Unconfigured review and connection form screenshots.</evidence>
</check>

<check id="BC-14" section="review and delivery">
<text>When the provider accepted a reply but local recording failed, the recovery action clearly finishes recording rather than sending another reply.</text>
<evidence>Receipt-pending state and successful local-recording recovery.</evidence>
</check>

<check id="BC-15" section="morning">
<text>The morning list shows actual outstanding work or an honest empty state, and selecting an item opens its relevant task or review.</text>
<evidence>Empty and populated morning views plus selected destination.</evidence>
</check>

<check id="BC-16" section="bug intake">
<text>A bug missing build, screen, reciter or log proof identifies each missing field and cannot be filed.</text>
<evidence>Missing-evidence form and its attempted-submit validation state.</evidence>
</check>

<check id="BC-17" section="bug intake">
<text>Issue review shows the exact repository, title, body and proposed labels, while suggested severity is visibly subject to human confirmation.</text>
<evidence>Completed evidence-backed issue preview before approval.</evidence>
</check>

<check id="BC-18" section="inbox and thread">
<text>The operational layout uses the inherited Inter hierarchy and neutral surfaces, reflowing to one readable pane on narrow screens with no decorative metrics or marketing sections.</text>
<evidence>1280px and 390px light/dark full-page views of the populated Ops surface.</evidence>
</check>

Screenshot checks supplement, rather than replace, measured contrast and DOM
accessibility checks, API authorization tests and provider request assertions.
