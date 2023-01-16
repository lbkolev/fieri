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
### Unofficial Rust client for the OpenAI API.

fieri provides an asynchronous Rust interface for interacting with the OpenAI API, allowing you to easily use OpenAI's state-of-the-art machine learning models in your Rust projects.

## Prerequisites
Before you can use the Rust Client for OpenAI, you'll need to sign up for an API key at the OpenAI Developer Portal. Once you've signed up, you'll be able to find your API key in the [API Keys](https://beta.openai.com/account/api-keys) section of the developer portal.

## Installation
To use the client in your project, add the following to your `Cargo.toml` file:
```toml
[dependencies]
fieri = "0.5"
```

## Basic Usage

### Generate text based on a prompt
```rust
use fieri::{
    completion::{create, CompletionParamBuilder},
    Client, Error,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
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

### Generate and stream back text based on a prompt
```rust
use fieri::{
    completion::{create_with_stream, Completion, CompletionParamBuilder},
    Client, Error,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new(env::var("OPENAI_API_KEY")?);

    let param = CompletionParamBuilder::new("ada")
        .prompt("unnecessarily lo")
        .temperature(0.5)
        .build()?;

    let mut resp = create_with_stream(&client, &param).await?;

    while let Some(chunk) = resp.chunk().await? {
        if chunk.to_vec() == b"data: [DONE]\n\n" {
            break;
        }

        let v: Completion = serde_json::from_slice(&chunk[5..])?;
        v.choices.iter().for_each(|c| println!("{:?}", c.text));
    }

    Ok(())
}
```

### Generate an image based on a prompt and save it locally.
```rust
use fieri::{
    image::{ImageSize, GenerateImageParamBuilder, generate},
    Client, Error,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new(env::var("OPENAI_API_KEY")?);

    let param = GenerateImageParamBuilder::new("A bunch of cats dancing tango on top of the highest mountain on Mars.")
        .size(ImageSize::S1024x1024)
        .n(1)
        .build()?;

    generate(&client, &param)
        .await?
        .save("/tmp/")
        .await?;

    Ok(())
}
```

Examples for each endpoint can be found in the [docs](https://docs.rs/fieri).

## Limitations
Note that the Rust Client for OpenAI is provided as-is, and is not officially supported by OpenAI. While we will do our best to keep the library up-to-date and bug-free, we cannot guarantee that it will always work as expected.

Additionally, the API has usage limits that may affect your ability to use the models. You can view your current usage and limits in the [Usage](https://beta.openai.com/account/usage) section of your account.

## License
fieri is provided under the MIT license. See [LICENSE](LICENSE).
