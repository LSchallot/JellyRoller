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
