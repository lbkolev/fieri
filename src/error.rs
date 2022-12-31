//! A composite error type for errors that can occur while interacting with OpenAI.

/// A set of errors that can occur during interaction with OpenAI.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Errors that occur given invalid values.
    #[error("{}, {}", .0.error.r#type, .0.error.message)]
    APIError(RequestError),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("{0}")]
    VarError(#[from] std::env::VarError),

    #[error("{0}")]
    FileError(#[from] std::io::Error),

    #[error("{0}")]
    UrlError(#[from] url::ParseError),

    #[error("{0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("{0}")]
    FieldError(#[from] derive_builder::UninitializedFieldError),

    #[error("Invalid values provided. {0}")]
    CompletionParamBuilderError(#[from] crate::completion::CompletionParamBuilderError),

    #[error("Invalid values provided. {0}")]
    EditParamBuilderError(#[from] crate::edit::EditParamBuilderError),

    #[error("Invalid values provided. {0}")]
    EmbeddingParamBuilderError(#[from] crate::embedding::EmbeddingParamBuilderError),

    #[error("Invalid values provided. {0}")]
    FineTuneParamBuilderError(#[from] crate::fine_tune::CreateFineTuneParamBuilderError),

    #[error("Invalid values provided. {0}")]
    ModerationParamBuilderError(#[from] crate::moderation::ModerationParamBuilderError),

    #[error("Invalid values provided. {0}")]
    GenerateImageParamBuilderError(#[from] crate::image::GenerateImageParamBuilderError),
}

/// Possible Errors returned by responses from OpenAI.
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename(deserialize = "error"))]
pub struct RequestError {
    pub error: RequestErrorValues,
}

#[derive(Clone, Debug, std::default::Default, serde::Deserialize, derive_getters::Getters)]
pub struct RequestErrorValues {
    pub message: String,
    pub r#type: String,

    // those are most frequently returned as null from OpenAI, even in the occurence of an error.
    pub param: serde_json::Value,
    pub code: serde_json::Value,
}
