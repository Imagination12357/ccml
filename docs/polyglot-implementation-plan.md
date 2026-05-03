# CCML Polyglot Implementation Plan

## 1. Goal
Ship CCML 1.0 with one Rust core and multiple language bindings without behavior drift.

## 2. Delivery Phases

### Phase A: Spec and Contract Freeze
- Freeze grammar and diagnostic schema
- Define machine-readable test vector format
- Exit gate:
  - draft spec updated
  - conformance format approved

### Phase B: Rust Core Foundation
- Implement `ccml-core`:
  - lexer/parser/AST/diagnostics
  - `parse`, `to_json`, `diagnose`
- Implement `ccml-cli` binary
- Exit gate:
  - core unit tests pass
  - CLI smoke tests pass

### Phase C: Shared Conformance Suite
- Build language-neutral test vectors:
  - valid parse cases
  - invalid diagnostic cases
  - JSON output golden cases
- Exit gate:
  - Rust core passes 100%

### Phase D: Python Binding
- Build Python package over `ccml-ffi`
- Preserve API shape close to current MVP expectations
- Exit gate:
  - Python binding passes shared conformance suite

### Phase E: Node Binding + WASM
- Build Node native binding and WASM package
- Add browser-focused entry point from WASM artifact
- Exit gate:
  - Node binding passes shared conformance suite
  - WASM smoke tests pass

### Phase F: Release Pipeline
- CI matrix:
  - Rust core tests
  - Python binding tests
  - Node binding tests
  - conformance cross-check
- Publish:
  - crates.io
  - PyPI
  - npm
- Exit gate:
  - release checklist complete

## 3. Repository Ownership Model
- `core/rust/**`: core team ownership
- `bindings/python/**`: python binding owner
- `bindings/node/**`: node binding owner
- `tests/conformance/**`: spec + qa joint ownership

## 4. Risk and Mitigation
1. FFI instability
- Mitigation: narrow C-ABI and version it

2. Runtime behavior drift
- Mitigation: mandatory shared conformance gating

3. Packaging complexity
- Mitigation: staged release automation and per-target smoke checks

## 5. Immediate Next Steps
1. Define conformance case file schema
2. Scaffold Rust workspace (`ccml-core`, `ccml-cli`, `ccml-ffi`, `ccml-wasm`)
3. Migrate MVP tests into conformance vectors

## 6. Week 2 Implementation Plan (FFI and Binding Kickoff)

### 6.1 Scope
- Implement `ccml-ffi` phase-1 surface for Python/Node binding kickoff.
- Keep `ccml-core` as single behavior source of truth.
- Validate FFI behavior against existing conformance assets.

### 6.2 FFI Contract (Phase 1)
- ABI style: JSON-only C API (no AST handle API in Week 2).
- Memory policy: Rust allocates output strings, caller releases via `ccml_free`.
- Error policy: return code + error JSON out-parameter.
- Diagnostic payload: JSON array/object with `code`, `message`, `line`, `column`, `severity`.

### 6.3 Planned FFI Functions
- `ccml_to_json(...)` for transcode path.
- `ccml_diagnose(...)` for diagnostics path.
- `ccml_version(...)` for runtime contract/version check.
- `ccml_free(...)` for buffer release.

### 6.4 Week 2 Validation Targets
- FFI contract unit tests:
  - null/invalid argument handling
  - UTF-8 validation
  - memory release contract
- Behavior parity tests:
  - representative `valid/invalid/warn` vectors through FFI path
  - duplicate key warning (`CCML2001`) propagation check
- Smoke adapters:
  - minimal Python and Node binding calls using identical fixtures

### 6.5 Output Artifacts
- `ccml-ffi` function-level contract document.
- binding TODO matrix for Python/Node.
- Week 2 validation report with pass/fail summary.
