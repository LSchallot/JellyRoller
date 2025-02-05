use super::{
    handle_others, handle_unauthorized, responder::simple_get, PluginDetails, PluginRootJson,
};
use reqwest::StatusCode;

#[derive(Clone)]
pub struct PluginInfo {
    server_url: String,
    api_key: String,
}

impl PluginInfo {
    pub fn new(endpoint: &str, server_url: &str, api_key: String) -> PluginInfo {
        PluginInfo {
            server_url: format!("{}{}", server_url, endpoint),
            api_key,
        }
    }

    pub fn get_plugins(self) -> Result<Vec<PluginDetails>, Box<dyn std::error::Error>> {
        let response = simple_get(self.server_url, self.api_key, Vec::new());
        match response.status() {
            StatusCode::OK => {
                let json = response.text()?;
                let plugins = serde_json::from_str::<PluginRootJson>(&json)?;
                return Ok(plugins);
            }
            StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            }
            _ => {
                handle_others(response);
            }
        }

        Ok(Vec::new())
    }
}
