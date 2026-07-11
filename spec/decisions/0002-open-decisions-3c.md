# ADR 0002: Open Decisions Evaluation with 3C

## Status
Accepted (except CLI output default)

## Date
2026-04-27

## Principle
3C = Convenient for humans / Compatible with JSON / Clear to parsers

## Decision A: Duplicate Keys

Options:
- keep-last
- keep-first
- reject as error

Evaluation summary:
- keep-last: best balance across 3C in real-world config usage
- keep-first: lower convenience and weaker ecosystem expectation
- reject: highest parser clarity but weak compatibility with existing JSON corpora

Accepted outcome:
- language default = `keep-last`
- default behavior MUST emit warning diagnostics on overwrite
- tooling strict mode MAY upgrade warning to error

## Decision B: Bare Key Character Class

Options:
- strict class
- permissive class

Evaluation summary:
- strict class improves parser clarity and cross-language consistency
- permissive class improves convenience but increases tokenizer edge cases

Accepted outcome:
- bare key regex = `[A-Za-z0-9_.-]+`
- punctuation-only bare keys (`[.-]+`) are disallowed
- quoted keys remain the compatibility escape hatch

## Decision C: CLI Output Default

Options:
- pretty
- compact

Evaluation summary:
- pretty is more human-friendly and review-friendly
- compact is useful for transport/embedding and can be opt-in

Proposed outcome:
- default = pretty
- flag = `--compact`

## Finalization Gate
Accepted items (A, B) become Release-Ready after:
1. Conformance suite examples are added.
2. Rust core and at least one binding implement identical behavior.

Remaining open item:
- C (CLI default output) is intentionally deferred until implementation feedback is available.
