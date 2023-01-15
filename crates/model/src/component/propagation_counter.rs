use crate::model::behaviour_ty;
use crate::model::component_behaviour_ty;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_CONNECTOR;

properties!(PropagationCounterProperties, (PROPAGATION_COUNT, "propagation_count", 0));

component_ty!(COMPONENT_PROPAGATION_COUNTER, NAMESPACE_CONNECTOR, COMPONENT_NAME_PROPAGATION_COUNTER, "propagation_counter");
behaviour_ty!(BEHAVIOUR_PROPAGATION_COUNTER, NAMESPACE_CONNECTOR, BEHAVIOUR_NAME_PROPAGATION_COUNTER, "propagation_counter");
component_behaviour_ty!(COMPONENT_BEHAVIOUR_PROPAGATION_COUNTER, COMPONENT_PROPAGATION_COUNTER, BEHAVIOUR_PROPAGATION_COUNTER);
