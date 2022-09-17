use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::component::http::Http;
use crate::behaviour::component::http::HTTP;
use crate::behaviour::component::json_rpc::JsonRpc;
use crate::behaviour::component::json_rpc::JSON_RPC;
use crate::di::*;
use crate::model::ReactiveEntityInstance;
use crate::plugins::ComponentBehaviourProvider;

#[wrapper]
pub struct HttpStorage(RwLock<HashMap<Uuid, Arc<Http>>>);

#[wrapper]
pub struct JsonRpcStorage(RwLock<HashMap<Uuid, Arc<JsonRpc>>>);

#[provides]
fn create_http_storage() -> HttpStorage {
    HttpStorage(RwLock::new(HashMap::new()))
}

#[provides]
fn create_json_rpc_storage() -> JsonRpcStorage {
    JsonRpcStorage(RwLock::new(HashMap::new()))
}

#[async_trait]
pub trait HttpComponentBehaviourProvider: ComponentBehaviourProvider + Send + Sync {
    fn create_http(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_json_rpc(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_http(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_json_rpc(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

#[component]
pub struct HttpComponentBehaviourProviderImpl {
    http: HttpStorage,
    json_rpc: JsonRpcStorage,
}

interfaces!(HttpComponentBehaviourProviderImpl: dyn ComponentBehaviourProvider);

impl HttpComponentBehaviourProviderImpl {}

#[async_trait]
#[provides]
impl HttpComponentBehaviourProvider for HttpComponentBehaviourProviderImpl {
    fn create_http(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        if let Ok(http) = Http::new(entity_instance.clone()) {
            self.http.0.write().unwrap().insert(id, Arc::new(http));
            entity_instance.add_behaviour(HTTP);
            debug!("Added behaviour {} to entity instance {}", HTTP, id);
        }
    }

    fn create_json_rpc(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        if let Ok(json_rpc) = JsonRpc::new(entity_instance.clone()) {
            self.json_rpc.0.write().unwrap().insert(id, Arc::new(json_rpc));
            entity_instance.add_behaviour(JSON_RPC);
            debug!("Added behaviour {} to entity instance {}", JSON_RPC, id);
        }
    }

    fn remove_http(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if self.http.0.write().unwrap().remove(&entity_instance.id).is_some() {
            entity_instance.remove_behaviour(HTTP);
            debug!("Removed behaviour {} from entity instance {}", HTTP, entity_instance.id);
        }
    }

    fn remove_json_rpc(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if self.json_rpc.0.write().unwrap().remove(&entity_instance.id).is_some() {
            entity_instance.remove_behaviour(JSON_RPC);
            debug!("Removed behaviour {} from entity instance {}", JSON_RPC, entity_instance.id);
        }
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.http.0.write().unwrap().contains_key(&id) {
            if let Some(http) = self.http.0.write().unwrap().remove(&id) {
                http.entity.remove_behaviour(HTTP);
                debug!("Removed behaviour {} from entity instance {}", HTTP, id);
            }
        }
        if self.json_rpc.0.write().unwrap().contains_key(&id) {
            if let Some(json_rpc) = self.json_rpc.0.write().unwrap().remove(&id) {
                json_rpc.entity.remove_behaviour(JSON_RPC);
                debug!("Removed behaviour {} from entity instance {}", JSON_RPC, id);
            }
        }
    }
}

impl ComponentBehaviourProvider for HttpComponentBehaviourProviderImpl {
    fn add_behaviours_to_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if entity_instance.is_a(HTTP) {
            self.create_http(entity_instance.clone());
        }
        if entity_instance.is_a(JSON_RPC) {
            self.create_json_rpc(entity_instance);
        }
    }

    fn add_behaviours_to_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        match component.name.as_str() {
            HTTP => self.create_http(entity_instance),
            JSON_RPC => self.create_json_rpc(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if entity_instance.behaves_as(HTTP) {
            self.remove_http(entity_instance.clone());
        }
        if entity_instance.behaves_as(JSON_RPC) {
            self.remove_json_rpc(entity_instance);
        }
    }

    fn remove_behaviours_from_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        match component.name.as_str() {
            HTTP => self.remove_http(entity_instance),
            JSON_RPC => self.remove_json_rpc(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
