//! The Client used to establish a connection and interact with the OpenAI API.
//!
//! ## Usage
//! ```no_run
//! use std::env;
//! use fieri::Client;
//!
//! let client = Client::new(env::var("OPENAI_API_KEY").unwrap());
//! ```
//!
//! ## Usage with a specified [Organization](https://beta.openai.com/docs/api-reference/requesting-organization)
//! ```no_run
//! use std::env;
//! use fieri::Client;
//!
//! let client = Client::new(env::var("OPENAI_API_KEY").unwrap())
//!     .organization(env::var("OPENAI_ORGANIZATION").unwrap());
//! ```

use std::fmt::Debug;

use derive_getters::Getters;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    multipart,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    config::Config,
    error::{Error, RequestError},
    Result,
};

// Response returned by each interaction with OpenAI, either an error or a valid generic.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Response<T> {
    Invalid(RequestError),
    Valid(T),
}

/// The Client used to interact with the OpenAI API.
#[derive(Clone, Debug, Getters)]
pub struct Client {
    /// Configuration needed to authorize against the API.
    config: Config,

    /// The HTTP client that'll execute requests.
    handler: reqwest::Client,
}

impl Client {
    pub fn new<T: Into<String> + std::fmt::Display>(api_key: T) -> Self {
        let mut headers = HeaderMap::new();

        headers.insert(
            AUTHORIZATION,
            format!("Bearer {api_key}")
                .parse()
                .expect("Unable to parse the API key."),
        );
        let config = Config::new(api_key).headers(headers.clone());

        Self {
            config,
            handler: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .expect("Err creating a request handler."),
        }
    }

    /// For users who belong to multiple organizations, you can pass a header
    /// to specify which organization is used for an API request.
    pub fn organization(mut self, organization: String) -> Self {
        let mut headers = self.config.headers;
        headers.insert(
            "OpenAI-Organization",
            organization
                .parse()
                .expect("Unable to parse the given Organization."),
        );

        self.config.organization = organization;
        self.config.headers = headers.clone();

        Self {
            config: self.config,
            handler: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .expect("Err creating a request handler."),
        }
    }

    pub async fn get<X, Y>(&self, identifier: &str, param: Option<&X>) -> Result<Y>
    where
        X: Serialize,
        Y: DeserializeOwned,
    {
        let resp = self
            .handler
            .get(self.config().url().join(identifier)?)
            .query(&param)
            .send()
            .await?
            .json::<Response<Y>>()
            .await?;
        //println!("{:?}", resp);

        match resp {
            Response::Invalid(resp) => Err(Error::APIError(resp)),
            Response::Valid(resp) => Ok(resp),
        }
    }

    pub async fn post<X, Y>(&self, identifier: &str, param: Option<&X>) -> Result<Y>
    where
        X: Serialize,
        Y: DeserializeOwned,
    {
        let resp = self
            .handler
            .post(self.config().url().join(identifier)?)
            .json(&param)
            .send()
            .await?
            .json::<Response<Y>>()
            .await?;

        match resp {
            Response::Invalid(resp) => Err(Error::APIError(resp)),
            Response::Valid(resp) => Ok(resp),
        }
    }

    pub async fn delete<X, Y>(&self, identifier: &str, param: Option<&X>) -> Result<Y>
    where
        X: Serialize,
        Y: DeserializeOwned,
    {
        let resp = self
            .handler
            .delete(self.config().url().join(identifier)?)
            .query(&param)
            .send()
            .await?
            .json::<Response<Y>>()
            .await?;

        match resp {
            Response::Invalid(resp) => Err(Error::APIError(resp)),
            Response::Valid(resp) => Ok(resp),
        }
    }

    pub async fn post_data<Y>(&self, identifier: &str, data: multipart::Form) -> Result<Y>
    where
        Y: DeserializeOwned,
    {
        let resp = self
            .handler
            .post(self.config().url().join(identifier)?)
            .multipart(data)
            .send()
            .await?
            .json::<Response<Y>>()
            .await?;

        match resp {
            Response::Invalid(resp) => Err(Error::APIError(resp)),
            Response::Valid(resp) => Ok(resp),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_client() -> Result<()> {
        let client = Client::new(std::env::var("OPENAI_API_KEY")?);

        assert!(client.config().headers.get(AUTHORIZATION).is_some());
        assert!(client.config().headers.get("OpenAI-Organization").is_none());

        let client = Client::new(std::env::var("OPENAI_API_KEY")?)
            .organization(std::env::var("OPENAI_ORGANIZATION")?);

        assert!(client.config().headers.get(AUTHORIZATION).is_some());
        assert!(client.config().headers.get("OpenAI-Organization").is_some());

        Ok(())
    }
}
