use lazy_static::lazy_static;

use crate::model::EntityTypeId;
use crate::ENTITY_TYPE_NAME_IF_THEN_ELSE;
use crate::NAMESPACE_LOGICAL;

lazy_static! {
    pub static ref ENTITY_TYPE_IF_THEN_ELSE: EntityTypeId = EntityTypeId::new_from_type(NAMESPACE_LOGICAL, ENTITY_TYPE_NAME_IF_THEN_ELSE);
}
