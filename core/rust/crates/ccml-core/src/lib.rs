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

pub fn to_json(text: &str, options: &ToJsonOptions) -> Result<String, CcmlError> {
    let ast = parse(text)?;
    Ok(to_json_string(&ast, options.pretty))
}

pub fn diagnose(text: &str) -> Vec<Diagnostic> {
    match parse(text) {
        Ok(_) => Vec::new(),
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
}
