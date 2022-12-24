//! Files are used to upload documents that can be used with features like [`Fine-tuning`](crate::api_resources::fine_tune).

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::path::Path;

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

type Files = Vec<File>;

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

#[derive(Debug)]
pub struct UploadFileParam<P: AsRef<Path>> {
    /// Name of the `JSON Lines` file to be uploaded.
    file: Option<P>,

    /// The intended purpose of the uploaded documents.
    purpose: Option<Purpose>,
}

impl<P: AsRef<Path>> Default for UploadFileParam<P> {
    fn default() -> Self {
        Self {
            file: None,
            purpose: None,
        }
    }
}

impl<P: AsRef<Path>> UploadFileParam<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn file(mut self, file: Option<P>) -> Self
    where
        P: AsRef<Path>,
    {
        self.file = file;

        self
    }

    pub fn purpose(mut self, purpose: Option<Purpose>) -> Self {
        self.purpose = purpose;

        self
    }
}

#[derive(Debug, Deserialize, Getters)]
pub struct DeleteFileResp {
    id: Option<String>,
    object: Option<String>,
    deleted: Option<bool>,
    token_usage: Option<TokenUsage>,
    error: Option<ErrorResp>,
}

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
/// ```no_run
/// use std::env;
/// use openai_rs::{
///     client::Client,
///     config::Config,
///     api_resources::file::{UploadFileParam, File, Purpose, upload},
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new(env::var("OPENAI_API_KEY")?)
///         .organization(Some(env::var("OPENAI_ORGANIZATION")?));
///     let client = Client::new(&config);
///
///     let param = UploadFileParam::new()
///        .file(Some(std::path::Path::new("/path/to/file.jsonl")))
///        .purpose(Some(Purpose::FineTune));
///
///     let resp: File = upload(&client, &param).await?;
///     println!("{:#?}", resp);
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
/// ## Example
/// ```no_run
/// use std::env;
/// use openai_rs::{
///     client::Client,
///     config::Config,
///     api_resources::file::{DeleteFileResp, delete},
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new(env::var("OPENAI_API_KEY")?)
///         .organization(Some(env::var("OPENAI_ORGANIZATION")?));
///     let client = Client::new(&config);
///
///     let resp: DeleteFileResp = delete(&client, "file-to-delete").await?;
///     println!("{:#?}", resp);
///     Ok(())
/// }
/// ```
pub async fn delete<T: Into<String>>(client: &Client<'_>, file_id: T) -> Result<DeleteFileResp> {
    client.delete_file(file_id).await
}

/// Returns information about a specific file.
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
///     Ok(())
/// }
/// ```
pub async fn retrieve<T: Into<String>>(client: &Client<'_>, file_id: T) -> Result<File> {
    client.retrieve_file(file_id).await
}

impl<'a> Client<'a> {
    async fn list_files(&self) -> Result<ListFilesResp> {
        let resp = self.get::<&str, (), ListFilesResp>("/files", None).await?;

        Ok(resp)
    }

    async fn upload_file<P: AsRef<Path>>(&self, param: &UploadFileParam<P>) -> Result<File> {
        let data = std::fs::read(param.file.as_ref().unwrap()).unwrap();
        let part = reqwest::multipart::Part::bytes(data).file_name("tmp101");
        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("purpose", param.purpose.as_ref().unwrap().to_string());

        let resp = self
            .post_data::<&str, reqwest::multipart::Form, File>("/files", form)
            .await?;

        Ok(resp)
    }

    async fn delete_file<T: Into<String>>(&self, file_id: T) -> Result<DeleteFileResp> {
        let resp = self
            .delete::<&str, &String, DeleteFileResp>(&format!("/files/{}", file_id.into()), None)
            .await?;

        Ok(resp)
    }

    async fn retrieve_file<T: Into<String>>(&self, file_id: T) -> Result<File> {
        let resp = self
            .get::<&str, (), File>(&format!("/files/{}", file_id.into()), None)
            .await?;

        Ok(resp)
    }

    // TODO
    //async fn retrieve_file_content(T: Into<String>>(&self, file_id: T) -> Result<FileContent> {
    //    let resp = self
    //        .get::<&str, (), FileContent>(&format!("/files/{}/content", file_id.into()), None)
    //        .await?;
    //
    //    Ok(resp)
    //}
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

    #[ignore = "requires file upload"]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_upload_file() -> Result<()> {
        let config = Config::new(env::var("OPENAI_API_KEY")?)
            .organization(Some(env::var("OPENAI_ORGANIZATION")?));
        let client = Client::new(&config);

        let param: UploadFileParam<&std::path::Path> = UploadFileParam::new()
            .file(Some(std::path::Path::new("../../resources/example.jsonl")))
            .purpose(Some(Purpose::FineTune));

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

        let resp = retrieve(&client, "file-1FZQ73L5AK8UknTTT0PxWMBE").await?;
        println!("{:#?}", resp);

        assert_eq!(resp.error().is_some(), true);
        Ok(())
    }
}
