# EP-001 bounded work record: visual director

Date: 2026-09-29. Parent: [EXECPLAN.md](EXECPLAN.md).
This is an ordinary task record, not a newly numbered Task, sealed Checkpoint, accepted ADR or archived plan. The existing controller-unavailable workflow remains explicit.

## Intent and scope

The owner requested a reusable mindframe-visual-director Skill based on the inspected article-illustrator, infographic and image-director patterns. Preserve already-dense source material, especially named concepts, quotations and formulas. Produce individual mobile-readable pages rather than a contact sheet. Retain the chat-first / no-model-key / local-material boundaries.

Base: main at 0df0319770adafb5b2c7f0b628ae6f93a0ae94d4 (PR #1 already merged). Working branch: feat/visual-director-skill. This task does not retroactively seal EP-001, modify old verification evidence or merge a new PR automatically.

## Changes

- Standalone skills/mindframe-visual-director/SKILL.md, a page worksheet, provenance notes and an original synthetic three-page example.
- Author Skill and prompt route through the director; distinguish finished carousel pages from editing backgrounds.
- materials::init embeds the canonical director and worksheet in chat-request.md before the existing generated Schemas. No runtime repository lookup, new dependencies, model call, output-format migration or sidecar import.
- Add four Rust tests for embedded guidance, exact source/formula fixtures, real CLI import/export with synthetic images, and portable files/guardrail presence.
- Usage guide at docs/visual-director.md; AGENTS and README make the new workflow discoverable.

No original vault notes or generated user media are added to the repository. Upstream prose/code is not vendored; source notes distinguish ideas consulted from dependencies and document the observed licenses.

## Acceptance declared before CI — observed results

- [x] Implementation f0f9f7c passes Clippy and all 24 Rust tests, including four visual_director tests, through PR test merge df94112.
- [x] Actual compiled CLI initializes outside the repository with empty PATH and no keys, embedding full canonical text and unchanged source.
- [x] Examples validate against existing Storyboard/Assets types; unsupported added metadata is rejected.
- [x] Synthetic three-page material import/export preserves actual image bytes and excludes source/working notes; no audio means no SRT.
- [x] Existing chat-material smoke, TypeScript checks, 12 Node tests and optional real-media gates passed in run36575979698.
- [x] Local preparation inspected Markdown reference targets and literal example source references; this is separate from Rust and image execution evidence.

## Current handoff

Implementation: f0f9f7c84e5e3f90f7be3bc34996828f536309bc; PR #3. Actual CI run36575979698 has both jobs completed successfully. Full verify-job logs and primary smoke-step results were inspected. The exact merge revision, commands, results, artifacts and limits are in [the verification record](../../../verification/visual-director-2026-09-29.md).

The scoped implementation and functional verification are complete; PR review/merge and a real article's image-generation/visual review are separate actions. Main is unchanged by this work. Parent EP-001 remains active; this does not fabricate a formal completed-plan seal.

A pre-existing optional-renderer dependency warning remains: npm install reported 5 high-severity findings. Detailed audit/remediation was not run and no security-audit pass is claimed.

## Compatibility and review limits

Only init's human-readable request gains content. Project, Storyboard, Assets and Timeline remain version 1. Existing output directories are not overwritten and old generated requests are not silently upgraded. P0/P1/P2, must_keep and review status live in Markdown working notes, not unsupported import fields.

The Skill's art/content checks are instructions for a human/agent looking at actual images, not a new automatic visual validator. No images are generated as part of implementing this Skill. Synthetic tiny image fixtures prove file behavior, not image quality. Underlying image-model selection, editing capabilities and after-generation response behavior follow the active host contract.

No new durable architecture choice or external research gate is required: this is a bounded implementation of the owner's clarified authoring behavior on the existing file contract. No formal RepoFoundry controller validation, Spec activation or lifecycle transition is claimed.
