use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_CONNECTOR;

properties!(BufferProperties, (BUFFER_SIZE, "buffer_size", 10), (BUFFER, "buffer", Vec::<serde_json::Value>::new()));

component_ty!(COMPONENT_BUFFER, NAMESPACE_CONNECTOR, COMPONENT_NAME_BUFFER, "buffer");
