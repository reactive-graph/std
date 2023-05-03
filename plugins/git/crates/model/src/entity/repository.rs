use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_base::Describable;
use crate::model_base::Named;
use crate::model_file::File;
use crate::model_file::FilePath;
use crate::model_http::ParsedUrl;
use crate::model_http::Url;
use crate::model_runtime::Action;
use crate::ComponentRepository;
use crate::GitRepository;
use crate::TransferProgress;
use crate::NAMESPACE_GIT;

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
