# CCML 1.0 Architecture (Design Freeze)

## 1. Direction
CCML 1.0 adopts `Single Core + Bindings`.

- Core language: Rust
- First bindings: Python, Node.js
- Delivery targets: native libraries, CLI, and WASM package

The architecture goal is one semantic source of truth across ecosystems.

## 2. Service Structure
Program -> Services -> Feature Groups -> Files

### 2.1 spec_service
- Responsibility:
  - Own language spec, error code catalog, conformance contract
- Output:
  - `spec/ccml-1.0-draft.md`
  - decision records under `spec/decisions/`
- Dependency:
  - none

### 2.2 core_rust_service
- Responsibility:
  - Lexer, parser, AST, diagnostics, JSON transcoding
- Output:
  - `ccml-core` crate
  - stable C-ABI boundary for bindings
- Dependency:
  - spec_service

### 2.3 bindings_service
- Responsibility:
  - Runtime integration for language ecosystems
- Feature groups:
  - Python binding
  - Node binding
  - WASM wrapper
- Dependency:
  - core_rust_service only through public interface

### 2.4 tooling_service
- Responsibility:
  - CLI UX and developer tooling behavior
- Output:
  - `ccml` CLI binary
  - formatter/linter entry points (phase-gated)
- Dependency:
  - core_rust_service

### 2.5 qa_service
- Responsibility:
  - Shared test vectors and cross-language compliance
- Output:
  - conformance/property/regression suites
  - compatibility reports per implementation
- Dependency:
  - spec + all deliverables

### 2.6 release_service
- Responsibility:
  - Build, version, publish, signing, release notes
- Output:
  - crates.io / PyPI / npm releases
  - tagged artifacts and changelog
- Dependency:
  - qa_service gates

## 3. Dependency Rules
- Allowed direction:
  - `spec -> core -> bindings/tooling -> release`
- Disallowed:
  - binding-specific logic inside core
  - parser internals imported directly by bindings
  - divergent syntax rules per runtime

## 4. Interface Contracts

### 4.1 Core Public API Contract
- `parse(text) -> ast`
- `to_json(text, options) -> json_string`
- `diagnose(text) -> [diagnostic]`

### 4.2 Diagnostic Contract
Every parsing/semantic diagnostic must expose:
- `code`
- `message`
- `line`
- `column`

### 4.3 Conformance Contract
All bindings must pass identical test vectors from `tests/conformance`.

## 5. Proposed Repository Layout
```text
ccml/
  PROJECT_CHARTER.md
  spec/
    ccml-1.0-draft.md
    decisions/
      0001-core-rust-bindings.md
  docs/
    architecture.md
    polyglot-implementation-plan.md
  core/
    rust/
      Cargo.toml
      crates/
        ccml-core/
        ccml-cli/
        ccml-ffi/
        ccml-wasm/
  bindings/
    python/
    node/
  tests/
    conformance/
      valid/
      invalid/
    regression/
    property/
```

## 6. Non-Negotiable Constraints
- One file -> one responsibility
- Explicit data flow only
- No hidden runtime dependency
- No public behavior without documentation and tests

## 7. Crate Ownership Table (Week 1 Freeze)
1. `core/rust/crates/ccml-core`
- Responsibility: parser/AST/diagnostics/transcoding source of truth
- Exposes: `parse`, `to_json`, `diagnose`
- Must not depend on: binding-specific runtime logic

2. `core/rust/crates/ccml-cli`
- Responsibility: command-line entry point and UX
- Depends on: `ccml-core`
- Must not implement: grammar or semantic rules on its own

3. `core/rust/crates/ccml-ffi`
- Responsibility: stable FFI surface for non-Rust bindings
- Depends on: `ccml-core`
- Must keep: narrow and versionable ABI

4. `core/rust/crates/ccml-wasm`
- Responsibility: WASM bridge for browser/JS runtime usage
- Depends on: `ccml-core`
- Must not diverge: behavior from core conformance rules
