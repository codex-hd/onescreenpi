# Changelog

All notable changes to **OneScreenPI** are documented here.
This project is a Windows-first private screen memory product forked from [screenpipe](https://github.com/screenpipe/screenpipe).

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Progress is tracked via [Paperclip issues](https://pc.badnet.gr/ONE/issues/) — each entry links to its source issue.

> **Repo moved 2026-04-13:** canonical repo is now [codex-hd/onescreenpi](https://github.com/codex-hd/onescreenpi) (previously `cflev/OneScreenPI`).

---

## [Unreleased]

### In Progress

- **[ONE-84] Implementation planning and sprint coordination** _(in progress)_
  Coordination of implementation plan, subtask breakdown across six phases, and team handoffs based on research from ONE-78 through ONE-83.

- **[ONE-88] Retention cleanup and hard delete system** _(in progress — unblocked)_
  Retention enforcement job, batched hard deletes via deletion_job table, delete scopes (single item/time range/app/domain/all), partial failure tracking and retry. Implementation complete (7/7 DB tests green); unblocked by ONE-133 (native build deps). Engine compile verification in progress.

- **[ONE-94] QA test strategy and rollout readiness verification** _(blocked — waiting on ONE-88)_
  Automated test suite covering capture, search, delete, retention, and exclusion. Manual QA walkthrough of all 11 rollout checklist items from ONE-83. Waiting on ONE-88 to land. ONE-87/ONE-89/ONE-91 now accepted; build env fully provisioned.

### Blocked

- **[ONE-121] Sprint marketing deliverables for OneScreenPI execution** _(in progress — needs demoable build)_
  Weekly stakeholder update format, demo narrative and trust/value talking points for first reviewable Windows build. Initial update due 2026-04-14; Sprint 1 review 2026-04-24.

- **[ONE-117] Repo rebrand: QA and release validation** _(blocked — no reviewer assigned)_
  Validation of repo rename from `screenpipe` to `OneScreenPI` — checking for broken release flows, workflow names, packaging, installer/update paths, and residual reference cleanup.

### Blocked

- **[ONE-121] Sprint marketing deliverables for OneScreenPI execution** _(blocked — needs demoable build)_
  Weekly stakeholder update format, demo narrative and trust/value talking points for first reviewable Windows build. Initial update due 2026-04-14; Sprint 1 review 2026-04-24.

- **[ONE-117] Repo rebrand: QA and release validation** _(blocked — no reviewer assigned)_
  Validation of repo rename from `screenpipe` to `OneScreenPI` — checking for broken release flows, workflow names, packaging, installer/update paths, and residual reference cleanup.

---

## [2026-04-13] — Repo migrated to codex-hd/onescreenpi

### Infrastructure

- **Repo migration** — canonical repository moved from `cflev/OneScreenPI` to [codex-hd/onescreenpi](https://github.com/codex-hd/onescreenpi). Paperclip project workspace updated. The `update-changelog` GitHub Actions workflow (`windows-desktop-test` runner) carries over with the codebase. Push protection bypassed for three test-token patterns in `pii_removal.rs` (confirmed fake — PII detection unit tests only).

---

## [2026-04-10] — GO for private beta; build environment complete; three core features accepted

### Added / Completed

- **[ONE-135] Go/no-go decision: private beta approved** _(done)_
  CEO accepted risk on the remaining open item and issued GO for private beta launch (8–12 users). Rationale: pause mechanism code-verified; Windows VM DLL failure is an environment issue, not a product issue.
  **Conditions:** (1) Day 0 onboarding includes live pause/resume test per participant; (2) any observed pause delay >1s = P0, halt expansion; (3) Windows VM DLL fix tracked as non-blocking follow-up.
  CTO proceeding with beta launch. CMO notified for final marketing comms.

- **[ONE-133] Native build dependencies provisioned in agent workspaces** _(done)_
  CTO installed full native Linux build dependency sysroot at `/paperclip/.local/sysroot`. Packages: libssl-dev, libwayland-dev, libpipewire-0.3-dev, libspa-0.2-dev, libasound2-dev, libegl-dev, libclang-19-dev, cmake, libopenblas-pthread-dev. All 5 engineering agent adapters updated with `PKG_CONFIG_PATH`, `OPENSSL_*`, `LIBCLANG_PATH`, `LD_LIBRARY_PATH`, `BINDGEN_EXTRA_CLANG_ARGS`. Verified: `cargo check -p screenpipe-engine` ✅, `cargo check -p screenpipe-db` ✅, `cargo test -p screenpipe-db` ✅ 7/7. Also installed `bun@1.3.10` and ran `bun install` for frontend test coverage. Unblocks ONE-88, ONE-89, ONE-91, ONE-94.

- **[ONE-87] Clipboard capture and indexing** _(done — CTO review complete)_
  Clipboard text monitoring as a separate `memory_item` type with `item_type=clipboard` and `retention_bucket=short`. Copy/cut events indexed in `memory_text_segment` + app/window metadata as provenance segments. Paste-only filtering prevents tracking reads. Verified: `cargo check -p screenpipe-db` ✅, DB tests 7/7 green.

- **[ONE-89] System tray + capture indicator + pause controls** _(done — CTO review complete)_
  Always-visible capture state UX: persisted `running`/`paused`/`off` state, tray menu with pause (30m/1h/rest-of-day), resume, turn off/on, and quick-delete (5m/today/all). In-app capture-state pill with same controls. Startup honors saved state. Verified: `cargo check -p screenpipe-engine` ✅, frontend tests 191/195.

- **[ONE-91] Privacy settings, app exclusions, and onboarding walkthrough** _(done — CTO review complete)_
  Privacy walkthrough at first launch explains capture/storage/deletion. `assistantAccessEnabled` defaults to `false` on new installs with explicit UI toggle in Privacy settings. Retention controls surfaced in Privacy tab. Sensitive-app exclusion shortcuts for password managers and 2FA apps. Verified: frontend tests 191/195 passing.

- **[ONE-129] Resolve Rust toolchain availability in agent workspaces** _(done)_
  Rust toolchain was unavailable in the agent workspace, blocking ONE-88 and ONE-94. CTO resolved the toolchain setup. ONE-88 (retention/delete) and ONE-94 (QA) are now unblocked and ready to proceed.

---

## [2026-04-09] — Critical path complete: capture pipeline and storage layer done

### Added / Completed

- **[ONE-85] Windows capture pipeline: screenshot + metadata + OCR ingest** _(done)_
  Event-driven screenshot capture using Windows Graphics Capture API, window metadata extraction via UI Automation, capture preflight policy enforcement, session lifecycle management, and content-hash dedup at ~5s interval. This was the primary critical path item — its completion unblocks ONE-88 and advances ONE-94.

- **[ONE-86] SQLite storage layer + FTS5 search engine** _(done)_
  Full SQLite schema implementation (capture_session, memory_item, memory_asset, memory_text_segment, retention_policy, deletion_job), content-addressed screenshot file store, FTS5 virtual table with BM25 + recency ranking, and provenance-aware search API. Unblocks ONE-88 (retention system).

- **[ONE-128] Simplify implementation plan and pending task visibility** _(done)_
  Board requested clearer view of pending work. CEO rewrote the [ONE-84 plan document](/ONE/issues/ONE-84#document-plan) with a single critical path: ONE-85 → ONE-86 → ONE-88 → ONE-94 → Beta. Three tasks are code-complete and awaiting board review (ONE-87, ONE-89, ONE-91). ONE-128 closed immediately after CEO addressed it.

- **CHANGELOG automation: `update-changelog` GitHub Actions workflow** _(added)_
  Added `.github/workflows/update-changelog.yml`. Paperclip agents now trigger CHANGELOG updates via `repository_dispatch` (event type `update-changelog`). The self-hosted `windows-desktop-test` runner on the Windows host handles the git commit and push — agents no longer need `GITHUB_TOKEN` for git operations.

- **[ONE-125] Isolate Windows audio/native dependency blockers from OCR validation** _(done)_
  Confirmed OCR-specific Windows targets compile (`cargo check -p screenpipe-screen`, `cargo bench -p screenpipe-screen --bench ocr_benchmark --no-run` passed under MSVC). Separated remaining native dependency failures (cblas/OpenBLAS, libclang via audio deps) from OCR path. Proposed narrowest isolation path to keep OCR validation unblocked.

- **[ONE-92] OCR pipeline integration and tuning** _(done)_
  Benchmarked Windows native OCR (WinRT) vs Tesseract for desktop screenshot content. Optimized for dense UI text (code editors, spreadsheets, browser content). Delivered async OCR pipeline producing memory_text_segment rows with per-segment confidence scores.

- **[ONE-119] Repo rebrand: frontend and product naming to OneScreenPI** _(done)_
  App UI copy, onboarding screens, landing page content, marketing labels, README, and visible product references updated from `screenpipe` to `OneScreenPI`.

- **[ONE-118] Repo rebrand: backend, package, and infrastructure naming** _(done)_
  Rust crate and workspace metadata updated where safe, repository metadata, package names, config, workflow labels, and internal identifiers aligned with OneScreenPI. Compatibility risks for build, update channel, and MCP integration documented.

- **[ONE-116] Repo sync and GitHub push handoff** _(done)_
  Accumulated implementation work committed and pushed to [cflev/OneScreenPI](https://github.com/cflev/OneScreenPI). Remote `cflev-onescreenpi` added alongside existing `origin`. Branch `cflev-handoff-20260409-1539` established as the active working branch.

- **[ONE-55] Provide Gmail MCP test auth context for Windows QA gate** _(done)_
  Gmail/MCP test auth context routed and provided to QA team for Windows VM OAuth validation gate.

---

## [2026-04-08] — Core implementation subtasks launched; early deliverables complete

### Added / Completed

- **[ONE-96] Local data export flow** _(done)_
  Export bundle implemented: manifest.json + items.ndjson + assets/ directory. NDJSON rows include item metadata, text segments with source_kind, and linked asset paths. User-triggered export with progress indication from settings UI.

- **[ONE-95] Beta cohort recruitment and validation session design** _(done)_
  Recruited 8–12 Windows 11 knowledge worker profiles, session script (30 min), seeded recall task set (link/snippet/file/app context/number), exit interview prompts, participant diary template, and Day 0/2/5/10 validation moment scripts.

- **[ONE-90] Search UI with provenance result cards** _(done)_
  Search-first recall interface: single search bar, result cards with screenshot thumbnail/timestamp/app/window title/matched text, filter controls by app/time/content-type, keyboard navigation, empty and loading states. Match provenance visible per result (beta acceptance criterion).

- **[ONE-84] Implementation plan published and subtasks created** _(in progress since 2026-04-08)_
  Six-phase implementation plan created covering: Capture Pipeline → Storage & Search → Trust Controls & Privacy UX → Retention & Data Management → Brand & Landing Page → Beta Readiness & QA. Twelve subtasks created and assigned across the team.

### Launched (in progress/review)

- ONE-85 Windows capture pipeline (Alex Rivera) — critical path
- ONE-86 SQLite storage layer + FTS5 search (Alex Rivera) — critical path
- ONE-87 Clipboard capture and indexing (Alex Rivera) — high priority
- ONE-88 Retention cleanup and hard delete (Alex Rivera) — high priority _(now blocked)_
- ONE-89 System tray + capture indicator (Leo Martens) — critical
- ONE-91 Privacy settings, app exclusions, onboarding (Leo Martens) — high
- ONE-94 QA test strategy and rollout readiness (Tomas Reid) — high _(now blocked)_

### Cancelled

- **[ONE-93] Landing page and beta signup flow** _(cancelled 2026-04-09)_
  Descoped from current sprint. Can be revived when core product reaches demoable state.

---

## [2026-04-06] — Project inception and research phase

### Research & Design (pre-implementation — done)

The following research and design issues were completed before implementation work began:

- **[ONE-83] Beta validation plan and anti-creepy acceptance criteria** _(done)_
  Beta cohort profile, usefulness and trust/creepiness success criteria, red flags and stop conditions, rollout readiness checklist (11 items), and instrumentation plan.

- **[ONE-82] Brand system and homepage messaging** _(done)_
  Visual identity: ink navy (#1F3559), warm ivory (#F7F3EC), soft fog (#E9EEF5), muted coral (#E58C73). Homepage structure, copy tone, trust block, approved/banned language lists, and search-first hero composition.

- **[ONE-81] Trust controls and privacy UX specification** _(done)_
  System tray spec, pause control flows, quick-delete scopes, app exclusion defaults, preview surface, onboarding walkthrough, and assistant-access toggle.

- **[ONE-80] Local memory architecture** _(done)_
  SQLite schema (memory_item, memory_asset, memory_text_segment, FTS5), content-addressed asset storage, retention/deletion model, export format (manifest + NDJSON + assets), and search API design.

- **[ONE-79] Tauri capture foundation and preflight policy** _(done)_
  Capture command scaffolding, preflight policy enforcement, app denylist, private window detection, and capture session lifecycle model.

- **[ONE-78] Product strategy and vision** _(done)_
  Core product direction: Windows-first, local-only v1, privacy as a product feature, search-first UX. Guiding principles: earn trust fastest, solve recall best, make control obvious.

---

## Legend

| Status | Meaning |
|---|---|
| **done** | Work completed and merged |
| **in review** | Implementation complete, pending review/merge |
| **in progress** | Actively being worked on |
| **blocked** | Waiting on a dependency or decision |
| **cancelled** | Descoped |

---

_This file is maintained by the Paperclip agent team. Updated automatically when issue statuses change._
_Project repo: [codex-hd/onescreenpi](https://github.com/codex-hd/onescreenpi)_
