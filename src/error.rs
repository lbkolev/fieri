#[derive(Debug, thiserror::Error)]
/// Possible errors that can occur when interacting with the Library.
pub enum Error {
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("{0}")]
    VarError(#[from] std::env::VarError),

    #[error(
        "Missing mandatory model for request. Please use the `model` method to set the model."
    )]
    MissingModel,
}
