use ccml_core::{diagnose, to_json, Diagnostic, Severity, ToJsonOptions};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn ccml_to_json_wasm(input: &str) -> Result<String, JsValue> {
    to_json(input, &ToJsonOptions { pretty: false }).map_err(|err| {
        let payload = format_error_payload("ccml parse error", &err.diagnostics);
        JsValue::from_str(&payload)
    })
}

#[wasm_bindgen]
pub fn ccml_diagnose_wasm(input: &str) -> String {
    diagnostics_json_payload(&diagnose(input))
}

fn format_error_payload(message: &str, diagnostics: &[Diagnostic]) -> String {
    format!(
        "{{\"message\":\"{}\",\"diagnostics\":{}}}",
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
