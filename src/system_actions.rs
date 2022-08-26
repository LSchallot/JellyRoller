use crate::entities::task_details::TaskDetails;

use super::{ DeviceDetails, DeviceRootJson, LogDetails, responder::*};
use reqwest::{blocking::Client, StatusCode};
use serde_json::Value;

pub type LogFileVec = Vec<LogDetails>;
pub type ScheduledTasksVec = Vec<TaskDetails>;

#[derive(Clone)]
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

    // Currently used for server-info, restart-jellyfin, shutdown-jellyfin
    pub fn get_server_info(self) -> Result<(), reqwest::Error> {
        let response = simple_get(self.server_url, self.api_key);
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

    pub fn restart_or_shutdown(self) -> Result<(), reqwest::Error> {
        let response = simple_post(self.server_url, self.api_key, "".to_string());
        match response.status() {
            StatusCode::NO_CONTENT => {
                println!("Command successful.");
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
            }
        }
        Ok(())
    }

    pub fn get_log_filenames(self) -> Result<Vec<LogDetails>, reqwest::Error> {
        let response = simple_get(self.server_url, self.api_key);
        let mut details = Vec::new();
        match response.status() {
            StatusCode::OK => {
                let logs = response.json::<LogFileVec>().unwrap();
                for log in logs {
                    details.push(LogDetails::new(log.date_created, log.date_modified, log.name, log.size/1024));
                }
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
            }
        }

        Ok(details)
    }

    pub fn get_devices(self) -> Result<Vec<DeviceDetails>, reqwest::Error> {
        let response = simple_get(self.server_url, self.api_key);
            let mut details = Vec::new();
            match response.status() {
            StatusCode::OK => {
                let json = response.text().unwrap();
                let devices = serde_json::from_str::<DeviceRootJson>(&json).unwrap();
                for device in devices.items {
                    details.push(DeviceDetails::new(device.id, device.name, device.lastusername));
                }
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
            }
        }

        Ok(details)
    }

    pub fn get_taskid_by_taskname(self, taskname: String) -> Result<String, reqwest::Error> {
        let response = simple_get(self.server_url, self.api_key);
        match response.status() {
            StatusCode::OK => {
                let tasks = response.json::<ScheduledTasksVec>().unwrap();
                for task in tasks {
                    if task.name.to_lowercase() == taskname.to_lowercase() {
                        return Ok(task.id);
                    }
                }
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
            }
        }
        Ok("".to_string())
    }

    pub fn execute_task_by_id(self, taskname: String, taskid: String) -> Result<(), reqwest::Error> {
        let response = simple_post(self.server_url.replace("{taskId}", &taskid), self.api_key, "".to_string());
        match response.status() {
            StatusCode::NO_CONTENT => {
                println!("Task \"{}\" initiated.", taskname);
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
            }
        }
        Ok(())
    }
    
    pub fn get_deviceid_by_username(self, username: String) -> Result<Vec<String>, reqwest::Error> {
        let response = simple_get(self.server_url, self.api_key);
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

    pub fn get_scheduled_tasks(self) -> Result<Vec<TaskDetails>, reqwest::Error> {
        let response = simple_get(self.server_url, self.api_key);
        let mut details = Vec::new();
        match response.status() {
            StatusCode::OK => {
                let scheduled_tasks = response.json::<ScheduledTasksVec>().unwrap();
                for task in scheduled_tasks {
                    details.push(TaskDetails::new(task.name, task.state, task.percent_complete, task.id));
                }
            } StatusCode::UNAUTHORIZED => {
                println!("Authentication failed.  Try reconfiguring with \"jellyroller reconfigure\"");
            } _ => {
                println!("Status Code: {}", response.status());
            }
        }

        Ok(details)
    }

    pub fn scan_library(self) -> Result<(), reqwest::Error> {
        let response = simple_post(
            self.server_url, 
            self.api_key, 
            "".to_string());
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