use crate::NAMESPACE_CONNECTOR;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(BufferProperties, (BUFFER_SIZE, "buffer_size", 10), (BUFFER, "buffer", Vec::<serde_json::Value>::new()));

component_ty!(COMPONENT_BUFFER, NAMESPACE_CONNECTOR, COMPONENT_NAME_BUFFER, "buffer");
