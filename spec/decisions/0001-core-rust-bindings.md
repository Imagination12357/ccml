# ADR 0001: Core Language and Multi-Language Strategy

## Status
Accepted

## Date
2026-04-27

## Context
CCML aims to support multiple language ecosystems while preserving one exact grammar and behavior model.

The MVP implementation proved syntax direction, but production scale requires:
- strict cross-language consistency
- predictable diagnostics
- high performance under larger inputs
- maintainable release process across runtimes

## Decision
Adopt `Single Core + Bindings` with Rust as the core language.

- Core parser/AST/diagnostics/transcoding are implemented in Rust.
- Python and Node.js are first-class bindings.
- WASM artifact is produced from the same Rust core for browser and JS portability.

## Why Rust
- Strong memory safety guarantees with no GC pause model
- High runtime performance for parser workloads
- Mature tooling for CLI, FFI, and WASM targets
- Better long-term consistency than maintaining N independent parser implementations

## Alternatives Considered
1. Pure Polyglot (independent parser per language)
- Pros: native feel in each ecosystem
- Cons: behavior drift, duplicated bug fixes, high maintenance cost

2. C++ Core + Bindings
- Pros: top-tier performance
- Cons: safety and binding complexity risk is higher

3. Go Core + Bindings
- Pros: fast development
- Cons: weaker fit for low-level shared core + broad binding strategy

## Consequences

### Positive
- One semantic source of truth
- Shared conformance suite and release gates
- Faster propagation of parser fixes to all runtimes

### Negative
- Initial setup complexity (FFI, packaging, CI matrix)
- Team must maintain Rust competency

## Implementation Notes
- Keep C-ABI minimal and stable in `ccml-ffi`.
- Bindings should not implement syntax behavior locally.
- Conformance tests are shared assets and mandatory for release.

## Follow-up Decisions
- Duplicate key policy (`keep-last` vs `error`)
- Bare key character policy (strict vs permissive)
- CLI default output mode (`pretty` vs `compact`)
