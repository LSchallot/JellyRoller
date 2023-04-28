use reqwest::blocking::Response;

pub fn handle_unauthorized() {
    println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
}

pub fn handle_others(response: Response) {
    println!("Status Code: {}", response.status());
}