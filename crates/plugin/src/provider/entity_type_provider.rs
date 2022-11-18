use log::debug;
use log::error;
use paste::paste;
use rust_embed::RustEmbed;

use crate::di::*;
use crate::model::EntityType;
use crate::plugins::embedded_asset_provider_impl;
use crate::plugins::entity_type_provider_impl;
use crate::plugins::EntityTypeProvider;

entity_type_provider_impl!(Base, "../../assets/types/entities");
