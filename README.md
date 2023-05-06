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
    <a href="https://docs.rs/fieri">
        <img src="https://img.shields.io/docsrs/fieri/latest">
    </a>
</p>

## Overview
### Unofficial Rust client for OpenAI.

fieri provides an asynchronous Rust interface for interaction with OpenAI, allowing you to easily use OpenAI's state-of-the-art machine learning models in your Rust projects.

## Prerequisites
Before you can use the Rust Client for OpenAI, you'll need to sign up for an API key at the OpenAI Developer Portal. Once you've signed up, you'll be able to find your API key in the [API Keys](https://beta.openai.com/account/api-keys) section of the developer portal.

## Installation
Run `cargo add fieri` in your terminal to add the latest version of the client.


## ChatGPT
```rust
use fieri::{
    chat::{chat, ChatMessageBuilder, ChatParamBuilder},
    Client, Error,
};

let client = Client::new();
let message = ChatMessageBuilder::new("user", "Hello!").build()?;
let param = ChatParamBuilder::new("gpt-3.5-turbo", vec![message]).build()?;

let resp = chat(&client, &param).await?;
println!("{:#?}", resp);
```

By default, the api key and organization are implicitly loaded from environment variables `OPENAI_API_KEY` & `OPENAI_ORGANIZATION`. It's possible to configure/overwrite them per client, using for example:
```rust
use fieri::Client

let client = Client::new().api_key("<key>");
let client_with_org = Client::new().organization("<organization>");
```

More examples can be found in the [docs](https://docs.rs/fieri).

## Limitations
Note that the Rust Client for OpenAI is provided as-is, and is not officially supported by OpenAI. While we will do our best to keep the library up-to-date and bug-free, we cannot guarantee that it will always work as expected.

Additionally, the API has usage limits that may affect your ability to use the models. You can view your current usage and limits in the [Usage](https://beta.openai.com/account/usage) section of your account.

## License
fieri is provided under the MIT license. See [LICENSE](LICENSE).
