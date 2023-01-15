use log::info;
use num_traits::FromPrimitive;
use serde_json::Value;

pub fn as_f64(v: Value) -> Option<f64> {
    info!("{}", v);
    if let Some(v) = v.as_f64() {
        info!("f64 {} -> f64", v);
        return Some(v);
    }
    if let Some(v) = v.as_i64() {
        info!("i64 {} -> f64 {:?}", v, f64::from_i64(v));
        return f64::from_i64(v);
    }
    if let Some(v) = v.as_u64() {
        info!("u64 {} -> f64 {:?}", v, f64::from_u64(v));
        return f64::from_u64(v);
    }
    None
}

pub fn as_i64(v: Value) -> Option<i64> {
    info!("{}", v);
    if let Some(v) = v.as_i64() {
        info!("i64 {} -> i64", v);
        return Some(v);
    }
    if let Some(v) = v.as_u64() {
        info!("u64 {} -> i64 {:?}", v, i64::from_u64(v));
        return i64::from_u64(v);
    }
    if let Some(v) = v.as_f64() {
        info!("f64 {} -> i64 {:?}", v, i64::from_f64(v));
        return i64::from_f64(v);
    }
    None
}

pub fn as_u64(v: Value) -> Option<u64> {
    info!("{}", v);
    if let Some(v) = v.as_u64() {
        info!("u64 {} -> u64", v);
        return Some(v);
    }
    if let Some(v) = v.as_i64() {
        info!("i64 {} -> u64 {:?}", v, u64::from_i64(v));
        return u64::from_i64(v);
    }
    if let Some(v) = v.as_f64() {
        info!("f64 {} -> u64 {:?}", v, u64::from_f64(v));
        return u64::from_f64(v);
    }
    None
}
