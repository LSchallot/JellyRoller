use reqwest::blocking::Response;

pub fn handle_unauthorized() {
    eprintln!("[ERROR] Authentication failed (HTTP 401 Unauthorized)");
    eprintln!("[HINT] Please run 'jellyroller reconfigure' to re-authenticate");
    std::process::exit(1);
}

pub fn handle_others(response: &Response) {
    eprintln!("[ERROR] Request failed with status: {} {}", 
        response.status().as_u16(), 
        response.status().canonical_reason().unwrap_or("Unknown")
    );
    std::process::exit(1);
}

/// Handle HTTP errors with response body for better debugging
pub fn handle_error_with_body(status_code: u16, status_text: &str, body: Option<&str>) {
    eprintln!("[ERROR] Request failed with status: {} {}", status_code, status_text);
    if let Some(b) = body {
        if !b.is_empty() && super::debug::is_verbose() {
            eprintln!("[DEBUG] Response body:");
            eprintln!("{}", b);
        }
    }
    std::process::exit(1);
}
