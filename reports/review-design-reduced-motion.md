# Reduced-motion correction — review in progress

Root inspected PR19 product `cebce09d0df73205277d59c7048e733cc4b588f4`.
The three-line popup-only addition applies `motion-reduce:transition-none`.
Existing transforms, duration/state overrides, nested-layer structure, swipe
variables, portal ownership and focus/dismissal handlers are unchanged.
Installed Base UI1.7.0 `useAnimationsFinished.mjs` handles an empty animation
collection; no fabricated completion event or lifecycle replacement is added.
NOTICE records the existing Codeg Apache pattern and pinned dependency references.

Root read the actual CLI measurement, guard and local export proxy sources.
The proxy serves a real isolated Next export and forwards API/WebSocket traffic
to the owned no-engine Rust fixture. It does not manufacture drawer DOM or API
approval results. This is a local browser test, not native WebView certification.

Structured before/after frame evidence independently inspected by root:

- Before, reduced-motion opening has53 distinct sampled positions and a450ms
  transform transition. Escape has47 positions with height/opacity/transform
  transitions. Normal mode exhibits the same travel.
- After, reduce has transition-property none and zero popup animations; opening
  has only the closed and final positions, with no intermediate slide. Escape
  removes the popup after its state change. Normal opening retains53 sampled
  positions and the original transition properties.
- Both preferences preserve331.5×828 geometry inside390×844, Escape returns
  focus to Show Sidebar, and outside click dismisses. Guard counts are all0.

Worker final nested/swipe checks, relevant regression results, report and exact
head are pending. Root final integrated browser recheck follows acceptance.
