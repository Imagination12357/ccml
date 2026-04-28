#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToJsonOptions {
    pub pretty: bool,
}

impl Default for ToJsonOptions {
    fn default() -> Self {
        Self { pretty: true }
    }
}

#[derive(Debug)]
pub struct CcmlError {
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstNode {
    Placeholder,
}

pub fn parse(_text: &str) -> Result<AstNode, CcmlError> {
    Err(CcmlError {
        diagnostics: vec![Diagnostic {
            code: "CCML9000".to_string(),
            message: "parse is not implemented yet".to_string(),
            line: 1,
            column: 1,
            severity: Severity::Error,
        }],
    })
}

pub fn to_json(text: &str, options: &ToJsonOptions) -> Result<String, CcmlError> {
    let _ = options;
    let _ = parse(text)?;
    Ok("{}".to_string())
}

pub fn diagnose(text: &str) -> Vec<Diagnostic> {
    match parse(text) {
        Ok(_) => Vec::new(),
        Err(e) => e.diagnostics,
    }
}
