use log::error;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;

use crate::model::*;
use crate::model_http::HttpProperties::METHOD;
use crate::model_http::HttpProperties::PAYLOAD;
use crate::model_http::HttpProperties::REQUEST_HEADERS;
use crate::model_http::HttpProperties::RESPONSE_HEADERS;
use crate::model_http::HttpProperties::RESULT;
use crate::model_http::HttpProperties::STATUS;
use crate::model_http::HttpProperties::URL;
use crate::model_logical::ActionProperties::TRIGGER;
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

// use std::convert::AsRef;
// use std::sync::Arc;
//
// use log::error;
// use serde_json::json;
//
// use crate::behaviour::component::HttpProperties;
// use crate::model::PropertyInstanceGetter;
// use crate::model::PropertyInstanceSetter;
// use crate::model::ReactiveEntityInstance;
// use crate::reactive::entity::Disconnectable;
// use crate::reactive::BehaviourCreationError;
//
// pub const HTTP: &str = "http";
//
// pub struct Http {
//     pub entity: Arc<ReactiveEntityInstance>,
//
//     pub handle_id: u128,
// }
//
// impl Http {
//     pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<Http, BehaviourCreationError> {
//         let entity = e.clone();
//         let handle_id = e.id.as_u128();
//         let trigger = e.properties.get(HttpProperties::TRIGGER.as_ref()).ok_or(BehaviourCreationError)?;
//         trigger.stream.read().unwrap().observe_with_handle(
//             move |trigger| {
//                 if !trigger.is_boolean() || !trigger.as_bool().unwrap_or(false) {
//                     return;
//                 }
//                 let method = match entity.get(HttpProperties::METHOD).and_then(|v| v.as_str().map(String::from)) {
//                     Some(method) => method,
//                     None => return,
//                 };
//                 let url = match entity.get(HttpProperties::URL).and_then(|v| v.as_str().map(String::from)) {
//                     Some(url) => url,
//                     None => return,
//                 };
//                 let request_headers = match entity.get(HttpProperties::REQUEST_HEADERS).and_then(|v| v.as_object().cloned()) {
//                     Some(request_headers) => request_headers,
//                     None => return,
//                 };
//                 let payload = match entity.get(HttpProperties::PAYLOAD) {
//                     Some(payload) => payload,
//                     None => return,
//                 };
//
//                 let mut request = ureq::request(method.as_str(), url.as_str());
//                 for (request_header, value) in request_headers.into_iter() {
//                     if let Some(value) = value.as_str() {
//                         request = request.set(request_header.as_ref(), value);
//                     }
//                 }
//                 let result = request.send_json(payload);
//                 match result {
//                     Ok(response) => {
//                         entity.set(HttpProperties::STATUS.as_ref(), json!(response.status()));
//                         let mut response_headers = json!({});
//                         for header_name in response.headers_names() {
//                             response_headers[header_name] = json!(response.header(header_name.as_str()));
//                         }
//                         entity.set(HttpProperties::RESPONSE_HEADERS.as_ref(), response_headers);
//                         match response.into_json() {
//                             Ok(result) => {
//                                 entity.set(HttpProperties::RESULT.as_ref(), result);
//                             }
//                             Err(e) => error!("Failed to parse response as JSON: {}", e.to_string()),
//                         }
//                     }
//                     Err(e) => {
//                         error!("Failed to send request: {}", e.to_string());
//                     }
//                 }
//             },
//             handle_id,
//         );
//         // Initially send HTTP request if trigger is initially true
//         if trigger.get().as_bool().unwrap_or(false) {
//             trigger.tick();
//         }
//         Ok(Http { entity: e.clone(), handle_id })
//     }
// }
//
// impl Disconnectable for Http {
//     fn disconnect(&self) {
//         if let Some(property) = self.entity.properties.get(HttpProperties::TRIGGER.as_ref()) {
//             property.stream.read().unwrap().remove(self.handle_id);
//         }
//     }
// }
//
// impl Drop for Http {
//     fn drop(&mut self) {
//         self.disconnect();
//     }
// }
