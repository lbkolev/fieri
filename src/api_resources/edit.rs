//! Given a prompt and an instruction, the model will return an edited version of the prompt.
//!
//! The edits endpoint can be used to edit text, rather than just completing it. You provide some text and an instruction for how to modify it.
//!
//! This is a natural interface for translating, editing, and tweaking text. This is also useful for refactoring and working with code.

use crate::{
    types::{Edit, EditParam},
    Client, Result,
};

/// Creates a new edit for the provided input, instruction, and parameters.
///
/// Related OpenAI docs: [Create an Edit](https://beta.openai.com/docs/api-reference/edits/create)
///
/// ## Example
/// ```no_run
/// use fieri::{Client, edit::{create, EditParamBuilder}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new();
///
///     let param = EditParamBuilder::new("text-davinci-edit-001", "Fix the spelling mistakes")
///         .input("What dey of the wek is it?")
///         .temperature(0.5)
///         .build()?;
///
///     let resp = create(&client, &param).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn create(client: &Client, param: &EditParam) -> Result<Edit> {
    client.create_edit(param).await
}

impl Client {
    async fn create_edit(&self, param: &EditParam) -> Result<Edit> {
        self.post::<EditParam, Edit>("edits", Some(param)).await
    }
}

#[cfg(test)]
mod tests {}
