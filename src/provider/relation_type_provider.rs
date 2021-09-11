use async_trait::async_trait;
use log::{debug, error};
use rust_embed::RustEmbed;
use waiter_di::*;

use crate::model::relation_type::RelationType;
use crate::plugins::RelationTypeProvider;

#[derive(RustEmbed)]
#[folder = "./assets/types/relations"]
struct ConnectorRelationTypeAsset;

#[async_trait]
pub trait ConnectorRelationTypeProvider: RelationTypeProvider + Send + Sync {}

#[derive(Clone)]
pub struct ConnectorRelationTypeProviderImpl {}

interfaces!(ConnectorRelationTypeProviderImpl: dyn RelationTypeProvider);

#[component]
impl ConnectorRelationTypeProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl ConnectorRelationTypeProvider for ConnectorRelationTypeProviderImpl {}

impl RelationTypeProvider for ConnectorRelationTypeProviderImpl {
    fn get_relation_types(&self) -> Vec<RelationType> {
        let mut relation_types = Vec::new();
        for file in ConnectorRelationTypeAsset::iter() {
            let filename = file.as_ref();
            debug!("Loading relation_type from resource {}", filename);
            let asset = ConnectorRelationTypeAsset::get(filename).unwrap();
            let json_str = std::str::from_utf8(asset.data.as_ref());
            if json_str.is_err() {
                error!("Could not decode UTF-8 {}", filename);
                continue;
            }
            let relation_type: RelationType = match serde_json::from_str(json_str.unwrap()) {
                Result::Ok(relation_type) => relation_type,
                Result::Err(err) => {
                    error!("Error in parsing JSON file {}: {}", filename, err);
                    continue;
                }
            };
            relation_types.push(relation_type);
        }
        relation_types
    }
}
