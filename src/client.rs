use derive_getters::Getters;
use reqwest::{header::HeaderMap, multipart};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Config, Result};

/// The Client used to interact with the OpenAI API.
#[derive(Debug, Getters)]
pub struct Client<'a> {
    /// The HTTP client that'll execute requests.
    handler: reqwest::Client,

    /// Configuration needed to authorize against the API.
    config: &'a Config,
}

impl<'a> Client<'a> {
    pub fn new(config: &'a Config) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", config.api_key())
                .parse()
                .expect("Unable to parse the API key."),
        );
        if let Some(org) = &config.organization {
            headers.insert(
                "OpenAI-Organization",
                org.parse()
                    .expect("Unable to parse the given Organization."),
            );
        }

        Self {
            handler: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .expect("Err creating a request handler."),
            config,
        }
    }

    pub async fn get<X, Y, Z>(&self, identifier: X, param: Option<&Y>) -> Result<Z>
    where
        X: Into<String> + std::fmt::Display,
        Y: Serialize,
        Z: DeserializeOwned,
    {
        let resp = self
            .handler
            .get(format!("{}{}", self.config().url(), identifier))
            .query(&param)
            .send()
            .await?
            .json::<Z>()
            .await?;

        Ok(resp)
    }

    pub async fn post<X, Y, Z>(&self, identifier: X, param: Option<&Y>) -> Result<Z>
    where
        X: Into<String> + std::fmt::Display,
        Y: Serialize,
        Z: DeserializeOwned,
    {
        let resp = self
            .handler
            .post(format!("{}{}", self.config().url(), identifier))
            .json(&param)
            .send()
            .await?
            .json::<Z>()
            .await?;

        Ok(resp)
    }

    pub async fn delete<X, Y, Z>(&self, identifier: X, param: Option<&Y>) -> Result<Z>
    where
        X: Into<String> + std::fmt::Display,
        Y: Serialize,
        Z: DeserializeOwned,
    {
        let resp = self
            .handler
            .delete(format!("{}{}", self.config().url(), identifier))
            .query(&param)
            .send()
            .await?
            .json::<Z>()
            .await?;

        Ok(resp)
    }

    pub async fn post_data<X, Y, Z>(&self, identifier: X, param: multipart::Form) -> Result<Z>
    where
        X: Into<String> + std::fmt::Display,
        Z: DeserializeOwned,
    {
        let resp = self
            .handler
            .post(format!("{}{}", self.config().url(), identifier))
            .multipart(param)
            .send()
            .await?
            .json::<Z>()
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {}
