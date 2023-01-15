use random_string::generate;

pub use create_connector::*;
pub use create_random_entity_instance::*;

pub mod create_connector;
pub mod create_random_entity_instance;

const CHARSET_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn r_string() -> String {
    generate(10, CHARSET_LETTERS).to_string()
}
