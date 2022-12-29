//! A composite error type for errors that can occur while interacting with OpenAI.

/// A set of errors that can occur during interaction with OpenAI.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("{0}")]
    VarError(#[from] std::env::VarError),

    #[error("{0}")]
    FileError(#[from] std::io::Error),

    #[error("{0}")]
    UrlError(#[from] url::ParseError),

    #[error("{0}")]
    FieldError(#[from] derive_builder::UninitializedFieldError),
}
