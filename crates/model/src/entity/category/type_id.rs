use lazy_static::lazy_static;

use crate::model::ComponentTypeId;
use crate::ENTITY_TYPE_NAME_CATEGORY;
use crate::NAMESPACE_TAXONOMY;

lazy_static! {
    pub static ref ENTITY_TYPE_CATEGORY: ComponentTypeId = ComponentTypeId::new_from_type(NAMESPACE_TAXONOMY, ENTITY_TYPE_NAME_CATEGORY);
}
