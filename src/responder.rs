use reqwest::{blocking::{Client, Response}, header::{CONTENT_TYPE, HeaderMap}};
use crate::utils::debug::{log_request, log_response};

pub fn simple_get(server_url: String, api_key: &str, query: Vec<(&str, &str)>) -> Response {
    log_request("GET", &server_url, None);
    
    let client = Client::new();
    let response = client
        .get(&server_url)
        .header("Authorization", format!("MediaBrowser Token=\"{api_key}\""))
        .query(&query)
        .send();
    if let Ok(resp) = response {
        log_response(resp.status().as_u16(), None);
        resp
    } else {
        eprintln!("[ERROR] GET request failed: {}", server_url);
        if let Err(e) = &response {
            eprintln!("[ERROR] {}", e);
        }
        std::process::exit(1);
    }
}

pub fn simple_post(server_url: String, api_key: &str, body: String, content_type: &str, query: &[(&str, &str)]) -> Response {
    log_request("POST", &server_url, Some(&body));
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(CONTENT_TYPE, content_type.parse().unwrap());

    if api_key.is_empty() {
        // Used when JellyRoller is not yet authenticated.
        headers.insert("Authorization","MediaBrowser Client=\"JellyRoller\", Device=\"jellyroller\", DeviceId=\"1\", Version=\"0.0.1\"".parse().unwrap());
    } else {
        // Used after JellyRoller has been authenticated.
        headers.insert("Authoriztion", format!("MediaBrowser Token=\"{api_key}\"").parse().unwrap());
    }

    let client = Client::new();
    let response = client
        .post(&server_url)
        .headers(headers)
        .body(body)
        .query(&query)
        .send();
    if let Ok(resp) = response {
        log_response(resp.status().as_u16(), None);
        resp
    } else {
        eprintln!("[ERROR] POST request failed: {}", server_url);
        if let Err(e) = &response {
            eprintln!("[ERROR] {}", e);
        }
        std::process::exit(1);
    }
}