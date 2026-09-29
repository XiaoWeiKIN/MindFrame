# EP-001 bounded work: text-first knowledge lectures

Date: 2026-09-29. Parent: [EXECPLAN.md](EXECPLAN.md).
Ordinary work record, not a sealed Checkpoint, accepted ADR or archived plan.

## Owner clarification

The owner prefers simple concept/argument/formula screens over dense cinematic posters, with a reusable low-interference ocean/star background, separately authored narration and subtitles following actual speech. Persist this workflow as a Skill rather than generating more images in a project discussion.

## Scope

Base: 53b01d1d443fbb8b610ccd72fbc8530c1f5e3c38, existing PR #3 / feat/visual-director-skill.
Implementation: ec8e3b85a93a8048a0c6eb4f943f84a2b59e64a1.
Update the canonical visual director, embedded worksheet, author prompt/router and usage guide.
Knowledge lecture mode separates background, emphasis, narration and subtitles; retains carousel mode for explicit carousel requests. Ocean Depth is a replaceable style preset, not a hard-coded provider.
No runtime orchestration, dependency, schema, renderer, paid API or native editor change. The existing init embedding carries the new text without new runtime code. No user notes, screenshots or generated artwork are committed.

## Acceptance and observed evidence

- [x] Clippy and full Rust suite: 27 tests passed, including all three lecture_guidance tests, on the implementation's actual PR test merge.
- [x] Actual compiled CLI embeds canonical layer/style/timing guidance outside the repository with empty PATH and no keys.
- [x] One real synthetic PNG maps to multiple scenes; import/export preserves its bytes and separate screen_text.
- [x] Unsupported motion/layers/background_video fields remain rejected; untimed export stays untimed.
- [x] Existing chat-material CLI smoke passed.
- [x] Existing full renderer/media job concluded success, including static/build, actual legacy render, media inspection and artifact upload steps.

Inspected full decoded primary-job logs and actual step summaries for run36583386083, job109457158543 / chat-materials. GitHub checked out test merge f971120b3e840bae3dab363bdd595d7fa4e60fe3 of ec8e3b8 into main0df0319. The primary job completed successfully at 2026-09-29T14:33:13Z. Commands were cargo clippy --workspace --all-targets -- -D warnings, cargo test --workspace, cargo build --workspace, and python3 scripts/chat_smoke.py --out output/chat-smoke.

Actual new test names:
- lecture_intent_does_not_extend_assets_schema
- lecture_handoff_embeds_layers_style_and_truthful_timing_boundaries
- explicit_shared_background_preserves_bytes_and_separate_screen_text

The log reports 4 boundary + 3 lecture + 8 material + 4 visual-director + 8 core tests, all passed. It records untimed and supplied-WAV/cue smoke exports. Artifact11040262439 contains47 fixture files; ZIP38,596bytes, SHA-256 9d6028abfae87e6a576363e5ba8d8e35d2be680aa3e9009c3577544bd045ce3c. Fixtures are synthetic static images and silent WAV, not generated background video, speech or visual-quality evidence. Hosted artifacts can expire.

The full-media job109457157937 was still running when the first evidence update was made. A subsequent read confirmed status completed / conclusion success with every recorded step successful. This conclusion comes from actual job/step results; full decoded verify-job logs and its MP4 bytes were not re-inspected in this task. The media test is the preserved legacy path, not new-format layered-video acceptance.

Local Rust was not run. Local standalone links, existing JSON syntax and required guidance checks passed. Standalone package's canonical Skill/worksheet blob hashes match the submitted files. Follow-up evidence updates change documentation only; they are not new independently verified code revisions. Historical24-test evidence remains unchanged.

## Limits and recovery

The scoped Skill change and functional checks are complete; PR remains unmerged. The Skill instructs layered production; this change does not implement MP4-background import, animated overlays, word-level captions or automatic new-format rendering. Those require a separate bounded implementation and actual media acceptance. Existing material commands remain available; working Markdown and extra motion files are not auto-exported.

Old project requests do not auto-update. Reinstall and init a new directory or supply the updated Skill and worksheet. Do not overwrite existing projects. Preserve previous verification records. Main is not modified or auto-merged by this task; PR #3 holds implementation and current check status.

The previously reported optional-renderer dependency findings remain outside this change. No security audit, RepoFoundry Harness/controller activation or formal lifecycle completion is claimed.
