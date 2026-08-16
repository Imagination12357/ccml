# CCML FFI Detailed Transcode Interface

## 1. Goal

Expose JSON output and non-fatal diagnostics from one CCML parse without changing the frozen Phase 1 functions.

## 2. Function Signature

```c
int32_t ccml_to_json_with_diagnostics(
  const uint8_t* input_ptr,
  uintptr_t input_len,
  char** out_json,
  char** out_diag_json,
  char** out_error_json
);
```

All three output pointers are required writable pointers. Passing `NULL` for any output pointer returns `CCML_STATUS_INVALID_ARGUMENT`.

## 3. Success Contract

On `CCML_STATUS_OK`:

1. `out_json` contains compact JSON text.
2. `out_diag_json` contains a JSON array of diagnostics from the same parse.
3. `out_diag_json` contains `[]` when there are no diagnostics.
4. `out_error_json` is `NULL`.

Each diagnostic object contains:

```json
{
  "code": "CCML2001",
  "message": "duplicate key overwritten",
  "line": 2,
  "column": 1,
  "severity": "warning"
}
```

## 4. Failure Contract

After valid output pointers have been supplied, a non-zero status leaves `out_json` and `out_diag_json` as `NULL`.

Parse failures return `CCML_STATUS_PARSE_ERROR` and write the existing error envelope, including error diagnostics, to `out_error_json`:

```json
{
  "status": 3,
  "error_code": "ParseError",
  "message": "ccml parse error",
  "diagnostics": []
}
```

Invalid UTF-8 and internal failures use the existing Phase 1 status codes and error envelope.

## 5. Memory Ownership

1. Rust allocates every non-null output string.
2. The caller must release `out_json`, `out_diag_json`, and `out_error_json` independently with `ccml_free`.
3. Both success strings are prepared before either pointer is transferred to the caller.
4. `ccml_free(NULL)` remains a no-op.

## 6. Compatibility

This function is an additive export. It does not change:

- `ccml_to_json`
- `ccml_diagnose`
- `ccml_version`
- `ccml_free`
- the Phase 1 status code set
- the Phase 1 memory ownership rules

Python and Node bindings remain on the Phase 1 exports until their detailed-transcode APIs are designed separately.
