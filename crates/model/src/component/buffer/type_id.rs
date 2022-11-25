use lazy_static::lazy_static;

use crate::model::ComponentTypeId;
use crate::COMPONENT_NAME_BUFFER;
use crate::NAMESPACE_CONNECTOR;

lazy_static! {
    pub static ref COMPONENT_BUFFER: ComponentTypeId = ComponentTypeId::new_from_type(NAMESPACE_CONNECTOR, COMPONENT_NAME_BUFFER);
}
