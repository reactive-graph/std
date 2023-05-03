use log::error;
use serde_json::json;
use serde_json::Value;

use crate::model::*;
use crate::model_http::JsonRpcProperties::ERROR;
use crate::model_http::JsonRpcProperties::JSON_RPC_VERSION;
use crate::model_http::JsonRpcProperties::METHOD;
use crate::model_http::JsonRpcProperties::PARAMS;
use crate::model_http::UrlProperties::URL;
use crate::model_result::ResultObjectProperties::RESULT;
use crate::model_runtime::ActionProperties::TRIGGER;
use crate::reactive::*;

entity_behaviour!(JsonRpc, JsonRpcFactory, JsonRpcFsm, JsonRpcBehaviourTransitions, JsonRpcValidator);

behaviour_validator!(
    JsonRpcValidator,
    ReactiveEntityInstance,
    METHOD.as_ref(),
    JSON_RPC_VERSION.as_ref(),
    PARAMS.as_ref(),
    ERROR.as_ref(),
    RESULT.as_ref(),
    URL.as_ref()
);

impl BehaviourInit<ReactiveEntityInstance> for JsonRpcBehaviourTransitions {}

impl BehaviourConnect<ReactiveEntityInstance> for JsonRpcBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            let Some(method) = reactive_instance.as_string(METHOD) else {
                return;
            };
            let Some(url) = reactive_instance.as_string(URL) else {
                return;
            };
            let Some(params) = reactive_instance.get(PARAMS) else {
                return;
            };
            if !params.is_object() && !params.is_array() {
                // params must be either object or array
                return;
            }
            let json_rpc_version = reactive_instance
                .as_string(JSON_RPC_VERSION)
                .unwrap_or_else(|| JSON_RPC_VERSION.default_value().to_string());

            // TODO: increase ID (new property)
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
                        match json_rpc_response.get(RESULT.as_ref()) {
                            Some(result) => {
                                reactive_instance.set(RESULT, result.clone());
                                reactive_instance.set(ERROR, json!({}));
                            }
                            None => {
                                if let Some(error) = json_rpc_response.get(ERROR.as_ref()) {
                                    reactive_instance.set(ERROR, error.clone());
                                    reactive_instance.set(RESULT, json!({}));
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
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for JsonRpcBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for JsonRpcBehaviourTransitions {}
