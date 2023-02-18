use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::NAMESPACE_DATE_TIME;

properties!(HourProperties, (HOUR, "hour_of_day", 0));

entity_ty!(ENTITY_TYPE_HOUR, NAMESPACE_DATE_TIME, ENTITY_TYPE_NAME_HOUR, "hour");

entity_model!(Hour, get hour_of_day u64);
