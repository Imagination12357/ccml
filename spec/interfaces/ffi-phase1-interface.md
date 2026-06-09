# CCML FFI Phase 1 Interface

## 1. Goal
Define the phase-1 C ABI boundary for Python, Node, WASM-adjacent tooling, and other non-Rust callers.

## 2. Function Signatures (Frozen for Phase 1)

```c
typedef enum CcmlStatus {
  CCML_STATUS_OK = 0,
  CCML_STATUS_INVALID_ARGUMENT = 1,
  CCML_STATUS_INVALID_UTF8 = 2,
  CCML_STATUS_PARSE_ERROR = 3,
  CCML_STATUS_INTERNAL_ERROR = 255
} CcmlStatus;

int32_t ccml_to_json(
  const uint8_t* input_ptr,
  uintptr_t input_len,
  char** out_json,
  char** out_error_json
);

int32_t ccml_diagnose(
  const uint8_t* input_ptr,
  uintptr_t input_len,
  char** out_diag_json,
  char** out_error_json
);

int32_t ccml_version(char** out_version);

void ccml_free(char* ptr);
```

## 3. Memory Ownership Contract
1. On success, any non-null output buffer (`out_json`, `out_diag_json`, `out_version`) is heap-allocated by Rust and must be released by caller via `ccml_free`.
2. On non-zero status, `out_error_json` may contain a heap-allocated JSON error envelope and must be released via `ccml_free`.
3. Caller must pass valid writable pointers for all out-parameters.
4. Passing `NULL` output pointers is invalid argument.
5. `ccml_free(NULL)` is a no-op.

## 4. Error JSON Envelope (Draft)

```json
{
  "status": 255,
  "error_code": "InternalError",
  "message": "human-readable message"
}
```

Notes:
- Parser and UTF-8 failures map to the frozen status codes above.
- `InternalError` is reserved for unexpected implementation failures.

## 5. cbindgen Workflow
Configuration file:
- `core/rust/crates/ccml-ffi/cbindgen.toml`

Header generation command (from repository root):

```powershell
cbindgen core/rust/crates/ccml-ffi --config core/rust/crates/ccml-ffi/cbindgen.toml --output core/rust/crates/ccml-ffi/include/ccml_ffi.h
```

## 6. Contract Scope
1. Function signatures are fixed for phase 1.
2. Return code set is fixed for phase 1.
3. Memory ownership rules are fixed for phase 1.
4. Header generation uses the repository `cbindgen` config.
