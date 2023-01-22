use std::path::PathBuf;

use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_FILE;

properties!(FileProperties, (FILENAME, "filename", ""));

component_ty!(COMPONENT_FILE, NAMESPACE_FILE, COMPONENT_NAME_FILE, "file");

component_model!(
    File,
    data filename string,
);

pub trait FilePath: File {
    fn get_path(&self) -> Option<PathBuf> {
        self.get_filename().map(PathBuf::from)
    }
}
