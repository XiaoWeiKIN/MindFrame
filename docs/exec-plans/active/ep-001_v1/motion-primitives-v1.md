# EP-001 bounded work: Motion Primitives V1

Date: 2026-09-30. Parent: [EXECPLAN.md](EXECPLAN.md).

## Goal

Extend the verified Motion V1 with a small, source-controlled visual vocabulary for sentence-paced
knowledge explanations without exposing arbitrary executable animation code.

The optional `motion.json` is a strict Rust contract. Each step is a complete visual-state snapshot
bound to a storyboard narration index; actual frame timing still comes from reviewed audio cues.

## Primitive set

- text: short proposition/result.
- stat: label/value object; same id with changed value is a replacement.
- relation: relation row between two non-relation ids in the same snapshot.
- matrix: 2–4 columns, 1–4 rows.
- formula: exact display string, optional verbatim highlight and note.

Slots are limited to top/left/center/right/bottom. New ids appear; missing ids exit; emphasis marks the
current focus. Renderer behavior is fixed. No JS/CSS, pixel coordinates, arbitrary action names, external
code, or model-generated executable components enter the contract.

## Compatibility

`motion.json` is optional. Existing Storyboard/Assets contracts remain unchanged. Old projects without a
plan keep Motion V1 screen_text behavior. Import/validate/export preserve motion.json when present. Init
embeds the generated MotionPlan schema so chat authoring does not guess fields.

## Acceptance and observed results

Verified implementation: `0640d7fe941a90d4a609f66d50cd29857124db2c`.
GitHub Actions run: `36600494037`.

- [x] Both jobs completed success: chat-materials `109516312892`, verify `109516312639`.
- [x] Clippy with `-D warnings`, Cargo build, and all Rust tests passed: **30 tests / 0 failed**.
- [x] CLI import/validate/export round-trips optional `motion.json`; no-plan smoke remains valid.
- [x] TypeScript passed; renderer Node suite passed: **29 tests / 0 failed**.
- [x] Renderer validates step coverage, primitive shapes, relation endpoints and timed-cue binding.
- [x] Existing legacy media integration passed both preview layouts and full decode.
- [x] Real Motion integration rendered **text/stat/relation/matrix/formula** snapshots with one reusable
  background, actual local eSpeak WAV, exact fixture cue boundaries, both aspect ratios and full FFmpeg decode.
- [x] Final rendering refinement keeps unchanged ids visually stable; only new/changed elements receive the
  entry transition.

Verification artifact `11049545042`: 10,000,771 bytes at observation time,
SHA-256 `56d346026749585b0b6e4df50e05e6580994a47a90a5f4401d13a0901713f73b`.
Artifacts can expire under the workflow retention policy.

The unchanged optional Remotion dependency install still reports **5 high-severity npm findings**. This work
does not claim a dependency security audit or remediation.

Synthetic fixtures prove orchestration and contract behavior, not content accuracy, natural speech quality,
artistic quality, semantic quality or platform performance.

## Outcome

The scoped Motion Primitives V1 implementation and automated media acceptance are complete. PR review/merge
is the remaining repository action. EP-001 remains active; no RepoFoundry Harness/controller validation,
sealed checkpoint or formal archive is claimed.
