use crate::NAMESPACE_CONNECTOR;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(BufferProperties, (BUFFER_SIZE, "buffer_size", 10), (BUFFER, "buffer", Vec::<serde_json::Value>::new()));

component_ty!(COMPONENT_BUFFER, NAMESPACE_CONNECTOR, COMPONENT_NAME_BUFFER, "buffer");
