use lazy_static::lazy_static;

use crate::model::ComponentTypeId;
use crate::COMPONENT_NAME_STATE;
use crate::NAMESPACE_STATE;

lazy_static! {
    pub static ref COMPONENT_STATE: ComponentTypeId = ComponentTypeId::new_from_type(NAMESPACE_STATE, COMPONENT_NAME_STATE);
}
