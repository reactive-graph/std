use std::sync::Arc;

use crate::di::*;
use async_trait::async_trait;
use indradb::EdgeKey;
use log::debug;

use crate::behaviour::relation::connector::Connector;
use crate::behaviour::relation::connector::CONNECTORS;
use crate::model::ReactiveRelationInstance;
use crate::plugins::RelationBehaviourProvider;

#[wrapper]
pub struct ConnectorStorage(
    std::sync::RwLock<std::collections::HashMap<EdgeKey, std::sync::Arc<Connector>>>,
);

#[provides]
fn create_connector_storage() -> ConnectorStorage {
    ConnectorStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait ConnectorRelationBehaviourProvider: RelationBehaviourProvider + Send + Sync {
    fn create_connector(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_connector(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_by_key(&self, edge_key: EdgeKey);
}

pub struct ConnectorRelationBehaviourProviderImpl {
    connectors: ConnectorStorage,
}

interfaces!(ConnectorRelationBehaviourProviderImpl: dyn RelationBehaviourProvider);

#[component]
impl ConnectorRelationBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            connectors: create_connector_storage(),
        }
    }
}

impl ConnectorRelationBehaviourProviderImpl {}

#[async_trait]
#[provides]
impl ConnectorRelationBehaviourProvider for ConnectorRelationBehaviourProviderImpl {
    fn create_connector(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        let mut type_name = relation_instance.type_name.clone();
        let mut function = CONNECTORS.get(type_name.as_str());
        if function.is_none() {
            if let Some(connector_type_name) = CONNECTORS
                .keys()
                .into_iter()
                .find(|connector_type_name| type_name.starts_with(*connector_type_name))
            {
                function = CONNECTORS.get(connector_type_name);
                type_name = String::from(*connector_type_name);
            }
        }
        if let Some(connector) = function.map(|function| {
            Arc::new(Connector::from_relation(
                relation_instance.clone(),
                *function,
            ))
        }) {
            self.connectors
                .0
                .write()
                .unwrap()
                .insert(edge_key.clone(), connector);
            relation_instance.add_behaviour(type_name.clone());
            debug!(
                "Added behaviour {} to relation instance {:?}",
                type_name, edge_key
            );
        };
    }

    fn remove_connector(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        self.connectors.0.write().unwrap().remove(&edge_key);
        let mut type_name = relation_instance.type_name.clone();
        if CONNECTORS.get(type_name.as_str()).is_none() {
            if let Some(connector_type_name) = CONNECTORS
                .keys()
                .into_iter()
                .find(|connector_type_name| type_name.starts_with(*connector_type_name))
            {
                type_name = String::from(*connector_type_name);
            }
        }
        relation_instance.remove_behaviour(type_name);
        debug!(
            "Removed behaviour connector to relation instance {:?}",
            edge_key
        );
    }

    fn remove_by_key(&self, edge_key: EdgeKey) {
        if self.connectors.0.write().unwrap().contains_key(&edge_key) {
            self.connectors.0.write().unwrap().remove(&edge_key);
            debug!(
                "Removed behaviour connector to relation instance {:?}",
                edge_key
            );
        }
    }
}

impl RelationBehaviourProvider for ConnectorRelationBehaviourProviderImpl {
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        self.create_connector(relation_instance);
    }

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        self.remove_connector(relation_instance);
    }

    fn remove_behaviours_by_key(&self, edge_key: EdgeKey) {
        self.remove_by_key(edge_key);
    }
}
