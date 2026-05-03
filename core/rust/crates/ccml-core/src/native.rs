use std::collections::BTreeMap;

use crate::ast::AstNode;

#[derive(Debug, Clone, PartialEq)]
pub enum NativeValue {
    Object(BTreeMap<String, NativeValue>),
    Array(Vec<NativeValue>),
    String(String),
    Number(NativeNumber),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeNumber {
    pub raw: String,
}

impl NativeNumber {
    pub fn as_i128(&self) -> Option<i128> {
        self.raw.parse::<i128>().ok()
    }

    pub fn as_f64(&self) -> Option<f64> {
        self.raw.parse::<f64>().ok()
    }
}

pub fn ast_to_native(node: &AstNode) -> NativeValue {
    match node {
        AstNode::Object(obj) => NativeValue::Object(
            obj.iter()
                .map(|(k, v)| (k.clone(), ast_to_native(v)))
                .collect::<BTreeMap<_, _>>(),
        ),
        AstNode::Array(items) => NativeValue::Array(items.iter().map(ast_to_native).collect()),
        AstNode::String(s) => NativeValue::String(s.clone()),
        AstNode::Number(n) => NativeValue::Number(NativeNumber { raw: n.clone() }),
        AstNode::Bool(b) => NativeValue::Bool(*b),
        AstNode::Null => NativeValue::Null,
    }
}
