use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::component_behaviour_ty;
use reactive_graph_graph::component_ty;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;

use crate::NAMESPACE_FILE;

component_ty!(COMPONENT_FS_NOTIFY, NAMESPACE_FILE, COMPONENT_NAME_FS_NOTIFY, "fs_notify");
behaviour_ty!(BEHAVIOUR_FS_NOTIFY, NAMESPACE_FILE, BEHAVIOUR_NAME_FS_NOTIFY, "fs_notify");
component_behaviour_ty!(COMPONENT_BEHAVIOUR_FS_NOTIFY, COMPONENT_FS_NOTIFY, BEHAVIOUR_FS_NOTIFY);

entity_model!(FsNotify, get trigger bool, set filename string);
impl Action for FsNotify {}
