use std::path::PathBuf;

use crate::NAMESPACE_FILE;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

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
