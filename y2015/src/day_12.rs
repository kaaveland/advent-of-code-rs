// I know how to make a json-parser, I just refuse to do it.

use serde_json::{from_str, Value};

pub fn part_1(s: &str) -> anyhow::Result<String> {
    fn visit(v: &Value) -> i64 {
        match v {
            Value::Number(n) => n.as_i64().unwrap_or(0),
            Value::Array(v) => v.iter().map(visit).sum(),
            Value::Object(kv) => kv.values().map(visit).sum(),
            _ => 0,
        }
    }
    Ok(visit(&from_str(s)?).to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    fn visit(v: &Value) -> i64 {
        match v {
            Value::Number(n) => n.as_i64().unwrap_or(0),
            Value::Array(v) => v.iter().map(visit).sum(),
            Value::Object(kv) => {
                if kv.values().any(|v| v == &Value::String("red".to_string())) {
                    0
                } else {
                    kv.values().map(visit).sum()
                }
            }
            _ => 0,
        }
    }
    Ok(visit(&from_str(s)?).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_examples() {}
}
