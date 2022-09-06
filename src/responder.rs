use reqwest::{blocking::Client, blocking::Response, header::CONTENT_TYPE};

pub fn simple_get(server_url: String, api_key: String) -> Response {
    let client = Client::new();
    let response = client
        .get(server_url)
        .header("X-Emby-Token", api_key)
        .send();
    match response {
        Ok(resp) => resp,
        Err(_) => {
            println!("Post response error.");
            std::process::exit(0)
        }
    }
}

pub fn simple_post(server_url: String, api_key: String, body: String) -> Response {
    let client = Client::new();
    let response = client
        .post(server_url)
        .header(CONTENT_TYPE, "application/json")
        .header("X-Emby-Token", api_key)
        .body(body)
        .send();
    match response {
        Ok(resp) => resp,
        Err(_) => {
            println!("Post response error.");
            std::process::exit(0)
        }
    }
}