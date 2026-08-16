use ccml_core::{
    diagnose, parse, to_json, to_json_with_diagnostics, AstNode, Severity, ToJsonOptions,
};
use jsonschema::JSONSchema;
use serde::Deserialize;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct ConformanceCase {
    input: String,
    expect: Expectation,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "status")]
enum Expectation {
    #[serde(rename = "ok")]
    Ok { output_json: String },
    #[serde(rename = "ok_with_warnings")]
    OkWithWarnings {
        output_json: String,
        warnings: Vec<ExpectedWarning>,
    },
    #[serde(rename = "error")]
    Error { error: ExpectedError },
}

#[derive(Debug, Deserialize)]
struct ExpectedWarning {
    code: String,
    message_contains: String,
}

#[derive(Debug, Deserialize)]
struct ExpectedError {
    code: String,
    line: usize,
    column: usize,
    message_contains: Option<String>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 && args[1] == "conformance" {
        let root = if args.len() >= 3 {
            PathBuf::from(&args[2])
        } else {
            PathBuf::from("../../tests/conformance")
        };
        let failed = run_conformance(&root);
        std::process::exit(if failed == 0 { 0 } else { 1 });
    }

    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("failed to read stdin: {e}");
        std::process::exit(2);
    }

    match to_json_with_diagnostics(&input, &ToJsonOptions::default()) {
        Ok((json, diagnostics)) => {
            println!("{json}");
            for d in diagnostics {
                if d.severity == Severity::Warning {
                    eprintln!(
                        "{}:{} warning {} {}",
                        d.line, d.column, d.code, d.message
                    );
                }
            }
        }
        Err(err) => {
            for d in err.diagnostics {
                eprintln!(
                    "{}:{} error {} {}",
                    d.line, d.column, d.code, d.message
                );
            }
            std::process::exit(1);
        }
    }
}

fn run_conformance(root: &Path) -> usize {
    let schema_validator = match load_conformance_schema_validator(root) {
        Ok(validator) => validator,
        Err(msg) => {
            eprintln!("FAIL schema: {msg}");
            println!("Conformance summary: total=0, failed=1");
            return 1;
        }
    };

    let mut total = 0usize;
    let mut failed = 0usize;

    for category in ["valid", "invalid", "warn"] {
        let dir = root.join(category);
        let mut files = list_json_files(&dir);
        files.sort();
        for path in files {
            total += 1;
            if let Err(msg) = run_case(&path, &schema_validator) {
                failed += 1;
                eprintln!("FAIL {}: {}", path.display(), msg);
            } else {
                println!("PASS {}", path.display());
            }
        }
    }

    println!("Conformance summary: total={}, failed={}", total, failed);
    failed
}

fn load_conformance_schema_validator(root: &Path) -> Result<JSONSchema, String> {
    let schema_path = root.join("_schema").join("ccml-conformance-case.schema.json");
    let schema_text = fs::read_to_string(&schema_path)
        .map_err(|e| format!("schema read error ({}): {e}", schema_path.display()))?;
    let schema_text = strip_utf8_bom(&schema_text);
    let schema_value: serde_json::Value = serde_json::from_str(schema_text)
        .map_err(|e| format!("schema json decode error ({}): {e}", schema_path.display()))?;
    JSONSchema::options()
        .compile(&schema_value)
        .map_err(|e| format!("schema compile error ({}): {e}", schema_path.display()))
}

fn list_json_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(read_dir) = fs::read_dir(dir) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                out.push(path);
            }
        }
    }
    out
}

fn run_case(path: &Path, schema_validator: &JSONSchema) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read error: {e}"))?;
    let text = strip_utf8_bom(&text);
    let case_value: serde_json::Value =
        serde_json::from_str(text).map_err(|e| format!("load error: vector json decode error: {e}"))?;

    if let Err(mut errors) = schema_validator.validate(&case_value) {
        if let Some(err) = errors.next() {
            return Err(format!("schema error: {err}"));
        }
        return Err("schema error: unknown validation failure".to_string());
    }

    let case: ConformanceCase =
        serde_json::from_value(case_value).map_err(|e| format!("load error: vector decode error: {e}"))?;

    match &case.expect {
        Expectation::Ok { output_json } => validate_ok(&case.input, output_json),
        Expectation::OkWithWarnings {
            output_json,
            warnings,
        } => validate_ok_with_warnings(&case.input, output_json, warnings),
        Expectation::Error { error } => validate_error(&case.input, error),
    }
}

fn strip_utf8_bom(input: &str) -> &str {
    input.strip_prefix('\u{feff}').unwrap_or(input)
}

fn validate_ok(input: &str, expected_json: &str) -> Result<(), String> {
    let actual_json = to_json(input, &ToJsonOptions { pretty: false })
        .map_err(|e| format!("unexpected parse error: {:?}", e.diagnostics))?;
    assert_json_semantic_eq(&actual_json, expected_json)
}

fn validate_ok_with_warnings(input: &str, expected_json: &str, warnings: &[ExpectedWarning]) -> Result<(), String> {
    let actual_json = to_json(input, &ToJsonOptions { pretty: false })
        .map_err(|e| format!("unexpected parse error: {:?}", e.diagnostics))?;
    assert_json_semantic_eq(&actual_json, expected_json)?;

    let actual_diags = diagnose(input);
    for w in warnings {
        let found = actual_diags
            .iter()
            .any(|d| {
                d.code == w.code
                    && d.message
                        .to_lowercase()
                        .contains(&w.message_contains.to_lowercase())
            });
        if !found {
            return Err(format!(
                "missing warning code={} contains='{}'",
                w.code, w.message_contains
            ));
        }
    }
    Ok(())
}

fn validate_error(input: &str, expected: &ExpectedError) -> Result<(), String> {
    match parse(input) {
        Ok(_) => Err("expected parse error, got success".to_string()),
        Err(err) => {
            let d = err
                .diagnostics
                .first()
                .ok_or_else(|| "empty diagnostics".to_string())?;
            if d.code != expected.code {
                return Err(format!("error code mismatch: {} != {}", d.code, expected.code));
            }
            if d.line != expected.line || d.column != expected.column {
                return Err(format!(
                    "location mismatch: {}:{} != {}:{}",
                    d.line, d.column, expected.line, expected.column
                ));
            }
            if let Some(needle) = &expected.message_contains {
                if !d.message.to_lowercase().contains(&needle.to_lowercase()) {
                    return Err(format!(
                        "message mismatch: '{}' does not contain '{}'",
                        d.message, needle
                    ));
                }
            }
            Ok(())
        }
    }
}

fn assert_json_semantic_eq(actual_json: &str, expected_json: &str) -> Result<(), String> {
    let left = parse(actual_json).map_err(|e| format!("actual json parse failed: {:?}", e.diagnostics))?;
    let right =
        parse(expected_json).map_err(|e| format!("expected json parse failed: {:?}", e.diagnostics))?;
    if ast_semantic_eq(&left, &right) {
        Ok(())
    } else {
        Err(format!(
            "json mismatch\nactual={}\nexpected={}",
            actual_json, expected_json
        ))
    }
}

fn ast_semantic_eq(a: &AstNode, b: &AstNode) -> bool {
    match (a, b) {
        (AstNode::Object(oa), AstNode::Object(ob)) => {
            if oa.len() != ob.len() {
                return false;
            }
            oa.iter().all(|(k, va)| ob.get(k).is_some_and(|vb| ast_semantic_eq(va, vb)))
        }
        (AstNode::Array(aa), AstNode::Array(ab)) => {
            aa.len() == ab.len() && aa.iter().zip(ab.iter()).all(|(x, y)| ast_semantic_eq(x, y))
        }
        (AstNode::String(sa), AstNode::String(sb)) => sa == sb,
        (AstNode::Bool(ba), AstNode::Bool(bb)) => ba == bb,
        (AstNode::Null, AstNode::Null) => true,
        (AstNode::Number(na), AstNode::Number(nb)) => numeric_eq(na, nb),
        _ => false,
    }
}

fn numeric_eq(a: &str, b: &str) -> bool {
    if let (Ok(ia), Ok(ib)) = (a.parse::<i128>(), b.parse::<i128>()) {
        return ia == ib;
    }
    if let (Ok(fa), Ok(fb)) = (a.parse::<f64>(), b.parse::<f64>()) {
        return fa == fb;
    }
    a == b
}
