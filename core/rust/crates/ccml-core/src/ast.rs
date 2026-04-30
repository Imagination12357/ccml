use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Object(BTreeMap<String, AstNode>),
    Array(Vec<AstNode>),
    String(String),
    Number(String),
    Bool(bool),
    Null,
}
