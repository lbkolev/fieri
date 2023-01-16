//! Files are used to upload documents that can be used with features like [`Fine-tuning`](crate::api_resources::fine_tune).

use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fs, path::Path};

use crate::{
    api_resources::{Delete, File, Files, TokenUsage},
    Client, Result,
};

/// Response from [`List File`](list) request.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ListFiles {
    pub data: Files,
    pub object: String,

    pub token_usage: Option<TokenUsage>,
}

/// The Possible Purposes of the uploaded documents.
#[derive(Debug, Default, Deserialize, Serialize)]
pub enum Purpose {
    #[default]
    FineTune,
    Answers,
    Search,
    Classifications,
}

impl std::fmt::Display for Purpose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Purpose::FineTune => write!(f, "fine-tune"),
            Purpose::Answers => write!(f, "answers"),
            Purpose::Search => write!(f, "search"),
            Purpose::Classifications => write!(f, "classifications"),
        }
    }
}

/// Returns a [`list`][ListFiles] of files that belong to the user's organization.
///
/// Related OpenAI docs: [List Files](https://beta.openai.com/docs/api-reference/files/list)
///
/// ## Example
/// ```rust
/// use std::env;
/// use fieri::{Client, file::list};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?)
///         .organization(env::var("OPENAI_ORGANIZATION")?);
///
///     let resp = list(&client).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn list(client: &Client) -> Result<ListFiles> {
    client.list_files().await
}

/// Upload a file that contains document(s) to be used across various endpoints/features.
///
/// Related OpenAI docs: [Upload File](https://beta.openai.com/docs/api-reference/files/upload)
///
/// ## Example
/// ```no_run
/// use std::env;
/// use std::path::Path;
/// use fieri::{Client, file::{Purpose, upload}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?)
///         .organization(env::var("OPENAI_ORGANIZATION")?);
///
///     let resp = upload(&client, "/path/to/file.jsonl", Purpose::FineTune).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn upload<P>(client: &Client, file: P, purpose: Purpose) -> Result<File>
where
    P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
{
    client.upload_file(file, purpose).await
}

/// Delete a file.
///
/// Related OpenAI docs: [Delete File](https://beta.openai.com/docs/api-reference/files/delete)
///
/// ## Example
/// ```no_run
/// use std::env;
/// use fieri::{Client, file::delete};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?)
///         .organization(env::var("OPENAI_ORGANIZATION")?);
///
///     let resp = delete(&client, "file-to-delete").await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn delete(client: &Client, file_id: impl Into<String>) -> Result<Delete> {
    client.delete_file(file_id.into()).await
}

/// Returns information about a specific file.
///
/// Related OpenAI docs: [Retrieve File](https://beta.openai.com/docs/api-reference/files/retrieve)
///
/// ## Example
/// ```no_run
/// use std::env;
/// use fieri::{Client, file::retrieve, api_resources::File};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?)
///         .organization(env::var("OPENAI_ORGANIZATION")?);
///
///     let resp: File = retrieve(&client, "file-to-retrieve").await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn retrieve(client: &Client, file_id: impl Into<String>) -> Result<File> {
    client.retrieve_file(file_id.into()).await
}

impl Client {
    async fn list_files(&self) -> Result<ListFiles> {
        self.get::<(), ListFiles>("files", None).await
    }

    async fn upload_file<P>(&self, file: P, purpose: Purpose) -> Result<File>
    where
        P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
    {
        let data = fs::read(file.as_ref())?;
        let part = Part::bytes(data).file_name(file);
        let form = Form::new()
            .part("file", part)
            .text("purpose", purpose.to_string());

        self.post_data::<File>("files", form).await
    }

    async fn delete_file(&self, file_id: String) -> Result<Delete> {
        self.delete::<(), Delete>(&format!("files/{file_id}"), None)
            .await
    }

    async fn retrieve_file(&self, file_id: String) -> Result<File> {
        self.get::<(), File>(&format!("files/{file_id}"), None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_list_files() -> Result<()> {
        let client =
            Client::new(env::var("OPENAI_API_KEY")?).organization(env::var("OPENAI_ORGANIZATION")?);

        let resp = list(&client).await?;
        println!("{:#?}", resp);

        assert_eq!(resp.object, "list");
        assert!(resp.token_usage.is_none());
        Ok(())
    }

    #[ignore = "requires file upload"]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_upload_file() -> Result<()> {
        let client =
            Client::new(env::var("OPENAI_API_KEY")?).organization(env::var("OPENAI_ORGANIZATION")?);

        let resp = upload(
            &client,
            "./assets/file_upload_example.jsonl",
            Purpose::FineTune,
        )
        .await?;
        println!("{:#?}", resp);

        assert_eq!(resp.object, "file");
        assert!(resp.token_usage.is_none());
        Ok(())
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_delete_file() -> Result<()> {
        let client =
            Client::new(env::var("OPENAI_API_KEY")?).organization(env::var("OPENAI_ORGANIZATION")?);

        let resp = delete(&client, "rand-file").await?;
        println!("{:#?}", resp);

        assert_eq!(resp.deleted, false);
        assert!(resp.token_usage.is_none());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_retrieve_file() -> Result<()> {
        let client =
            Client::new(env::var("OPENAI_API_KEY")?).organization(env::var("OPENAI_ORGANIZATION")?);

        let resp = retrieve(&client, "file-1FZQ73L5AK8UknTTT0PxWMBE").await?;
        println!("{:#?}", resp);

        assert_eq!(resp.object, "file");
        assert!(resp.token_usage.is_none());
        Ok(())
    }
}
