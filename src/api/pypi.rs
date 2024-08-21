use crate::api::client;

pub struct PypiApi {
    base_url: String,
    cli: client::Client,
}

impl PypiApi {
    pub fn new() -> PypiApi {
        PypiApi {
            base_url: String::from("https://pypi.org/pypi"),
            cli: client::Client::new(),
        }
    }

    pub async fn get_pkg_info(&self, pkg: &str) -> Option<serde_json::Value> {
        let url = format!("{}/{}/json", self.base_url, pkg);

        match self
            .cli
            .perform_request(reqwest::Method::GET, url, None)
            .await
        {
            Ok(response) => Some(response["info"].clone()),
            Err(err) => {
                eprintln!("There was some error getting package info. Error: {}", err);
                None
            }
        }
    }
}
