---
schema_version: "2.8"
metadata_schema: "1"
artifact_type: exec-plan
id: EP-001
title: "MindFrame V1: source to narrated video"
status: active
latest_checkpoint:
research_refs: []
research_gate: not_required
research_gate_reason: "The existing repository and user-confirmed workflow define this bounded implementation; no unresolved architecture research is required."
adr_refs: []
adr_constraint_refs: []
adr_evidence: []
design_refs: []
design_evidence: []
architecture_entrypoint: "ARCHITECTURE.md"
architecture_decision_gate: not_required
architecture_decision_gate_reason: "Implement the already documented Rust/Remotion direction without creating or accepting a new ADR."
architecture_compliance: not_applicable
architecture_compliance_reason: "No accepted ADR or governed Design inputs exist; the ordinary architecture document remains an implementation constraint."
required_benchmark_scenarios: []
verified_revision:
verification_evidence: []
archive_sha256:
created: 2026-09-29
updated: 2026-09-29
author: "ChatGPT"
owner: "Unassigned"
---

# MindFrame V1: source to narrated video

## Purpose / Big Picture

Turn one user-selected Markdown source into editable key points, narration and storyboard, then
images, speech, synchronized subtitles, covers and landscape/portrait MP4 files. No external research.

## Current Snapshot

- Latest checkpoint: none.
- Current milestone: implement and verify the complete local media path.
- Current state: source implemented in the V1 feature branch; local JS contract tests passed.
- Next action: execute repository CI; fix compilation/render failures, then record real evidence.
- Live provider verification and RepoFoundry controller execution remain unverified.

## Context and Orientation

Read ARCHITECTURE.md. The Rust core owns contracts; CLI owns source/API/filesystem/media orchestration;
renderer/remotion owns React composition and MP4 output. No additional crates are pre-created.

## Constraints and References

| Source | Why it matters | When to read |
|---|---|---|
| ARCHITECTURE.md | Runtime and ownership boundaries | Before implementation |
| docs/engineering/development-principles.md | Small sufficient changes, early failure, no speculative frameworks | Every task |
| README.md | Actual commands and supported input/recovery limits | Before acceptance |

## Research and Architecture Inputs

Research is not required for the bounded implementation. No new ADR or formal Design package is
created or approved. The existing Rust + TypeScript/Remotion + FFmpeg direction is preserved.
Remaining unknowns concern live provider compatibility, visual quality and untested operating systems.

## Architecture Compliance Matrix

| Architecture input | Implementation | Verification |
|---|---|---|
| Rust contract boundary | core/lib.rs and generated schemas | cargo test |
| No generated executable code | Six fixed React templates | Renderer contract tests / code review |
| Source remains authoritative | Literal source ranges and quotes | Invalid-source integration case |
| Voice drives timing | 48 kHz PCM padded to 30 fps | MP4 duration and SRT checks |

## Benchmark Gate Set

No benchmark scenario is required; acceptance is functional, with no performance or reliability SLO claim.

## Plan of Work

Implement contracts and CLI, concrete model endpoints, local test voice and frame timing. Add the
Remotion templates, both layouts, failure tests, fixture integration, CI and user instructions.

## Milestones

### M1: Inspectable content package

`plan` produces grounded key-points.md, script.md and storyboard.json. Bad source references fail.

### M2: Timed media package

`produce` creates PNG/WAV assets, complete voice.wav, SRT and timeline.json. No implicit paid retry.

### M3: Two video layouts

`render` produces H.264/AAC MP4 and cover PNG in both layouts. Visual edits do not resynthesize speech.

## Concrete Steps

From the repository root: install renderer dependencies; run `python3 scripts/check.py`; run
`cargo build --workspace`; run `python3 scripts/integration_test.py --out output/integration`.
Inspect the saved MP4, cover PNG and verification.json. Do not equate fixtures with live API verification.

## Validation and Acceptance

- [x] Local `node --test renderer/remotion/test/*.test.mjs`: 12 contract tests passed.
- [ ] `python3 scripts/check.py`: Rust clippy/tests and real TypeScript dependency checks pass.
- [ ] Generated Storyboard/Timeline JSON Schemas match Rust types.
- [ ] Loopback provider integration exercises all six visuals and both actual MP4 outputs.
- [ ] H.264/AAC, pixel format, frame rate, dimensions, duration and full decoding pass.
- [ ] Actual cloud-provider credentials: not provided; live verification remains separate.
- [ ] RepoFoundry controller validation: not run in this environment.

## Idempotence and Recovery

Use a fresh output directory for planning. Prepared audio is retained after render failure and can be
rendered again without model calls. Regenerating narration/assets requires a new project; no hidden cache
or auto-charge retry exists. Failed renders do not promote pending MP4 files over completed outputs.

## Progress

- [x] Inspected the current repository and RepoFoundry skill/controller guidance.
- [x] Implemented Rust planning/production and TypeScript visual rendering source.
- [x] Added a local deterministic fixture and 12 passing JS contract tests.
- [ ] Obtain compiler and actual video evidence from the branch CI.

## Surprises & Discoveries

The coding container has Node/FFmpeg/Chromium/eSpeak, but no Rust toolchain and no direct network DNS.
The repository initially contained only a CLI scaffold. Compiling and fetching renderer packages locally
is unavailable; CI is the intended verification path, not an assumed success.

## Decision Log

Use RepoFoundry's documented template fallback while the controller is unavailable. Repository tree
inspection showed no EP artifacts, indexes or high-water state, so the first allocated plan is EP-001.
No Harness, Spec activation, authority decision, sealed Checkpoint or archival success is claimed.

## Blockers

| ID | Status | Opened | Resolved | Missing capability | Impact | Unblock or resolution |
|---|---|---|---|---|---|---|
| ENV-1 | open | 2026-09-29 | | Local Rust/network | Local Rust/Remotion verification unavailable | Run actual GitHub CI and record results |
| LIVE-1 | open | 2026-09-29 | | User provider credentials | Live model quality/compatibility unverified | User configures endpoints and runs a reviewed source |

## Outcomes & Retrospective

Implementation and fixture checks are being completed. Do not archive as completed before collecting
required evidence. Live provider testing and governance-controller validation must remain explicitly
unverified unless actually performed.

### Knowledge promotion candidates

None.

## Interfaces and Dependencies

Storyboard schema v1, Timeline schema v1, 30 fps, two Rust crates, Node 22, Remotion 4.0.421, FFmpeg,
optional eSpeak, Chat Completions JSON, Speech WAV and GPT Image PNG/base64 endpoints.

## Artifacts and Notes

The Actions verification artifact contains schemas, test media, dependency resolution and verification.json.
Upstream template: XiaoWeiKIN/RepoFoundryAI, engineering-execution-plan/assets/execplan.md,
blob 547f5ef5097d896907cd376df0c4495472ed990e.

## Revision Notes

2026-09-29 — Initial V1 execution plan, source implementation and local JS evidence.
