# MindFrame development

Read README.md, ARCHITECTURE.md, docs/engineering/development-principles.md and the active ExecPlan
before implementation. Preserve the owner's minimal-complexity, fail-fast and single-validation-boundary rules.

Two crates: core owns contracts; CLI owns I/O. Remotion owns visual templates. Do not generate or execute
arbitrary React, shell or Python code from a model. Do not commit vault notes, API keys or generated media.

Run `python3 scripts/check.py`. The media acceptance command is
`python3 scripts/integration_test.py --out output/<new-test-directory>`.
Do not describe mock HTTP providers as live integration or an unrun check as passed.

Use RepoFoundry's professional execution workflow for durable progress. The initial EP was written
using the documented controller-unavailable template fallback. Harness bootstrap, Spec selection
and formal archive have not been performed. Do not infer ADR/Design approval from general development authority.
