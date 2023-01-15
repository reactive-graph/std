// TODO: remove this file and use model_file::FileProperties

use crate::model::component_ty;
use crate::model::properties;

pub const NAMESPACE_FILE: &str = "file";

properties!(FileProperties, (FILENAME, "filename", ""));

component_ty!(COMPONENT_FILE, NAMESPACE_FILE, COMPONENT_NAME_FILE, "file");
