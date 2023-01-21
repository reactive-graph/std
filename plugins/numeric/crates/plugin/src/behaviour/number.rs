use num_traits::FromPrimitive;
use serde_json::Value;

pub fn as_f64(v: Value) -> Option<f64> {
    if let Some(v) = v.as_f64() {
        return Some(v);
    }
    if let Some(v) = v.as_i64() {
        return f64::from_i64(v);
    }
    if let Some(v) = v.as_u64() {
        return f64::from_u64(v);
    }
    None
}

pub fn as_i64(v: Value) -> Option<i64> {
    if let Some(v) = v.as_i64() {
        return Some(v);
    }
    if let Some(v) = v.as_u64() {
        return i64::from_u64(v);
    }
    if let Some(v) = v.as_f64() {
        return i64::from_f64(v);
    }
    None
}

pub fn as_u64(v: Value) -> Option<u64> {
    if let Some(v) = v.as_u64() {
        return Some(v);
    }
    if let Some(v) = v.as_i64() {
        return u64::from_i64(v);
    }
    if let Some(v) = v.as_f64() {
        return u64::from_f64(v);
    }
    None
}
