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

    // Day 1 skeleton only: conversion logic lands on Day 2.
    let _ = input_ptr;
    let _ = input_len;
    write_error_json(
        out_error_json,
        CcmlStatus::InternalError,
        "FFI skeleton: ccml_to_json is not implemented yet",
    );
    CcmlStatus::InternalError as i32
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

    // Day 1 skeleton only: diagnostics logic lands on Day 2.
    let _ = input_ptr;
    let _ = input_len;
    write_error_json(
        out_error_json,
        CcmlStatus::InternalError,
        "FFI skeleton: ccml_diagnose is not implemented yet",
    );
    CcmlStatus::InternalError as i32
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
