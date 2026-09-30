# EP-001 bounded work: editor materials first

Date: 2026-09-30. Parent: [EXECPLAN.md](EXECPLAN.md).
Base: main 50994f30e77938c22038dd7f9a89fd8e0c75fe22.
Branch: refactor/editor-materials-first. PR: #7.
Verified implementation: 4886e46e3453de4a7ee40c55651526b23c8680ac.

## Owner decision and scope

The owner explicitly asks MindFrame to prepare materials, leaving voice, timing, effects and final editing to Jianying. Implement that boundary instead of improving our own video renderer. Retain old commands/files; do not delete work or infer authorization to merge this PR.

- Primary path remains init → import → validate → export; no model keys, Node, FFmpeg, audio or cues required.
- Add plain narration and per-scene screen text/editing notes. Convert optional motion snapshots into human-readable semantic-trigger instructions even without audio.
- Add optional layers.json to register actual overlay rasters with scene/index references. Keep existing Assets/Storyboard/Motion v1 contracts unchanged.
- Report raster dimensions/transparency and reference-canvas warnings without calling them an artistic or publishing-quality gate.
- Isolate old Motion orchestration as optional preview; retain motion as a CLI alias and existing output names. Clearly mark previews and fail before silently ignoring new layers.
- Update canonical Skill, author prompt, worksheet, README and architecture together; no generated user media, vault documents, secrets or font files committed.

## Acceptance (declared before execution; now observed)

- [x] Existing Rust suite and primary PATH-empty smoke pass unchanged.
- [x] A layered bundle round-trips actual PNG/WebP/JPEG bytes and transparency, without audio or media executables.
- [x] Pure narration has no generated titles, IDs or Markdown; all scene scripts and screen-text files correspond to the sole edit sources.
- [x] Edit notes preserve relation/matrix/formula content and indicate add/update/keep/remove at narration indices, not invented seconds.
- [x] Missing/malformed/unsafe layers, duplicate IDs, bad scene/index references and symlinks fail atomically; old no-layer projects remain supported.
- [x] Re-export reflects edits; old directories are not overwritten; source, arbitrary notes and previews are not accidentally published.
- [x] Preview alias remains compatible and adds explicit preview metadata; existing media CI is not weakened.

## Observed execution

GitHub Actions run 36658340104 checked out test merge 134a89275b593ee8b6b8412cea30331e27ab0313 of implementation 4886e46 into main50994f3. Both jobs completed successfully: chat-materials109707373262 and verify109707373449. Full decoded job logs were inspected.

Clippy with -D warnings, Cargo build, all38 Rust tests (including8 new editor_pack tests), TypeScript and all29 Node tests passed. The primary actual-CLI smoke passed with no model keys and an empty PATH. Existing legacy and Motion media regression scripts also passed, including both preview layouts and full FFmpeg decode; their scripts/templates/dependencies were not changed to obtain a pass.

Downloaded both actual artifacts. The primary pack contains57 files, including copy-ready narration/scripts/screen-text, human editing notes and timed/untimed reports. Inspection confirmed that untimed export has no SRT or fake timestamps. The optional-media artifact contains79 files; both Motion outputs include PREVIEW.txt and preview-report.json with purpose=structural_preview and publish_ready=false. The old motion alias still produced its compatible filenames. These checks inspect files/metadata, not aesthetic quality or speech listening.

Exact commands, checksums and limitations are in [the verification record](../../../verification/editor-materials-first-2026-09-30.md).

## Handoff and limits

The scoped code and automated verification are complete. PR #7 review/merge remains separate; main was not changed by this task. The evidence follow-up changes documentation only. EP-001 remains active under the existing RepoFoundry controller-unavailable bounded-record fallback; no Harness activation, accepted ADR, sealed checkpoint or formal archive is claimed.

Local Cargo was not run: this container has no usable Cargo and cannot resolve github.com. The authorized GitHub connector and actual Actions execution supplied code/test evidence. Do not mislabel that network/toolchain limit a GitHub permission failure.

This task did not regenerate or improve the user's previous artwork, produce a new final video, validate the user's Jianying UI, implement native editor tracks, or remediate optional-renderer dependencies. npm installation still reported5 high-severity findings; no security-audit pass. Layers are actual independent rasters, not editable editor text objects. Existing old binaries do not understand the new optional layers manifest. Synthetic images/voice test file and orchestration behavior, not publishing quality.
