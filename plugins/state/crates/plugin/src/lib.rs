#![feature(lazy_cell)]
#![feature(register_tool)]
#![register_tool(tarpaulin)]
#![allow(clippy::map_entry)]

use inexor_rgf_model_state as model_state;
use inexor_rgf_model_value as model_value;

pub mod behaviour;
pub mod plugin;
pub mod providers;

// #[cfg(test)]
// #[tarpaulin::ignore]
// pub mod tests;
