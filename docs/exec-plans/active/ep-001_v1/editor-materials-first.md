# EP-001 bounded work: editor materials first

Date: 2026-09-30. Parent: [EXECPLAN.md](EXECPLAN.md).
Base: main 50994f30e77938c22038dd7f9a89fd8e0c75fe22.
Branch: refactor/editor-materials-first.

## Owner decision and scope

The owner explicitly asks MindFrame to prepare materials, leaving voice, timing, effects and final editing to Jianying. Implement that boundary instead of improving our own video renderer. Retain old commands/files; do not delete work or infer authorization to merge this PR.

- Primary path remains init → import → validate → export; no model keys, Node, FFmpeg, audio or cues required.
- Add plain narration and per-scene screen text/editing notes. Convert optional motion snapshots into human-readable semantic-trigger instructions even without audio.
- Add optional layers.json to register actual overlay rasters with scene/index references. Keep existing Assets/Storyboard/Motion v1 contracts unchanged.
- Report raster dimensions/transparency and reference-canvas warnings without calling them an artistic or publishing-quality gate.
- Isolate old Motion orchestration as optional preview; retain motion as a CLI alias and existing output names. Clearly mark previews and fail before silently ignoring new layers.
- Update canonical Skill, author prompt, worksheet, README and architecture together; no generated user media, vault documents, secrets or font files committed.

## Acceptance (declared before execution)

- [ ] Existing Rust suite and primary PATH-empty smoke pass unchanged.
- [ ] A layered bundle round-trips actual PNG/WebP/JPEG bytes and transparency, without audio or media executables.
- [ ] Pure narration has no generated titles, IDs or Markdown; all scene scripts and screen-text files correspond to the sole edit sources.
- [ ] Edit notes preserve relation/matrix/formula content and indicate add/update/keep/remove at narration indices, not invented seconds.
- [ ] Missing/malformed/unsafe layers, duplicate IDs, bad scene/index references and symlinks fail atomically; old no-layer projects remain supported.
- [ ] Re-export reflects edits; old directories are not overwritten; source, arbitrary notes and previews are not accidentally published.
- [ ] Preview alias remains compatible and adds explicit preview metadata; existing media CI is not weakened.

Local container has no Cargo and cannot resolve github.com; this is not an authorization failure. Use the connected GitHub tools and actual Actions compiler/tests for execution evidence. Record observations, not assumed passes. Retain the existing RepoFoundry controller-unavailable bounded-record workflow; no Harness activation, accepted ADR, sealed checkpoint or formal archive is claimed.
