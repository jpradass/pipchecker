use crate::api::client;

use super::client::ApiError;

// pub struct PypiApi {
//     base_url: String,
//     cli: client::Client,
// }

const BASE_URL: &str = "https://pypi.org/pypi";

// impl PypiApi {
//     pub fn new() -> PypiApi {
//         PypiApi {
//             base_url: String::from("https://pypi.org/pypi"),
//             cli: client::Client::new(),
//         }
//     }

pub async fn get_pkg_info(pkg: &str) -> Option<serde_json::Value> {
    match pypi_request(pkg).await {
        Ok(response) => Some(response["info"].clone()),
        Err(err) => {
            match err {
                ApiError::NotFound(_) => println!("Package {pkg} not found on PyPi"),
                ApiError::Forbidden(e) => println!("Couldn't access Pypi API. Error: {e}"),
                ApiError::Server(e) => {
                    println!("There was an error on Pypi when processing request. Error: {e}")
                }
            }
            None
        }
    }
}

pub async fn get_pkg_urls(pkg: &str) -> Option<serde_json::Value> {
    match pypi_request(pkg).await {
        Ok(response) => Some(response["urls"].clone()),
        Err(err) => {
            eprintln!("There was some error getting package info. Error: {}", err);
            None
        }
    }
}

pub async fn get_pkg_vulnerabilities(pkg: &str) -> Option<serde_json::Value> {
    match pypi_request(pkg).await {
        Ok(response) => Some(response["vulnerabilities"].clone()),
        Err(err) => {
            eprintln!("There was some error getting package info. Error: {}", err);
            None
        }
    }
}

pub async fn get_pkg_releases(pkg: &str) -> Option<serde_json::Value> {
    match pypi_request(pkg).await {
        Ok(response) => Some(response["releases"].to_owned()),
        Err(err) => {
            eprintln!("There was some error getting package info. Error: {}", err);
            None
        }
    }
}

async fn pypi_request(pkg: &str) -> Result<serde_json::Value, ApiError> {
    let url = format!("{}/{}/json", BASE_URL, pkg);

    client::Client::new()
        .perform_request(reqwest::Method::GET, url, None)
        .await
}
// }
