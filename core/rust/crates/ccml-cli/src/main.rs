use ccml_core::{diagnose, parse, to_json, AstNode, ToJsonOptions};
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

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

    match to_json(&input, &ToJsonOptions::default()) {
        Ok(json) => println!("{json}"),
        Err(err) => {
            for d in err.diagnostics {
                eprintln!("{}:{} {} {}", d.line, d.column, d.code, d.message);
            }
            std::process::exit(1);
        }
    }
}

fn run_conformance(root: &Path) -> usize {
    let mut total = 0usize;
    let mut failed = 0usize;

    for category in ["valid", "invalid", "warn"] {
        let dir = root.join(category);
        let mut files = list_json_files(&dir);
        files.sort();
        for path in files {
            total += 1;
            if let Err(msg) = run_case(&path) {
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

fn run_case(path: &Path) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read error: {e}"))?;
    let case_root = parse(&text).map_err(|e| format!("vector parse error: {:?}", e.diagnostics))?;
    let case_obj = as_object(&case_root)?;
    let input = as_string(req(case_obj, "input")?)?;
    let expect = as_object(req(case_obj, "expect")?)?;
    let status = as_string(req(expect, "status")?)?;

    match status {
        "ok" => validate_ok(input, expect),
        "ok_with_warnings" => validate_ok_with_warnings(input, expect),
        "error" => validate_error(input, expect),
        _ => Err(format!("unknown expect.status: {status}")),
    }
}

fn validate_ok(input: &str, expect: &BTreeMap<String, AstNode>) -> Result<(), String> {
    let expected_json = as_string(req(expect, "output_json")?)?;
    let actual_json = to_json(input, &ToJsonOptions { pretty: false })
        .map_err(|e| format!("unexpected parse error: {:?}", e.diagnostics))?;
    assert_json_semantic_eq(&actual_json, expected_json)
}

fn validate_ok_with_warnings(
    input: &str,
    expect: &BTreeMap<String, AstNode>,
) -> Result<(), String> {
    let expected_json = as_string(req(expect, "output_json")?)?;
    let actual_json = to_json(input, &ToJsonOptions { pretty: false })
        .map_err(|e| format!("unexpected parse error: {:?}", e.diagnostics))?;
    assert_json_semantic_eq(&actual_json, expected_json)?;

    let expected_warnings = as_array(req(expect, "warnings")?)?;
    let actual_diags = diagnose(input);
    for w in expected_warnings {
        let wobj = as_object(w)?;
        let code = as_string(req(wobj, "code")?)?;
        let needle = as_string(req(wobj, "message_contains")?)?;
        let found = actual_diags
            .iter()
            .any(|d| d.code == code && d.message.to_lowercase().contains(&needle.to_lowercase()));
        if !found {
            return Err(format!("missing warning code={} contains='{}'", code, needle));
        }
    }
    Ok(())
}

fn validate_error(input: &str, expect: &BTreeMap<String, AstNode>) -> Result<(), String> {
    let expected_err = as_object(req(expect, "error")?)?;
    let expected_code = as_string(req(expected_err, "code")?)?;
    let expected_line = as_usize(req(expected_err, "line")?)?;
    let expected_column = as_usize(req(expected_err, "column")?)?;
    let expected_contains = expected_err
        .get("message_contains")
        .map(as_string)
        .transpose()?;

    match parse(input) {
        Ok(_) => Err("expected parse error, got success".to_string()),
        Err(err) => {
            let d = err
                .diagnostics
                .first()
                .ok_or_else(|| "empty diagnostics".to_string())?;
            if d.code != expected_code {
                return Err(format!("error code mismatch: {} != {}", d.code, expected_code));
            }
            if d.line != expected_line || d.column != expected_column {
                return Err(format!(
                    "location mismatch: {}:{} != {}:{}",
                    d.line, d.column, expected_line, expected_column
                ));
            }
            if let Some(needle) = expected_contains {
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

fn req<'a>(obj: &'a BTreeMap<String, AstNode>, key: &str) -> Result<&'a AstNode, String> {
    obj.get(key).ok_or_else(|| format!("missing field '{}'", key))
}

fn as_object(node: &AstNode) -> Result<&BTreeMap<String, AstNode>, String> {
    match node {
        AstNode::Object(v) => Ok(v),
        _ => Err("expected object".to_string()),
    }
}

fn as_array(node: &AstNode) -> Result<&Vec<AstNode>, String> {
    match node {
        AstNode::Array(v) => Ok(v),
        _ => Err("expected array".to_string()),
    }
}

fn as_string(node: &AstNode) -> Result<&str, String> {
    match node {
        AstNode::String(v) => Ok(v),
        _ => Err("expected string".to_string()),
    }
}

fn as_usize(node: &AstNode) -> Result<usize, String> {
    match node {
        AstNode::Number(v) => v
            .parse::<usize>()
            .map_err(|_| format!("expected integer number, got '{}'", v)),
        _ => Err("expected numeric value".to_string()),
    }
}
