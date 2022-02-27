use std::sync::Arc;

use crate::di::*;
use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::entity::http::{Http, HTTP};
use crate::behaviour::entity::jsonrpc::{JsonRpc, JSONRPC};
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct HttpStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<Http>>>);

#[wrapper]
pub struct JsonRpcStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<JsonRpc>>>);

#[provides]
fn create_http_storage() -> HttpStorage {
    HttpStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[provides]
fn create_json_rpc_storage() -> JsonRpcStorage {
    JsonRpcStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait HttpEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_http(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn create_json_rpc(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_http(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_json_rpc(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct HttpEntityBehaviourProviderImpl {
    http: HttpStorage,
    jsonrpc: JsonRpcStorage,
}

interfaces!(HttpEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl HttpEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            http: create_http_storage(),
            jsonrpc: create_json_rpc_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl HttpEntityBehaviourProvider for HttpEntityBehaviourProviderImpl {
    fn create_http(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        if let Ok(http) = Http::new(entity_instance.clone()) {
            let http = Arc::new(http);
            self.http.0.write().unwrap().insert(id, http);
            entity_instance.add_behaviour(HTTP);
            debug!("Added behaviour {} to entity instance {}", HTTP, id);
        }
    }

    fn create_json_rpc(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        if let Ok(jsonrpc) = JsonRpc::new(entity_instance.clone()) {
            let jsonrpc = Arc::new(jsonrpc);
            self.jsonrpc.0.write().unwrap().insert(id, jsonrpc);
            entity_instance.add_behaviour(JSONRPC);
            debug!("Added behaviour {} to entity instance {}", JSONRPC, id);
        }
    }

    fn remove_http(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.http.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(HTTP);
        debug!("Removed behaviour {} from entity instance {}", HTTP, entity_instance.id);
    }

    fn remove_json_rpc(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.jsonrpc.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(JSONRPC);
        debug!("Removed behaviour {} from entity instance {}", JSONRPC, entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.http.0.write().unwrap().contains_key(&id) {
            self.http.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", HTTP, id);
        }
        if self.jsonrpc.0.write().unwrap().contains_key(&id) {
            self.jsonrpc.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", JSONRPC, id);
        }
    }
}

impl EntityBehaviourProvider for HttpEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            HTTP => self.create_http(entity_instance),
            JSONRPC => self.create_json_rpc(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            HTTP => self.remove_http(entity_instance),
            JSONRPC => self.remove_json_rpc(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
