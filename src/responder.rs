use reqwest::{blocking::Client, blocking::Response, header::CONTENT_TYPE};

pub fn simple_get(server_url: String, api_key: String, query: Vec<(&str, &str)>) -> Response {
    let client = Client::new();
    let response = client
        .get(server_url)
        .header("Authorization", format!("MediaBrowser Token=\"{api_key}\""))
        .query(&query)
        .send();
    if let Ok(resp) = response { resp } else {
        println!("Post response error.");
        std::process::exit(1);
    }
}

pub fn simple_post(server_url: String, api_key: String, body: String) -> Response {
    let client = Client::new();
    let response = client
        .post(server_url)
        .header(CONTENT_TYPE, "application/json")
        .header("Authorization", format!("MediaBrowser Token=\"{api_key}\""))
        .body(body)
        .send();
    if let Ok(resp) = response { resp } else {
        println!("Post response error.");
        std::process::exit(1);
    }
}

pub fn simple_post_image(server_url: String, api_key: String, body: String) -> Response {
    let client = Client::new();
    let response = client
        .post(server_url)
        .header(CONTENT_TYPE, "image/png")
        .header("Authorization", format!("MediaBrowser Token=\"{api_key}\""))
        .body(body)
        .send();
    if let Ok(resp) = response { resp } else {
        println!("Post response error.");
        std::process::exit(1);
    }
}