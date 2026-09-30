# Bounded follow-up: diagram prompt handoff

Date: 2026-09-30. Parent: [EXECPLAN.md](EXECPLAN.md).

## Owner request and scope

Derive core diagram ideas from the original article and current narration; deliver prompts to the user, who generates and returns a material pack. Review and map returned images before editor handoff. Host image generation remains available when explicitly requested.

Update the canonical author/director skills, embedded author prompt, worksheet and product documentation. Keep Storyboard/Assets/Layers/Motion schemas and import/export unchanged. Prompt plans are not media manifests. Formula highlighting and quote cards do not automatically require additional bitmaps; no fixed image count.

This uses the existing bounded-record fallback, not a new governance lifecycle. No new ADR, schema, runtime service or image-generation integration is needed. Generated user media remains outside repository changes.

## Acceptance and observations

- Trace each diagram to source and current scene/utterance; specify exact text, directed relations, limitations and file naming before handoff.
- Distinguish prompts delivered, images returned, reviewed assets and editor work. Do not invent media paths or require the image tool to author JSON.
- Preserve actual file validation and no-audio/no-API primary CLI behavior.
- Both skill-creator quick validations and git diff whitespace checks passed.
- scripts/check.py passed with the existing Node v24.19.0 selected via a command-local PATH: clippy, 39 Rust tests, TypeScript check and 29 Node tests. The first attempt used the old system Node and stopped at unsupported --test; no checks were weakened.
- cargo build --workspace and primary scripts/chat_smoke.py passed; smoke output is in ignored output/diagram-prompt-handoff-smoke. Fixtures are synthetic, not production artwork.
- Reinstalled the local CLI with cargo install --path crates/mindframe-cli --force --offline. A temporary init from that installed binary confirmed the updated handoff guidance is embedded. Existing project requests are not rewritten automatically.
- No new artwork, final video, media-render regression run or Jianying UI acceptance claimed. Optional renderer implementation and existing regression gates are unchanged.

## Prompt template follow-up

The owner asks for reusable prompt constraints based on the reviewed diagrams. Added a filled-per-project visual specification, per-image exact copy and directed-relation template, local correction template and batch delivery rules. Each diagram remains grounded in source/current narration; style and image count remain project choices. init embeds the reference so external authoring receives it without repository access. No media schema changes. Follow-up validation passed: skill quick validation, full scripts/check.py with Node v24.19.0, cargo build, primary chat_smoke.py and git diff --check. Reinstalled the CLI; a temporary init from the installed binary contains the full template verbatim. Logs and synthetic smoke fixtures are under ignored output/diagram-prompt-template-*. No new images or editor playback tested.
