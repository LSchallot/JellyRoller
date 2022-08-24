use reqwest::{blocking::Client, header::CONTENT_TYPE, StatusCode};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct LogFileJson {
    #[serde(rename = "DateCreated")]
    pub date_created: String,
    #[serde(rename = "DateModified")]
    pub date_modified: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Size")]
    pub size: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceRootJson {
    #[serde(rename = "Items")]
    pub items: Vec<DeviceJson>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceJson {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "LastUserName")]
    pub lastusername: String,

}

#[derive(Serialize, Deserialize)]
pub struct ScheduledTasksJson {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "CurrentProgressPercentage")]
    pub progress: Option<f64>
}

pub type LogFileVec = Vec<LogFileJson>;
pub type ScheduledTasksVec = Vec<ScheduledTasksJson>;

pub struct ServerInfo {
    server_url: String,
    api_key: String
}

impl ServerInfo {
    pub fn new(endpoint: String, server_url: String, api_key: String) -> ServerInfo {
        ServerInfo {
            server_url: format!("{}{}",server_url, endpoint),
            api_key
        }
    }

    pub fn get_server_info(self) -> Result<(), reqwest::Error> {
        let client = Client::new();
        let response = client
            .get(self.server_url)
            .header("X-Emby-Token", self.api_key)
            .send()?;
        match response.status() {
            StatusCode::OK => {
                let body: Value = response.json()?;
                println!("{:#}", body);
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
            }
        }

        Ok(())
    }

    pub fn get_log_filenames(self) -> Result<(), reqwest::Error> {
        let client = Client::new();
        let response = client
            .get(self.server_url)
            .header("X-Emby-Token", self.api_key)
            .send()?;
        match response.status() {
            StatusCode::OK => {
                let logfiles = response.json::<LogFileVec>().unwrap();
                println!("Logfiles found: ");
                for file in logfiles {
                    println!("\t{}\t{}K", file.name, file.size/1024)
                }
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
            }
        }

        Ok(())
    }

    pub fn get_devices(self) -> Result<(), reqwest::Error> {
        let client = Client::new();
        let response = client
            .get(self.server_url)
            .header("X-Emby-Token", self.api_key)
            .send()?;
        match response.status() {
            StatusCode::OK => {
                let json = response.text().unwrap();
                //let j = json.as_str();
                let devices = serde_json::from_str::<DeviceRootJson>(&json).unwrap();
                println!("Active devices (UserName, Session Id):");
                for device in devices.items {
                    //println!("\t{}, {}", device.username, session.id);
                    println!("{} | {} | {}", device.id, device.name, device.lastusername);
                }
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
            }
        }

        Ok(())
    }

    pub fn get_deviceid_by_username(self, username: String) -> Result<Vec<String>, reqwest::Error> {
        let client = Client::new();
        let response = client
            .get(self.server_url)
            .header("X-Emby-Token", self.api_key)
            .send()?;
        let mut filtered = Vec::new();
        match response.status() {
            StatusCode::OK => {
                let json = response.text().unwrap();
                let devices = serde_json::from_str::<DeviceRootJson>(&json).unwrap();
                for device in devices.items {
                    if device.lastusername == username {
                        filtered.push(device.id);
                    }
                }
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
            }
        }

        Ok(filtered)
    }

    pub fn remove_device(self, id: String) -> Result<(), reqwest::Error> {
        let client = Client::new();
        let response = client
            .delete(self.server_url)
            .header("X-Emby-Token", self.api_key)
            .query(&[("id", &id)])
            .send()?;
            match response.status() {
                StatusCode::NO_CONTENT => {
                    println!("\t Removes device with id = {}.", id);
                } StatusCode::UNAUTHORIZED => {
                    println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
                } _ => {
                    println!("Status Code: {}", response.status());
                }
            }
        Ok(())
    }

    pub fn get_scheduled_tasks(self) -> Result<(), reqwest::Error> {
        let client = Client::new();
        let response = client
            .get(self.server_url)
            .header("X-Emby-Token", self.api_key)
            .send()?;
        match response.status() {
            StatusCode::OK => {
                let scheduled_tasks = response.json::<ScheduledTasksVec>().unwrap();
                println!("Scheduled tasks (Name, State, Percent Complete):");
                for task in scheduled_tasks {
                    println!("\t{}, {}, {:?}", task.name, task.state, 
                        match task.progress {
                            Some(ref x) => x.to_string(),
                            None => "N/A".to_string()
                        });
                }
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
            }
        }

        Ok(())
    }

    pub fn scan_library(self) -> Result<(), reqwest::Error> {
        let client = Client::new();
        let response = client
            .post(self.server_url)
            .header(CONTENT_TYPE, "application/json")
            .header("X-Emby-Token", self.api_key)
            .body("".to_string())
            .send()?;
        match response.status() {
            StatusCode::NO_CONTENT => {
                println!("Library scan initiated.");
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
            }
        }

        Ok(())
    }
}

pub struct LogFile {
    server_url: String,
    api_key: String,
    logname: String
}

impl LogFile {
    pub fn new(endpoint: String, server_url: String, api_key: String, logname: String) -> LogFile {
        LogFile { 
            server_url: format!("{}{}",server_url, endpoint),
            api_key,
            logname
        }
    }

    pub fn get_logfile(self) -> Result<(), reqwest::Error> {
        let client = Client::new();
        let response = client
            .get(self.server_url)
            .query(&[("name", self.logname)])
            .header("X-Emby-Token", self.api_key)
            .send()?;
        match response.status() {
            StatusCode::OK => {
                let body = response.text();
                println!("{:#}", body.unwrap());
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
            }
        }
        Ok(())
    }
}