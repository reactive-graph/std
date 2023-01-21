use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_RANDOM;

properties!(RangeF64Properties, (LOW, "low", 0.0), (HIGH, "high", 1.0));
properties!(RangeI64Properties, (LOW, "low", -100), (HIGH, "high", 100));
properties!(RangeU64Properties, (LOW, "low", 0), (HIGH, "high", 100));

component_ty!(COMPONENT_RANGE, NAMESPACE_RANDOM, COMPONENT_NAME_RANGE, "range");

component_model!(
    RangeF64,
    set low f64,
    set high f64,
);

component_model!(
    RangeI64,
    set low i64,
    set high i64,
);

component_model!(
    RangeU64,
    set low u64,
    set high u64,
);
