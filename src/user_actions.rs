use super::{ UserDetails, Policy, responder::{simple_get, simple_post}, handle_others, handle_unauthorized } ;
use reqwest::{StatusCode, blocking::Client, header::{CONTENT_TYPE, CONTENT_LENGTH}};


#[derive(Serialize, Deserialize)]
pub struct ApiKey {

}

#[derive(Serialize, Deserialize)]
pub struct ResetPass {
    username: String,
    newpw: String,
    server_url: String,
    api_key: String
}

impl ResetPass {
    pub fn new(username: &str, newpw: String, server_url: &str, api_key: &str) -> ResetPass{
        ResetPass{
            username: username.to_owned(),
            newpw,
            server_url: format!("{}/Users/{}/Password", server_url, username),
            api_key: api_key.to_owned()
        }
    }
    
    pub fn reset(self)  -> Result<(), Box<dyn std::error::Error>> {
        let response = simple_post(
            self.server_url.clone(), 
            self.api_key.clone(), 
            serde_json::to_string_pretty(&self)?);
        match response.status() {
            StatusCode::NO_CONTENT => {
                println!("Password updated successfully.");
            } StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            } _ => {
                handle_others(response);
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserAdd {
    name: String,
    password: String,
    server_url: String,
    api_key: String
}

impl UserAdd {
    pub fn new(username: String, password: String, server_url: &str, api_key: String) -> UserAdd{
        UserAdd{
            name: username,
            password,
            server_url: format!("{}/Users/New",server_url),
            api_key
        }
    }

    pub fn create(self) -> Result<(), Box<dyn std::error::Error>> {
        let response = simple_post(
            self.server_url.clone(), 
            self.api_key.clone(), 
            serde_json::to_string_pretty(&self)?);
        match response.status() {
            StatusCode::OK => {
                println!("User \"{}\" successfully created.", &self.name);
            } StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            } _ => {
                handle_others(response);
            }
        }
        
        Ok(())
    }
}

pub struct UserDel {
    server_url: String,
    api_key: String,
    username: String
}

impl UserDel {
    pub fn new(username: String, server_url: &str, api_key: String) -> UserDel{
        UserDel{
            server_url: format!("{}/Users/{}",server_url,&username),
            api_key,
            username
        }
    }

    pub fn remove(self) -> Result<(), reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .delete(self.server_url)
            .header("X-Emby-Token", self.api_key)
            .header(CONTENT_TYPE, "application/json")
            .send()?;
            match response.status() {
            StatusCode::NO_CONTENT => {
                println!("User \"{}\" successfully removed.", &self.username);
            } StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            } _ => {
                handle_others(response);
            }
        }

        Ok(())
        
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
    pw: String
}

impl UserAuth {
    pub fn new(server_url: &str, username: &str, password: String) -> UserAuth{
        UserAuth{ 
            server_url: format!("{}/Users/authenticatebyname",server_url),
            username: username.to_owned(), 
            pw: password
        }
    }
    
    pub fn auth_user(self) -> Result<String, Box<dyn std::error::Error>> {  
        let client = Client::new();
        let response = client
            .post(self.server_url.clone())
            .header(CONTENT_TYPE, "application/json")
            .header("X-Emby-Authorization", "MediaBrowser Client=\"JellyRoller\", Device=\"jellyroller\", DeviceId=\"1\", Version=\"0.0.1\"")
            .body(serde_json::to_string_pretty(&self)?)
            .send()?;
        match response.status() {
            StatusCode::OK => {
                let result = response.json::<UserAuthJson>()?;
                println!("User authenticated successfully.");
                Ok(result.access_token)
            } _ => {
                // Panic since the application requires an authenticated user
                panic!("[ERROR] Unable to authenticate user.  Please assure your configuration information is correct.\n");
            }
        }
    }

    
}

#[derive(Clone)]
pub struct UserList {
    server_url: String,
    api_key: String
}

impl UserList {
    pub fn new(endpoint: &str, server_url: &str, api_key: String) -> UserList{
        UserList{
            server_url: format!("{}{}",server_url, endpoint),
            api_key
        }
    }

    pub fn create_api_token(self) {
        let client = Client::new();
        let response = client
            .post(self.server_url)
            .header("x-emby-token", self.api_key)
            .header(CONTENT_LENGTH, 0)
            .query(&[("app", "JellyRoller")])
            .send()
            .unwrap();
        
        match response.status() {
            StatusCode::NO_CONTENT => {
                println!("API key created.");
            } _ => {
                handle_others(response);
            }
            
        }
    }

    pub fn list_users(self) -> Result<Vec<UserDetails>, Box<dyn std::error::Error>> {
        let response = simple_get(self.server_url, self.api_key, Vec::new());
        let mut users = Vec::new();
        match response.status() {
            StatusCode::OK => {
                users = response.json::<UserInfoVec>()?;
            } StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            } _ => {
                handle_others(response);
            }
        }
        
        Ok(users)
    }

    // TODO: Standardize the GET request?
    pub fn get_user_id(self, username: &String) -> String {
        let client = Client::new();
        let response = client
            .get(self.server_url)
            .header("X-Emby-Token", self.api_key)
            .send()
            .unwrap();
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
        let response = simple_get(self.server_url.replace("{userId}", id), self.api_key, Vec::new());
        Ok(serde_json::from_str(response.text()?.as_str())?)
    }
    
    pub fn get_current_user_information(self) -> Result<UserDetails, Box<dyn std::error::Error>> {
        let response = simple_get(self.server_url, self.api_key, Vec::new());
        Ok(response.json::<UserDetails>()?)
    }

    pub fn update_user_config_bool(self, user_info: &Policy, id: &str, username: &str) -> Result<(), Box<dyn std::error::Error>> {
        let body = serde_json::to_string_pretty(user_info)?;
        let response = simple_post(
            self.server_url.replace("{userId}", id), 
            self.api_key.clone(), 
            body);
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
    pub fn update_user_info(self, id: &str, info: &UserDetails) -> Result<(), Box<dyn std::error::Error>> {
        let body = serde_json::to_string_pretty(&info)?;
        // So we have to update the Policy and the user info separate even though they are the same JSON object :/
        let policy_url = format!("{}/Policy",self.server_url);
        let user_response = simple_post(self.server_url.replace("{userId}", id), self.api_key.clone(), body);
        if user_response.status() == StatusCode::NO_CONTENT {} else {
            println!("Unable to update user information.");
            println!("Status Code: {}", user_response.status());
            match user_response.text() {
                Ok(t) => println!("{}", t),
                Err(_) => eprintln!("Could not get response text from user information update.")
            }            
        }
        
        let response = simple_post(policy_url.replace("{userId}", id), self.api_key, serde_json::to_string_pretty(&info.policy)?);
        if response.status() == StatusCode::NO_CONTENT {
            println!("{} successfully updated.", info.name);
        } else {
            println!("Unable to update user information.");
            println!("Status Code: {}", response.status());
            match response.text() {
                Ok(t) => println!("{}", t),
                Err(_) => eprintln!("Could not get response text from user policy update.")
            }
        }

        Ok(())
    
    }
}