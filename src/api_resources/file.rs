//! Files are used to upload documents that can be used with features like [`Fine-tuning`](crate::api_resources::fine_tune).

use derive_getters::Getters;
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::{
    api_resources::{ErrorResp, TokenUsage},
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
    P: AsRef<Path>,
{
    /// Name of the `JSON Lines` file to be uploaded.
    file: P,

    /// The intended purpose of the uploaded documents.
    purpose: Purpose,
}

impl<P> UploadFileParam<P>
where
    P: AsRef<Path>,
{
    pub fn new(file: P, purpose: Purpose) -> Self {
        Self { file, purpose }
    }
}

/// Response from [`Delete File`](delete) request.
#[derive(Debug, Deserialize, Getters)]
pub struct DeleteFile {
    id: Option<String>,
    object: Option<String>,
    deleted: Option<bool>,
    token_usage: Option<TokenUsage>,
    error: Option<ErrorResp>,
}

/// Returns a [`list`][ListFiles] of files that belong to the user's organization.
///
/// Related OpenAI Docs: [List Files](https://beta.openai.com/docs/api-reference/files/list)
///
/// ## Example
/// ```rust
/// use std::env;
/// use openai_rs::{
///     Config, Client,
///     file::{ListFiles, list},
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new(env::var("OPENAI_API_KEY")?)
///         .organization(Some(env::var("OPENAI_ORGANIZATION")?));
///     let client = Client::new(&config);
///
///     let resp: ListFiles = list(&client).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn list(client: &Client<'_>) -> Result<ListFiles> {
    client.list_files().await
}

/// Upload a file that contains document(s) to be used across various endpoints/features.
///
/// Related OpenAI Docs: [Upload File](https://beta.openai.com/docs/api-reference/files/upload)
///
/// ## Example
/// ```no_run
/// use std::env;
/// use std::path::Path;
/// use openai_rs::{
///     Config, Client,
///     file::{UploadFileParam, File, Purpose, upload},
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new(env::var("OPENAI_API_KEY")?)
///         .organization(Some(env::var("OPENAI_ORGANIZATION")?));
///     let client = Client::new(&config);
///
///     let param = UploadFileParam::new(
///         Path::new("/path/to/file.jsonl"),
///         Purpose::FineTune
///     );
///
///     let resp: File = upload(&client, &param).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn upload<P: AsRef<Path>>(
    client: &Client<'_>,
    param: &UploadFileParam<P>,
) -> Result<File> {
    client.upload_file(param).await
}

/// Delete a file.
///
/// Related OpenAI Docs: [Delete File](https://beta.openai.com/docs/api-reference/files/delete)
///
/// ## Example
/// ```no_run
/// use std::env;
/// use openai_rs::{
///     Config, Client,
///     file::{DeleteFile, delete},
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new(env::var("OPENAI_API_KEY")?)
///         .organization(Some(env::var("OPENAI_ORGANIZATION")?));
///     let client = Client::new(&config);
///
///     let resp: DeleteFile = delete(&client, "file-to-delete").await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn delete<T: Into<String>>(client: &Client<'_>, file_id: T) -> Result<DeleteFile> {
    client.delete_file(file_id).await
}

/// Returns information about a specific file.
///
/// Related OpenAI Docs: [Retrieve File](https://beta.openai.com/docs/api-reference/files/retrieve)
///
/// ## Example
/// ```no_run
/// use std::env;
/// use openai_rs::{
///     client::Client,
///     config::Config,
///     api_resources::file::{File, retrieve},
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new(env::var("OPENAI_API_KEY")?)
///         .organization(Some(env::var("OPENAI_ORGANIZATION")?));
///     let client = Client::new(&config);
///
///     let resp: File = retrieve(&client, "file-to-retrieve").await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn retrieve<T: Into<String>>(client: &Client<'_>, file_id: T) -> Result<File> {
    client.retrieve_file(file_id).await
}

impl<'a> Client<'a> {
    async fn list_files(&self) -> Result<ListFiles> {
        let resp = self.get::<(), ListFiles>("files", None).await?;

        Ok(resp)
    }

    async fn upload_file<P: AsRef<Path>>(&self, param: &UploadFileParam<P>) -> Result<File> {
        let data = fs::read(param.file.as_ref())?;
        let part = Part::bytes(data).file_name("tmp101");
        let form = Form::new()
            .part("file", part)
            .text("purpose", param.purpose.to_string());

        let resp = self.post_data::<Form, File>("files", form).await?;

        Ok(resp)
    }

    async fn delete_file<T: Into<String>>(&self, file_id: T) -> Result<DeleteFile> {
        let resp = self
            .delete::<(), DeleteFile>(&format!("files/{}", file_id.into()), None)
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
    use crate::Config;
    use std::env;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_list_files() -> Result<()> {
        let config = Config::new(env::var("OPENAI_API_KEY")?)
            .organization(Some(env::var("OPENAI_ORGANIZATION")?));
        let client = Client::new(&config);

        let resp = list(&client).await?;
        println!("{:#?}", resp);

        assert_eq!(resp.error().is_none(), true);
        assert_eq!(resp.token_usage().is_none(), true);
        Ok(())
    }

    #[ignore = "requires file upload"]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_upload_file() -> Result<()> {
        let config = Config::new(env::var("OPENAI_API_KEY")?)
            .organization(Some(env::var("OPENAI_ORGANIZATION")?));
        let client = Client::new(&config);

        let param: UploadFileParam<&std::path::Path> = UploadFileParam::new(
            Path::new("../../resources/file_upload_example.jsonl"),
            Purpose::FineTune,
        );

        let resp = upload(&client, &param).await?;
        println!("{:#?}", resp);

        assert_eq!(resp.error().is_none(), true);
        Ok(())
    }

    #[ignore = "requires file deletion"]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_delete_file() -> Result<()> {
        let config = Config::new(env::var("OPENAI_API_KEY")?)
            .organization(Some(env::var("OPENAI_ORGANIZATION")?));
        let client = Client::new(&config);

        let resp = delete(&client, "rand-file").await?;
        println!("{:#?}", resp);

        assert_eq!(resp.deleted().is_none(), true);
        assert_eq!(resp.error().is_some(), true);
        Ok(())
    }

    #[ignore = "requires a file to retrieve"]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_retrieve_file() -> Result<()> {
        let config = Config::new(env::var("OPENAI_API_KEY")?)
            .organization(Some(env::var("OPENAI_ORGANIZATION")?));
        let client = Client::new(&config);

        let resp = retrieve(&client, "rand-file").await?;
        println!("{:#?}", resp);

        assert_eq!(resp.error().is_some(), true);
        Ok(())
    }
}
