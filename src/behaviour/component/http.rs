use std::convert::AsRef;
use std::sync::Arc;

use log::error;
use serde_json::json;

use crate::behaviour::component::HttpProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const HTTP: &str = "http";

pub struct Http {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl Http {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<Http, BehaviourCreationError> {
        let entity = e.clone();
        let handle_id = e.id.as_u128();
        let trigger = e.properties.get(HttpProperties::TRIGGER.as_ref()).ok_or(BehaviourCreationError)?;
        trigger.stream.read().unwrap().observe_with_handle(
            move |trigger| {
                if !trigger.is_boolean() || !trigger.as_bool().unwrap_or(false) {
                    return;
                }
                let method = match entity.get(HttpProperties::METHOD).and_then(|v| v.as_str().map(String::from)) {
                    Some(method) => method,
                    None => return,
                };
                let url = match entity.get(HttpProperties::URL).and_then(|v| v.as_str().map(String::from)) {
                    Some(url) => url,
                    None => return,
                };
                let request_headers = match entity.get(HttpProperties::REQUEST_HEADERS).and_then(|v| v.as_object().cloned()) {
                    Some(request_headers) => request_headers,
                    None => return,
                };
                let payload = match entity.get(HttpProperties::PAYLOAD) {
                    Some(payload) => payload,
                    None => return,
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
                        entity.set(HttpProperties::STATUS.as_ref(), json!(response.status()));
                        let mut response_headers = json!({});
                        for header_name in response.headers_names() {
                            response_headers[header_name] = json!(response.header(header_name.as_str()));
                        }
                        entity.set(HttpProperties::RESPONSE_HEADERS.as_ref(), response_headers);
                        match response.into_json() {
                            Ok(result) => {
                                entity.set(HttpProperties::RESULT.as_ref(), result);
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
        // Initially send HTTP request if trigger is initially true
        if trigger.get().as_bool().unwrap_or(false) {
            trigger.tick();
        }
        Ok(Http { entity: e.clone(), handle_id })
    }
}

impl Disconnectable for Http {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(HttpProperties::TRIGGER.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

impl Drop for Http {
    fn drop(&mut self) {
        self.disconnect();
    }
}
