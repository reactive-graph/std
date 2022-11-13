use lazy_static::lazy_static;

use crate::model::ComponentTypeId;
use crate::COMPONENT_NAME_NAMED;
use crate::NAMESPACE_BASE;

lazy_static! {
    pub static ref COMPONENT_NAMED: ComponentTypeId = ComponentTypeId::new_from_type(NAMESPACE_BASE, COMPONENT_NAME_NAMED);
}
