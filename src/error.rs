//! A composite error type for errors that can occur while interacting with OpenAI.

/// A set of errors that can occur during interaction with OpenAI.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{}, {}", .0.error.r#type, .0.error.message)]
    APIError(crate::types::RequestError),

    #[error("{}, {}", .0.r#type, .0.message)]
    MissingParameter(crate::types::ErrorMessage),

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
    CompletionParamBuilderError(#[from] crate::types::CompletionParamBuilderError),

    #[error("Invalid values provided. {0}")]
    EditParamBuilderError(#[from] crate::types::EditParamBuilderError),

    #[error("Invalid values provided. {0}")]
    EmbeddingParamBuilderError(#[from] crate::types::EmbeddingParamBuilderError),

    #[error("Invalid values provided. {0}")]
    FineTuneParamBuilderError(#[from] crate::types::CreateFineTuneParamBuilderError),

    #[error("Invalid values provided. {0}")]
    ModerationParamBuilderError(#[from] crate::types::ModerationParamBuilderError),

    #[error("Invalid values provided. {0}")]
    GenerateImageParamBuilderError(#[from] crate::types::GenerateImageParamBuilderError),

    #[error("Invalid values provided. {0}")]
    ChatParamBuilderError(#[from] crate::types::ChatParamBuilderError),

    #[error("Invalid values provided. {0}")]
    ChatMessageBuilderError(#[from] crate::types::ChatMessageBuilderError),
}
