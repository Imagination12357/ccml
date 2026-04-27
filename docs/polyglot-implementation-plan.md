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
