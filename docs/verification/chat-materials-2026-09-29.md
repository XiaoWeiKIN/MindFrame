# Chat-material workflow verification — 2026-09-29

This is an ordinary verification record, not a sealed RepoFoundry Checkpoint or an archived ExecPlan.

## Verified revision and execution

- Application/test revision: `c04cb33b3f8782cf5dd2a488ae084cbdc102dcdc`.
- Workflow: [run 36569380103](https://github.com/XiaoWeiKIN/MindFrame/actions/runs/36569380103).
- Primary job: [chat-materials / 109409249398](https://github.com/XiaoWeiKIN/MindFrame/actions/runs/36569380103/job/109409249398).
- Actual job conclusion: **success**, completed `2026-09-29T12:42:45Z`.
- Environment recorded by the job: Ubuntu 24.04.5, Rust 1.98.1.
- Full decoded job logs and the completed step summaries were inspected, not inferred from workflow configuration.
- Follow-up commit `37d1155c767b33242280f728091d8df6d6b51146` changes only .gitignore and optional-media diagnostic output in CI. Application code and tests are unchanged from the verified revision. This record and the corresponding EP update are documentation-only changes.

## Actual results

| Executed check | Observed result |
|---|---|
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed; no warnings accepted |
| `cargo test --workspace` | 20 tests passed, 0 failed: 4 CLI boundaries, 8 CLI materials, 8 core tests |
| `cargo build --workspace` | Passed |
| `python3 scripts/chat_smoke.py --out output/chat-smoke` | Passed using the actual compiled CLI |
| Upload `mindframe-chat-materials` | Succeeded; 47 files in the retained fixture artifact |

Selected actual CLI test output:

```text
test existing_destinations_are_preserved ... ok
test audio_without_alignment_does_not_invent_cues ... ok
test full_untimed_import_export_without_keys_or_external_programs ... ok
test malformed_project_marker_does_not_fall_back_to_legacy_mode ... ok
test symlink_assets_are_not_followed ... ok
test supplied_timing_produces_exact_srt_and_rejects_stale_narration ... ok
test truncated_or_too_short_audio_fails_before_import ... ok
test invalid_inputs_leave_no_partial_content ... ok
Chat material CLI smoke: passed
```

The CLI tests clear the inherited environment and set PATH to an empty string for the primary commands.
The smoke script likewise starts the actual CLI with no keys and an empty PATH. It successfully runs
init, import, validate and export; checks original PNG bytes, CSV fields and non-overwrite behavior;
and exercises both untimed output and WAV plus explicit cue output.

The unit/CLI suite also covers PNG/JPEG/WebP decoding, explicit scene mapping, portable asset paths,
bad quotations, corrupted media, symlinks, absent timings, overlap, changed narration and recording bounds.
This is functional coverage of the stated cases, not a proof that all invalid input shapes are rejected.

## Retained fixture artifact

- Name: `mindframe-chat-materials`.
- Artifact ID: `11033741157`.
- [Download from the verified run](https://github.com/XiaoWeiKIN/MindFrame/actions/runs/36569380103/artifacts/11033741157).
- Archive size reported by upload: 22,228 bytes.
- Archive SHA-256 reported by upload: `fac2ccc82604c341908fd9276f3490a37cb2579dfd6804b98da49b41520e1b17`.
- Configured retention: seven days; the hosted artifact can expire.

The pack contains synthetic PNG and silent WAV test fixtures, actual exported material directories,
source fixtures, and verification.json. It is **not** GPT-generated artwork, natural speech, semantic
accuracy evidence or an editor UI test. The CLI export itself excludes the full source snapshot;
the CI artifact also retains test inputs for inspection.

## Separate remaining boundaries

- No actual Jianying UI/version was exercised. Output is standard media plus editing instructions,
  not a native draft, automatic timeline, transition application or platform publishing.
- Supplied cue checks establish ordering/text/recording bounds, not acoustic speech alignment.
- No live LLM/image/TTS calls were made. They are not required by the local primary workflow.
- The portable author Skill was added to the repository, not automatically installed into a chat host.
- No claim is made about selecting or verifying the chat image tool's underlying model.
- RepoFoundry controller/Harness validation, formal Spec activation and archival were not performed.
- Historical revision e3bfca7 passed compiler/static checks but failed optional-media acceptance at
  the H.264/yuv420p assertion. In run36569380103, the optional verify job had passed its static/build
  steps and was still running its real-render step when this record was prepared. Primary job success
  is not an assertion that the whole workflow or optional video path passed.

PR #1 remains draft and main is unchanged. EP-001 remains active. No completed-plan seal is fabricated.
