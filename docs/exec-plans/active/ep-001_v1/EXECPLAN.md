---
schema_version: "2.8"
metadata_schema: "1"
artifact_type: exec-plan
id: EP-001
title: "MindFrame V1: chat-authored materials to editor handoff"
status: active
latest_checkpoint:
research_refs: []
research_gate: not_required
research_gate_reason: "The owner explicitly redirected the workflow to chat-authored files and local editor export; no new architecture research is needed for this bounded implementation."
adr_refs: []
adr_constraint_refs: []
adr_evidence: []
design_refs: []
design_evidence: []
architecture_entrypoint: "ARCHITECTURE.md"
architecture_decision_gate: not_required
architecture_decision_gate_reason: "Preserve the existing two-crate Rust and optional Remotion architecture; implement the owner's explicit workflow clarification without inventing ADR acceptance."
architecture_compliance: not_applicable
architecture_compliance_reason: "No accepted ADR or governed Design inputs exist; ARCHITECTURE.md remains an implementation constraint."
required_benchmark_scenarios: []
verified_revision:
verification_evidence: []
archive_sha256:
created: 2026-09-29
updated: 2026-09-29
author: "ChatGPT"
owner: "Unassigned"
---

# MindFrame V1: chat-authored materials to editor handoff

## Purpose / Big Picture

The owner supplies documents in chat, reviews key ideas and narration, and obtains actual images
from the chat image-generation tool. MindFrame imports the saved storyboard and images, then exports
standard materials for manual editing in Jianying. The primary CLI path requires no model key,
network call, FFmpeg, Node or editor installation. The original API/media pipeline remains optional.

## Current Snapshot

- Active work: init/import/validate/export and portable authoring instructions.
- Added Project/Assets/Cue contracts while retaining Storyboard and legacy Timeline v1.
- Added real image decoding, WAV inspection, exact scene mapping and staged non-overwriting output.
- Added Rust unit/CLI tests and scripts/chat_smoke.py; new revision awaits actual CI results.
- Historical e3bfca7: compiler/static checks passed in Actions; real-media acceptance failed at the
  H.264/yuv420p assertion. Do not reinterpret that run as full media acceptance.
- Next action: run the new chat-materials CI job, inspect failures and fix them; preserve optional
  media failures as visible work rather than weakening acceptance.

## Context and Orientation

- crates/mindframe-core/src/lib.rs: source-grounded Storyboard v1 and old generated Timeline.
- crates/mindframe-core/src/materials.rs: Project, Assets, Cue, path/mapping/timing contracts.
- crates/mindframe-cli/src/materials.rs: local init/import/validate/export I/O.
- crates/mindframe-cli/src/main.rs: primary commands and explicit legacy-mode boundaries.
- crates/mindframe-cli/src/pipeline.rs and renderer/remotion/: preserved optional API/media path.
- prompts/chat-authoring.md and skills/mindframe-author/SKILL.md: portable authoring handoff.
- scripts/chat_smoke.py: actual CLI acceptance with synthetic assets and no model credentials.

## Constraints and References

| Source | Why it matters | When to read |
|---|---|---|
| docs/engineering/development-principles.md | Minimal sufficient complexity, fail fast, validation ownership | Before edits |
| ARCHITECTURE.md | Local primary path versus optional API/media boundary | Before contract changes |
| prompts/chat-authoring.md | Actual files, not guessed attachments or generated-image claims | When creating bundles |
| README.md | Supported input, editing, export and non-goals | At handoff |

Source is one explicit UTF-8 Markdown snapshot, not a vault scan. Narration has one editing authority
in Storyboard. Images are actual PNG/JPEG/WebP bytes. No arbitrary code execution, session-cookie
access, auto-publishing, model fallback or silent paid generation. No audio/timings means no SRT.
Source references establish provenance, not semantic correctness.

## Research and Architecture Inputs

No formal Research/ADR/Design was required for this bounded implementation of the clarified product
workflow. Existing ordinary architecture and owner rules apply. This is not an assertion of human
approval of a newly drafted ADR. RepoFoundry controller/Harness was unavailable locally; the existing
template fallback remains explicit. No formal lifecycle seal or archive has been fabricated.

## Architecture Compliance Matrix

| Input | Implementation | Verification |
|---|---|---|
| Two Rust crates | Core owns contracts; CLI owns I/O | Cargo checks |
| Content reviewed in chat | Authoring handoff and local bundle import | No-key CLI tests |
| Source grounding | Reuse existing Storyboard validation | Invalid quote tests |
| Editor handoff | Standard media, CSV, Markdown and optional SRT only | Export assertions |
| Preserve old path | Existing pipeline/renderer and original media gate remain | Full-media CI |

## Benchmark Gate Set

None. This is functional verification, not a throughput or quality benchmark. No measured performance
or model-quality result is claimed.

## Plan of Work

Publish the local-material implementation and tests on feat/v1-knowledge-to-video, retaining PR #1
as draft while acceptance is evaluated. Update product docs and this live plan with observed results.
Do not merge main or archive the plan merely because code has been written.

## Milestones

1. Chat handoff: init emits source snapshot, line-number aid and exact authoring schemas.
2. Material intake: import maps every scene to an actual image, preserves bytes and handles optional audio.
3. Editor export: untimed text works without recordings; supplied timing is checked before SRT export.
4. Verification: actual CLI tests, no-key/PATH-empty smoke test and inspectable output artifacts.
5. Optional media: retain existing tests and record the separate remaining renderer failure.

## Concrete Steps

From repository root:

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
python3 scripts/chat_smoke.py --out output/chat-smoke
python3 scripts/check.py
python3 scripts/integration_test.py --out output/integration
```

## Validation and Acceptance

- [ ] New revision compiles and passes Clippy and all Rust tests.
- [ ] Actual CLI init/import/validate/export succeeds with no keys and empty PATH.
- [ ] PNG/JPEG/WebP inputs are decoded and their original bytes preserved.
- [ ] Untimed export has plain subtitles and empty CSV times, not invented SRT.
- [ ] WAV plus exact supplied cues yields SRT; stale text, overlap and audio overrun fail.
- [ ] Missing/bad files, traversal, symlinks and existing destinations fail without partial publication.
- [ ] Smoke artifact documents synthetic image/audio provenance and no editor UI acceptance.
- [ ] Optional media path passes the unchanged H.264/yuv420p and full-decode gates.

This plan remains active. verified_revision, verification_evidence and archive_sha256 stay unset
until a formal controller-supported archival action with real completion evidence is appropriate.

## Idempotence and Recovery

init, import and export do not overwrite existing destinations. A failed import leaves initialized
source files in place and does not publish content/. Edit content/storyboard.json and assets.json
for local revisions; export to a new directory. New full imports use a new project. Temporary staging
is cleaned on normal failure. Concurrent writes to the same project are unsupported.

## Progress

- [x] Earlier e3bfca7 implemented source-to-video API and fixed-template rendering code.
- [x] Its CI compiled and passed static checks; media acceptance failed, so no completion was recorded.
- [x] Owner clarified that authoring and image generation happen in chat, with Jianying material output.
- [x] Added local workflow implementation, contracts, tests and authoring guide.
- [ ] Inspect the new revision's CI and repair actual failures.

## Surprises & Discoveries

The earlier claim that GitHub write permission was missing was incorrect. The connector reports
repository write permissions. Local shell network/toolchain limitations are not GitHub authorization failures.

## Decision Log

- 2026-09-29: Preserve Storyboard v1 instead of adding independent script/import-script authority.
- 2026-09-29: Images map by scene ID; no filename-order guessing or native editor-draft generation.
- 2026-09-29: Omit SRT when recording/timing data is absent. User-supplied timing is not ASR verification.
- 2026-09-29: Keep legacy API/media commands explicit and separate; no silent paid fallback.

## Blockers

| ID | Status | Opened | Resolved | Missing capability | Impact | Unblock or resolution |
|---|---|---|---|---|---|---|
| ENV-1 | open | 2026-09-29 | | Local Rust and package network | Cannot run Rust locally | Use actual Actions compiler/test results |
| MEDIA-1 | open | 2026-09-29 | | Verified legacy MP4 encoding | Optional media acceptance failed | Inspect ffprobe output; keep assertion |
| EDITOR-1 | open | 2026-09-29 | | User's Jianying UI/version | No editor UI acceptance | Manual import verification; do not claim native draft compatibility |

Live API testing has not been performed and is not required by the new no-key primary workflow.

## Outcomes & Retrospective

Implementation written; new CI evidence pending. This is not an archived completed plan.

### Knowledge promotion candidates

Separate prose, recorded audio, supplied timestamps and verified speech alignment in all future exports.

## Interfaces and Dependencies

Rust serde/schemars contracts, existing Clap CLI, image crate with PNG/JPEG/WebP only, hound for
PCM WAV inspection, tempfile for staged local publication. No new provider framework or cloud service.

## Artifacts and Notes

PR #1 retains implementation and verification history. Earlier Actions run: 36556040924, job109365306455.
New CI output belongs in its own artifacts; synthetic fixtures are never described as GPT-generated images.

## Revision Notes

- 2026-09-29: Original V1 API/media work recorded using the controller-unavailable template fallback.
- 2026-09-29: Scope explicitly changed to chat-first materials at the owner's request; prior media failure retained.
