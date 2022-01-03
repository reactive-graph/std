use inexor_rgf_core_model::PropertyInstanceSetter;
use std::convert::AsRef;
use std::sync::Arc;

use crate::behaviour::entity::jsonrpc_properties::JsonRpcProperties;
use crate::reactive::BehaviourCreationError;
use log::error;
use serde_json::{json, Value};

use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;

pub const JSONRPC: &'static str = "jsonrpc";

pub struct JsonRpc {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl JsonRpc {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<JsonRpc, BehaviourCreationError> {
        let url = e.properties.get(JsonRpcProperties::URL.as_ref());
        if url.is_none() {
            error!("Missing property {}", JsonRpcProperties::URL.as_ref());
            return Err(BehaviourCreationError.into());
        }

        let method = e.properties.get(JsonRpcProperties::METHOD.as_ref());
        if method.is_none() {
            error!("Missing property {}", JsonRpcProperties::METHOD.as_ref());
            return Err(BehaviourCreationError.into());
        }

        let params = e.properties.get(JsonRpcProperties::PARAMS.as_ref());
        if params.is_none() {
            error!("Missing property {}", JsonRpcProperties::PARAMS.as_ref());
            return Err(BehaviourCreationError.into());
        }

        let result = e.properties.get(JsonRpcProperties::RESULT.as_ref());
        if result.is_none() {
            error!("Missing property {}", JsonRpcProperties::RESULT.as_ref());
            return Err(BehaviourCreationError.into());
        }

        let error = e.properties.get(JsonRpcProperties::ERROR.as_ref());
        if error.is_none() {
            error!("Missing property {}", JsonRpcProperties::ERROR.as_ref());
            return Err(BehaviourCreationError.into());
        }

        let entity = e.clone();
        let handle_id = e.properties.get(JsonRpcProperties::PARAMS.as_ref()).unwrap().id.as_u128();
        e.properties
            .get(JsonRpcProperties::PARAMS.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |v: &Value| {
                    if !v.is_object() {
                        // Invalid type
                        return;
                    }

                    let url = entity
                        .properties
                        .get(JsonRpcProperties::URL.as_ref())
                        .unwrap()
                        .as_string()
                        .unwrap_or(JsonRpcProperties::URL.default_value().to_string());
                    if url.is_empty() {
                        // Invalid URL
                        return;
                    }

                    let method = entity
                        .properties
                        .get(JsonRpcProperties::METHOD.as_ref())
                        .unwrap()
                        .as_string()
                        .unwrap_or(JsonRpcProperties::METHOD.default_value().to_string());
                    if method.is_empty() {
                        // Invalid JSON RPC method
                        return;
                    }

                    let params = entity.properties.get(JsonRpcProperties::PARAMS.as_ref()).unwrap().get();
                    if !params.is_object() && !params.is_array() {
                        // params must be either object or array
                        return;
                    }

                    let jsonrpc_version = entity
                        .properties
                        .get(JsonRpcProperties::JSONRPC_VERSION.as_ref())
                        .unwrap()
                        .as_string()
                        .unwrap_or(JsonRpcProperties::JSONRPC_VERSION.default_value().to_string());

                    let payload = json!({
                        "jsonrpc": jsonrpc_version,
                        "method": method,
                        "params": params,
                        "id": 1 as u32
                    });

                    let request = ureq::post(url.as_str()).set("content-type", "application/json");

                    let result = request.send_json(payload);
                    match result {
                        Ok(response) => {
                            match response.into_json() {
                                Ok(response_payload) => {
                                    let json_rpc_response: Value = response_payload;
                                    // let result = json_rpc_response.get("result");
                                    match json_rpc_response.get(JsonRpcProperties::RESULT.as_ref()) {
                                        Some(result) => {
                                            entity.set(JsonRpcProperties::RESULT.as_ref(), result.clone());
                                            entity.set(JsonRpcProperties::ERROR.as_ref(), json!({}));
                                        }
                                        None => match json_rpc_response.get(JsonRpcProperties::ERROR.as_ref()) {
                                            Some(error) => {
                                                entity.set(JsonRpcProperties::ERROR.as_ref(), error.clone());
                                                entity.set(JsonRpcProperties::RESULT.as_ref(), json!({}));
                                            }
                                            _ => {
                                                // Invalid JSON-RPC response: The payload missing both: result and error
                                            }
                                        },
                                    }
                                }
                                Err(e) => error!("Failed to parse response as JSON: {}", e.to_string()),
                            }
                        }
                        Err(e) => {
                            error!("Failed to send request: {}", e.to_string());
                        }
                    }
                },
                handle_id,
            );

        Ok(JsonRpc { entity: e.clone(), handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for JsonRpc {
    fn disconnect(&self) {
        match self.entity.properties.get(JsonRpcProperties::PARAMS.as_ref()) {
            Some(property) => {
                property.stream.read().unwrap().remove(self.handle_id);
            }
            _ => {}
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for JsonRpc {
    fn drop(&mut self) {
        self.disconnect();
    }
}
