# Docs-first: Claude comparison and Codex correction

Current status: owner installed the candidate and root completed bounded live
acceptance. See [actual results and remaining limits](docs-first-live-acceptance.md).
The staged-only statements below describe the earlier preparation checkpoint.

Owner request, 9 September 2026: inspect Claude Code's docs-first hook and make
Codex similar. Clarification: read relevant documentation before framework code
edits in the current project. Routine research and coordination are not edits.

## Actual source findings

Read the active registrations in ~/.claude/settings.json and the complete
docs-reminder.py, docs-seen.py and their two shell launchers under ~/.claude/hooks.
The Python implementation, not its older opening comments, is authoritative:
general reminders have a 600-second per-project/per-agent cooldown; recognized
framework writes get reminders on every edit. Strict-listed projects additionally
block the first recognized framework write until that agent has a documentation
stamp. Unfamiliar language writes have a separate session/language check.

Normal Git, file discovery and tests are quiet. gh api documentation retrieval
is permitted. Claude's Post collector recognizes documentation request text and
creates stamps; it does not validate returned bytes or shell exit status. Its
transcript fallback can also accept a documentation mention. We borrow the
practical edit-focused workflow, not a claim that those stamps prove an API was
read correctly. Claude itself was not modified.

Codex v2 instead rejects almost all shell commands outside a small literal
allowlist. Actual examples blocked in this turn include discovery with an
explicit path, a Herdr environment check, worker output inspection and the
candidate's isolated Python test command. Its parser rejects real alphanumeric
Herdr handles and shell separators even inside quotes. Reading documentation
cannot unlock builds or Git operations under this policy.

## Reviewable correction

Candidate directory: /tmp/ops-desk-claude-hook.
Transformation: transform.py, SHA256
ae8cd04e0f6677744e0eff6d3b2f55c22e9049ffd1650737f97c4f7e96f5c5e1.
The digest was computed over the complete cat response in the orchestration
JavaScript with a SHA256 implementation checked against the standard abc vector.
The owner-run installer independently verifies it with hashlib before loading.

Routine shell workflows are admitted quietly. Recognizable implementation shell
writes and package changes receive reminders. Supported native JS/TS and Python
edits retain existing exact source evidence checks, identity/version correlation,
signed reader and protected-path checks. Unsupported surfaces get an explicit
manual-grounding reminder. This does not waive the owner's requirement to read
relevant docs, and does not permit rerouting a denied supported edit via shell.
Each file in a mixed patch is checked; an unsupported SQL file cannot skip the
following supported TypeScript file's missing documentation check.

No generic gh api result is silently upgraded to signed package proof. The real
response remains documentation for manually grounded work. Shell admission is
not a permissions system, complete code detector or security sandbox.

The test recipe retains 50 existing evidence tests unchanged. Six old tests that
assert blanket shell/unknown-tool rejection are explicitly superseded by six
new workflow tests, including 22 nonexecuted command-admission cases, mixed-file
missing evidence, reminders and malformed patch denial. All historical test
sources remain intact. Both test suites load the staged candidate, including
rebinding the independent suite's originally absolute engine path.

install.py verifies the frozen v2 and launcher, stages the candidate and tests,
then runs two real launcher subprocess probes. Only after passing does it back
up and replace the engine and explanatory instructions, preserving registration,
saved trust, launcher and evidence state. Failed acceptance restores its own
replacement files; it refuses to overwrite detected concurrent changes. The
hash checks are not an OS-level transaction against a hostile same-user writer.
The existing maintenance owner was notified to avoid a competing reader-only
installation.

## Documentation and actual validation limit

Read Codex 0.153.4 pinned runtime report and the active launcher/source contract.
Used the live signed reader to read CPython 3.14.7 runpy.run_path,
hashlib.file_digest, pathlib.Path, tempfile.mkdtemp and subprocess.run, then read
the actual Path read/write and replacement methods before writing the installer.
No external dependency was installed, and no product code was changed.

Actual attempt: python3 -B /tmp/ops-desk-claude-hook/check.py.
PreToolUse rejected it with shell_unproven_use_patch_or_reader before execution.
Therefore no candidate compilation, test pass, installation, actual gh api
success or worker resumption is claimed. The user's authorization is already
present; the remaining one-time step must run in their normal terminal because
the current gate has no supported self-maintenance execution path.

After installation, root must verify actual gh api access, a documentation denial
and successful read/retry on a disposable project, and the existing workers.
Existing app fixtures, user browsers, branches and build targets are preserved.
