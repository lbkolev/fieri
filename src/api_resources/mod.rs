pub mod chat;
pub mod completion;
pub mod edit;
pub mod embedding;
pub mod file;
pub mod fine_tune;
pub mod image;
pub mod model;
pub mod moderation;

/// Tokens used for the requested action from OpenAI.
#[derive(Clone, Debug, std::default::Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Clone, Debug, std::default::Default, serde::Deserialize, serde::Serialize)]
pub struct Choices {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<f32>,
}

/// Information from requests wishing for a resource to be deleted, like [`Delete File`](crate::file::delete) and [`Delete Fine-tune`](crate::fine_tune::delete).
#[derive(Debug, std::default::Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Delete {
    pub id: String,
    pub object: String,
    pub deleted: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_usage: Option<TokenUsage>,
}

/// Response from endpoints like [`Upload File`](crate::file::upload), [`Retrieve file`][crate::file::retrieve] & [`Create Fine-tune`](crate::fine_tune::create).
#[derive(Debug, std::default::Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct File {
    pub id: String,
    pub object: String,
    pub bytes: i64,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
    pub status: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_usage: Option<TokenUsage>,
}

type Files = Vec<File>;
