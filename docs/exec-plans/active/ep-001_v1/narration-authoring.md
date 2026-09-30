# EP-001 bounded work: narration authoring and review

Date: 2026-09-30. Parent: [EXECPLAN.md](EXECPLAN.md).

## Scope

The owner requests that the repeated narration corrections become reusable MindFrame skill and product behavior.
Knowledge explainers should stand alone for viewers, preserve precise source judgments, and build understanding
through examples, mechanisms and explanations throughout the script. User-requested readings or reviews remain supported.

Add canonical narration guidance, embedded by init, and a derived narration-review.md showing scene provenance,
actual indexed speech and explicitly marked visual emphasis. Keep v1 schemas and existing plain TTS text unchanged.
No automatic semantic scores, TTS, renderer expansion, native editor output or publishing.

Core owns the pure review formatter; CLI owns file emission. No new dependency or governed design is needed.
The execution controller is available in this session: status reports EP-001 active. Historical verification and
controller-unavailable notes describe prior sessions and remain unchanged; this work does not complete the parent EP.

## Acceptance

- init embeds the complete canonical narration reference, with no repository-relative read required by the recipient.
- Every scene is represented in the review, including scenes without a motion plan; source refs remain scene-specific.
- Review text comes from current JSON. It cannot pollute narration.txt, scripts, captions or timings.
- Only explicit visual emphasis is shown, at actual trigger indices; it is not inferred vocal delivery or semantic approval.
- Run repository check.py, chat_smoke.py, targeted CLI regressions and skill validation; record actual outcomes below.

## Verification

Completed locally on 2026-09-30, against the uncommitted working tree:

- `python3 scripts/check.py` passed: Clippy with warnings denied, all 39 Rust tests, TypeScript check and 29 renderer contract tests. Node 24.19.0 from the bundled runtime was used for the final run; log: ignored `output/narration-authoring-check.log`.
- `python3 scripts/chat_smoke.py --out output/narration-authoring-smoke` passed with empty PATH and no API keys, including timed and untimed synthetic fixtures. This is contract verification, not real voice or artwork acceptance.
- Both author and visual-director skills passed the skill-creator `quick_validate.py` check. The CLI init test verifies byte-for-byte inclusion of the canonical narration reference.
- Regression cases verify scene-specific provenance, explicit stat/text/formula emphasis at the correct utterance, cleared states, projects without motion, updated JSON rather than stale derivative files, and unchanged plain speech/caption text.
- `git diff --check` passed. No schema changes, new dependencies or renderer changes.
- `cargo install --path crates/mindframe-cli --force` succeeded; the installed local executable includes the new guidance and review export.
- Applied installed export to the user's existing 10-scene, 110-utterance project outside this repository. Every scene and utterance appears in the review; narration.txt is byte-identical to the previous pack and no SRT was invented. Private source/media remain outside the repository.

Environment corrections: the first check lacked Clippy; it was installed with rustup. The first npm-ci attempt found no committed lockfile; dependencies were installed with npm install --no-package-lock. An intermediate check reached TypeScript before that installation finished. The final full check passed after dependencies were available. These attempts did not change acceptance criteria.

No semantic-quality score, speech listening, editor UI verification, optional full-media rendering or publishing is claimed. Existing full-media regression gates were retained unchanged. No commit, push or video export was performed. EP-001 remains active; this bounded improvement does not resolve or rewrite the parent's historical blockers.
