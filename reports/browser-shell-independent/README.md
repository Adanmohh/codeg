# Independent shell correction browser check

Root Playwright CLI on the worker's loopback 4324 fixture, 2026-09-08.
Product `f8ba4597` plus the reviewed working SettingsShell menu correction.
This is not yet final committed-head acceptance. Read-only fixture guard reused
from the worker; no API/config writes, model prompts, off-origin requests or Pi
connections occurred (all guard counters zero). Session is closed.

- Actual fixture login then `/settings/agents?agent=pi`, 390×844.
- Navigation button uniquely named and 44×44 at x12/y1.5. Click opens the real
  dialog with 13 destination buttons; Escape closes it and aria-expanded changes
  true → false without changing route. Drawer settles at x8/y8, 260×828.
- Root inspected [open menu](mobile-menu-open.png) and
  [Pi fields](mobile-pi-fields.png). Provider, Model, Thinking and API Key are
  uniquely labelled; model placeholder is gpt-6-astra. Page scroll width is390.
- First capture preceded the Base UI opening transition. Recaptured only after
  the actual dialog geometry moved inside the viewport; an aria-expanded value
  alone was not treated as visual evidence. An initial selector used API key
  instead of the actual API Key label; corrected case-sensitive query passed.
- Settings initially showed the expected unauthenticated/session-expired state
  before login. Those 401 console errors are not a zero-console claim. No
  provider credential values were inspected, entered or persisted.

Custom-provider and saved-value behavior passed independent component tests;
worker supplies full light/dark contrast, setup-message and custom-control
browser evidence. Root's final integrated Design Studio loop remains pending.
