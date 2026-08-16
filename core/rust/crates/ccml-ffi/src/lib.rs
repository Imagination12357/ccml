use ccml_core::{
    diagnose, to_json, to_json_with_diagnostics, Diagnostic, Severity, ToJsonOptions,
};
use std::ffi::{c_char, CString};
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CcmlStatus {
    Ok = 0,
    InvalidArgument = 1,
    InvalidUtf8 = 2,
    ParseError = 3,
    InternalError = 255,
}

const VERSION: &str = "1.0.0";

#[unsafe(no_mangle)]
pub extern "C" fn ccml_to_json(
    input_ptr: *const u8,
    input_len: usize,
    out_json: *mut *mut c_char,
    out_error_json: *mut *mut c_char,
) -> i32 {
    if !validate_common_args(input_ptr, input_len, out_json, out_error_json) {
        return CcmlStatus::InvalidArgument as i32;
    }

    clear_out(out_json);
    clear_out(out_error_json);

    let result = std::panic::catch_unwind(|| {
        let input = decode_input(input_ptr, input_len)?;
        match to_json(&input, &ToJsonOptions { pretty: false }) {
            Ok(json) => {
                if write_owned_c_string(out_json, &json) {
                    Ok(CcmlStatus::Ok)
                } else {
                    Err(("failed to allocate output json".to_string(), CcmlStatus::InternalError))
                }
            }
            Err(err) => {
                let payload = error_payload_with_diagnostics(
                    CcmlStatus::ParseError,
                    "ccml parse error",
                    &err.diagnostics,
                );
                let _ = write_owned_c_string(out_error_json, &payload);
                Err(("ccml parse error".to_string(), CcmlStatus::ParseError))
            }
        }
    });

    match result {
        Ok(Ok(status)) => status as i32,
        Ok(Err((msg, status))) => {
            if status != CcmlStatus::ParseError {
                write_error_json(out_error_json, status, &msg);
            }
            status as i32
        }
        Err(_) => {
            write_error_json(
                out_error_json,
                CcmlStatus::InternalError,
                "panic in ccml_to_json",
            );
            CcmlStatus::InternalError as i32
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ccml_to_json_with_diagnostics(
    input_ptr: *const u8,
    input_len: usize,
    out_json: *mut *mut c_char,
    out_diag_json: *mut *mut c_char,
    out_error_json: *mut *mut c_char,
) -> i32 {
    if !validate_detailed_args(
        input_ptr,
        input_len,
        out_json,
        out_diag_json,
        out_error_json,
    ) {
        return CcmlStatus::InvalidArgument as i32;
    }

    clear_out(out_json);
    clear_out(out_diag_json);
    clear_out(out_error_json);

    let result = std::panic::catch_unwind(|| {
        let input = decode_input(input_ptr, input_len)?;
        match to_json_with_diagnostics(&input, &ToJsonOptions { pretty: false }) {
            Ok((json, diagnostics)) => {
                let diagnostics_json = diagnostics_json_payload(&diagnostics);
                let json_c_string = CString::new(json).map_err(|_| {
                    (
                        "failed to allocate output json".to_string(),
                        CcmlStatus::InternalError,
                    )
                })?;
                let diagnostics_c_string = CString::new(diagnostics_json).map_err(|_| {
                    (
                        "failed to allocate diagnostics json".to_string(),
                        CcmlStatus::InternalError,
                    )
                })?;

                // Safety: all out-pointers were validated above. Both C strings are
                // prepared before either pointer is transferred to the caller.
                unsafe {
                    *out_json = json_c_string.into_raw();
                    *out_diag_json = diagnostics_c_string.into_raw();
                }
                Ok(CcmlStatus::Ok)
            }
            Err(err) => {
                let payload = error_payload_with_diagnostics(
                    CcmlStatus::ParseError,
                    "ccml parse error",
                    &err.diagnostics,
                );
                let _ = write_owned_c_string(out_error_json, &payload);
                Err(("ccml parse error".to_string(), CcmlStatus::ParseError))
            }
        }
    });

    match result {
        Ok(Ok(status)) => status as i32,
        Ok(Err((msg, status))) => {
            if status != CcmlStatus::ParseError {
                write_error_json(out_error_json, status, &msg);
            }
            status as i32
        }
        Err(_) => {
            write_error_json(
                out_error_json,
                CcmlStatus::InternalError,
                "panic in ccml_to_json_with_diagnostics",
            );
            CcmlStatus::InternalError as i32
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ccml_diagnose(
    input_ptr: *const u8,
    input_len: usize,
    out_diag_json: *mut *mut c_char,
    out_error_json: *mut *mut c_char,
) -> i32 {
    if !validate_common_args(input_ptr, input_len, out_diag_json, out_error_json) {
        return CcmlStatus::InvalidArgument as i32;
    }

    clear_out(out_diag_json);
    clear_out(out_error_json);

    let result = std::panic::catch_unwind(|| {
        let input = decode_input(input_ptr, input_len)?;
        let diagnostics = diagnose(&input);
        let payload = diagnostics_json_payload(&diagnostics);
        if write_owned_c_string(out_diag_json, &payload) {
            Ok(CcmlStatus::Ok)
        } else {
            Err((
                "failed to allocate diagnostics json".to_string(),
                CcmlStatus::InternalError,
            ))
        }
    });

    match result {
        Ok(Ok(status)) => status as i32,
        Ok(Err((msg, status))) => {
            write_error_json(out_error_json, status, &msg);
            status as i32
        }
        Err(_) => {
            write_error_json(
                out_error_json,
                CcmlStatus::InternalError,
                "panic in ccml_diagnose",
            );
            CcmlStatus::InternalError as i32
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ccml_version(out_version: *mut *mut c_char) -> i32 {
    if out_version.is_null() {
        return CcmlStatus::InvalidArgument as i32;
    }
    clear_out(out_version);
    if !write_owned_c_string(out_version, VERSION) {
        return CcmlStatus::InternalError as i32;
    }
    CcmlStatus::Ok as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn ccml_free(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    // Safety: pointer must come from CString::into_raw in this crate.
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

fn validate_common_args(
    input_ptr: *const u8,
    input_len: usize,
    out_primary: *mut *mut c_char,
    out_error_json: *mut *mut c_char,
) -> bool {
    if out_primary.is_null() || out_error_json.is_null() {
        return false;
    }
    if input_len > 0 && input_ptr.is_null() {
        return false;
    }
    true
}

fn validate_detailed_args(
    input_ptr: *const u8,
    input_len: usize,
    out_json: *mut *mut c_char,
    out_diag_json: *mut *mut c_char,
    out_error_json: *mut *mut c_char,
) -> bool {
    !out_diag_json.is_null()
        && validate_common_args(input_ptr, input_len, out_json, out_error_json)
}

fn clear_out(out: *mut *mut c_char) {
    // Safety: caller guarantees out is a valid writable pointer.
    unsafe {
        *out = ptr::null_mut();
    }
}

fn write_owned_c_string(out: *mut *mut c_char, s: &str) -> bool {
    match CString::new(s) {
        Ok(cs) => {
            // Safety: caller guarantees out is a valid writable pointer.
            unsafe {
                *out = cs.into_raw();
            }
            true
        }
        Err(_) => false,
    }
}

fn write_error_json(out_error_json: *mut *mut c_char, status: CcmlStatus, message: &str) {
    let payload = format!(
        "{{\"status\":{},\"error_code\":\"{:?}\",\"message\":\"{}\"}}",
        status as i32,
        status,
        escape_json_string(message)
    );
    let _ = write_owned_c_string(out_error_json, &payload);
}

fn error_payload_with_diagnostics(
    status: CcmlStatus,
    message: &str,
    diagnostics: &[Diagnostic],
) -> String {
    format!(
        "{{\"status\":{},\"error_code\":\"{:?}\",\"message\":\"{}\",\"diagnostics\":{}}}",
        status as i32,
        status,
        escape_json_string(message),
        diagnostics_json_payload(diagnostics)
    )
}

fn diagnostics_json_payload(diagnostics: &[Diagnostic]) -> String {
    let mut out = String::from("[");
    for (idx, d) in diagnostics.iter().enumerate() {
        if idx > 0 {
            out.push(',');
        }
        out.push_str(&format!(
            "{{\"code\":\"{}\",\"message\":\"{}\",\"line\":{},\"column\":{},\"severity\":\"{}\"}}",
            escape_json_string(&d.code),
            escape_json_string(&d.message),
            d.line,
            d.column,
            severity_str(d.severity)
        ));
    }
    out.push(']');
    out
}

fn severity_str(sev: Severity) -> &'static str {
    match sev {
        Severity::Error => "error",
        Severity::Warning => "warning",
    }
}

fn decode_input(input_ptr: *const u8, input_len: usize) -> Result<String, (String, CcmlStatus)> {
    if input_len == 0 {
        return Ok(String::new());
    }
    if input_ptr.is_null() {
        return Err(("input_ptr is null with non-zero length".to_string(), CcmlStatus::InvalidArgument));
    }
    // Safety: pointer + length validated by caller contract.
    let bytes = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };
    let s = std::str::from_utf8(bytes)
        .map_err(|_| ("input is not valid utf-8".to_string(), CcmlStatus::InvalidUtf8))?;
    Ok(s.to_owned())
}

fn escape_json_string(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{000c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{0000}'..='\u{001f}' => {
                out.push_str(&format!("\\u{:04x}", ch as u32));
            }
            _ => out.push(ch),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonschema::JSONSchema;
    use std::ffi::CStr;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::OnceLock;
    use serde_json::Value;

    fn into_string_and_free(ptr: *mut c_char) -> String {
        assert!(!ptr.is_null());
        // Safety: ptr must be allocated by this crate via CString::into_raw.
        let s = unsafe { CStr::from_ptr(ptr) }
            .to_str()
            .expect("ffi output should be valid utf-8")
            .to_string();
        ccml_free(ptr);
        s
    }

    #[test]
    fn version_allocates_and_free_releases() {
        let mut out: *mut c_char = ptr::null_mut();
        let rc = ccml_version(&mut out as *mut *mut c_char);
        assert_eq!(rc, CcmlStatus::Ok as i32);
        assert!(!out.is_null());

        // Safety: out came from ccml_version allocation contract.
        let s = unsafe { CStr::from_ptr(out) }
            .to_str()
            .expect("version should be valid utf-8");
        assert_eq!(s, VERSION);

        ccml_free(out);
    }

    #[test]
    fn free_accepts_null() {
        ccml_free(ptr::null_mut());
    }

    #[test]
    fn to_json_success_path_returns_json_and_empty_error() {
        let input = b"a: 1";
        let mut out_json: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();

        let rc = ccml_to_json(
            input.as_ptr(),
            input.len(),
            &mut out_json as *mut *mut c_char,
            &mut out_err as *mut *mut c_char,
        );
        assert_eq!(rc, CcmlStatus::Ok as i32);
        assert!(out_err.is_null());
        let json = into_string_and_free(out_json);
        assert_eq!(json, "{\"a\":1}");
    }

    #[test]
    fn to_json_parse_error_sets_error_json() {
        let input = b"a:";
        let mut out_json: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();

        let rc = ccml_to_json(
            input.as_ptr(),
            input.len(),
            &mut out_json as *mut *mut c_char,
            &mut out_err as *mut *mut c_char,
        );
        assert_eq!(rc, CcmlStatus::ParseError as i32);
        assert!(out_json.is_null());
        let err = into_string_and_free(out_err);
        assert!(err.contains("\"error_code\":\"ParseError\""));
        assert!(err.contains("\"diagnostics\""));
    }

    #[test]
    fn to_json_invalid_argument_rejects_null_out_pointer() {
        let input = b"a: 1";
        let mut out_err: *mut c_char = ptr::null_mut();
        let rc = ccml_to_json(
            input.as_ptr(),
            input.len(),
            ptr::null_mut(),
            &mut out_err as *mut *mut c_char,
        );
        assert_eq!(rc, CcmlStatus::InvalidArgument as i32);
        assert!(out_err.is_null());
    }

    #[test]
    fn to_json_invalid_utf8_maps_status_and_error_json() {
        let input: [u8; 1] = [0xff];
        let mut out_json: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();
        let rc = ccml_to_json(
            input.as_ptr(),
            input.len(),
            &mut out_json as *mut *mut c_char,
            &mut out_err as *mut *mut c_char,
        );
        assert_eq!(rc, CcmlStatus::InvalidUtf8 as i32);
        assert!(out_json.is_null());
        let err = into_string_and_free(out_err);
        assert!(err.contains("\"error_code\":\"InvalidUtf8\""));
    }

    #[test]
    fn diagnose_warn_path_emits_duplicate_key_warning() {
        let input = b"x: 1\nx: 2";
        let mut out_diag: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();

        let rc = ccml_diagnose(
            input.as_ptr(),
            input.len(),
            &mut out_diag as *mut *mut c_char,
            &mut out_err as *mut *mut c_char,
        );
        assert_eq!(rc, CcmlStatus::Ok as i32);
        assert!(out_err.is_null());
        let diag = into_string_and_free(out_diag);
        assert!(diag.contains("\"code\":\"CCML2001\""));
        assert!(diag.contains("\"severity\":\"warning\""));
    }

    #[test]
    fn diagnose_escapes_control_characters_in_warning_payload() {
        let input = br#""\u0000": 1
"\u0000": 2"#;
        let mut out_diag: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();

        let rc = ccml_diagnose(
            input.as_ptr(),
            input.len(),
            &mut out_diag as *mut *mut c_char,
            &mut out_err as *mut *mut c_char,
        );
        assert_eq!(rc, CcmlStatus::Ok as i32);
        assert!(out_err.is_null());

        let payload = into_string_and_free(out_diag);
        let diagnostics: Value = serde_json::from_str(&payload).expect("valid diagnostics json");
        assert_eq!(diagnostics[0]["code"], "CCML2001");
        assert_eq!(diagnostics[0]["severity"], "warning");
    }

    #[test]
    fn detailed_transcode_returns_json_and_empty_diagnostics() {
        let input = b"a: 1";
        let mut out_json: *mut c_char = ptr::null_mut();
        let mut out_diag: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();

        let rc = ccml_to_json_with_diagnostics(
            input.as_ptr(),
            input.len(),
            &mut out_json as *mut *mut c_char,
            &mut out_diag as *mut *mut c_char,
            &mut out_err as *mut *mut c_char,
        );
        assert_eq!(rc, CcmlStatus::Ok as i32);
        assert!(out_err.is_null());
        assert_eq!(into_string_and_free(out_json), "{\"a\":1}");
        assert_eq!(into_string_and_free(out_diag), "[]");
    }

    #[test]
    fn detailed_transcode_returns_warning_from_same_parse() {
        let input = b"x: 1\nx: 2";
        let mut out_json: *mut c_char = ptr::null_mut();
        let mut out_diag: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();

        let rc = ccml_to_json_with_diagnostics(
            input.as_ptr(),
            input.len(),
            &mut out_json as *mut *mut c_char,
            &mut out_diag as *mut *mut c_char,
            &mut out_err as *mut *mut c_char,
        );
        assert_eq!(rc, CcmlStatus::Ok as i32);
        assert!(out_err.is_null());
        assert_eq!(into_string_and_free(out_json), "{\"x\":2}");

        let payload = into_string_and_free(out_diag);
        let diagnostics: Value = serde_json::from_str(&payload).expect("valid diagnostics json");
        assert_eq!(diagnostics[0]["code"], "CCML2001");
        assert_eq!(diagnostics[0]["severity"], "warning");
    }

    #[test]
    fn detailed_transcode_parse_error_uses_error_envelope() {
        let input = b"a:";
        let mut out_json: *mut c_char = ptr::null_mut();
        let mut out_diag: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();

        let rc = ccml_to_json_with_diagnostics(
            input.as_ptr(),
            input.len(),
            &mut out_json as *mut *mut c_char,
            &mut out_diag as *mut *mut c_char,
            &mut out_err as *mut *mut c_char,
        );
        assert_eq!(rc, CcmlStatus::ParseError as i32);
        assert!(out_json.is_null());
        assert!(out_diag.is_null());

        let payload = into_string_and_free(out_err);
        let error: Value = serde_json::from_str(&payload).expect("valid error json");
        assert_eq!(error["error_code"], "ParseError");
        assert_eq!(error["diagnostics"][0]["severity"], "error");
    }

    #[test]
    fn detailed_transcode_rejects_null_diagnostics_out_pointer() {
        let input = b"a: 1";
        let mut out_json: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();

        let rc = ccml_to_json_with_diagnostics(
            input.as_ptr(),
            input.len(),
            &mut out_json as *mut *mut c_char,
            ptr::null_mut(),
            &mut out_err as *mut *mut c_char,
        );
        assert_eq!(rc, CcmlStatus::InvalidArgument as i32);
        assert!(out_json.is_null());
        assert!(out_err.is_null());
    }

    #[test]
    fn detailed_transcode_invalid_utf8_uses_error_envelope() {
        let input: [u8; 1] = [0xff];
        let mut out_json: *mut c_char = ptr::null_mut();
        let mut out_diag: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();

        let rc = ccml_to_json_with_diagnostics(
            input.as_ptr(),
            input.len(),
            &mut out_json as *mut *mut c_char,
            &mut out_diag as *mut *mut c_char,
            &mut out_err as *mut *mut c_char,
        );
        assert_eq!(rc, CcmlStatus::InvalidUtf8 as i32);
        assert!(out_json.is_null());
        assert!(out_diag.is_null());

        let payload = into_string_and_free(out_err);
        let error: Value = serde_json::from_str(&payload).expect("valid error json");
        assert_eq!(error["error_code"], "InvalidUtf8");
    }

    #[test]
    fn diagnose_rejects_null_out_error_pointer() {
        let input = b"a: 1";
        let mut out_diag: *mut c_char = ptr::null_mut();
        let rc = ccml_diagnose(
            input.as_ptr(),
            input.len(),
            &mut out_diag as *mut *mut c_char,
            ptr::null_mut(),
        );
        assert_eq!(rc, CcmlStatus::InvalidArgument as i32);
        assert!(out_diag.is_null());
    }

    #[test]
    fn conformance_smoke_valid_invalid_warn() {
        run_valid_case("valid/001-basic-object.json");
        run_invalid_case("invalid/001-missing-colon.json");
        run_warn_case("warn/001-duplicate-key-keep-last.json");
    }

    fn run_valid_case(rel: &str) {
        let case = load_case(rel);
        let input = case["input"].as_str().expect("input string");
        let expected_json = case["expect"]["output_json"]
            .as_str()
            .expect("output_json string");

        let mut out_json: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();
        let rc = ccml_to_json(
            input.as_ptr(),
            input.len(),
            &mut out_json as *mut *mut c_char,
            &mut out_err as *mut *mut c_char,
        );
        assert_eq!(rc, CcmlStatus::Ok as i32);
        assert!(out_err.is_null());

        let actual_json = into_string_and_free(out_json);
        let actual_v: Value = serde_json::from_str(&actual_json).expect("actual json parse");
        let expected_v: Value = serde_json::from_str(expected_json).expect("expected json parse");
        assert_eq!(actual_v, expected_v);
    }

    fn run_invalid_case(rel: &str) {
        let case = load_case(rel);
        let input = case["input"].as_str().expect("input string");
        let expected_code = case["expect"]["error"]["code"]
            .as_str()
            .expect("error code");
        let expected_msg = case["expect"]["error"]["message_contains"]
            .as_str()
            .expect("message_contains");

        let mut out_json: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();
        let rc = ccml_to_json(
            input.as_ptr(),
            input.len(),
            &mut out_json as *mut *mut c_char,
            &mut out_err as *mut *mut c_char,
        );
        assert_eq!(rc, CcmlStatus::ParseError as i32);
        assert!(out_json.is_null());
        let err = into_string_and_free(out_err);
        assert!(err.contains(expected_code));
        assert!(err.to_lowercase().contains(&expected_msg.to_lowercase()));
    }

    fn run_warn_case(rel: &str) {
        let case = load_case(rel);
        let input = case["input"].as_str().expect("input string");
        let expected_json = case["expect"]["output_json"]
            .as_str()
            .expect("output_json string");
        let warning_code = case["expect"]["warnings"][0]["code"]
            .as_str()
            .expect("warning code");
        let warning_msg = case["expect"]["warnings"][0]["message_contains"]
            .as_str()
            .expect("warning message");

        let mut out_json: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();
        let rc = ccml_to_json(
            input.as_ptr(),
            input.len(),
            &mut out_json as *mut *mut c_char,
            &mut out_err as *mut *mut c_char,
        );
        assert_eq!(rc, CcmlStatus::Ok as i32);
        assert!(out_err.is_null());
        let actual_json = into_string_and_free(out_json);
        let actual_v: Value = serde_json::from_str(&actual_json).expect("actual json parse");
        let expected_v: Value = serde_json::from_str(expected_json).expect("expected json parse");
        assert_eq!(actual_v, expected_v);

        let mut out_diag: *mut c_char = ptr::null_mut();
        let mut out_diag_err: *mut c_char = ptr::null_mut();
        let drc = ccml_diagnose(
            input.as_ptr(),
            input.len(),
            &mut out_diag as *mut *mut c_char,
            &mut out_diag_err as *mut *mut c_char,
        );
        assert_eq!(drc, CcmlStatus::Ok as i32);
        assert!(out_diag_err.is_null());
        let diag = into_string_and_free(out_diag);
        assert!(diag.contains(warning_code));
        assert!(diag.to_lowercase().contains(&warning_msg.to_lowercase()));
    }

    fn load_case(rel: &str) -> Value {
        let base = conformance_base_dir();
        let text = fs::read_to_string(base.join(rel)).expect("read conformance case");
        let case: Value = serde_json::from_str(strip_bom(&text)).expect("parse conformance case json");
        let validator = schema_validator();
        if let Err(mut errors) = validator.validate(&case) {
            if let Some(err) = errors.next() {
                panic!("schema error in {}: {}", rel, err);
            }
            panic!("schema error in {}: unknown validation failure", rel);
        }
        case
    }

    fn conformance_base_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("..")
            .join("..")
            .join("tests")
            .join("conformance")
    }

    fn schema_validator() -> &'static JSONSchema {
        static SCHEMA: OnceLock<JSONSchema> = OnceLock::new();
        SCHEMA.get_or_init(|| {
            let schema_path = conformance_base_dir()
                .join("_schema")
                .join("ccml-conformance-case.schema.json");
            let schema_text = fs::read_to_string(&schema_path).expect("read conformance schema");
            let schema_json: Value =
                serde_json::from_str(strip_bom(&schema_text)).expect("parse conformance schema json");
            JSONSchema::options()
                .compile(&schema_json)
                .expect("compile conformance schema")
        })
    }

    fn strip_bom(s: &str) -> &str {
        s.strip_prefix('\u{feff}').unwrap_or(s)
    }
}
