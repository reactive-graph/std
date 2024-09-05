#![feature(register_tool)]
#![register_tool(tarpaulin)]
#![allow(clippy::map_entry)]

use reactive_graph_model_state as model_state;
use reactive_graph_model_value as model_value;

pub mod behaviour;
pub mod plugin;
pub mod providers;

// #[cfg(test)]
// #[tarpaulin::ignore]
// pub mod tests;
