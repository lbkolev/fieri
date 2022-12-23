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
#[derive(Debug, Clone, derive_getters::Getters, serde::Deserialize)]
pub struct TokenUsage {
    prompt_tokens: Option<u32>,
    completion_tokens: Option<u32>,
    total_tokens: Option<u32>,
}

#[derive(Debug, Clone, derive_getters::Getters, serde::Deserialize)]
pub struct Choices {
    text: Option<String>,
    index: Option<u32>,
    logprobs: Option<f32>,
    finish_reason: Option<String>,
}
