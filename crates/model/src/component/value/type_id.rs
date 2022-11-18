use lazy_static::lazy_static;

use crate::model::ComponentTypeId;
use crate::COMPONENT_NAME_VALUE;
use crate::NAMESPACE_VALUE;

lazy_static! {
    pub static ref COMPONENT_VALUE: ComponentTypeId = ComponentTypeId::new_from_type(NAMESPACE_VALUE, COMPONENT_NAME_VALUE);
}
