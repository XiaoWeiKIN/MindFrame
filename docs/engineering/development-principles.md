# Development Principles

These rules are adapted from the owner's existing coding guidelines and apply to MindFrame.

## 1. Build for the real workflow

Implement only capabilities required by the current knowledge-to-video path. Do not add compatibility layers, fallback chains, generic frameworks, or configuration for hypothetical future use.

## 2. Keep the common path direct

The high-frequency path should be linear and locally readable:

```text
read input
-> validate boundary conditions
-> execute current pipeline stage
-> write artifact
-> return
```

Rare supported behavior should be isolated. Unsupported behavior should fail fast.

## 3. Validate once, at the owning boundary

Validate external/untrusted data at the system boundary. Core types enforce domain invariants. Downstream code trusts established preconditions and does not repeat the same checks.

## 4. Prefer fast-fail over guessing

When a required input, format, executable, or invariant is missing, report the concrete failure close to its source. Do not silently repair input or try multiple uncertain fallbacks unless a real product requirement demands it.

## 5. Avoid speculative abstractions

Do not create traits, factories, adapters, helpers, generic frameworks, or configuration systems only because they may be useful later.

An abstraction must do at least one of the following:

- represent a stable domain concept;
- hide meaningful complexity;
- remove meaningful repetition;
- define a real external boundary with multiple implementations.

Small duplication is preferable to a wrong abstraction.

## 6. Keep orchestration readable

For multi-stage orchestration functions, keep the major stages visible from top to bottom. Use short numbered comments only when they clarify real stages; do not split simple linear work into trivial forwarding functions.

## 7. Error handling stays simple

Preserve the original cause. Add context only when it helps locate the failing operation. Avoid wrapping or reclassifying the same error at every layer.

## 8. Optimize measured hotspots

Do not introduce caches, concurrency layers, custom allocators, or algorithmic complexity without evidence from the actual workload.

## 9. Make the smallest sufficient change

A task should change only what is required to deliver and verify its behavior. Do not combine local delivery with unrelated architectural cleanup.

## 10. Pre-merge complexity check

Before accepting a change, ask:

- Does this code solve a current workflow need?
- Can unsupported input fail earlier?
- Is validation duplicated?
- Is there a one-line forwarding abstraction?
- Is a low-frequency case complicating the main path?
- Could code be deleted with no loss of required behavior?

If code has no concrete required behavior behind it, remove it.
