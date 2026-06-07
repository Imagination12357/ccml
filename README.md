# CCML

CCML is a human-friendly configuration language that maps cleanly to JSON.

Project principles:
1. Convenient to humans
2. Compatible with JSON
3. Clear to parsers

JSON compatibility is strict: every CCML document must map to one valid JSON value, and features that cannot map cleanly to JSON are out of scope for 1.0.

## Status
This repository is in Week 5 release-preparation for the Rust-core rebuild.

Current package manifests are aligned at `1.0.0`. The planned release target is `v1.0.0`, and release execution is gated by `docs/release-go-no-go-week5.md`.

## What CCML Adds
CCML keeps the JSON data model while adding a smaller config-oriented syntax:

```ccml
# comments are allowed
name: "example"
enabled: true
limits: {
  retries: 3
  timeout_ms: 1000
}
tags: ["alpha", "beta"]
```

Transcoded JSON:

```json
{"enabled":true,"limits":{"retries":3,"timeout_ms":1000},"name":"example","tags":["alpha","beta"]}
```

## Core Semantics
1. Top-level object braces are optional for key/value documents.
2. Comments start with `#` and continue to end of line.
3. Commas are optional separators where whitespace is unambiguous.
4. Bare keys match `[A-Za-z0-9_.-]+`, except punctuation-only keys such as `.` or `--`.
5. Quoted keys are required for spaces or reserved separators.
6. Duplicate object keys use keep-last semantics and emit warning `CCML2001`.
7. Parser diagnostics include `code`, `message`, `line`, and `column`.

See [spec/ccml-1.0-draft.md](spec/ccml-1.0-draft.md) for the draft specification.

## Repository Layout
```text
core/rust/              Rust workspace
  crates/ccml-core/     parser, AST, diagnostics, JSON transcode
  crates/ccml-cli/      CLI and conformance runner
  crates/ccml-ffi/      C ABI for bindings
  crates/ccml-wasm/     WASM bridge
bindings/python/        Python thin adapter over ccml-ffi
bindings/node/          Node thin adapter over ccml-ffi
tests/conformance/      shared machine-readable conformance vectors
tests/bindings/         cross-binding parity fixtures
tests/wasm/             WASM runtime smoke
docs/                   architecture, plans, release-readiness notes
spec/                   CCML draft spec and decisions
```

Architecture overview: [docs/architecture.md](docs/architecture.md).

## CLI Usage
Build the CLI:

```powershell
Set-Location core\rust
cargo build --offline -p ccml-cli
```

Transcode CCML from stdin:

```powershell
'a: 1' | cargo run --offline -p ccml-cli
```

Run conformance vectors:

```powershell
Set-Location core\rust
cargo run --offline -p ccml-cli -- conformance ..\..\tests\conformance
```

## Validation
Windows PowerShell baseline:

```powershell
Set-Location core\rust
cargo test --offline -p ccml-core
cargo test --offline -p ccml-ffi
cargo run --offline -p ccml-cli -- conformance ..\..\tests\conformance
Set-Location ..\..
powershell -ExecutionPolicy Bypass -File tests\bindings\run-parity-smoke.ps1
node bindings\node\tests\day1-regression.mjs
powershell -ExecutionPolicy Bypass -File scripts\ci\check-version-consistency.ps1
```

Linux/macOS bash baseline:

```bash
cd core/rust
cargo test --offline -p ccml-core
cargo test --offline -p ccml-ffi
cargo run --offline -p ccml-cli -- conformance ../../tests/conformance
cd ../..
bash scripts/ci/check-version-consistency.sh
bash scripts/wasm/build-and-run-smoke.sh
```

Cross-platform details: [docs/cross-platform-validation-playbook.md](docs/cross-platform-validation-playbook.md).

## Bindings
Python:
1. Build `ccml-ffi` first when needed.
2. Run smoke from `bindings/python` with repo-local uv cache:

```powershell
Set-Location bindings\python
$env:UV_CACHE_DIR = ".uv-cache"
uv run python tests\smoke.py
```

Node:
1. Install dependencies in `bindings/node` when needed.
2. Run parity/regression smoke:

```powershell
node bindings\node\tests\day2-smoke.mjs
node bindings\node\tests\day1-regression.mjs
```

## Release Readiness
Release-prep documents:
1. [docs/pre-release-checklist-week5.md](docs/pre-release-checklist-week5.md)
2. [docs/release-go-no-go-week5.md](docs/release-go-no-go-week5.md)
3. [docs/release-notes-v1.0.0-draft.md](docs/release-notes-v1.0.0-draft.md)
4. [docs/release-runbook-v1.0.0.md](docs/release-runbook-v1.0.0.md)

Current release state is Go-ready according to `docs/release-go-no-go-week5.md`; publish/tag execution still requires an explicit operator action.

## Contributing
Contributions must preserve the project principles in [CONTRIBUTING.md](CONTRIBUTING.md). JSON compatibility is non-negotiable.
