use ccml_core::{diagnose, to_json, Diagnostic, Severity, ToJsonOptions};
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

const VERSION: &str = "0.1.0";

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
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

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
}
