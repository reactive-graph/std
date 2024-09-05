use data_url::DataUrl;

use crate::NAMESPACE_BINARY;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(BinaryDataProperties, (DATA_URL, "data_url", ""));

component_ty!(COMPONENT_BINARY_DATA, NAMESPACE_BINARY, COMPONENT_NAME_BINARY_DATA, "binary_data");
behaviour_ty!(BEHAVIOUR_BINARY_DATA, NAMESPACE_BINARY, BEHAVIOUR_NAME_BINARY_DATA, "binary_data");

component_model!(
    BinaryData,
    data data_url string
);

pub trait BinaryDataUrl: BinaryData {
    fn mime_type(&self) -> Option<String> {
        if let Some(data_url) = self.get_data_url() {
            if let Ok(data_url) = DataUrl::process(&data_url) {
                return Some(data_url.mime_type().type_.clone());
            }
        }
        None
    }

    fn subtype(&self) -> Option<String> {
        if let Some(data_url) = self.get_data_url() {
            if let Ok(data_url) = DataUrl::process(&data_url) {
                return Some(data_url.mime_type().subtype.clone());
            }
        }
        None
    }

    fn charset(&self) -> Option<String> {
        if let Some(data_url) = self.get_data_url() {
            if let Ok(data_url) = DataUrl::process(&data_url) {
                return data_url.mime_type().get_parameter("charset").map(String::from);
            }
        }
        None
    }
}
