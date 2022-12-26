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
    <a href="https://github.com/lbkolev/fieri/actions?query=workflow%3ACI+branch%3Amaster">
        <img src="https://github.com/lbkolev/fieri/actions/workflows/ci.yml/badge.svg">
    </a>
    <a href="https://github.com/lbkolev/fieri/actions?query=workflow%3TESTS+branch%3Amaster">
        <img src="https://github.com/lbkolev/fieri/actions/workflows/tests.yml/badge.svg">
    </a>
</p>

> **Note**: fieri's [master](https://github.com/lbkolev/fieri) branch might
> contain breaking changes. For the most recently *released* code, look to the latest tag.

## Introduction
### Unofficial Rust client for the OpenAI API's GPT-3 & DALL·E.

This library provides a Rust interface for interacting with the OpenAI API, allowing you to easily use OpenAI's state-of-the-art machine learning models in your Rust projects.

## Prerequisites
Before you can use the Rust Client for OpenAI, you'll need to sign up for an API key at the OpenAI Developer Portal. Once you've signed up, you'll be able to find your API key in the API Keys section of the developer portal.

## Installation
To use the Rust Client for OpenAI in your project, add the following to your Cargo.toml file:
```toml
[dependencies]
fieri = "0.1"
```

## Usage
To use the Rust Client for OpenAI, you'll first need to create a client object:
```rust
use fieri::Client;

let client = Client::new(env::var("OPENAI_API_KEY")?);
```

## Examples
### ...
```rust
```

### ...
```rust
```

More examples can be found in the [examples/](examples) directory and the [docs](https://docs.rs/fieri).

## Documentation
### [Client Documentation](https://docs.rs/fieri/)
### [The official OpenAI docs](https://beta.openai.com/docs/introduction/overview)

## Limitations
Note that the Rust Client for OpenAI is provided as-is, and is not officially supported by OpenAI. While we will do our best to keep the library up-to-date and bug-free, we cannot guarantee that it will always work as expected.

Additionally, the API has usage limits that may affect your ability to use the models. You can view your current usage and limits in the [Usage](https://beta.openai.com/account/usage) section of your account.

## License
fieri is provided under the MIT license. See [LICENSE](LICENSE).