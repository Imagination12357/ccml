export const CCML_STATUS = Object.freeze({
  OK: 0,
  INVALID_ARGUMENT: 1,
  INVALID_UTF8: 2,
  PARSE_ERROR: 3,
  INTERNAL_ERROR: 255
});

export const PHASE1_EXPORTS = Object.freeze([
  "ccml_to_json",
  "ccml_diagnose",
  "ccml_version",
  "ccml_free"
]);

export const FFI_OVERRIDE_ENV = "CCML_FFI_LIB";
