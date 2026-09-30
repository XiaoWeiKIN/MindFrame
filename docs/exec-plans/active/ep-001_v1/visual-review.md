# EP-001 bounded work: visual roles and master-background review

Date: 2026-09-30. Parent: [EXECPLAN.md](EXECPLAN.md).

## Scope and constraints

The owner asks to promote the image-review lessons into the related MindFrame skills. Keep the existing
whole-script narration guidance; add visual-role decisions, user-generated-image handoff, reusable master
backgrounds and content-over-background review. Do not make the example palette, four image variants or
a user approval round mandatory for every task. No user source text or generated media enters this repository.

Canonical guidance lives in the visual director's references/visual-review.md, routed from the two skills,
the authoring prompt and worksheet. init embeds it in chat-request.md so a user generating in another chat
does not need repository access. Existing schemas, renderer and export semantics remain unchanged.

The execution controller reports EP-001 active. This bounded work does not complete the parent plan,
rewrite historical evidence, or resolve its media/editor blockers. No new Research, ADR or governed Design
is needed for this authoring-guidance change within the existing architecture.

## Acceptance

- Review requests and user-generated-image workflows yield advice/prompts/review without implicit image generation.
- Guidance distinguishes backgrounds, explanatory graphics and covers, and does not mandate a fixed image count.
- Bare-image suitability, real-content composition, mobile readability and playback/effectiveness have separate evidence.
- Existing init integration test verifies full canonical guidance inclusion outside the repository, with no keys and empty PATH.
- Run skill quick validation, repository check.py, actual CLI chat_smoke and a newly installed CLI handoff check.
- No generated video, image or native editor task is part of this change.

## Verification

- Both related skills pass skill-creator quick validation; their relative Markdown links resolve.
- Repository `python3 scripts/check.py` passes: Clippy, 39 Rust tests, TypeScript checking and 29 renderer tests. The first attempt hit the shell's Node 16 unsupported test runner; the successful run explicitly uses the existing Node 24.19.0. Final log: `output/visual-review-check-final.log`.
- `cargo install --path crates/mindframe-cli --force` rebuilt the local executable at `/Users/wangxiaowei1/.cargo/bin/mindframe`; log: `output/visual-review-install.log`.
- `scripts/chat_smoke.py` passes against that installed executable, covering untimed and timed synthetic material packs. Log: `output/visual-review-smoke.log`. Its new `chat-request.md` contains the complete canonical visual-review and narration references.
- No user images, video export or native editor project was changed. These checks establish guidance distribution and existing material-pack behavior, not visual quality or platform effectiveness. Existing project requests are not automatically rewritten.
