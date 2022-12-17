pub mod api_resources;
pub mod client;
pub mod config;
pub mod error;

type Result<T> = std::result::Result<T, error::Error>;
