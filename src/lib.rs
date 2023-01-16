//! Unofficial OpenAI API client for Rust.
//!
//! Fieri provides an asynchronous Rust interface for interacting with the OpenAI API,
//! allowing you to easily use OpenAI's state-of-the-art machine learning models in your Rust projects.
//!
//! Before you can use the Rust Client for OpenAI, you'll need to sign up for an API key at the OpenAI Developer Portal.
//! Once you've signed up, you'll be able to find your API key in the API Keys section of the developer portal.
//!
//! Each request requires a Client, initialized with your API key.
//!
//! ## Examples
//!
//! ### Generate text based on a prompt
//! ```rust
//! use fieri::{
//!     completion::{create, CompletionParamBuilder},
//!     Client, Error,
//! };
//! use std::env;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let client = Client::new(env::var("OPENAI_API_KEY")?);
//! 
//!     let param = CompletionParamBuilder::new("ada")
//!         .prompt("Generate a plot for an absurd interstellar parody.")
//!         .max_tokens(500)
//!         .temperature(0.9)
//!         .top_p(1.0)
//!         .frequency_penalty(0.0)
//!         .presence_penalty(0.0)
//!         .build()?;
//! 
//!     let resp = create(&client, &param).await?;
//!     println!("Generated text: {:#?}", resp);
//! 
//!     Ok(())
//! }
//! ```
//! 
//! ### Generate and stream back text based on a prompt
//! ```rust
//! use fieri::{
//!     completion::{create_with_stream, Completion, CompletionParamBuilder},
//!     Client, Error,
//! };
//! use std::env;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let client = Client::new(env::var("OPENAI_API_KEY")?);
//! 
//!     let param = CompletionParamBuilder::new("ada")
//!         .prompt("unnecessarily lo")
//!         .temperature(0.5)
//!         .build()?;
//! 
//!     let mut resp = create_with_stream(&client, &param).await?;
//! 
//!     while let Some(chunk) = resp.chunk().await? {
//!         if chunk.to_vec() == b"data: [DONE]\n\n" {
//!             break;
//!         }
//! 
//!         let v: Completion = serde_json::from_slice(&chunk[5..])?;
//!         v.choices().iter().for_each(|c| println!("{:?}", c.text()));
//!     }
//! 
//!     Ok(())
//! }
//! ```
//! 
//! ### Generate an image based on a prompt and save it locally.
//! ```rust
//! use fieri::{
//!     image::{ImageSize, GenerateImageParamBuilder, generate},
//!     Client, Error,
//! };
//! use std::env;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let client = Client::new(env::var("OPENAI_API_KEY")?);
//! 
//!     let param = GenerateImageParamBuilder::new("A bunch of cats dancing tango on top of the highest mountain on Mars.")
//!         .size(ImageSize::S1024x1024)
//!         .n(1)
//!         .build()?;
//! 
//!     generate(&client, &param)
//!         .await?
//!         .save("/tmp/")
//!         .await?;
//! 
//!     Ok(())
//! }
//! ```

#![deny(missing_debug_implementations, rust_2018_idioms)]

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
pub type Result<T> = std::result::Result<T, Error>;
