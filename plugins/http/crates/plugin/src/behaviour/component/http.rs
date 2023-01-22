use log::error;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;

use crate::model::*;
use crate::model_http::RequestProperties::METHOD;
use crate::model_http::RequestProperties::PAYLOAD;
use crate::model_http::RequestProperties::REQUEST_HEADERS;
use crate::model_http::ResponseProperties::RESPONSE_HEADERS;
use crate::model_http::ResponseProperties::STATUS;
use crate::model_http::UrlProperties::URL;
use crate::model_result::ResultObjectProperties::RESULT;
use crate::model_trigger::ActionProperties::TRIGGER;
use crate::reactive::*;

entity_behaviour!(Http, HttpFactory, HttpFsm, HttpBehaviourTransitions, HttpValidator);

behaviour_validator!(
    HttpValidator,
    ReactiveEntityInstance,
    METHOD.as_ref(),
    PAYLOAD.as_ref(),
    REQUEST_HEADERS.as_ref(),
    RESPONSE_HEADERS.as_ref(),
    RESULT.as_ref(),
    STATUS.as_ref(),
    URL.as_ref()
);

impl BehaviourInit<ReactiveEntityInstance> for HttpBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if self.reactive_instance.as_bool(TRIGGER).unwrap_or(false) {
            send_request(&self.reactive_instance);
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for HttpBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            send_request(&reactive_instance);
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for HttpBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for HttpBehaviourTransitions {}

fn send_request(reactive_instance: &Arc<ReactiveEntityInstance>) {
    let Some(method) = reactive_instance.as_string(METHOD) else {
        return;
    };
    let Some(url) = reactive_instance.as_string(URL) else {
        return;
    };
    let Some(request_headers) = reactive_instance.as_object(REQUEST_HEADERS) else {
        return;
    };
    let Some(payload) = reactive_instance.get(PAYLOAD) else {
        return;
    };
    let mut request = ureq::request(method.as_str(), url.as_str());
    for (request_header, value) in request_headers.into_iter() {
        if let Some(value) = value.as_str() {
            request = request.set(request_header.as_ref(), value);
        }
    }
    let result = request.send_json(payload);
    match result {
        Ok(response) => {
            reactive_instance.set(STATUS, json!(response.status()));
            let mut response_headers = json!({});
            for header_name in response.headers_names() {
                response_headers[header_name] = json!(response.header(header_name.as_str()));
            }
            reactive_instance.set(RESPONSE_HEADERS, response_headers);
            match response.into_json() {
                Ok(result) => {
                    reactive_instance.set(RESULT, result);
                }
                Err(e) => error!("Failed to parse response as JSON: {}", e.to_string()),
            }
        }
        Err(e) => {
            error!("Failed to send request: {}", e.to_string());
        }
    }
}
