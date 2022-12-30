//! Holds all necessary resources for direct interaction with the endpoints.

pub mod completion;
pub mod edit;
pub mod embedding;
pub mod file;
pub mod fine_tune;
pub mod image;
pub mod model;
pub mod moderation;

/// Possible Errors returned by responses from OpenAI.
#[derive(Clone, Debug, std::default::Default, serde::Deserialize, derive_getters::Getters)]
#[serde(default)]
pub struct RequestError {
    message: String,
    r#type: String,

    // those are most frequently returned as null from OpenAI, even in the occurence of an error.
    param: Option<String>,
    code: Option<i32>,
}

/// Tokens used by the requested action from OpenAI.
#[derive(Clone, Debug, std::default::Default, serde::Deserialize, derive_getters::Getters)]
#[serde(default)]
pub struct TokenUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Clone, Debug, std::default::Default, serde::Deserialize, derive_getters::Getters)]
#[serde(default)]
pub struct Choices {
    text: String,
    index: u32,
    finish_reason: String,

    logprobs: Option<f32>,
}

/// Information from requests wishing for a resource to be deleted, like [`Delete File`](crate::file::delete) and [`Delete Fine-tune`](crate::fine_tune::delete).
#[derive(Debug, std::default::Default, serde::Deserialize, derive_getters::Getters)]
#[serde(default)]
pub struct Delete {
    id: String,
    object: String,
    deleted: bool,

    token_usage: Option<TokenUsage>,
    error: Option<RequestError>,
}

/// Response from endpoints like [`Upload File`](crate::file::upload), [`Retrieve file`][crate::file::retrieve] & [`Create Fine-tune`](crate::fine_tune::create).
#[derive(Debug, std::default::Default, serde::Deserialize, derive_getters::Getters)]
#[serde(default)]
pub struct File {
    id: String,
    object: String,
    bytes: i64,
    created_at: i64,
    filename: String,
    purpose: String,
    status: String,

    token_usage: Option<TokenUsage>,
    error: Option<RequestError>,
}

type Files = Vec<File>;
