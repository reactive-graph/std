use inexor_rgf_core_model::PropertyInstanceSetter;
use std::convert::AsRef;
use std::sync::Arc;

use crate::reactive::BehaviourCreationError;
use log::error;
use serde_json::{json, Value};

use crate::behaviour::entity::HttpProperties;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;

pub const HTTP: &str = "http";

pub struct Http {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl Http {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<Http, BehaviourCreationError> {
        let url = e.properties.get(HttpProperties::URL.as_ref());
        if url.is_none() {
            error!("Missing property {}", HttpProperties::URL.as_ref());
            return Err(BehaviourCreationError);
        }

        let method = e.properties.get(HttpProperties::METHOD.as_ref());
        if method.is_none() {
            error!("Missing property {}", HttpProperties::METHOD.as_ref());
            return Err(BehaviourCreationError);
        }

        let payload = e.properties.get(HttpProperties::PAYLOAD.as_ref());
        if payload.is_none() {
            error!("Missing property {}", HttpProperties::PAYLOAD.as_ref());
            return Err(BehaviourCreationError);
        }

        let entity = e.clone();
        let handle_id = e.properties.get(HttpProperties::PAYLOAD.as_ref()).unwrap().id.as_u128();
        e.properties
            .get(HttpProperties::PAYLOAD.as_ref())
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
                        .get(HttpProperties::URL.as_ref())
                        .unwrap()
                        .as_string()
                        .unwrap_or_else(|| HttpProperties::URL.default_value().to_string());
                    if url.is_empty() {
                        // Invalid URL
                        return;
                    }

                    let method = entity
                        .properties
                        .get(HttpProperties::METHOD.as_ref())
                        .unwrap()
                        .as_string()
                        .unwrap_or_else(|| HttpProperties::METHOD.default_value().to_string());

                    let empty_map = json!({});
                    let request_headers = entity.properties.get(HttpProperties::REQUEST_HEADERS.as_ref()).unwrap().get();
                    let request_headers = request_headers.as_object().unwrap_or_else(|| empty_map.as_object().unwrap());

                    let payload = entity.properties.get(HttpProperties::PAYLOAD.as_ref()).unwrap().get();

                    let mut request = ureq::request(method.as_str(), url.as_str());

                    for (request_header, value) in request_headers.into_iter() {
                        match value.as_str() {
                            Some(value) => {
                                request = request.set(request_header.as_ref(), value);
                            }
                            None => {}
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

        Ok(Http { entity: e.clone(), handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for Http {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(HttpProperties::PAYLOAD.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for Http {
    fn drop(&mut self) {
        self.disconnect();
    }
}
