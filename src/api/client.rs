#[derive(Debug)]
pub(crate) struct Client {
    client: reqwest::Client,
}
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Forbidden to access resource {0}")]
    Forbidden(String),
    #[error("Resource not found {0}")]
    NotFound(String),
    #[error("Server error {0}")]
    Server(#[source] reqwest::Error),
}

impl Client {
    pub(crate) fn new() -> Client {
        Client {
            client: reqwest::Client::new(),
        }
    }

    pub(crate) async fn perform_request(
        &self,
        method: reqwest::Method,
        url: String,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, ApiError> {
        let mut request_builder = self.client.request(method, url);
        if body.is_some() {
            request_builder = request_builder
                .json(&body.unwrap())
                .header("Content-Type", "application/json");
        }

        let response: reqwest::Response = request_builder.send().await.map_err(ApiError::Server)?;

        let status = response.status();
        let response_body: serde_json::Value = response.json().await.map_err(ApiError::Server)?;

        match status {
            StatusCode::NOT_FOUND => Err(ApiError::NotFound(response_body.to_string())),
            StatusCode::FORBIDDEN => Err(ApiError::Forbidden(response_body.to_string())),
            _ => Ok(response_body),
        }
    }
}
