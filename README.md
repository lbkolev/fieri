<div align="center">
    <a href="https://github.com/lbkolev/openai-rs">
        <img width="1250px" height="120px" src=".github/logo.png">
    </a>
</div>

# <p align="center">openai-rs</p>

<p align="center">
    <a href="https://github.com/lbkolev/openai-rs/blob/master/LICENSE">
        <img src="https://img.shields.io/badge/license-MIT-blue.svg">
    </a>
    <a href="https://github.com/lbkolev/openai-rs/actions?query=workflow%3ACI+branch%3Amaster">
        <img src="https://github.com/lbkolev/openai-rs/actions/workflows/ci.yml/badge.svg">
    </a>
</p>

## Introduction
### Unofficial Rust client for the OpenAI API.
This library provides a Rust interface for interacting with the OpenAI API, allowing you to easily use OpenAI's state-of-the-art machine learning models in your Rust projects.

## Prerequisites
Before you can use the Rust Client for OpenAI, you'll need to sign up for an API key at the OpenAI Developer Portal. Once you've signed up, you'll be able to find your API key in the API Keys section of the developer portal.

## Installation
To use the Rust Client for OpenAI in your project, add the following to your Cargo.toml file:
```toml
[dependencies]
openai_rs = "0.1"
```

## Usage
To use the Rust Client for OpenAI, you'll first need to create a client object:
```rust
use openai_rs::Client;

let client = Client::new(env::var("OPENAI_API_KEY")?);
```

## Examples
### ...

### ...


## Documentation


## Limitations
Note that the Rust Client for OpenAI is provided as-is, and is not officially supported by OpenAI. While we will do our best to keep the library up-to-date and bug-free, we cannot guarantee that it will always work as expected.

Additionally, the API has usage limits that may affect your ability to use the models. You can view your current usage and limits in the [Usage](https://beta.openai.com/account/usage) section of your account.

## License
OpenAI-rs is provided under the MIT license. See [LICENSE](LICENSE).