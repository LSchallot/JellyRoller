pub struct ServerInfo {
    pub server_url: String,
    pub api_key: String
}

impl ServerInfo {
    pub fn new(endpoint: &str, server_url: &str, api_key: &str) -> ServerInfo {
        ServerInfo {
            server_url: format!("{}{}",server_url, endpoint),
            api_key: api_key.to_owned()
        }
    }
}