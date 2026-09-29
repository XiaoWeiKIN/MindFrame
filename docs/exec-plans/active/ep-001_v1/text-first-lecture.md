# EP-001 bounded work: text-first knowledge lectures

Date: 2026-09-29. Parent: [EXECPLAN.md](EXECPLAN.md).
Ordinary work record, not a sealed Checkpoint, accepted ADR or archived plan.

## Owner clarification

The owner prefers simple concept/argument/formula screens over dense cinematic posters, with a reusable low-interference ocean/star background, separately authored narration and subtitles following actual speech. Persist this workflow as a Skill rather than generating more images in a project discussion.

## Scope

Base: 53b01d1d443fbb8b610ccd72fbc8530c1f5e3c38, existing PR #3 / feat/visual-director-skill.
Update the canonical visual director, embedded worksheet, author prompt/router and usage guide.
Knowledge lecture mode separates background, emphasis, narration and subtitles; retains carousel mode for explicit carousel requests. Ocean Depth is a replaceable style preset, not a hard-coded provider.
No runtime orchestration, dependency, schema, renderer, paid API or native editor change. The existing init embedding carries the new text without new runtime code. No user notes, screenshots or generated artwork are committed.

## Acceptance before CI

- [ ] Clippy and full Rust suite, including three lecture_guidance tests, pass on the new implementation.
- [ ] Actual compiled CLI embeds canonical layer/style/timing guidance outside the repository with empty PATH and no keys.
- [ ] One real synthetic PNG can map to multiple scenes; import/export preserves its bytes and separate screen_text.
- [ ] Unsupported motion/layers/background_video fields remain rejected; untimed export stays untimed.
- [ ] Existing chat-material smoke and renderer/media gates remain unchanged and passing.

Local Rust toolchain is absent. Inspect actual Actions results; historical 24-test pass on f0f9f7c does not cover this revision. The new fixtures are static test images, not generated background video or speech evidence.

## Limits and recovery

The Skill instructs layered production; this change does not implement MP4-background import, animated overlays, word-level captions or automatic new-format rendering. Those require a separate bounded implementation and actual media acceptance. Existing material commands remain available; working Markdown and extra motion files are not auto-exported.

Old project requests do not auto-update. Reinstall and init a new directory or supply the updated Skill and worksheet. Do not overwrite existing projects. Preserve previous verification records. Main is not modified or auto-merged by this task; update PR #3 with the actual revision and checks.

The previously reported optional-renderer dependency findings remain outside this change. No security audit, RepoFoundry Harness/controller activation or formal lifecycle completion is claimed.
