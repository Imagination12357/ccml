# Node Phase-1 Parity Checklist (Week 3 Day 1 Draft)

## 1. Goal
Define parity targets between Python thin adapter and upcoming Node adapter before feature implementation.

## 2. Contract Parity Targets
1. Shared FFI entrypoints:
- `ccml_to_json`
- `ccml_diagnose`
- `ccml_free`

2. Shared status semantics:
- `0` success
- non-zero mapped to runtime exception/error with status visibility

3. Shared memory contract:
- every returned non-null pointer must be released via `ccml_free`
- null-safe free behavior remains valid

4. Shared dynamic library resolution policy:
- env override first: `CCML_FFI_LIB`
- fallback search in `core/rust/target/{debug,release}`

## 3. Smoke Parity Targets (Week 3)
1. Success path:
- `to_json('name: "Alice"\\nage: 30')` returns stable JSON payload.

2. Error path:
- malformed input raises typed runtime error with parse status and message.

3. Warn path:
- duplicate key warning (`CCML2001`) observable in `diagnose` output.

## 4. Exclusions for Day 1
- No full symbol binding/wrapper implementation yet.
- No npm publish/package hardening.
- No API ergonomics beyond minimal phase-1 contract.
