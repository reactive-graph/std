use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_FILE;

properties!(FileProperties, (FILENAME, "filename", ""));

component_ty!(COMPONENT_FILE, NAMESPACE_FILE, COMPONENT_NAME_FILE, "file");

component_model!(
    File,
    set filename string,
);
