use crate::model::properties;

properties!(BufferProperties, (BUFFER_SIZE, "buffer_size", 10), (BUFFER, "buffer", Vec::<serde_json::Value>::new()));
