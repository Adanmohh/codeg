# Hafidh intake

Read-only MCP stdio package. Requires Python 3.12+; tested on Python 3.13.
From this directory, create an isolated environment and install the tested pins:

```sh
uv venv .venv
uv pip install --python .venv/bin/python -r requirements.lock
uv pip install --python .venv/bin/python --no-deps -e .
.venv/bin/python -m pytest -q
.venv/bin/python -m hafidh_intake.server
```

Trusted launcher configuration names are `HAFIDH_INTAKE_ORIGIN` (HTTPS origin,
without `/api/v1`), `HAFIDH_INTAKE_PRODUCT_ID`, `HAFIDH_INTAKE_BEARER` (existing
Hafidh admin bearer). Loopback HTTP is supported for isolated fixtures. No
credential belongs in MCP tool arguments, prompts, source files or output.
The adapter's bearer remains an upstream admin credential; its reduced GET-only
tool surface does not narrow that credential's upstream permissions.

Exactly three tools are registered: `hafidh_feedback_list(request)`,
`hafidh_feedback_get(source_ref)`, `hafidh_intake_status()`. List input supports
source, status/severity/tag/query, limit 1–100 and a five-minute process-local
cursor. Each continuation reads one page; at five pages it returns
`scan_complete=false` with no next cursor. Restart scans with overlap. Source
offset paging is not lossless. GET revalidates a listed reference by scanning
at most 500 current records; failure is explicit, never a cached success.

In-app intake is unavailable: the existing Hafidh feedback route only accepts
POST and sends a notification. TestFlight uses GET list and GET sync/status;
no source writes, sync triggers, ASC polling or attachment fetches are exposed.

The Desk's trusted host runs `python -I -m hafidh_intake.host` as a closed child
process using the same adapter. It accepts bounded NDJSON frames with
`request_id`, `operation` (`list`, `get`, `status`) and `input`; replies carry
the same request ID and the strict adapter `result`. It is not a generic MCP
proxy. Unknown fields/operations are rejected without echoing input. Its
process-local list state is required for get/revalidation, and failure never
substitutes a cached record. The parent must kill the child on timeout or
protocol failure. This entry point does not add or alter the three MCP tools.

DTOs omit identity fields, notes, signed URLs and raw attachment bytes. Known
contact values, tokens and common private paths are removed from free text.
This is a bounded redactor, not a universal personal-data detector: source text
remains untrusted and all outward issue text still requires human review.
Missing build/screen/reciter/log is explicit; heuristic triage is never a human
confirmation. A screenshot or recorder session ID cannot satisfy the log gate.

Exact source revisions and retained licenses are in root `NOTICE` and
`reports/intake-github.md`. No Hafidh modules are imported into this environment.
