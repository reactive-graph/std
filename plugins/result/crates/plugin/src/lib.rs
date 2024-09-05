#![feature(register_tool)]
#![register_tool(tarpaulin)]
#![allow(clippy::map_entry)]

pub mod plugin;
pub mod providers;

// #[cfg(test)]
// #[tarpaulin::ignore]
// pub mod tests;
