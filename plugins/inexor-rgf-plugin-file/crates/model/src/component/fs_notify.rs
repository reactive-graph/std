use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::NAMESPACE_FILE;

properties!(FsNotifyProperties, (TRIGGER, "trigger", false));

component_ty!(COMPONENT_FS_NOTIFY, NAMESPACE_FILE, COMPONENT_NAME_FS_NOTIFY, "fs_notify");

entity_model!(FsNotify, get trigger bool, set filename string);
