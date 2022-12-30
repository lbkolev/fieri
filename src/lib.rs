//! Unofficial OpenAI API client for Rust.
//!
//! Fieri provides an asynchronous Rust interface for interacting with the OpenAI API,
//! allowing you to easily use OpenAI's state-of-the-art machine learning models in your Rust projects.
//!
//! Before you can use the Rust Client for OpenAI, you'll need to sign up for an API key at the OpenAI Developer Portal.
//! Once you've signed up, you'll be able to find your API key in the API Keys section of the developer portal.
//!
//! ## Example
//! ```rust
//! // Generate an image based on a prompt and save it locally.
//! use std::env;
//! use fieri::{
//!     Client,
//!     image::{ImageSize, GenerateImageParamBuilder, generate},
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new(env::var("OPENAI_API_KEY")?);
//!
//!     let param = GenerateImageParamBuilder::new("A bunch of cats dancing tango on the top of the highest mountain in Mars.")
//!         .size(ImageSize::S256x256)
//!         .n(1)
//!         .build()?;
//!
//!     let image = generate(&client, &param)
//!         .await?
//!         .save("/tmp/")
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

// #![deny(warnings)]

pub mod api_resources;
pub mod client;
mod config;
pub mod error;

#[doc(inline)]
pub use api_resources::{completion, edit, embedding, file, fine_tune, image, model, moderation};

#[doc(inline)]
pub use client::Client;

#[doc(inline)]
pub use error::Error;

/// Result returned from each interaction with the OpenAI API.
type Result<T> = std::result::Result<T, error::Error>;
