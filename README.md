# CCML

CCML is a human-friendly configuration language that maps cleanly to JSON.

Project principles:
1. Convenient for humans
2. Compatible with JSON
3. Clear to parsers

JSON compatibility is strict: every CCML document must map to one valid JSON value, and features that cannot map cleanly to JSON are out of scope for 1.0.

## Quick Start
Use this section if you want to try CCML first. The design background starts at [Why CCML?](#why-ccml).

### 1. Try CCML Syntax
Create a small CCML document:

```ccml
# comments are allowed
name: "Ada"
enabled: true
limits: {
  retries: 3
  timeout_ms: 1000
}
tags: ["alpha", "beta"]
```

It transcodes to one JSON value:

```json
{"enabled":true,"limits":{"retries":3,"timeout_ms":1000},"name":"Ada","tags":["alpha","beta"]}
```

### 2. Use the CLI
Install the published CLI from crates.io:

```powershell
cargo install ccml-cli
'answer: 42' | ccml-cli
```

From a source checkout:

```powershell
Set-Location core\rust
'answer: 42' | cargo run --offline -p ccml-cli
```

Output:

```json
{
  "answer": 42
}
```

Run the shared conformance vectors from source:

```powershell
Set-Location core\rust
cargo run --offline -p ccml-cli -- conformance ..\..\tests\conformance
```

### 3. Use Rust
Use the published Rust crate from crates.io:

```toml
[dependencies]
ccml-core = "1.0.0"
```

Inside this repository's Rust workspace:

```toml
[dependencies]
ccml-core = { path = "core/rust/crates/ccml-core" }
```

Example:

```rust
use ccml_core::{to_json, ToJsonOptions};

fn main() -> Result<(), ccml_core::CcmlError> {
    let input = "name: \"Ada\"\nenabled: true";
    let json = to_json(input, &ToJsonOptions { pretty: false })?;
    assert_eq!(json, "{\"enabled\":true,\"name\":\"Ada\"}");
    Ok(())
}
```

### 4. Use Python
Install the published Python package from PyPI:

```powershell
pip install ccml-py
python -c "from ccml_py import to_json; print(to_json('answer: 42'))"
```

From a source checkout:

```powershell
Set-Location core\rust
cargo build --offline -p ccml-ffi
Set-Location ..\..\bindings\python
$env:UV_CACHE_DIR = ".uv-cache"
uv run python -c "from ccml_py import to_json; print(to_json('answer: 42'))"
```

`ccml-py` loads the bundled prebuilt `ccml-ffi` library when the installed wheel includes one for your platform. If you are using an unsupported platform or a custom native build, set `CCML_FFI_LIB` to the built library path.

### 5. Use Node.js
Install the published Node package from npm:

```powershell
npm install ccml-node
node --input-type=module -e "const { createCcmlFfi } = await import('ccml-node'); const ffi = await createCcmlFfi(); console.log(ffi.toJson('answer: 42'));"
```

From a source checkout:

```powershell
Set-Location core\rust
cargo build --offline -p ccml-ffi
Set-Location ..\..\bindings\node
npm install
node --input-type=module -e "const { createCcmlFfi } = await import('./src/index.js'); const ffi = await createCcmlFfi(); console.log(ffi.toJson('answer: 42'));"
```

`ccml-node` loads the bundled prebuilt `ccml-ffi` library when the installed package includes one for your platform. If you are using an unsupported platform or a custom native build, set `CCML_FFI_LIB` to the built library path.

### 6. Use the C ABI
The C ABI is the stable boundary used by Python and Node bindings. See [spec/interfaces/ffi-phase1-interface.md](spec/interfaces/ffi-phase1-interface.md) for function signatures, status codes, memory ownership, and error payload rules.

## Why CCML?
CCML exists for configuration files that should be easier to write than JSON without becoming a separate data model.

Its design is governed by 3C, also documented in [CONTRIBUTING.md](CONTRIBUTING.md) and [PROJECT_CHARTER.md](PROJECT_CHARTER.md):
1. Convenient for humans: less punctuation, comments, and readable key/value files.
2. Compatible with JSON: every valid CCML document maps to one valid JSON value.
3. Clear to parsers: syntax choices must stay deterministic and easy to diagnose.

The practical advantages are:
1. You can write common config shapes with less visual noise than JSON.
2. You can keep JSON as the interchange, storage, and tooling boundary.
3. You can reject ambiguous convenience features before they become parser edge cases.
4. You can use one semantic core across CLI, FFI, Python, Node, and WASM bindings.

See [spec/decisions/0002-open-decisions-3c.md](spec/decisions/0002-open-decisions-3c.md) for examples of how syntax decisions are evaluated against 3C.

## Difference With JSON
CCML keeps the JSON data model, but changes the authoring syntax for configuration use.

| Dimension | JSON | CCML |
| --- | --- | --- |
| Primary role | Universal data interchange format | Human-authored configuration syntax that transcodes to JSON |
| Data model | Objects, arrays, strings, numbers, booleans, null | Same JSON data model only |
| Top-level object config | Requires `{}` | Allows implicit top-level object for key/value documents |
| Comments | Not allowed | `#` line comments are allowed |
| Separators | Commas required | Whitespace is the default separator; commas remain available when the author wants explicit intent or visual grouping |
| Keys | Quoted strings required | Bare keys allowed when parser-safe; quoted keys remain available |
| Compatibility boundary | Already JSON | Must transcode to valid JSON |
| Parser goal | Standardized strict format | Convenient syntax with explicit diagnostics and JSON-compatible output |

The detailed syntax contract is in [spec/ccml-1.0-draft.md](spec/ccml-1.0-draft.md), and shared expected behavior is captured in [tests/conformance](tests/conformance).

## Difference With Other Languages
This comparison is about design goals, not a complete feature matrix.

| Language | Main orientation | Difference from CCML |
| --- | --- | --- |
| HJSON | Human-friendly JSON-like documents | HJSON looks syntactically similar, but it is primarily a relaxed authoring format. CCML is stricter about the [3C boundary](spec/decisions/0002-open-decisions-3c.md): convenience is allowed only when the result remains JSON-compatible and parser-clear. |
| YAML | Broad human-readable serialization | YAML is feature-rich and can encode many shapes and conventions beyond JSON-style config. CCML intentionally avoids a broad type system and keeps JSON as the semantic boundary. |
| TOML | Typed application configuration | TOML has its own table-oriented structure and type conventions. CCML keeps object/array structure close to JSON so transcoding remains direct. |
| JSON5 | JavaScript-friendly JSON extension | JSON5 relaxes JSON syntax using JavaScript-like affordances. CCML instead targets config files with parser clarity and a strict JSON transcode contract. |
| INI | Simple section/key configuration | INI is small but underspecified across implementations. CCML aims for machine-readable conformance and nested JSON-compatible data. |

The closest-looking alternative is HJSON. The important distinction is intent: CCML is not "JSON with every convenient relaxation." It is a 3C-constrained configuration language where each syntax feature must justify itself across human convenience, JSON compatibility, and parser clarity. The current release scope and exclusions are tracked in [spec/ccml-1.0-draft.md](spec/ccml-1.0-draft.md) and [docs/release-go-no-go-week5.md](docs/release-go-no-go-week5.md).

## Status
This repository is on the `v1.0.0` release track for the Rust-core rebuild.

Rust, Python, and Node packages have been published at `1.0.0`. Tagging and GitHub Release publication remain as the final release-record steps.

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
3. Whitespace is the default separator; commas are optional when the author wants explicit intent or visual grouping.
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
spec/                   CCML language spec, interface contracts, and decisions
```

Architecture overview: [docs/architecture.md](docs/architecture.md).
FFI interface contract: [spec/interfaces/ffi-phase1-interface.md](spec/interfaces/ffi-phase1-interface.md).

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
Release documents:
1. [docs/pre-release-checklist-week5.md](docs/pre-release-checklist-week5.md)
2. [docs/release-go-no-go-week5.md](docs/release-go-no-go-week5.md)
3. [docs/release-notes-v1.0.0-draft.md](docs/release-notes-v1.0.0-draft.md)
4. [docs/release-runbook-v1.0.0.md](docs/release-runbook-v1.0.0.md)

Current release state: Rust/Python/Node package publishing is complete for `v1.0.0`; tag and GitHub Release publication are still pending.

## Contributing
Contributions must preserve the project principles in [CONTRIBUTING.md](CONTRIBUTING.md). JSON compatibility is non-negotiable.
