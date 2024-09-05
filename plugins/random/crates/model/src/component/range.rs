use crate::NAMESPACE_RANDOM;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

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
