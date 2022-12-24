//! Files are used to upload documents that can be used with features like [`Fine-tuning`](crate::api_resources::fine_tune).

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{
    api_resources::{ErrorResp, TokenUsage},
    Client, Result,
};

/// Response for [`list`](crate::api_resources::file::list) file request.
#[derive(Debug, Deserialize, Getters)]
pub struct ListFilesResp {
    data: Files,
    object: Option<String>,
    token_usage: Option<TokenUsage>,
    error: Option<ErrorResp>,
}

#[derive(Debug, Deserialize, Getters)]
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

#[derive(Debug, Serialize)]
pub struct UploadFileParam {
    /// Name of the `JSON Lines` file to be uploaded.
    file: String,

    /// The intended purpose of the uploaded documents.
    purpose: String,
}

impl UploadFileParam {
    pub fn new(file: String, purpose: String) -> Self {
        Self { file, purpose }
    }

    pub fn file(mut self, file: String) -> Self {
        self.file = file;

        self
    }

    pub fn purpose(mut self, purpose: String) -> Self {
        self.purpose = purpose;

        self
    }
}

type Files = Vec<File>;

/// Returns a [`list`][ListFilesResp] of files that belong to the user's organization.
///
/// ## Example
/// ```rust
/// use std::env;
/// use openai_rs::{
///     client::Client,
///     config::Config,
///     api_resources::file::{ListFilesResp, list},
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new(env::var("OPENAI_API_KEY")?)
///         .organization(Some(env::var("OPENAI_ORGANIZATION")?));
///     let client = Client::new(&config);
///
///     let resp: ListFilesResp = list(&client).await?;
///     println!("{:#?}", resp);
///     Ok(())
/// }
/// ```
pub async fn list(client: &Client<'_>) -> Result<ListFilesResp> {
    client.list_files().await
}

/// Upload a file that contains document(s) to be used across various endpoints/features.
///
/// ## Example
/// ```rust
/// use std::env;
/// use openai_rs::{
///     client::Client,
///     config::Config,
///     api_resources::file::{UploadFileParam, upload},
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new(env::var("OPENAI_API_KEY")?)
///         .organization(Some(env::var("OPENAI_ORGANIZATION")?));
///     let client = Client::new(&config);
///
///     let param = UploadFileParam::new("file.jsonl".to_string(), "search".to_string());
///     let resp: File = upload(&client, &param).await?;
///     println!("{:#?}", resp);
///     Ok(())
/// }
/// ```
pub async fn upload(client: &Client<'_>, param: &UploadFileParam) -> Result<File> {
    client.upload_file(param).await
}

impl<'a> Client<'a> {
    async fn list_files(&self) -> Result<ListFilesResp> {
        let resp = self
            .get::<(), ListFilesResp>("/files".to_string(), None)
            .await?;

        Ok(resp)
    }

    async fn upload_file(&self, param: &UploadFileParam) -> Result<File> {
        let resp = self
            .post::<UploadFileParam, File>("/files".to_string(), Some(param))
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use std::env;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_list_files() -> Result<()> {
        let config = Config::new(env::var("OPENAI_API_KEY")?)
            .organization(Some(env::var("OPENAI_ORGANIZATION")?));
        let client = Client::new(&config);

        let resp: ListFilesResp = list(&client).await?;
        println!("{:#?}", resp);

        assert_eq!(resp.error().is_none(), true);
        assert_eq!(resp.token_usage().is_none(), true);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_upload_file() -> Result<()> {
        let config = Config::new(env::var("OPENAI_API_KEY")?)
            .organization(Some(env::var("OPENAI_ORGANIZATION")?));
        let client = Client::new(&config);

        let param = UploadFileParam::new("".to_string(), "fine_tune".to_string());
        let resp = upload(&client, &param).await?;
        println!(": {:#?}", resp);

        assert_eq!(resp.error().is_none(), true);
        Ok(())
    }
}
