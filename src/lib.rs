#![doc = include_str!("../README.md")]
#![deny(missing_debug_implementations, rust_2018_idioms)]

pub mod api_resources;
pub mod client;
mod config;
pub mod error;
pub mod types;
mod utils;

#[doc(inline)]
pub use api_resources::{
    chat, completion, edit, embedding, file, fine_tune, image, model, moderation,
};

#[doc(inline)]
pub use client::Client;

#[doc(inline)]
pub use error::Error;

/// Result returned from each interaction with the OpenAI API.
pub type Result<T> = std::result::Result<T, Error>;
