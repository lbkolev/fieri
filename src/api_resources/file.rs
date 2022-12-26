//! Files are used to upload documents that can be used with features like [`Fine-tuning`](crate::api_resources::fine_tune).

use derive_getters::Getters;
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fs;
use std::path::Path;

use crate::{
    api_resources::{Delete, ErrorResp, TokenUsage},
    Client, Result,
};

/// Response from [`List File`](list) request.
#[derive(Debug, Deserialize, Getters)]
pub struct ListFiles {
    data: Files,
    object: Option<String>,
    token_usage: Option<TokenUsage>,
    error: Option<ErrorResp>,
}

/// Response from [`Upload File`](upload) & [`Retrieve file`][retrieve] requests.
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

type Files = Vec<File>;

/// The Possible Purposes of the uploaded documents.
#[derive(Debug, Serialize)]
pub enum Purpose {
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

/// Parameters for [`Upload File`](upload) request.
#[derive(Debug)]
pub struct UploadFileParam<P>
where
    P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
{
    /// Name of the `JSON Lines` file to be uploaded.
    file: P,

    /// The intended purpose of the uploaded documents.
    purpose: Purpose,
}

impl<P> UploadFileParam<P>
where
    P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
{
    pub fn new(file: P, purpose: Purpose) -> Self {
        Self { file, purpose }
    }
}

/// Returns a [`list`][ListFiles] of files that belong to the user's organization.
///
/// Related OpenAI docs: [List Files](https://beta.openai.com/docs/api-reference/files/list)
///
/// ## Example
/// ```rust
/// use std::env;
/// use fieri::{Client, file::{ListFiles, list}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?)
///         .organization(env::var("OPENAI_ORGANIZATION")?);
///
///     let resp: ListFiles = list(&client).await?;
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
/// use fieri::{
///     Client,
///     file::{UploadFileParam, File, Purpose, upload},
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?)
///         .organization(env::var("OPENAI_ORGANIZATION")?);
///
///     let param = UploadFileParam::new(
///         "/path/to/file.jsonl",
///         Purpose::FineTune
///     );
///     let resp: File = upload(&client, &param).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn upload<P>(client: &Client, param: &UploadFileParam<P>) -> Result<File>
where
    P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
{
    client.upload_file(param).await
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
pub async fn delete<T: Into<String>>(client: &Client, file_id: T) -> Result<Delete> {
    client.delete_file(file_id).await
}

/// Returns information about a specific file.
///
/// Related OpenAI Docs: [Retrieve File](https://beta.openai.com/docs/api-reference/files/retrieve)
///
/// ## Example
/// ```no_run
/// use std::env;
/// use fieri::{Client, file::{File, retrieve}};
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
pub async fn retrieve<T: Into<String>>(client: &Client, file_id: T) -> Result<File> {
    client.retrieve_file(file_id).await
}

impl Client {
    async fn list_files(&self) -> Result<ListFiles> {
        let resp = self.get::<(), ListFiles>("files", None).await?;

        Ok(resp)
    }

    async fn upload_file<P>(&self, param: &UploadFileParam<P>) -> Result<File>
    where
        P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
    {
        let data = fs::read(param.file.as_ref())?;
        let part = Part::bytes(data).file_name(param.file);
        let form = Form::new()
            .part("file", part)
            .text("purpose", param.purpose.to_string());

        let resp = self.post_data::<Form, File>("files", form).await?;

        Ok(resp)
    }

    async fn delete_file<T: Into<String>>(&self, file_id: T) -> Result<Delete> {
        let resp = self
            .delete::<(), Delete>(&format!("files/{}", file_id.into()), None)
            .await?;

        Ok(resp)
    }

    async fn retrieve_file<T: Into<String>>(&self, file_id: T) -> Result<File> {
        let resp = self
            .get::<(), File>(&format!("files/{}", file_id.into()), None)
            .await?;

        Ok(resp)
    }

    // TODO
    //async fn retrieve_file_content(T: Into<String>>(&self, file_id: T) -> Result<FileContent> {}
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

        assert_eq!(resp.error().is_none(), true);
        assert_eq!(resp.token_usage().is_none(), true);
        Ok(())
    }

    #[ignore = "requires file upload"]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_upload_file() -> Result<()> {
        let client =
            Client::new(env::var("OPENAI_API_KEY")?).organization(env::var("OPENAI_ORGANIZATION")?);

        let param = UploadFileParam::new("payloads/file_upload_example.jsonl", Purpose::FineTune);
        let resp = upload(&client, &param).await?;
        println!("{:#?}", resp);

        assert_eq!(resp.error().is_none(), true);
        Ok(())
    }

    #[ignore = "requires file deletion"]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_delete_file() -> Result<()> {
        let client =
            Client::new(env::var("OPENAI_API_KEY")?).organization(env::var("OPENAI_ORGANIZATION")?);

        let resp = delete(&client, "rand-file").await?;
        println!("{:#?}", resp);

        assert_eq!(resp.deleted().is_none(), true);
        assert_eq!(resp.error().is_some(), true);
        Ok(())
    }

    #[ignore = "requires a file to retrieve"]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_retrieve_file() -> Result<()> {
        let client =
            Client::new(env::var("OPENAI_API_KEY")?).organization(env::var("OPENAI_ORGANIZATION")?);

        let resp = retrieve(&client, "rand-file").await?;
        println!("{:#?}", resp);

        assert_eq!(resp.error().is_some(), true);
        Ok(())
    }
}
