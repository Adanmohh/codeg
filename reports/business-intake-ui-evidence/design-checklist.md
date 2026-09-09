# Increment B measured review checklist

Derived from the accepted `docs/design/BRIEF.html` and the accepted business
interaction plan. This is a checklist, not evidence of a pass. Existing user
fonts/themes take precedence over generic Design Studio font/palette advice.
The default brand is the existing #245e58 / dark #9bd4c5 accent on inherited
semantic surfaces. No new global token or decorative imagery is introduced.

| ID | Section and observable check | Required rendered evidence |
| --- | --- | --- |
| BC-1 | My work, Shared work and Review remain primary; Sources is a secondary named destination. | Wide and narrow navigation, EN/AR |
| BC-2 | The Sources heading describes source-to-work decisions without engine jargon or invented account metrics. | Initial populated/empty Sources |
| BC-3 | A missing source has a concrete operator setup or member handoff; no connected badge without returned enabled/configuration state. | Operator/member empty and unavailable states |
| BC-4 | Source-private passages/drafts and the destination task audience have distinct visible labels. | Reader and full audience review |
| BC-5 | Exact source text remains readable at 16px with full wrapping; selection bounds are explicit and no text is clipped or paraphrased. | Long passage at 390/768/1280, EN/AR |
| BC-6 | The reading/preparation columns form one deliberate grid at wide widths and stack without overlap at narrow widths. | 390/768/1280 reader screenshots |
| BC-7 | Complete prepared title/brief/owner/assignee/reviewer/priority/date and the full destination audience are visible before explicit acceptance. | Reviewed private draft, unchecked/checked confirmation |
| BC-8 | Text-free link shows the current target title/status/revision/domain and states that no source text is copied and Review is invalidated. | Link target confirmation |
| BC-9 | Stale/rebase/conflict recovery labels the preserved base and current version; hidden private text is absent until authorized fresh disclosure. | Expired/source-changed/current comparison |
| BC-10 | Unknown outcomes show status/receipt recovery; only returned terminal decisions show task-created/linked/discarded copy. | Lost response and terminal outcome |
| BC-11 | Setup key remains masked; zero grants and explicit retained-history/current/future consent are visible. | Synthetic setup/grant screens, excluding credential values |
| BC-12 | Inputs and actions have named, visible keyboard focus; Enter submission and modal focus restoration work. | Settled focus screenshots and CLI assertions |
| BC-13 | Text contrast meets 4.5:1 (3:1 only for qualifying large text) on settled real light/dark surfaces, including selected controls. | Computed composite styles and screenshots |
| BC-14 | Arabic directions are coherent, Back arrow reverses, literal UTC/date-only values stay LTR, and local draft text survives locale changes. | EN→AR→EN same session and RTL narrow screenshot |
| BC-15 | Reduced-motion preference preserves all usable content and controls with no essential entrance animation. | Real reduced-motion state |

Methods: Design Studio source lint, aesthetic/a11y/flow specialist definitions
applied inline, and actual Playwright CLI evidence. No specialist agents or paid
inference will run. Source-lint alone cannot establish contrast, layout or
functional acceptance; every applicable final verdict needs actual UI evidence.
