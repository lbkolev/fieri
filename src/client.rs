use derive_getters::Getters;
use reqwest::header::HeaderMap;
use serde::{de::DeserializeOwned, Serialize};

use crate::{config::Config, Result};

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
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            "Authorization",
            format!("Bearer {}", config.api_key())
                .parse()
                .expect("Unable to parse the API key."),
        );

        if let Some(org) = config.organization().clone() {
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

    pub async fn get<T, Y>(&self, identifier: String, params: Option<T>) -> Result<Y>
    where
        T: Serialize,
        Y: DeserializeOwned,
    {
        let resp = self
            .handler()
            .get(format!("{}{}", self.config().url(), identifier))
            .query(&params)
            .send()
            .await?
            .json::<Y>()
            .await?;

        Ok(resp)
    }

    pub async fn post<T, Y>(&self, identifier: String, params: Option<T>) -> Result<Y>
    where
        T: Serialize,
        Y: DeserializeOwned,
    {
        let resp = self
            .handler()
            .post(format!("{}{}", self.config().url(), identifier))
            .form(&params)
            .send()
            .await?
            .json::<Y>()
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {}
