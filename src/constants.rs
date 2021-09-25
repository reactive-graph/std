use std::collections::HashMap;
use uuid::Uuid;

// TODO: define a unique namespace for Inexor RGF
pub static NAMESPACE_NUMERIC: Uuid = Uuid::from_u128(0x6ba7b8109dad11d180b400c04fd430c8);

lazy_static! {
    pub static ref NUMERIC_CONSTANTS: HashMap<&'static str, f64> = vec![
        ("E", std::f64::consts::E),
        ("FRAC_1_PI", std::f64::consts::FRAC_1_PI),
        ("FRAC_1_SQRT_2", std::f64::consts::FRAC_1_SQRT_2),
        ("FRAC_2_PI", std::f64::consts::FRAC_2_PI),
        ("FRAC_2_SQRT_PI", std::f64::consts::FRAC_2_SQRT_PI),
        ("FRAC_PI_2", std::f64::consts::FRAC_PI_2),
        ("FRAC_PI_3", std::f64::consts::FRAC_PI_3),
        ("FRAC_PI_4", std::f64::consts::FRAC_PI_4),
        ("FRAC_PI_6", std::f64::consts::FRAC_PI_6),
        ("FRAC_PI_8", std::f64::consts::FRAC_PI_8),
        ("LN_2", std::f64::consts::LN_2),
        ("LN_10", std::f64::consts::LN_10),
        ("LOG2_10", std::f64::consts::LOG2_10),
        ("LOG2_E", std::f64::consts::LOG2_E),
        ("LOG10_2", std::f64::consts::LOG10_2),
        ("LOG10_E", std::f64::consts::LOG10_E),
        ("PI", std::f64::consts::PI),
        ("SQRT_2", std::f64::consts::SQRT_2),
        ("TAU", std::f64::consts::TAU),
    ]
    .into_iter()
    .collect();
}
