//! Files are used to upload documents that can be used with features like [`Fine-tuning`](crate::api_resources::fine_tune).

use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fs, path::Path};

use crate::{
    types::{Delete, File, ListFiles, Purpose},
    Client, Result,
};

/// Returns a [`list`][ListFiles] of files that belong to the user's organization.
///
/// Related OpenAI docs: [List Files](https://beta.openai.com/docs/api-reference/files/list)
///
/// ## Example
/// ```no_run
/// use fieri::{Client, file::list};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new()
///         .organization("org-..");
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
/// use std::path::Path;
/// use fieri::{Client, file::{Purpose, upload}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new()
///         .organization("org-..");
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
/// use fieri::{Client, file::delete};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new()
///         .organization("org-..");
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
/// use fieri::{Client, file::retrieve, api_resources::File};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new()
///         .organization("org-..");
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
mod tests {}
