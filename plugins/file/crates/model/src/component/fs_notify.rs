use crate::NAMESPACE_FILE;
use inexor_rgf_behaviour_api::behaviour_ty;
use inexor_rgf_behaviour_api::component_behaviour_ty;
use inexor_rgf_graph::component_ty;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_reactive_api::entity_model;

component_ty!(COMPONENT_FS_NOTIFY, NAMESPACE_FILE, COMPONENT_NAME_FS_NOTIFY, "fs_notify");
behaviour_ty!(BEHAVIOUR_FS_NOTIFY, NAMESPACE_FILE, BEHAVIOUR_NAME_FS_NOTIFY, "fs_notify");
component_behaviour_ty!(COMPONENT_BEHAVIOUR_FS_NOTIFY, COMPONENT_FS_NOTIFY, BEHAVIOUR_FS_NOTIFY);

entity_model!(FsNotify, get trigger bool, set filename string);
impl Action for FsNotify {}
