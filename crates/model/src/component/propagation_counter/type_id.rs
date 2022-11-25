use lazy_static::lazy_static;

use crate::model::BehaviourTypeId;
use crate::model::ComponentTypeId;
use crate::COMPONENT_NAME_PROPAGATION_COUNTER;
use crate::NAMESPACE_CONNECTOR;

lazy_static! {
    pub static ref COMPONENT_PROPAGATION_COUNTER: ComponentTypeId = ComponentTypeId::new_from_type(NAMESPACE_CONNECTOR, COMPONENT_NAME_PROPAGATION_COUNTER);
    pub static ref BEHAVIOUR_PROPAGATION_COUNTER: BehaviourTypeId = BehaviourTypeId::new_from_type(NAMESPACE_CONNECTOR, COMPONENT_NAME_PROPAGATION_COUNTER);
}
