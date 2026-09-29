# EP-001 bounded work: Motion V1

Date: 2026-09-30. Parent: [EXECPLAN.md](EXECPLAN.md).

## Goal

Close the smallest useful production loop for chat-material projects:

```text
reviewed storyboard + actual background rasters + actual WAV + reviewed sentence cues
→ motion-input.json
→ Remotion
→ H.264/AAC MP4
```

This is not a native Jianying project, TTS provider, ASR/forced-alignment system, or arbitrary animation language.

## Contract

- New CLI command: `mindframe motion PROJECT --out OUT --preset douyin|bilibili`.
- Requires imported chat materials, a real PCM16 WAV and complete reviewed cues. Missing timing fails; no text-length estimation.
- Each scene keeps its imported raster background. The renderer applies a deterministic low-interference pan/zoom.
- `assets.images[].screen_text` becomes the editable emphasis layer; empty values fall back only to existing Title/KeyPoint headings.
- Sentence subtitles use the exact supplied cue boundaries. The imported WAV is the single narration track.
- Output is published atomically only after the renderer succeeds: motion.mp4, cover.png, motion-input.json, subtitles.srt, assets/.
- No new public Storyboard/Assets fields. Motion input is an internal renderer boundary and is separately validated in JS.

## Verification

Implementation revision: `2cc7fa7eb8bb4953a039db6645874a1345155eaf`.
Actual PR Actions run: `36596356617`.

Observed results:

- chat-materials job `109502156899`: completed success.
- verify job `109502156708`: completed success.
- Clippy with `-D warnings`, Cargo build and all Rust tests passed: 27 tests, 0 failed.
- Renderer TypeScript check passed; Node test suite passed: 22 tests, 0 failed (legacy + new motion-contract cases).
- Existing legacy real-media integration still passed for Bilibili 480×270 and Douyin 270×480 previews, H.264/AAC, full decode.
- New `scripts/motion_integration_test.py` passed: local eSpeak utterances concatenated at exact fixture boundaries, one reusable synthetic PNG background, reviewed-cue contract, subtitles, both preview layouts and full FFmpeg decode.
- Media artifact `11046073397` retained 7,301,211 bytes, digest `sha256:56871b42bab1ca5306c29ff1ef6488c9c7e49d79b12c72c21a72670887353a0b` at the time of verification.
- Existing optional renderer dependency install still reported 5 high-severity npm findings. They were not audited or remediated by Motion V1; no security-audit pass is claimed.

Synthetic speech/backgrounds establish orchestration and timing behavior, not natural voice, visual quality,
semantic correctness, word-level alignment or platform performance.

## Deferred

Word-level karaoke highlighting, ASR/forced alignment from un-timed recordings, external MP4 loop backgrounds, transparent overlay assets, richer element primitives (arrow/counter/matrix/formula-step), automatic TTS, and native editor projects remain separate tasks.


## Outcome

The scoped Motion V1 implementation and automated media acceptance are complete. PR review/merge is the
remaining repository action. EP-001 remains active because broader editor/UI and product work is not formally
archived through RepoFoundry controller/Harness.
