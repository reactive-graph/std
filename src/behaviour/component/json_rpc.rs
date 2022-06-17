use std::convert::AsRef;
use std::sync::Arc;

use log::error;
use serde_json::json;
use serde_json::Value;

use crate::behaviour::component::json_rpc_properties::JsonRpcProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const JSON_RPC: &str = "json_rpc";

pub struct JsonRpc {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl JsonRpc {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<JsonRpc, BehaviourCreationError> {
        let entity = e.clone();
        let handle_id = e.id.as_u128();
        let trigger = e.properties.get(JsonRpcProperties::TRIGGER.as_ref()).ok_or(BehaviourCreationError)?;
        trigger.stream.read().unwrap().observe_with_handle(
            move |trigger| {
                if !trigger.is_boolean() || !trigger.as_bool().unwrap_or(false) {
                    return;
                }
                let method = match entity.get(JsonRpcProperties::METHOD).and_then(|v| v.as_str().map(String::from)) {
                    Some(method) => method,
                    None => return,
                };
                let url = match entity.get(JsonRpcProperties::URL).and_then(|v| v.as_str().map(String::from)) {
                    Some(url) => url,
                    None => return,
                };
                let params = match entity.get(JsonRpcProperties::PARAMS) {
                    Some(params) => params,
                    None => return,
                };
                if !params.is_object() && !params.is_array() {
                    // params must be either object or array
                    return;
                }
                let json_rpc_version = entity
                    .properties
                    .get(JsonRpcProperties::JSON_RPC_VERSION.as_ref())
                    .unwrap()
                    .as_string()
                    .unwrap_or_else(|| JsonRpcProperties::JSON_RPC_VERSION.default_value().to_string());

                let payload = json!({
                    "jsonrpc": json_rpc_version,
                    "method": method,
                    "params": params,
                    "id": 1 as u32
                });

                let request = ureq::post(url.as_str()).set("content-type", "application/json");

                let result = request.send_json(payload);
                match result {
                    Ok(response) => match response.into_json() {
                        Ok(response_payload) => {
                            let json_rpc_response: Value = response_payload;
                            match json_rpc_response.get(JsonRpcProperties::RESULT.as_ref()) {
                                Some(result) => {
                                    entity.set(JsonRpcProperties::RESULT.as_ref(), result.clone());
                                    entity.set(JsonRpcProperties::ERROR.as_ref(), json!({}));
                                }
                                None => {
                                    if let Some(error) = json_rpc_response.get(JsonRpcProperties::ERROR.as_ref()) {
                                        entity.set(JsonRpcProperties::ERROR.as_ref(), error.clone());
                                        entity.set(JsonRpcProperties::RESULT.as_ref(), json!({}));
                                    }
                                }
                            }
                        }
                        Err(e) => error!("Failed to parse response as JSON: {}", e.to_string()),
                    },
                    Err(e) => {
                        error!("Failed to send request: {}", e.to_string());
                    }
                }
            },
            handle_id,
        );
        // Initially send HTTP request if trigger is initially true
        if trigger.get().as_bool().unwrap_or(false) {
            trigger.tick();
        }
        Ok(JsonRpc { entity: e.clone(), handle_id })
    }
}

impl Disconnectable for JsonRpc {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(JsonRpcProperties::TRIGGER.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

impl Drop for JsonRpc {
    fn drop(&mut self) {
        self.disconnect();
    }
}
