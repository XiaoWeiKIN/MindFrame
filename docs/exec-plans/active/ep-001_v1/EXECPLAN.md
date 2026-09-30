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

- Current owner clarification: [diagram prompt handoff](diagram-prompt-handoff.md) makes source/script-grounded prompts and user-returned artwork the default authoring flow, without changing media contracts.

- Current authoring follow-up: [visual review](visual-review.md) adds role-aware image guidance and embeds it in init;
  its bounded record tracks local verification. It preserves the existing [narration guidance](narration-authoring.md).
- Primary init/import/validate/export implementation is tested at c04cb33b3f8782cf5dd2a488ae084cbdc102dcdc.
- Actions run36569380103, job109409249398 chat-materials: completed success at 2026-09-29T12:42:45Z.
- Clippy, all 20 Rust tests, Cargo build and actual no-key/PATH-empty CLI smoke passed.
- Actual fixture pack uploaded as artifact11033741157; source is synthetic PNG/silent WAV, not GPT art.
- Follow-ups 37d1155 and 6023257 changed ignore rules, CI diagnostics and evidence only.
- The optional verify job109409249723 subsequently failed. Downloaded artifact11033467585 and
  checked both MP4s locally: H.264/AAC, 30fps, but pixel format yuvj420p and full range pc.
- Correction dfea5df7f7dd0bafc65e9e89229bee1e42f9fe5c selects PNG capture and explicit BT.709 in
  the optional renderMedia call. Its full-media acceptance is not yet claimed. Rust primary code is unchanged.
- Evidence: [primary verification](../../../verification/chat-materials-2026-09-29.md) and
  [media diagnosis and correction](../../../verification/legacy-media-diagnostic-2026-09-29.md).
- Next action: inspect the dfea5df correction's actual full-media CI, keeping the original codec and
  full-decode assertions. User-version Jianying UI acceptance remains a separate environment-dependent check.

## Context and Orientation

- crates/mindframe-core/src/lib.rs: source-grounded Storyboard v1 and old generated Timeline.
- crates/mindframe-core/src/materials.rs: Project, Assets, Cue, path/mapping/timing contracts.
- crates/mindframe-cli/src/materials.rs: local init/import/validate/export I/O.
- crates/mindframe-cli/src/main.rs: primary commands and explicit legacy-mode boundaries.
- crates/mindframe-cli/src/pipeline.rs and renderer/remotion/: optional API/media path.
- prompts/chat-authoring.md and skills/mindframe-author/SKILL.md: portable authoring handoff.
- scripts/chat_smoke.py: actual CLI acceptance with synthetic assets and no model credentials.

## Constraints and References

| Source | Why it matters | When to read |
|---|---|---|
| docs/engineering/development-principles.md | Minimal sufficient complexity, fail fast, validation ownership | Before edits |
| ARCHITECTURE.md | Local primary path versus optional API/media boundary | Before contract changes |
| prompts/chat-authoring.md | Actual files, not guessed attachments or model claims | When creating bundles |
| README.md | Supported input, editing, export and non-goals | At handoff |
| docs/verification/chat-materials-2026-09-29.md | Exact tested revision and primary CI evidence | Before interpreting completion |
| docs/verification/legacy-media-diagnostic-2026-09-29.md | Real ffprobe result and bounded correction | Before optional renderer work |

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
| Two Rust crates | Core owns contracts; CLI owns I/O | Clippy/build passed in job109409249398 |
| Content reviewed in chat | Authoring handoff and local bundle import | No-key CLI tests and smoke passed |
| Source grounding | Reuse Storyboard validation | Invalid quote tests passed |
| Editor handoff | Standard media, CSV, Markdown and optional SRT only | Export assertions passed; editor UI not tested |
| Preserve old path | Optional pipeline/renderer and original media gate remain | Failed pixel-format acceptance diagnosed; correction pending |

## Benchmark Gate Set

None. This is functional verification, not a throughput or quality benchmark. No measured performance
or model-quality result is claimed.

## Plan of Work

The primary implementation is published and tested on feat/v1-knowledge-to-video. Keep PR #1 draft
while optional-media correction acceptance is evaluated. Update this live plan with observed results.
Primary functional success does not authorize a fabricated governance seal or automatic merge.

## Milestones

1. Chat handoff: source snapshot, line-number aid and exact authoring schemas — verified.
2. Material intake: explicit image mapping, original bytes and optional audio — verified.
3. Editor export: untimed text and explicitly supplied WAV/cue timing paths — verified.
4. Verification: actual CLI tests, no-key smoke and inspectable fixture artifacts — passed.
5. Optional media/editor UI: separately outstanding; not inferred from material export tests.

## Concrete Steps

From repository root, use new output directories for each independent run:

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
python3 scripts/chat_smoke.py --out output/chat-smoke
python3 scripts/check.py
python3 scripts/integration_test.py --out output/integration
```

## Validation and Acceptance

Evidence for checked items: c04cb33, Actions run36569380103 / job109409249398,
full logs inspected and fixture artifact11033741157 retained.

- [x] New revision compiles and passes Clippy and all 20 Rust tests.
- [x] Actual CLI init/import/validate/export succeeds with no keys and empty PATH.
- [x] PNG/JPEG/WebP inputs are decoded and original bytes preserved.
- [x] Untimed export has plain subtitles and empty CSV times, not invented SRT.
- [x] WAV plus exact supplied cues yields SRT; stale text, overlap and audio overrun fail.
- [x] Missing/bad files, traversal, symlinks and existing destinations fail without partial publication.
- [x] Smoke artifact documents synthetic provenance and no editor UI acceptance.
- [ ] Corrected optional media passes unchanged H.264/yuv420p and full-decode gates.
- [ ] Standard images/audio/optional SRT manually imported in the user's Jianying version.

This plan remains active. verified_revision, verification_evidence and archive_sha256 stay unset
until a formal controller-supported archival action is appropriate. Ordinary verification notes
preserve the actual tested revisions without faking archival metadata.

## Idempotence and Recovery

init, import and export do not overwrite existing destinations. A failed import leaves initialized
source files in place and does not publish content/. Edit content/storyboard.json and assets.json
for local revisions; export to a new directory. New full imports use a new project. Temporary staging
is cleaned on normal failure. Concurrent writes to the same project are unsupported.

## Progress

- [x] Earlier e3bfca7 implemented API/media code; its compiler/static checks passed but media acceptance failed.
- [x] Owner clarified chat authoring/image generation and Jianying material output.
- [x] Added local workflow, contracts, tests and authoring guide at c04cb33.
- [x] Inspected actual primary CI: Clippy, 20 tests, build and CLI smoke all passed.
- [x] Recorded exact revision, job, artifact and limitations in ordinary verification documentation.
- [x] Inspected the optional failure's actual MP4 files: full-range yuvj420p; kept original assertions.
- [x] Submitted bounded PNG/BT.709 correction at dfea5df; no extra fallback transcode.
- [ ] Verify that correction end to end and perform editor UI acceptance.

## Surprises & Discoveries

Local shell network/toolchain limitations are not GitHub permission failures. Actual Actions execution
resolved the primary compiler/test evidence gap without installing Rust locally.
The old renderer already specified yuv420p but produced full-range yuvj420p; checking actual files
was necessary. Do not repeat the old option as a supposed fix or delete the assertion to obtain green CI.

## Decision Log

- 2026-09-29: Preserve Storyboard v1 instead of introducing an independent script editing authority.
- 2026-09-29: Map images explicitly by scene ID; no filename guessing or native editor-draft generation.
- 2026-09-29: Omit SRT without recording/timings; supplied cues are not ASR verification.
- 2026-09-29: Keep legacy API/media commands separate, with no silent paid fallback.
- 2026-09-29: Correct observed optional-media color conversion with documented renderer options, not weaker tests.

## Blockers

| ID | Status | Opened | Resolved | Missing capability | Impact | Unblock or resolution |
|---|---|---|---|---|---|---|
| ENV-1 | resolved | 2026-09-29 | 2026-09-29 | Local Rust/network | Local execution unavailable | Actual Actions compiler/tests passed; primary evidence no longer blocked |
| MEDIA-1 | open | 2026-09-29 | | Correction acceptance | Historical and c04 optional media acceptance failed | Inspect dfea5df real render/ffprobe; retain strict gate |
| EDITOR-1 | open | 2026-09-29 | | User's Jianying UI/version | No UI acceptance | Manual import verification; no native draft compatibility claim |

Live API testing has not been performed and is not required by the new no-key primary workflow.

## Outcomes & Retrospective

The chat-authored material path is implemented and passed actual Rust/CLI acceptance at c04cb33.
These tests establish local file/export behavior, not model creativity, natural speech, content correctness
or native-editor interoperability. Optional video correction and editor UI acceptance remain unfinished.
This is not an archived completed plan.

### Knowledge promotion candidates

Separate prose, recorded audio, supplied timestamps and verified speech alignment in all future exports.

## Interfaces and Dependencies

Rust serde/schemars, existing Clap CLI, image with PNG/JPEG/WebP only, hound for PCM WAV inspection,
tempfile for staged publication. No new provider framework or cloud service. The optional renderer
uses the existing Remotion options; primary commands do not launch it.

## Artifacts and Notes

- PR #1 retains implementation and verification history.
- Primary pass: run36569380103 / job109409249398 / artifact11033741157.
- Primary artifact SHA-256: fac2ccc82604c341908fd9276f3490a37cb2579dfd6804b98da49b41520e1b17.
- Failed optional media: same run / job109409249723 / artifact11033467585.
- Media artifact SHA-256: da9c3f4cb3d3dc49ab1f22d403d1664805a566eace7dea30590bae4e4af1efbf.
- Synthetic fixture art/audio is never described as GPT generation or natural narration.

## Revision Notes

- 2026-09-29: Original V1 API/media work recorded with the controller-unavailable template fallback.
- 2026-09-29: Owner changed scope to chat-first materials; prior media failure retained.
- 2026-09-29: Recorded passing primary CI, diagnosed full-range media, and submitted bounded correction.
