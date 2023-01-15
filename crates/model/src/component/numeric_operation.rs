use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::NAMESPACE_NUMERIC;

properties!(NumericOperationProperties, (LHS, "lhs", 0.0), (RESULT, "result", 0.0));

component_ty!(COMPONENT_NUMERIC_OPERATION, NAMESPACE_NUMERIC, COMPONENT_NAME_NUMERIC_OPERATION, "numeric_operation");

entity_model!(NumericOperationF64, get result f64, set lhs f64);
entity_model!(NumericOperationI64, get result i64, set lhs i64);
