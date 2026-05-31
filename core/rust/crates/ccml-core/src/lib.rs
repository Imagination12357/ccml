mod ast;
mod diag;
mod json;
mod native;
mod parser;

pub use ast::AstNode;
pub use diag::{CcmlError, Diagnostic, Severity};
pub use native::{ast_to_native, NativeNumber, NativeValue};
use json::to_json_string;
use parser::Parser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToJsonOptions {
    pub pretty: bool,
}

impl Default for ToJsonOptions {
    fn default() -> Self {
        Self { pretty: true }
    }
}

pub fn parse(text: &str) -> Result<AstNode, CcmlError> {
    Parser::new(text).parse_start()
}

fn parse_with_diagnostics(text: &str) -> Result<(AstNode, Vec<Diagnostic>), CcmlError> {
    Parser::new(text).parse_start_with_diagnostics()
}

pub fn to_json(text: &str, options: &ToJsonOptions) -> Result<String, CcmlError> {
    let ast = parse(text)?;
    Ok(to_json_string(&ast, options.pretty))
}

pub fn to_native(text: &str) -> Result<NativeValue, CcmlError> {
    let ast = parse(text)?;
    Ok(ast_to_native(&ast))
}

pub fn diagnose(text: &str) -> Vec<Diagnostic> {
    match parse_with_diagnostics(text) {
        Ok((_, diagnostics)) => diagnostics,
        Err(e) => e.diagnostics,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_implicit_root_object() {
        let src = "name: \"Alice\"\nage: 30";
        let json = to_json(src, &ToJsonOptions { pretty: false }).unwrap();
        assert_eq!(json, "{\"age\":30,\"name\":\"Alice\"}");
    }

    #[test]
    fn parses_array_root() {
        let src = "[1 2 3]";
        let json = to_json(src, &ToJsonOptions { pretty: false }).unwrap();
        assert_eq!(json, "[1,2,3]");
    }

    #[test]
    fn parses_unicode_escape_string() {
        let src = "k: \"A\\u0042\"";
        let json = to_json(src, &ToJsonOptions { pretty: false }).unwrap();
        assert_eq!(json, "{\"k\":\"AB\"}");
    }

    #[test]
    fn rejects_missing_value_after_colon() {
        let src = "a:";
        let err = parse(src).unwrap_err();
        assert_eq!(err.diagnostics[0].code, "CCML1001");
        assert_eq!(err.diagnostics[0].line, 1);
        assert_eq!(err.diagnostics[0].column, 3);
    }

    #[test]
    fn rejects_invalid_punctuation_only_key() {
        let src = "--: 1";
        let err = parse(src).unwrap_err();
        assert_eq!(err.diagnostics[0].code, "CCML1006");
    }

    #[test]
    fn parses_keyword_like_key_as_string_key() {
        let src = "true: 1";
        let json = to_json(src, &ToJsonOptions { pretty: false }).unwrap();
        assert_eq!(json, "{\"true\":1}");
    }

    #[test]
    fn skips_bom_unicode_whitespace_and_non_ascii_comments() {
        let src = "\u{feff}\u{00a0}a: 1\n# \u{2603}\nb: 2";
        let json = to_json(src, &ToJsonOptions { pretty: false }).unwrap();
        assert_eq!(json, "{\"a\":1,\"b\":2}");
    }

    #[test]
    fn emits_duplicate_key_warning_with_keep_last() {
        let src = "x: 1\nx: 2";
        let json = to_json(src, &ToJsonOptions { pretty: false }).unwrap();
        let diagnostics = diagnose(src);
        assert_eq!(json, "{\"x\":2}");
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, "CCML2001");
        assert_eq!(diagnostics[0].severity, Severity::Warning);
    }

    #[test]
    fn json_escape_preserves_control_sequences() {
        let src = "s: \"\\b\\f\\n\\r\\t\\\\\\\"\"";
        let json = to_json(src, &ToJsonOptions { pretty: false }).unwrap();
        assert_eq!(json, "{\"s\":\"\\b\\f\\n\\r\\t\\\\\\\"\"}");
    }

    #[test]
    fn preserves_number_raw_in_json_mode() {
        let src = "n: 1e10";
        let json = to_json(src, &ToJsonOptions { pretty: false }).unwrap();
        assert_eq!(json, "{\"n\":1e10}");
    }

    #[test]
    fn exposes_number_raw_in_native_mode() {
        let src = "n: 1e10";
        let native = to_native(src).unwrap();
        let obj = match native {
            NativeValue::Object(o) => o,
            _ => panic!("expected object"),
        };
        let n = match obj.get("n").unwrap() {
            NativeValue::Number(v) => v,
            _ => panic!("expected number"),
        };
        assert_eq!(n.raw, "1e10");
        assert_eq!(n.as_f64(), Some(1e10_f64));
    }
}
