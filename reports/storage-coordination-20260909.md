# Storage incident and bounded recovery

Approvals reported a failed native report write with117MiB free on the Data
volume. Source/report remained intact. Root paused generated writes and builds
in all three Ops lanes and coordinated with Farha, Uramax and EduBlend owners.
Free-space readings fluctuated between935MiB and1.3GiB before this recovery.
No active rustc was observed; Cargo parents of existing test fixtures were left
running. These process observations do not establish all system allocation causes.

Read-only inventory found314GB across the three Ops worktrees and79GB in root.
Root's own `src-tauri/target/debug/incremental` accounted for26,739,556,352 bytes
reported by du. This is compiler cache, not the final binaries or native bundle.
Grounding: installed Cargo1.98.0 (`797e8a9bc`) build-cache/profiles documentation
and [the matching official Cargo source](https://github.com/rust-lang/cargo/blob/797e8a9bc/src/doc/src/reference/build-cache.md)
read through gh api explicitly identify debug/incremental as regenerable cache.

Root removed only that exact root-owned directory after verifying its resolved
path was not a symlink, no contents were tracked, global lsof completed without
errors or matching open handles, and no rustc/rust-lld process was active.
All three root debug executables and all three app-bundle executables were hashed
before and after; all six match. No worker target, compiled dependency directory,
source, report, fixture, browser, credential, virtual environment or native bundle
was deleted or restarted. Rebuilding root later may take longer without its cache.

[Exact result and hashes](storage-reclamation-20260909.json) record physical
free space1,676,689,408 →18,746,576,896 bytes: observed increase17,069,887,488
bytes, about15.9GiB. Subsequent df shows17GiB available. This is a measured volume
change during the operation, not a claim that du's entire logical size became
free or that concurrent APFS/system activity was absent.

Other owners separately reported ordinary offline uv cache pruning (25.5GiB
logical, without equivalent physical headroom) and Farha's457MiB Next cache
removal. Root did not perform those actions. Further reclamation was stopped once
root's recovery restored headroom. EduBlend's separately assigned investigator
continues read-only RAM/swap/APFS diagnostics; no second investigator was spawned.

Small source/report writes are resumed. Tickets owns one bounded existing-target
Rust build window after the three fixes are ready, with fresh df before starting:
do not start below10GiB available, and stop/report if approaching5GiB. Approvals
continues source review until tickets explicitly releases the window. No parallel
Rust build, new target, static export or native rebuild is part of this recovery.
All active project owners received the measured capacity and preservation limits.
