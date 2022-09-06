use super::{ UserDetails, Policy, responder::*} ;
use reqwest::{StatusCode, blocking::Client, header::CONTENT_TYPE};

#[derive(Serialize, Deserialize)]
pub struct ResetPass {
    username: String,
    newpw: String,
    server_url: String,
    api_key: String
}

impl ResetPass {
    pub fn new(username: String, newpw: String, server_url: String, api_key: String) -> ResetPass{
        ResetPass{
            username: username.clone(),
            newpw,
            server_url: format!("{}/Users/{}/Password", server_url, username),
            api_key
        }
    }
    
    pub fn reset(self)  -> Result<(), reqwest::Error> {
        let response = simple_post(
            self.server_url.clone(), 
            self.api_key.clone(), 
            serde_json::to_string_pretty(&self).unwrap());
        match response.status() {
            StatusCode::NO_CONTENT => {
                println!("Password updated successfully.");
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
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
    pub fn new(username: String, password: String, server_url: String, api_key: String) -> UserAdd{
        UserAdd{
            name: username,
            password,
            server_url: format!("{}/Users/New",server_url),
            api_key
        }
    }

    pub fn create(self) -> Result<(), reqwest::Error> {
        let response = simple_post(
            self.server_url.clone(), 
            self.api_key.clone(), 
            serde_json::to_string_pretty(&self).unwrap());
        match response.status() {
            StatusCode::OK => {
                println!("User \"{}\" successfully created.", &self.name);
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
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
    pub fn new(username: String, server_url: String, api_key: String) -> UserDel{
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
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
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
    pub fn new(server_url: String, username: String, password: String) -> UserAuth{
        UserAuth{ 
            server_url: format!("{}/Users/authenticatebyname",server_url),
            username, 
            pw: password
        }
    }
    
    pub fn auth_user(self) -> Result<String, reqwest::Error> {  
        let client = Client::new();
        let response = client
            .post(self.server_url.clone())
            .header(CONTENT_TYPE, "application/json")
            .header("X-Emby-Authorization", "MediaBrowser Client=\"JellyRoller\", Device=\"jellyroller\", DeviceId=\"1\", Version=\"0.0.1\"")
            .body(serde_json::to_string_pretty(&self).unwrap())
            .send()?;
        match response.status() {
            StatusCode::OK => {
                let result = response.json::<UserAuthJson>().unwrap();
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
    pub fn new(endpoint: &str, server_url: String, api_key: String) -> UserList{
        UserList{
            server_url: format!("{}{}",server_url, endpoint),
            api_key
        }
    }

    pub fn list_users(self) -> Result<Vec<UserDetails>, reqwest::Error> {
        let response = simple_get(self.server_url, self.api_key);
        let mut users = Vec::new();
        match response.status() {
            StatusCode::OK => {
                users = response.json::<UserInfoVec>().unwrap();
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
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

    pub fn get_user_information(self, id: String) -> Result<UserDetails, reqwest::Error> {
        let response = simple_get(self.server_url.replace("{userId}", &id), self.api_key);
        Ok(serde_json::from_str(response.text().unwrap().as_str()).unwrap())
    }
    
    pub fn update_user_config_bool(self, user_info: Policy, id: String, username: String) -> Result<(), reqwest::Error> {
        let body = serde_json::to_string_pretty(&user_info).unwrap();
        println!("{}", body);
        let response = simple_post(
            self.server_url.replace("{userId}", &id), 
            self.api_key.clone(), 
            body);
        match response.status() {
            StatusCode::NO_CONTENT => {
                println!("User {} successfully updated.", username);
            } _ => {
                println!("Unable to update user policy information.");
                println!("Status Code: {}", response.status());
                println!("{}", response.text().unwrap());
            }
        }
        Ok(())
    }
}