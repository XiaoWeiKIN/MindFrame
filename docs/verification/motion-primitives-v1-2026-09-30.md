# Motion Primitives V1 verification — 2026-09-30

Verified implementation: `0640d7fe941a90d4a609f66d50cd29857124db2c`
Branch: `feat/motion-primitives`
PR: #6
Actions run: `36600494037`

## Results

Both jobs completed successfully:

- chat-materials / `109516312892`
- verify / `109516312639`

Observed verification:

- Clippy with `-D warnings` passed.
- Cargo build passed.
- Rust tests: **30 passed, 0 failed** across boundary, lecture, materials, motion-primitives,
  visual-director and core suites.
- TypeScript renderer check passed.
- Node tests: **29 passed, 0 failed**.
- Existing legacy real-media fixture rendered Bilibili 480×270 and Douyin 270×480 previews as
  H.264/AAC and fully decoded.
- Motion integration printed:
  `Motion primitives: text/stat/relation/matrix/formula snapshots, real WAV, reviewed cues, both MP4 layouts and full decode passed.`
- Existing ffprobe media inspection continued to report `yuv420p`.

## What the Motion fixture proves

The fixture uses one synthetic dark PNG for every scene and local eSpeak speech. Separate utterance WAV
frame counts establish exact synthetic cue boundaries before concatenation. It then renders:

1. stat snapshots with 18:00 → 19:00 replacement;
2. an emphasized boss “+1” stat and relation;
3. the final equal-time state plus a result text;
4. a compact payoff matrix;
5. a state-transition formula with one literal highlighted substring.

Both Douyin and Bilibili preview outputs are ffprobe-checked and fully decoded with FFmpeg. The final renderer
also compares element ids across adjacent steps so unchanged elements remain visually stable while new/changed
elements receive the entry transition.

This does **not** prove natural voice quality, content correctness, aesthetic quality, word-level alignment,
or publishing performance.

## Artifacts

- `mindframe-v1-verification`: ID `11049545042`, 10,000,771 bytes,
  SHA-256 `56d346026749585b0b6e4df50e05e6580994a47a90a5f4401d13a0901713f73b`.
- `mindframe-chat-materials`: ID `11048453126`, 43,244 bytes,
  SHA-256 `2a94ff0f5c7bcef55bb881ced8b7726ec6b4a97109bd86506bb82c0cbafcf481`.

Artifacts may expire according to the workflow retention policy.

## Known limits

- Steps bind to sentence/utterance starts, not arbitrary millisecond events.
- No word-level karaoke or ASR/forced alignment from untimed recordings.
- No external MP4 background loop contract.
- Formula is a controlled display string, not a full LaTeX typesetting engine.
- No arbitrary animation DSL, generated React/JS/CSS, or free pixel coordinates.
- No native Jianying project.
- The unchanged optional renderer dependency installation reports 5 high-severity npm findings; detailed
  audit/remediation was not performed.

No RepoFoundry Harness/controller validation, sealed checkpoint or formal EP archival is claimed.
