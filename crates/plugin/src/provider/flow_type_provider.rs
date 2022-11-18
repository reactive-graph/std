use log::debug;
use log::error;
use paste::paste;
use rust_embed::RustEmbed;

use crate::di::*;
use crate::model::FlowType;
use crate::plugins::embedded_asset_provider_impl;
use crate::plugins::flow_type_provider_impl;
use crate::plugins::FlowTypeProvider;

flow_type_provider_impl!(Base, "../../assets/types/flows");
