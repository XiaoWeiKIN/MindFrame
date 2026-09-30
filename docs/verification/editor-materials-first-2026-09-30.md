# Editor-materials-first verification — 2026-09-30

Verified implementation: `4886e46e3453de4a7ee40c55651526b23c8680ac`.
Base main: `50994f30e77938c22038dd7f9a89fd8e0c75fe22`.
PR: #7, `refactor/editor-materials-first`.
Actions run: `36658340104`.
Actual checkout: PR test merge `134a89275b593ee8b6b8412cea30331e27ab0313`.

## Actual execution

Both jobs completed successfully; their full decoded logs were inspected:

| Job | ID | Result |
|---|---|---|
| chat-materials | 109707373262 | success |
| verify | 109707373449 | success |

Commands executed by the retained workflow:

```text
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
python3 scripts/chat_smoke.py --out output/chat-smoke
python3 scripts/check.py
cargo run -p mindframe-cli -- schema --out output/schemas
python3 scripts/integration_test.py --out output/integration
python3 scripts/motion_integration_test.py --out output/motion
```

`check.py` ran Rust checks, `tsc --noEmit` and Node tests. Results: **38 Rust tests passed, 0 failed; 29 Node tests passed, 0 failed; Clippy and TypeScript passed**. Rust suites:4 boundary +8 editor_pack +3 lecture +8 materials +1 motion_primitives +4 visual_director +10 core.

All eight new tests passed:

- layered_pack_has_clean_scripts_real_layers_and_semantic_editing_notes_without_audio
- old_minimum_pack_stays_usable_and_new_exports_follow_current_json
- invalid_layer_contracts_and_files_fail_without_partial_publication
- layer_counts_and_unknown_manifest_fields_are_strict
- layer_symlinks_and_malformed_optional_manifest_are_not_silently_ignored
- schema_and_help_keep_editor_pack_primary_and_preview_optional
- preview_does_not_silently_drop_editor_layers_or_generate_timing
- supplied_cues_appear_in_notes_but_remain_separate_from_the_plan

The new CLI tests clear environment variables and PATH. They use actual synthetic PNG/WebP with transparent pixels, opaque JPEG, current JSON and optional WAV/cues. They assert byte preservation, normalized overlays, pure narration, correct per-scene text, semantic add/keep/update/remove instructions, literal formula/matrix/relation preservation, invalid-layer failures, stale-derivative handling and non-overwrite. No model call or media executable is needed for that primary path.

## Retained preview compatibility

The unchanged legacy script rendered Bilibili480×270 and Douyin270×480 previews as H.264/AAC, 30.167s, with full decode. The unchanged Motion script passed all five primitive types, actual local eSpeak fixture audio and known sentence boundaries, both preview layouts and full decode. It exercised the compatible `motion` alias after orchestration was moved to preview.rs.

No new video-rendering capability is claimed. The actual Motion artifact's verification.json reports24.348s and both layout checks passed. This is regression protection for the optional code, not a publishing-quality benchmark.

## Actual artifact inspection

Both archives were downloaded and read in the working container, independently checking ZIP hashes and emitted text/metadata:

| Artifact | ID | ZIP bytes | Entries | SHA-256 |
|---|---|---|---|---|
| mindframe-chat-materials | 11072599040 | 46,559 | 57 | cefbc780a74624699589cca2f3ea1fa82568b39f2752d57cd165936ed8ac78e3 |
| mindframe-v1-verification | 11073680513 | 10,001,466 | 79 | 6cb19d9b67f5d9154576ca354b9f0994f1ffe5bff022657546a3d0ebd4ef7c92 |

The primary artifact's untimed export contains narration.txt, scripts/, screen-text/, edit-guide.md, edit-notes/ and material-report.json. Narration contains only its spoken fixture sentence; the guide explicitly says timing is absent. No SRT is present for that untimed export. The timed fixture has its supplied-cue SRT. The report preserves artwork_quality_verified=false, editor_ui_verified=false and final_video_included=false. New layered round-trip assertions ran in temporary test directories; the primary artifact is the retained minimal smoke fixture, not those layered test directories.

Both optional Motion layout directories contain PREVIEW.txt and preview-report.json with purpose=structural_preview, publish_ready=false and media=motion.mp4. Existing output filenames remain present. The archive also includes the generated strict Layers schema.

Artifacts may expire under GitHub's retention policy. File/metadata inspection is not visual playback, speech listening or editor UI acceptance.

## Limits

- Local Rust execution was not performed; the container lacked usable Cargo and GitHub DNS. Actual compiler/runtime evidence is from Actions.
- No user knowledge-base text, generated media, fonts or credentials were committed.
- The change does not improve previous artwork or make rasters into editable editor text objects.
- No Jianying UI/version acceptance, native project output, automatic editor tracks, forced alignment, new TTS or publishing automation.
- Dimension/aspect/transparency/cover warnings are structural review prompts, not artistic certification or verified platform requirements.
- The unchanged optional-renderer dependency installation still reported **5 high-severity npm findings**. Detailed audit/remediation was not performed.
- No formal RepoFoundry Harness/controller validation, sealed checkpoint or EP archive. The evidence follow-up is documentation only and does not change the verified code.
