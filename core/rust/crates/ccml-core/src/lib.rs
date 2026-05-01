mod ast;
mod diag;
mod json;
mod parser;

pub use ast::AstNode;
pub use diag::{CcmlError, Diagnostic, Severity};
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
    fn emits_duplicate_key_warning_with_keep_last() {
        let src = "x: 1\nx: 2";
        let json = to_json(src, &ToJsonOptions { pretty: false }).unwrap();
        let diagnostics = diagnose(src);
        assert_eq!(json, "{\"x\":2}");
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, "CCML2001");
        assert_eq!(diagnostics[0].severity, Severity::Warning);
    }
}
