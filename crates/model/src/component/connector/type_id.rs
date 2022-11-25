use lazy_static::lazy_static;

use crate::model::ComponentTypeId;
use crate::COMPONENT_NAME_CONNECTOR;
use crate::NAMESPACE_CONNECTOR;

lazy_static! {
    pub static ref COMPONENT_CONNECTOR: ComponentTypeId = ComponentTypeId::new_from_type(NAMESPACE_CONNECTOR, COMPONENT_NAME_CONNECTOR);
}
