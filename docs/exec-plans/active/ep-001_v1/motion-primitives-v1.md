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

## Acceptance

- Rust contract rejects unknown fields, bad scene/utterance references, duplicate element ids, invalid relation
  endpoints, malformed matrices, and formula highlights not present in the formula string.
- CLI import/validate/export round-trips an optional motion plan without requiring Node/audio.
- Renderer contract validates step coverage and every primitive.
- Real Motion integration renders all five primitive types with one reusable background, actual local WAV,
  reviewed cue boundaries, both aspect-ratio presets, ffprobe assertions and full FFmpeg decode.
- Existing material and legacy-media gates remain unchanged.

Synthetic fixtures prove orchestration and contract behavior, not content/visual quality.
