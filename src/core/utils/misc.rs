use serde_json::Value;

pub fn is_lts(lts: Value) -> bool {
    match lts {
        Value::String(_) => true,
        Value::Bool(b) => b,
        _ => false,
    }
}
