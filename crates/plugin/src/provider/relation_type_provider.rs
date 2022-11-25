use log::debug;
use log::error;
use paste::paste;
use rust_embed::RustEmbed;

use crate::di::*;
use crate::model::RelationType;
use crate::plugins::embedded_asset_provider_impl;
use crate::plugins::relation_type_provider_impl;
use crate::plugins::RelationTypeProvider;

relation_type_provider_impl!(Connector, "../../assets/types/relations");
