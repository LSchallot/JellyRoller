use reqwest::{blocking::Client, blocking::Response, header::CONTENT_TYPE};
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
        log_response(resp.status().as_u16(), resp.status().as_str(), None);
        resp
    } else {
        eprintln!("[ERROR] GET request failed: {}", server_url);
        if let Err(e) = &response {
            eprintln!("[ERROR] {}", e);
        }
        std::process::exit(1);
    }
}

pub fn simple_post(server_url: String, api_key: &str, body: String, content_type: &str) -> Response {
    log_request("POST", &server_url, Some(&body));
    
    let client = Client::new();
    let response = client
        .post(&server_url)
        .header(CONTENT_TYPE, content_type)
        .header("Authorization", format!("MediaBrowser Token=\"{api_key}\""))
        .body(body)
        .send();
    if let Ok(resp) = response {
        log_response(resp.status().as_u16(), resp.status().as_str(), None);
        resp
    } else {
        eprintln!("[ERROR] POST request failed: {}", server_url);
        if let Err(e) = &response {
            eprintln!("[ERROR] {}", e);
        }
        std::process::exit(1);
    }
}

pub fn simple_post_with_query(
    server_url: String,
    api_key: &str,
    body: String,
    query: &[(&str, &str)],
) -> Response {
    log_request("POST", &server_url, Some(&body));
    
    let client = Client::new();
    let response = client
        .post(&server_url)
        .header(CONTENT_TYPE, "application/json")
        .header("Authorization", format!("MediaBrowser Token=\"{api_key}\""))
        .body(body)
        .query(&query)
        .send();
    if let Ok(resp) = response {
        log_response(resp.status().as_u16(), resp.status().as_str(), None);
        resp
    } else {
        eprintln!("[ERROR] POST request failed: {}", server_url);
        if let Err(e) = &response {
            eprintln!("[ERROR] {}", e);
        }
        std::process::exit(1);
    }
}