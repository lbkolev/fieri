//! Holds all necessary resources for direct interaction with the endpoints.

use std::default;

pub mod completion;
pub mod edit;
pub mod embedding;
pub mod file;
pub mod fine_tune;
pub mod image;
pub mod model;
pub mod moderation;

/// Possible Errors returned by responses from OpenAI API.
#[derive(Clone, Debug, default::Default, derive_getters::Getters, serde::Deserialize)]
#[serde(default)]
pub struct RequestError {
    message: String,
    r#type: String,

    // those are most frequently returned as null from OpenAI, even in the occurence of an error.
    param: Option<String>,
    code: Option<i32>,
}

/// Token usage information returned by responses from OpenAI API.
#[derive(Clone, Debug, default::Default, derive_getters::Getters, serde::Deserialize)]
#[serde(default)]
pub struct TokenUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Clone, Debug, default::Default, derive_getters::Getters, serde::Deserialize)]
#[serde(default)]
pub struct Choices {
    text: String,
    index: u32,
    logprobs: Option<f32>,
    finish_reason: String,
}

/// Information from requests wishing for a resource to be deleted, like [`Delete File`](crate::file::delete) and [`Delete Fine-tune`](crate::fine_tune::delete).
#[derive(Debug, serde::Deserialize, derive_getters::Getters, default::Default)]
#[serde(default)]
pub struct Delete {
    id: Option<String>,
    object: Option<String>,
    deleted: Option<bool>,
    token_usage: TokenUsage,
    error: RequestError,
}

/// Response from endpoints like [`Upload File`](crate::file::upload), [`Retrieve file`][crate::file::retrieve] & [`Create Fine-tune`](crate::fine_tune::create).
#[derive(Debug, serde::Deserialize, derive_getters::Getters)]
pub struct File {
    id: Option<String>,
    object: Option<String>,
    bytes: Option<i64>,
    created_at: Option<i64>,
    filename: Option<String>,
    purpose: Option<String>,
    token_usage: Option<TokenUsage>,
    error: Option<RequestError>,
}

type Files = Vec<File>;
