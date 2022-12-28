//! Holds all necessary resources for direct interaction with the endpoints.

pub mod completion;
pub mod edit;
pub mod embedding;
pub mod file;
pub mod fine_tune;
pub mod image;
pub mod model;
pub mod moderation;

/// Possible Errors returned by responses from OpenAI API.
#[derive(Debug, Clone, derive_getters::Getters, serde::Deserialize)]
pub struct ErrorResp {
    message: Option<String>,
    r#type: Option<String>,
    param: Option<String>,
    code: Option<i32>,
}

/// Token usage information returned by responses from OpenAI API.
#[derive(Debug, Clone, serde::Deserialize, derive_getters::Getters)]
pub struct TokenUsage {
    prompt_tokens: Option<u32>,
    completion_tokens: Option<u32>,
    total_tokens: Option<u32>,
}

#[derive(Debug, Clone, serde::Deserialize, derive_getters::Getters)]
pub struct Choices {
    text: Option<String>,
    index: Option<u32>,
    logprobs: Option<f32>,
    finish_reason: Option<String>,
}

/// Response from requests wishing for a resource to be deleted, like [`Delete File`](crate::file::delete) and [`Delete Fine-tune`](crate::fine_tune::delete).
#[derive(Debug, serde::Deserialize, derive_getters::Getters)]
pub struct Delete {
    id: Option<String>,
    object: Option<String>,
    deleted: Option<bool>,
    token_usage: Option<TokenUsage>,
    error: Option<ErrorResp>,
}

/// Response from endpoints like [`Upload File`](crate::file::upload), [`Retrieve file`][crate::file::retrieve] & [`Create Fine-tune`](crate::fine_tune::create) requests.
#[derive(Debug, serde::Deserialize, derive_getters::Getters)]
pub struct File {
    id: Option<String>,
    object: Option<String>,
    bytes: Option<i64>,
    created_at: Option<i64>,
    filename: Option<String>,
    purpose: Option<String>,
    token_usage: Option<TokenUsage>,
    error: Option<ErrorResp>,
}

type Files = Vec<File>;
