# Cross-Platform Validation Playbook (Week 5 Day 3)

## Goal
Provide one reproducible validation path per platform for core release-readiness checks, with explicit fallbacks when shell/tooling differs.

## Scope
1. Core/FFI/conformance checks
2. Python/Node parity and Node regression checks
3. Version consistency checks
4. WASM smoke execution path guidance

## Shared Preconditions
1. Repository root is current working directory.
2. Rust toolchain is installed and `core/rust/target` is writable.
3. Node and `bindings/node/node_modules` (including `koffi`) are available.
4. `uv` is installed for Python smoke.

## Canonical Commands by Platform

### Windows (PowerShell)
1. Core tests:
`Set-Location core\\rust; cargo test --offline -p ccml-core`
2. FFI tests:
`Set-Location core\\rust; cargo test --offline -p ccml-ffi`
3. Conformance:
`Set-Location core\\rust; cargo run --offline -p ccml-cli -- conformance ..\\..\\tests\\conformance`
4. Python/Node parity smoke:
`powershell -ExecutionPolicy Bypass -File tests\\bindings\\run-parity-smoke.ps1`
5. Node regression:
`node bindings\\node\\tests\\day1-regression.mjs`
6. Version consistency (Windows fallback):
`powershell -ExecutionPolicy Bypass -File scripts\\ci\\check-version-consistency.ps1`
7. Python smoke direct:
`Set-Location bindings\\python; $env:UV_CACHE_DIR='.uv-cache'; uv run python tests\\smoke.py`

### Linux/macOS (bash)
1. Core tests:
`cd core/rust && cargo test --offline -p ccml-core`
2. FFI tests:
`cd core/rust && cargo test --offline -p ccml-ffi`
3. Conformance:
`cd core/rust && cargo run --offline -p ccml-cli -- conformance ../../tests/conformance`
4. Python smoke:
`cd bindings/python && UV_CACHE_DIR=.uv-cache uv run python tests/smoke.py`
5. Node parity smoke:
`node bindings/node/tests/day2-smoke.mjs`
6. Node regression:
`node bindings/node/tests/day1-regression.mjs`
7. Version consistency:
`bash scripts/ci/check-version-consistency.sh`
8. WASM smoke:
`bash scripts/wasm/build-and-run-smoke.sh`

## Fallback Rules
1. If `bash` is unavailable on Windows:
- use PowerShell checks for core/ffi/conformance/parity/regression/version.
- treat WASM smoke as CI-only local gap and track it explicitly in review summaries.
2. If Python smoke fails with uv cache path issues:
- enforce repo-local cache: `UV_CACHE_DIR=.uv-cache`.
3. If dynamic library load fails in Python/Node:
- build FFI first: `cargo build --offline -p ccml-ffi` from `core/rust`.
- optionally set `CCML_FFI_LIB` to an explicit library path.

## Minimum Day-End Evidence (Day 3+)
1. One successful parity run (`run-parity-smoke.ps1` on Windows or equivalent on bash platforms).
2. One successful direct Python smoke run with repo-local uv cache.
3. One successful version consistency run (`.sh` on bash platforms or `.ps1` on Windows).
4. Explicit note when WASM local execution is unavailable.
