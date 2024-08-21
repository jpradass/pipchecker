#[derive(Debug)]
pub(crate) struct Client {
    client: reqwest::Client,
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
    ) -> Result<serde_json::Value, reqwest::Error> {
        let mut request_builder = self.client.request(method, url);
        if body.is_some() {
            request_builder = request_builder
                .json(&body.unwrap())
                .header("Content-Type", "application/json");
        }

        let response: reqwest::Response = request_builder.send().await?;
        let response_body: serde_json::Value = response.json().await?;

        Ok(response_body)
    }
}
