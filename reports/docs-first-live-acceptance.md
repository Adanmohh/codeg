# Claude-style hook: installed and live checks

The owner installed the prepared revision on 9 September 2026 using the normal
terminal. Root then verified live behavior in the existing Codex session without
restarting workers or changing hook registration/trust. This replaces the prior
staged-only status in the comparison and initial lightening reports.

Installer audit: /tmp/ops-desk-claude-hook/installed.md.
Backup: /Users/mohamedadan/.codex/hooks/backups/claude-workflow-n0d8pjua.
Installed engine digest reported by the guarded installer:
f5554c92a48451aa5b3fd7a596f0325cc4a797b3416cbc1b6d22479d4f223287.
Root read the actual retained tests.log: 56 tests passed in 0.308 seconds.
That comprises 50 retained evidence tests and six replacement workflow tests;
six older blanket-denial tests were superseded, not claimed passing. The
installer separately reports two staged and two installed launcher subprocess
checks, with registration, saved trust and launcher hashes unchanged.

## Actual current-session results

- Herdr environment check, help, list, alphanumeric worker reads, prompts and
  model-selection keys all execute normally.
- Actual gh api tag lookup returned 042fb41b7c813ac7999105e886b2b7aa715b5081 for
  rust-v0.153.4. A contents request at official Codex commit
  3d2ee51ca2d5db578f328aa75e20aa22c0197c9a fetched pre_tool_use.rs; root read
  lines193–239. No model inference, provider publication or credentials printed.
- Git fetch of all four current UI/backend/review branches succeeded. Real PR29
  API lookup and immutable review report retrieval succeeded.
- A disposable project importing docs-first-fixture1.0.0 was patched before a
  documentation read: PreToolUse denied docs_evidence_missing. A read confirmed
  answer1 remained unchanged.
- The normal signed reader returned the two-line actual fixture declaration and
  numeric identity behavior. Root read it, then the identical native patch passed.
  The file now contains answer2.
- A separate project with the same dependency/version still rejected the same
  edit; its file remains answer1. One project's read does not grant another's.

Synthetic projects are under
/private/var/folders/6k/w2fh6wy167726g5nr9zm_ph80000gn/T/ops-docs-first-live-iva9jbbz.
They contain no application credentials, production source or actual providers.

All three existing Ops workers were then observed working in Herdr. UI and review
footers show Astra/max. Tickets' visible picker was Luna/low while Herdr metadata
reported Astra; root switched its actual model picker to Astra, advanced Max,
verified the Model changed confirmation/footer and only then resumed product work.
This is current configuration evidence, not a claim about previous authoring.
Workers were told to verify gh api in their own sessions and record live evidence.

## Remaining limitations

Rebrand still reported package_unresolved on a supported edit after documentation
reads. Root requested the exact imported module/resolver failure, preserving the
denied patch. The worker remains active on independent validation/source work;
this is not claimed as fully unblocked editing.

The current reader-path guard incorrectly denies a shasum command that merely
names the engine file, with reader_arguments_invalid. No alternate hashing route
was used to bypass that denial. The digest above is installer-attributed. This
narrow maintenance-read false positive does not prevent GitHub, Git, ordinary
tests or worker coordination. The existing maintenance owner was notified.

Unsupported Rust/SQL/ambient surfaces require manually read and recorded relevant
docs. The hook does not pretend to verify those reads. It remains a workflow aid,
not a general shell security boundary. A supported denial must not be rerouted
through a shell. No fixture, user browser, old build target or app bundle changed.
