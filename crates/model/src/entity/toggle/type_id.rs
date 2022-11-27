use lazy_static::lazy_static;

use crate::model::EntityTypeId;
use crate::ENTITY_TYPE_NAME_TOGGLE;
use crate::NAMESPACE_LOGICAL;

lazy_static! {
    pub static ref ENTITY_TYPE_TOGGLE: EntityTypeId = EntityTypeId::new_from_type(NAMESPACE_LOGICAL, ENTITY_TYPE_NAME_TOGGLE);
}
