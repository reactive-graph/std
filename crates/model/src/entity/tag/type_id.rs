use lazy_static::lazy_static;

use crate::model::ComponentTypeId;
use crate::ENTITY_TYPE_NAME_TAG;
use crate::NAMESPACE_TAXONOMY;

lazy_static! {
    pub static ref ENTITY_TYPE_TAG: ComponentTypeId = ComponentTypeId::new_from_type(NAMESPACE_TAXONOMY, ENTITY_TYPE_NAME_TAG);
}
