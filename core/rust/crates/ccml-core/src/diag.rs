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
pub struct CcmlError {
    pub diagnostics: Vec<Diagnostic>,
}

impl CcmlError {
    pub fn single(code: &str, message: &str, line: usize, column: usize) -> Self {
        Self {
            diagnostics: vec![Diagnostic {
                code: code.to_string(),
                message: message.to_string(),
                line,
                column,
                severity: Severity::Error,
            }],
        }
    }
}
