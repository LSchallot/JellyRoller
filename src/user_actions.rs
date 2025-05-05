use crate::entities::token_details::TokenDetails;

use super::{
    handle_others, handle_unauthorized,
    responder::{simple_get, simple_post},
    Policy, UserDetails,
};
use reqwest::{
    blocking::Client,
    header::{CONTENT_LENGTH, CONTENT_TYPE},
    StatusCode,
};

#[derive(Serialize, Deserialize)]
pub struct UserWithPass {
    #[serde(rename = "Name")]
    username: Option<String>,
    #[serde(rename = "NewPw")]
    pass: Option<String>,
    #[serde(rename = "CurrentPw")]
    currentpwd: Option<String>,
    server_url: String,
    auth_key: String,
}

impl UserWithPass {
    pub fn new(
        username: Option<String>,
        pass: Option<String>,
        currentpwd: Option<String>,
        server_url: String,
        auth_key: String,
    ) -> UserWithPass {
        UserWithPass {
            //username: Some(username.unwrap_or_else(|| String::new())),
            username: Some(username.unwrap_or_default()),
            pass: Some(pass.unwrap_or_default()),
            currentpwd: Some(currentpwd.unwrap_or_default()),
            server_url,
            auth_key,
        }
    }

    pub fn resetpass(self) -> Result<(), Box<dyn std::error::Error>> {
        let response = simple_post(
            self.server_url.clone(),
            self.auth_key.clone(),
            serde_json::to_string_pretty(&self)?,
        );
        match response.status() {
            StatusCode::NO_CONTENT => {
                println!("Password updated successfully.");
            }
            StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            }
            _ => {
                println!("{}", response.status());
                handle_others(response);
            }
        }

        Ok(())
    }

    pub fn create_user(self) -> Result<(), Box<dyn std::error::Error>> {
        let response = simple_post(
            self.server_url.clone(),
            self.auth_key.clone(),
            serde_json::to_string_pretty(&self)?,
        );
        match response.status() {
            StatusCode::OK => {
                println!("User \"{}\" successfully created.", &self.username.unwrap());
            }
            StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            }
            _ => {
                handle_others(response);
            }
        }

        Ok(())
    }

    pub fn delete_user(self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let apikey = self.auth_key;
        let response = client
            .delete(self.server_url)
            .header("Authorization", format!("MediaBrowser Token=\"{apikey}\""))
            .header(CONTENT_TYPE, "application/json")
            .send()?;
        match response.status() {
            StatusCode::NO_CONTENT => {
                println!("User \"{}\" successfully removed.", &self.username.unwrap());
            }
            StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            }
            _ => {
                handle_others(response);
            }
        }

        Ok(())
    }

    pub fn create_api_token(self) {
        let client = Client::new();
        let apikey = self.auth_key;
        let response = client
            .post(self.server_url)
            .header("Authorization", format!("MediaBrowser Token=\"{apikey}\""))
            .header(CONTENT_LENGTH, 0)
            .query(&[("app", "JellyRoller")])
            .send()
            .unwrap();

        match response.status() {
            StatusCode::NO_CONTENT => {
                println!("API key created.");
            }
            _ => {
                handle_others(response);
            }
        }
    }

    pub fn retrieve_api_token(self) -> Result<String, Box<dyn std::error::Error>> {
        let response = simple_get(self.server_url, self.auth_key, Vec::new());
        match response.status() {
            StatusCode::OK => {
                let tokens = serde_json::from_str::<TokenDetails>(&response.text()?)?;
                for token in tokens.items {
                    if token.app_name == "JellyRoller" {
                        return Ok(token.access_token);
                    }
                }
            }
            StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            }
            _ => {
                handle_others(response);
            }
        }
        Ok(String::new())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAuthJson {
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    #[serde(rename = "ServerId")]
    pub server_id: String,
}

pub type UserInfoVec = Vec<UserDetails>;

#[derive(Serialize, Deserialize)]
pub struct UserAuth {
    server_url: String,
    username: String,
    pw: String,
}

impl UserAuth {
    pub fn new(server_url: &str, username: &str, password: String) -> UserAuth {
        UserAuth {
            server_url: format!("{}/Users/authenticatebyname", server_url),
            username: username.to_owned(),
            pw: password,
        }
    }

    pub fn auth_user(self) -> Result<String, Box<dyn std::error::Error>> {
        let client = Client::new();
        let response = client
            .post(self.server_url.clone())
            .header(CONTENT_TYPE, "application/json")
            .header("Authorization", "MediaBrowser Client=\"JellyRoller\", Device=\"jellyroller\", DeviceId=\"1\", Version=\"0.0.1\"")
            .body(serde_json::to_string_pretty(&self)?)
            .send()?;
        match response.status() {
            StatusCode::OK => {
                let result = response.json::<UserAuthJson>()?;
                println!("[INFO] User authenticated successfully.");
                Ok(result.access_token)
            }
            _ => {
                // Panic since the application requires an authenticated user
                handle_others(response);
                panic!("[ERROR] Unable to authenticate user.  Please assure your configuration information is correct.\n");
            }
        }
    }
}

#[derive(Clone)]
pub struct UserList {
    server_url: String,
    api_key: String,
}

impl UserList {
    pub fn new(endpoint: &str, server_url: &str, api_key: &str) -> UserList {
        UserList {
            server_url: format!("{}{}", server_url, endpoint),
            api_key: api_key.to_string(),
        }
    }

    pub fn list_users(self) -> Result<Vec<UserDetails>, Box<dyn std::error::Error>> {
        let response = simple_get(self.server_url, self.api_key, Vec::new());
        let mut users = Vec::new();
        match response.status() {
            StatusCode::OK => {
                users = response.json::<UserInfoVec>()?;
            }
            StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            }
            _ => {
                handle_others(response);
            }
        }

        Ok(users)
    }

    // TODO: Standardize the GET request?
    pub fn get_user_id(self, username: &String) -> String {
        let response = simple_get(self.server_url, self.api_key, Vec::new());
        let users = response.json::<UserInfoVec>().unwrap();
        for user in users {
            if user.name == *username {
                return user.id;
            }
        }

        // Supplied username could not be found.  Panic.
        panic!("Could not find user {}.", username);
    }

    pub fn get_user_information(self, id: &str) -> Result<UserDetails, Box<dyn std::error::Error>> {
        let response = simple_get(
            self.server_url.replace("{userId}", id),
            self.api_key,
            Vec::new(),
        );
        Ok(serde_json::from_str(response.text()?.as_str())?)
    }

    pub fn get_current_user_information(self) -> Result<UserDetails, Box<dyn std::error::Error>> {
        let response = simple_get(self.server_url, self.api_key, Vec::new());
        Ok(response.json::<UserDetails>()?)
    }

    pub fn update_user_config_bool(
        self,
        user_info: &Policy,
        id: &str,
        username: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let body = serde_json::to_string_pretty(user_info)?;
        let response = simple_post(
            self.server_url.replace("{userId}", id),
            self.api_key.clone(),
            body,
        );
        if response.status() == StatusCode::NO_CONTENT {
            println!("User {} successfully updated.", username);
        } else {
            println!("Unable to update user policy information.");
            println!("Status Code: {}", response.status());
            println!("{}", response.text()?);
        }
        Ok(())
    }

    //
    // I really hate this function but it works for now.
    //
    pub fn update_user_info(
        self,
        id: &str,
        info: &UserDetails,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let body = serde_json::to_string_pretty(&info)?;
        // So we have to update the Policy and the user info separate even though they are the same JSON object :/

        // First we will update the Policy
        let policy_url = format!("{}/Policy", self.server_url);
        let user_response = simple_post(
            self.server_url.replace("{userId}", id),
            self.api_key.clone(),
            body,
        );
        if user_response.status() == StatusCode::NO_CONTENT {
        } else {
            println!("Unable to update user information.");
            println!("Status Code: {}", user_response.status());
            match user_response.text() {
                Ok(t) => println!("{}", t),
                Err(_) => eprintln!("Could not get response text from user information update."),
            }
        }

        let response = simple_post(
            policy_url.replace("{userId}", id),
            self.api_key,
            serde_json::to_string_pretty(&info.policy)?,
        );
        if response.status() == StatusCode::NO_CONTENT {
            println!("{} successfully updated.", info.name);
        } else {
            println!("Unable to update user information.");
            println!("Status Code: {}", response.status());
            match response.text() {
                Ok(t) => println!("{}", t),
                Err(_) => eprintln!("Could not get response text from user policy update."),
            }
        }

        Ok(())
    }
}
