# Business workspace — owner scope amendment

2026-09-08. Owner explicitly requests marketing, channel management, pulling and
managing ads, website work and feedback, backed by engineering features. Shared
task management must support agents and humans in both business and engineering.
This supersedes interpreting Ops Desk as primarily an engineering or support UI.
The current implementation is a foundation, not the complete requested product.

## Product structure

Business outcomes lead the main experience. Engineering features remain available
and support those outcomes. Organize around:

| Area | Intended work | Current implementation boundary |
| --- | --- | --- |
| Overview | Owner decisions, customer activity and actionable cross-functional work | Existing Morning/approval/correspondence data; visual refresh active |
| Marketing | Campaign briefs, content, launches and results | Launch/social workflows were planned for Phase 2; campaign management not implemented |
| Channels | Connected accounts, publication calendar, incoming activity and per-channel results | Email foundation and private Telegram review exist; public marketing-channel management does not |
| Ads | Import campaigns/performance, review results, prepare controlled campaign/budget changes | New scope; platform/account contracts and metric semantics must be verified |
| Website | Content, proposed updates, review/preview and performance | New scope; source repository/CMS/deployment and analytics need grounding |
| Feedback | Customer correspondence, tester reports, product requests and responses | Email and Hafidh bug intake exist; broad feedback aggregation is incomplete |
| Work | Tasks shared by humans and agents across all areas | Inherited task engine/agent worktrees exist; cross-functional ownership and workflow gaps need audit |
| Engineering | Bugs, technical evidence, development tasks, agent runs and review | Existing engineering foundation; subordinate in main business presentation |

These are product domains, not instructions to ship empty navigation or pretend
integrations are live. Expose a capability when it has useful real behavior;
configuration states must describe the actual next step.

## Shared work model

A task belongs to a business outcome or an engineering outcome and may link across
them. Human owners and assigned agent execution must be distinguishable. Plan
support for status, priority, due date, dependencies, brief, deliverable, review
and audit history after inspecting the existing schema and borrowing sources.
This is a target contract, not a claim that each field already exists.

Examples:
- Campaign: human sets objective and reviews budget; agent researches and drafts
  content; publication is separately reviewed for each channel.
- Website: human requests a landing-page update; agent prepares a preview; linked
  engineering work handles implementation; release remains an explicit action.
- Feedback: agent summarizes a real customer report and proposes next work;
  human decides priority; an engineering task links back to the customer outcome.
- Ads: imported facts support analysis; recommendations become reviewable tasks
  or proposals. Never invent attribution, currencies, conversion windows or ROI.

Use the existing task/approval infrastructure where suitable. Do not create a
parallel marketing task engine. Agent work is visible and reviewable; human work
must not require starting an agent. Engineering tooling does not dominate the
business overview. Preserve evidence, scopes, exact-payload approval and audit.

## Delivery sequence

1. Finish the current visual refresh of real customer/decision/work surfaces.
   Audit against founder usability as well as visual/accessibility correctness.
2. Audit the existing task model and implement missing human/agent shared-work
   behavior required for both domains, with concrete scenarios and tests.
3. Verify existing platform/repository connections and immutable source contracts;
   first build useful read/import and draft/review paths for marketing, channels,
   ads, website and feedback. Stage independent, bounded worker PRs.
4. Add reviewed execution and scheduled actions after complete payloads, account
   permissions, failure semantics and no-duplicate behavior are demonstrated.
5. Validate the complete business-to-engineering loop with real configured
   integrations when available. Synthetic fixtures remain explicitly synthetic.

Platform names are requested from the owner; meanwhile use local project context
without exposing credentials. No platform choice is silently fixed by examples.
Prior Phase 2/3 dates and scope estimates require revision for these additions.
No deployment, live post, ad spend/budget change, account installation or external
message is authorized merely by this implementation scope amendment.

## Role and meeting-source clarification

Owner clarifies each organization member uses the same desktop app according to their role, with suitable MCP/CLI capabilities. Shared tasks belong across humans and agents, business and engineering. Fireflies and comparable meeting/note tools should feed source-backed task proposals and orchestration. Implementation is paused for ideation; source comparison is in reports/business-workspace-research.md. Organization identity/permissions and cross-desktop shared state are required foundations, not merely different dashboard layouts.

Research comparison is complete. The recommended foundation is one server-authoritative shared business task with explicit human ownership and human/agent execution assignments; link engineering runs and MCP approval handles as distinct records. Reuse existing task and approval patterns selectively after authorization/provenance hardening. Frappe/OpenProject/Flowable/Temporal were evaluated as mechanism references, not selected replacement stacks. Implementation remains paused for product discussion.
