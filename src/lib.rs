//! Unofficial OpenAI API client for Rust.
//!
//! # Example
//! ...

pub mod api_resources;
pub mod client;
pub mod config;
pub mod error;

pub use api_resources::{completion, edit, embedding, file, fine_tune, image, model, moderation};
pub use client::Client;
pub use config::{Config, Model};
pub use error::Error;

/// Result returned from each interaction with the OpenAI API.
type Result<T> = std::result::Result<T, error::Error>;
