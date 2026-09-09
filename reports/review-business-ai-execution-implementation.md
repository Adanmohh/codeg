# Independent E1 schema and DTO review

Review in progress at combined source `177d0f3e2b426d65de9573ba534ac8c11f9250f7` (PR32). No concrete blocking source finding has been established in this internal schema/closed-wire checkpoint. Independent execution is pending. E1 routes, launches, runtime authority, file import and publication consumers are absent from this source and are not accepted by this review.

The complete initial product `6195d9daf049b7cd16615e35b87d08c7f2870bf0`, owner report/evidence `fdfe342940d1a6881005e5534b3f0090f9391ef7`, and combined delta/report were read. Own `gh api` resolves177d0f3e to tree `1c4e7f0a6e56075329a2820b32edebfd73edf89c`, parent `b73940560a69748f6774f41eb1a8de44e7c4c11f`. The combined change adds accepted B source and improves two E1 tests; production E1 Rust/SQL/DTO/validation is identical to6195. Applied migrations10/11/12/13, Cargo manifest/lock and LICENSE match accepted B `c8453a48d441f40eb47f9c9af856235b4f9986e5`. Registration orders11,12,13,14.

## Implemented source assessment

- Migration14 retains every deliverable column and composite FK, restores immutable history triggers, and changes only the text-length lower bound for future file-only submissions. Existing text submission still rejects blank text. One pinned SQLx connection disables FK enforcement outside `BEGIN IMMEDIATE`; DDL/data and the completion marker commit together. A missing separate SeaORM receipt can retry without rebuilding. Errors roll back before FK restoration; cancellation marks the leased connection for closure.
- Session authority, original actor, captured tenant/task epochs and profile revision are immutable. Assignment/owner/reviewer/domain/archive and done/cancelled transitions advance task scope and revoke old generations. Profile changes also revoke sessions. These triggers do not prove future runtime consumers revalidate original authority.
- Operation identity, payload and lineage cannot be updated; confirmed/failed receipts cannot be updated or deleted. Managed versions and selected publication rows are immutable with composite task/tenant references. Runtime transitions, stable file bytes and atomic task/publication/receipt consumption still require implementation and tests.
- Caller DTOs reject unknown fields and contain no actor/org, command, environment, credential or filesystem path input. Private producer metadata and public selected-version metadata are distinct types. Prompt receipts include messageId/inputHash. `account_snapshot` is explicitly unavailable. NDJSON types alone do not prove stream bounds, replay order or current-authority delivery.

## Bounded execution plan and preservation

Owner evidence at177d0f3e records four schema cases: populated human history/real receipt retry; mid-DDL rollback with populated deliverable; every-column synthetic execution link/grant/epoch and agent deliverable retention; closed input validation. Those execution rows are schema fixtures, not live engine admission proof. Evidence-only `25badf45db086c3dd33d841d55175f724108c6cc` records four B epoch migration passes and integrated server check. These remain owner results.

Tickets released the serialized build window. Independent checks will use the four unchanged execution tests, relevant B migration selector and bounded cancellation/schema/DTO probes for uncovered boundaries. No broad unchanged A rerun is planned. Exact logs/source correspondence and any reviewer-only patch belong under `reports/business-ai-execution-review/177d0f3e/`.

Only source was extracted to `.build/business-integration-review/177d0f3e/source`; the existing target is `.build/business-integration-review/target-15bb402b`. No target/output was copied or removed. Space was observed at5.5GiB, not reserved. All fixture listeners, user browsers, old archives, paused reports and native artifacts are unchanged.

## Docs-first and provenance

Updated global AGENTS and docs-first preface were read. Actual own-session Git, immutable `gh api` and authorized Herdr commands succeed after the owner update. The previous blocked report is retained verbatim as [historical-hook-block.md](business-ai-execution-review/177d0f3e/historical-hook-block.md). A malformed delete/add report patch was denied `patch_target_changed`; the unchanged file was reread and corrected through the same native patch tool. No denied edit was rerouted.

Turn context line2814 at2026-09-09T13:05:16.458Z records `gpt-6-astra`, `max`, `never`, `danger-full-access`, correct worktree and session `01a084ee-129a-7561-9f10-64266115e759`. Old audit lines27340/27341 predate this resumed window. Root confirms the current launcher emits no generic Pre/Post JSONL and the rollout has no hook event_msg entries. These are not relabelled as live hook events. A read-only engine-path search was denied `reader_arguments_invalid`; it was not rerouted. Actual command successes and denials establish observed behavior only. No protected hook state/key was inspected or modified.

The code-context skill ran with existing rag-skills Python and `HF_HUB_OFFLINE=1`: guide exit0; installed-doc lookup exit3 because `data/code/approvals.db` is absent. No corpus coverage or install is claimed. Generic SQLite/defensive-boundary guidance was considered; unrelated per-tenant database/delegation rules do not override the accepted shared database and no-workers assignment.

Installed source read: SeaORM/migration1.1.19 (`migrator.rs`: SQLite migration and separate receipt); SQLx0.8.6 (`pool/connection.rs`, `transaction.rs`: close-on-drop and queued rollback); serde/derive1.0.228 (`de/struct_.rs`: unknown/duplicate fields). Cargo pins Tokio1.49.0 and UUID1.20.0. No package or source version was upgraded.

Apache2.0 LICENSE was read. Official `gh api` tree at base `9e61fe67261219921e0a6a860c45f6e08ea99c21` verifies NOTICE sources: migration12 blob `a305a682bbf55e750a5d5156ac0aa4695aac08c1`; migration10 `3cb107d5bc0ab375e1b82fc64c7fefc82396b318`; task store `d48fdf1e85feb4410dbff51e8a671d6c9d525691`; identity module `f7205760715d2516b11b6e2cb79fd133218672bb`. Complete migration10/12 source was compared with14. Existing NOTICE entries are retained; this reviewer added no AGPL/GPL/proprietary source. Upstream remains Codeg v0.30.4 / `6f6bd648b206412644842a98d9ffeebf57292bed`.

Reviewer branch: `review/business-integration`. Pushed recovery evidence `c0c7e522600a94bcf33674f640d5f3665cffd39b` and its31 new evidence digests remain intact. This report/evidence are the only intended commits. Final reviewer head and exact execution verdict will replace this checkpoint status.
