<div align="center">
    <a href="https://github.com/lbkolev/fieri">
        <img width="1250px" height="120px" src=".github/logo.png">
    </a>
</div>

# <p align="center">fieri</p>

<p align="center">
    <a href="https://github.com/lbkolev/fieri/blob/master/LICENSE">
        <img src="https://img.shields.io/badge/license-MIT-blue.svg">
    </a>
    <a href="https://crates.io/crates/fieri">
        <img src="https://img.shields.io/crates/v/fieri.svg">
    </a>
    <a href="https://github.com/lbkolev/fieri/actions?query=workflow%3ACI+branch%3Amaster">
        <img src="https://github.com/lbkolev/fieri/actions/workflows/ci.yml/badge.svg">
    </a>
    <a href="https://github.com/lbkolev/fieri/actions?query=workflow%3TESTS+branch%3Amaster">
        <img src="https://github.com/lbkolev/fieri/actions/workflows/tests.yml/badge.svg">
    </a>
    <a href="https://docs.rs/fieri">
        <img src="https://img.shields.io/docsrs/fieri/latest">
    </a>
</p>

> **Note**: fieri's [master](https://github.com/lbkolev/fieri) branch might
> contain breaking changes. For the most recently *released* code, look to the latest tag.

## Overview
### Unofficial Rust client for the OpenAI's API.

fieri provides an asynchronous Rust interface for interacting with the OpenAI API, allowing you to easily use OpenAI's state-of-the-art machine learning models in your Rust projects.

### Features
#### `models`
- [x] [list](https://beta.openai.com/docs/api-reference/models/list)
- [x] [retrieve](https://beta.openai.com/docs/api-reference/models/retrieve)

#### `completions`
- [x] [create](https://beta.openai.com/docs/api-reference/completions/create)
- [ ] [create with stream](https://beta.openai.com/docs/api-reference/completions/create#completions/create-stream)

#### `edits`
- [x] [create](https://beta.openai.com/docs/api-reference/edits/create)

#### `images`
- [x] [create image](https://beta.openai.com/docs/api-reference/images/create)
- [x] [create image edit](https://beta.openai.com/docs/api-reference/images/create-edit)
- [x] [create image variation](https://beta.openai.com/docs/api-reference/images/create-variation)

#### `embeddings`
- [x] [create](https://beta.openai.com/docs/api-reference/embeddings/create)

#### `files`
- [x] [list](https://beta.openai.com/docs/api-reference/files/list)
- [x] [upload](https://beta.openai.com/docs/api-reference/files/upload)
- [x] [delete](https://beta.openai.com/docs/api-reference/files/delete)
- [x] [retrieve](https://beta.openai.com/docs/api-reference/files/retrieve)
- [ ] [retrieve content](https://beta.openai.com/docs/api-reference/files/retrieve-content)

#### `fine-tunes`
- [x] [create](https://beta.openai.com/docs/api-reference/fine-tunes/create)
- [x] [list](https://beta.openai.com/docs/api-reference/fine-tunes/list)
- [x] [retrieve](https://beta.openai.com/docs/api-reference/fine-tunes/retrieve)
- [x] [cancel](https://beta.openai.com/docs/api-reference/fine-tunes/cancel)
- [x] [list events](https://beta.openai.com/docs/api-reference/fine-tunes/events)
- [ ] [list events with stream](https://beta.openai.com/docs/api-reference/fine-tunes/events#fine-tunes/events-stream)
- [x] [delete](https://beta.openai.com/docs/api-reference/fine-tunes/delete-model)

#### `moderations`
- [x] [create](https://beta.openai.com/docs/api-reference/moderations/create)

#### `/engines` - *DEPRECATED*

## Prerequisites
Before you can use the Rust Client for OpenAI, you'll need to sign up for an API key at the OpenAI Developer Portal. Once you've signed up, you'll be able to find your API key in the [API Keys](https://beta.openai.com/account/api-keys) section of the developer portal.

## Installation
To use the client in your project, add the following to your `Cargo.toml` file:
```toml
[dependencies]
fieri = "0.3"
```

## Basic Usage

### Generate an image based on a prompt and save it locally.
```rust
use std::env;
use fieri::{
    Client, Error,
    image::{ImageSize, GenerateImageParamBuilder, generate},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new(env::var("OPENAI_API_KEY")?);

    let param = GenerateImageParamBuilder::new("A bunch of cats dancing tango on the top of the highest mountain on Mars.")
        .size(ImageSize::S1024x1024)
        .n(1)
        .build()?;

    generate(&client, &param)
        .await?
        .save("/tmp/")
        .await?;

    Ok(())
}
}
```

### Generate text based on a prompt
```rust
use std::env;
use fieri::{
    Client, Error,
    completion::{CompletionParamBuilder, create}
};

#[tokio::main]
async fn main() -> std::result::Result<(), Error> {
    let client = Client::new(env::var("OPENAI_API_KEY")?);

    let param = CompletionParamBuilder::new("ada")
        .prompt("Generate a plot for an absurd interstellar parody.")
        .max_tokens(500)
        .temperature(0.9)
        .top_p(1.0)
        .frequency_penalty(0.0)
        .presence_penalty(0.0)
        .build()?;

    let resp = create(&client, &param).await?;
    println!("Generated text: {:#?}", resp);

    Ok(())
}
```

Examples for each implemented endpoint can be found in the [docs](https://docs.rs/fieri).

## Documentation
### [fieri Documentation](https://docs.rs/fieri/)
### [The official OpenAI documentation](https://beta.openai.com/docs/introduction/overview)

## Limitations
Note that the Rust Client for OpenAI is provided as-is, and is not officially supported by OpenAI. While we will do our best to keep the library up-to-date and bug-free, we cannot guarantee that it will always work as expected.

Additionally, the API has usage limits that may affect your ability to use the models. You can view your current usage and limits in the [Usage](https://beta.openai.com/account/usage) section of your account.

## License
fieri is provided under the MIT license. See [LICENSE](LICENSE).