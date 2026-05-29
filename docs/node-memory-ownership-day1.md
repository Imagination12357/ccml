# Node Memory Ownership Day 1 Design Note

## Goal
Close the Day 1 ownership ambiguity in Node FFI calls by removing dependence on `koffi` string marshaling side effects and using explicit `ccml_free` release calls.

## Acceptance Criteria
1. Node wrapper reads output pointers from `ccml_to_json` and `ccml_diagnose` as owned C strings.
2. Every non-null output pointer is released exactly once through `ccml_free`.
3. Error and success paths both preserve existing status/message behavior.
4. Day 1 regression covers repeated mixed calls plus large payload and multiple invalid variants.

## Ownership Model
1. Rust (`ccml-ffi`) allocates output with `CString::into_raw`.
2. Node receives raw output pointers (`char **` equivalent).
3. Node decodes pointer content with `koffi.decode(ptr, "char *")`.
4. Node releases pointer with `ccml_free(ptr)` immediately after consumption.
5. `finally` cleanup releases any remaining pointer to avoid leaks on exceptions.

## Non-Goals (Day 1)
1. No parser or diagnostic semantic changes.
2. No FFI API surface expansion.
3. No CI workflow changes (scheduled for Day 2).
