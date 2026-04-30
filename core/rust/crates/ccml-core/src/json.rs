use crate::ast::AstNode;

pub fn to_json_string(node: &AstNode, pretty: bool) -> String {
    if pretty {
        stringify_pretty(node, 0)
    } else {
        stringify_compact(node)
    }
}

fn stringify_compact(node: &AstNode) -> String {
    match node {
        AstNode::Object(map) => {
            let mut parts = Vec::with_capacity(map.len());
            for (k, v) in map {
                parts.push(format!("\"{}\":{}", escape(k), stringify_compact(v)));
            }
            format!("{{{}}}", parts.join(","))
        }
        AstNode::Array(items) => {
            let parts: Vec<String> = items.iter().map(stringify_compact).collect();
            format!("[{}]", parts.join(","))
        }
        AstNode::String(s) => format!("\"{}\"", escape(s)),
        AstNode::Number(n) => n.clone(),
        AstNode::Bool(b) => b.to_string(),
        AstNode::Null => "null".to_string(),
    }
}

fn stringify_pretty(node: &AstNode, depth: usize) -> String {
    match node {
        AstNode::Object(map) => {
            if map.is_empty() {
                return "{}".to_string();
            }
            let indent = "  ".repeat(depth);
            let child_indent = "  ".repeat(depth + 1);
            let mut parts = Vec::with_capacity(map.len());
            for (k, v) in map {
                parts.push(format!(
                    "{}\"{}\": {}",
                    child_indent,
                    escape(k),
                    stringify_pretty(v, depth + 1)
                ));
            }
            format!("{{\n{}\n{}}}", parts.join(",\n"), indent)
        }
        AstNode::Array(items) => {
            if items.is_empty() {
                return "[]".to_string();
            }
            let indent = "  ".repeat(depth);
            let child_indent = "  ".repeat(depth + 1);
            let parts: Vec<String> = items
                .iter()
                .map(|v| format!("{}{}", child_indent, stringify_pretty(v, depth + 1)))
                .collect();
            format!("[\n{}\n{}]", parts.join(",\n"), indent)
        }
        _ => stringify_compact(node),
    }
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out
}
