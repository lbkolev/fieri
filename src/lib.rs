//! Unofficial OpenAI API client for Rust.
//!
//! This library provides a Rust interface for interacting with the OpenAI API,
//! allowing you to easily use OpenAI's state-of-the-art machine learning models in your Rust projects.
//!
//! Before you can use the Rust Client for OpenAI, you'll need to sign up for an API key at the OpenAI Developer Portal.
//! Once you've signed up, you'll be able to find your API key in the API Keys section of the developer portal.
//!
//! ## Examples
//! ...

pub mod api_resources;
pub mod client;
mod config;
pub mod error;

pub use api_resources::{completion, edit, embedding, file, fine_tune, image, model, moderation};
pub use client::Client;
pub use config::Models;
pub use error::Error;

/// Result returned from each interaction with the OpenAI API.
type Result<T> = std::result::Result<T, error::Error>;
