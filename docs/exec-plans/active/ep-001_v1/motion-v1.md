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

GitHub CI must run:

- Clippy + all Rust tests.
- TypeScript + all renderer contract tests, including motion input rejection cases.
- Existing legacy real-media integration unchanged.
- `scripts/motion_integration_test.py`: local eSpeak utterances concatenated at known boundaries, one reusable synthetic background, both presets at 0.25 scale, ffprobe H.264/AAC/yuv420p/30fps checks and full FFmpeg decode.

Synthetic speech/backgrounds establish orchestration and timing behavior, not natural voice, visual quality, semantic correctness or platform performance.

## Deferred

Word-level karaoke highlighting, ASR/forced alignment from un-timed recordings, external MP4 loop backgrounds, transparent overlay assets, richer element primitives (arrow/counter/matrix/formula-step), automatic TTS, and native editor projects remain separate tasks.
