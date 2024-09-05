use crate::ComponentRepository;
use crate::GitRepository;
use crate::TransferProgress;
use crate::NAMESPACE_GIT;
use reactive_graph_graph::entity_ty;
use reactive_graph_model_base::Describable;
use reactive_graph_model_base::Named;
use reactive_graph_model_file::File;
use reactive_graph_model_file::FilePath;
use reactive_graph_model_http::ParsedUrl;
use reactive_graph_model_http::Url;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_REPOSITORY, NAMESPACE_GIT, ENTITY_TYPE_NAME_REPOSITORY, "repository");

entity_model!(Repository);
impl ComponentRepository for Repository {}
impl TransferProgress for Repository {}
impl GitRepository for Repository {}
impl File for Repository {}
impl FilePath for Repository {}
impl Url for Repository {}
impl ParsedUrl for Repository {}
impl Action for Repository {}
impl Named for Repository {}
impl Describable for Repository {}
