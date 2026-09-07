# Shell accessibility and Pi setup fixes

In progress. Branch `fix/design-shell`, from accepted main
`ba93e87200b16d9fd15411b885f8c852b92078c0`, including PR11 review merge
`625d8ed7266ed4cf49454942f3dbb5c48409bd07`. Scope is the five verified findings
in `reports/design-pi-specialist.md`: Pi control associations; terminal/alert
names; shell contrast; full setup reason/settings action and readiness copy;
scoped setup-banner contrast. No task/Ops/approval/locale-provider changes.

Docs first: complete current planning/AGENTS/brief and design specialist
definitions read. Code-context guide exits 0 (headed CLI and pinned-version
rules); docs exits 3, absent `data/code/tickets.db`. Local React 19.2.4,
@types/react 19.2.13 useId/htmlFor, radix-ui 1.6.0 / Select 2.3.1 trigger
source/types and shared Input/Button/Select source read before edits. Retain
all existing NOTICE sections and lockfiles. Upstream Codeg v0.30.4 resolves
through gh api to `6f6bd648b206412644842a98d9ffeebf57292bed`, Apache-2.0.

Presentation contract: the accepted Desk launcher explicitly requests
`gpt-6-astra` and `max`; generic saved Thinking values are not a readiness
check. Show requirements and a retry step without modifying saved settings,
credentials, login/install actions or the model guard. Reuse translated
control/action labels and current semantic foreground tokens. New guidance
will stay within Pi's message namespace; locale loading is owned elsewhere.

Browser plan: reuse only this worker's closed fixture 4324 (owner authorized),
fresh temporary DB and empty catalogue. No other fixtures, live actions or
provider/model calls. Preserve real owned companion. Existing accepted before
captures remain attributable to PR11; add focused after evidence and actual
before/after status measurements. Final lint/typecheck/build and meaningful
focused behavior tests pending. Report will record commands, exact sources,
commit/PR, limitations and completed evidence.
