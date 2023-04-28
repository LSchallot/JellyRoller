use crate::entities::{task_details::TaskDetails, activity_details::ActivityDetails};

use super::{ DeviceDetails, DeviceRootJson, LibraryDetails, LibraryRootJson, LogDetails, MovieDetails, responder::{ simple_get, simple_post }, handle_unauthorized, handle_others };
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
    pub fn new(endpoint: &str, server_url: &str, api_key: &str) -> ServerInfo {
        ServerInfo {
            server_url: format!("{}{}",server_url, endpoint),
            api_key: api_key.to_owned()
        }
    }

    // Currently used for server-info, restart-jellyfin, shutdown-jellyfin
    pub fn get_server_info(self) -> Result<(), Box<dyn std::error::Error>> {
        let response = simple_get(self.server_url, self.api_key, Vec::new());
        match response.status() {
            StatusCode::OK => {
                let body: Value = response.json()?;
                println!("{:#}", body);
            } StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            } _ => {
                handle_others(response);
            }
        }

        Ok(())
    }

    pub fn restart_or_shutdown(self) {
        let response = simple_post(self.server_url, self.api_key, String::new());
        match response.status() {
            StatusCode::NO_CONTENT => {
                println!("Command successful.");
            } StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            } _ => {
                handle_others(response);
            }
        }
    }

    pub fn get_log_filenames(self) -> Result<Vec<LogDetails>, Box<dyn std::error::Error>> {
        let response = simple_get(self.server_url, self.api_key, Vec::new());
        let mut details = Vec::new();
        match response.status() {
            StatusCode::OK => {
                let logs = response.json::<LogFileVec>()?;
                for log in logs {
                    details.push(LogDetails::new(log.date_created, log.date_modified, log.name, log.size/1024));
                }
            } StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            } _ => {
                handle_others(response);
            }
        }

        Ok(details)
    }

    pub fn get_devices(self) -> Result<Vec<DeviceDetails>, Box<dyn std::error::Error>> {
        let response = simple_get(self.server_url, self.api_key, Vec::new());
            let mut details = Vec::new();
            match response.status() {
            StatusCode::OK => {
                let json = response.text()?;
                let devices = serde_json::from_str::<DeviceRootJson>(&json)?;
                for device in devices.items {
                    details.push(DeviceDetails::new(device.id, device.name, device.lastusername));
                }
            } StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            } _ => {
                handle_others(response);
            }
        }

        Ok(details)
    }

    pub fn get_libraries(self) -> Result<Vec<LibraryDetails>, Box<dyn std::error::Error>> {
        let response = simple_get(self.server_url, self.api_key, Vec::new());
        let mut details = Vec::new();
        match response.status() {
            StatusCode::OK => {
                let json = response.text()?;
                let libraries = serde_json::from_str::<LibraryRootJson>(&json)?;
                for library in libraries {
                    details.push(LibraryDetails::new(library.name, library.collection_type, library.item_id, library.refresh_status));
                }
            } StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            } _ => {
                handle_others(response);
            }
        }
        Ok(details)
    }

    pub fn export_library(self, user_id: &str) -> Result<MovieDetails, Box<dyn std::error::Error>> {
        let query = 
            vec![
                ("SortBy", "SortName,ProductionYear"),
                ("IncludeItemTypes", "Movie"),
                ("Recursive", "true"),
                ("fields", "Genres,DateCreated,Width,Height,Path")
            ];
        let response = simple_get(self.server_url.replace("{userId}", user_id), self.api_key, query);
        match response.status() {
            StatusCode::OK => {
                let details = response.json::<MovieDetails>()?;
                Ok(details)
            } _ => {
                handle_others(response);
                std::process::exit(1)
            }
        }
    }

    pub fn get_activity(self, limit: &str) -> Result<ActivityDetails, Box<dyn std::error::Error>> {
        let query = vec![("limit", limit)];
        let response = simple_get(self.server_url, self.api_key, query);
        match response.status() {
            StatusCode::OK => {
                let activities = response.json::<ActivityDetails>()?;
                Ok(activities)
            } _ => {
                handle_others(response);
                std::process::exit(1);
            }
        }
    }
    
    pub fn get_taskid_by_taskname(self, taskname: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = simple_get(self.server_url, self.api_key, Vec::new());
        match response.status() {
            StatusCode::OK => {
                let tasks = response.json::<ScheduledTasksVec>()?;
                for task in tasks {
                    if task.name.to_lowercase() == taskname.to_lowercase() {
                        return Ok(task.id);
                    }
                }
            } StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            } _ => {
                handle_others(response);
            }
        }
        Ok(String::new())
    }

    pub fn execute_task_by_id(self, taskname: &str, taskid: &str) {
        let response = simple_post(self.server_url.replace("{taskId}", taskid), self.api_key, String::new());
        match response.status() {
            StatusCode::NO_CONTENT => {
                println!("Task \"{}\" initiated.", taskname);
            } StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            } _ => {
                handle_others(response);
            }
        }
    }
    
    pub fn get_deviceid_by_username(self, username: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let response = simple_get(self.server_url, self.api_key, Vec::new());
        let mut filtered = Vec::new();
        match response.status() {
            StatusCode::OK => {
                let json = response.text()?;
                let devices = serde_json::from_str::<DeviceRootJson>(&json)?;
                for device in devices.items {
                    if device.lastusername == username {
                        filtered.push(device.id);
                    }
                }
            } StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            } _ => {
                handle_others(response);
            }
        }

        Ok(filtered)
    }

    pub fn remove_device(self, id: &str) -> Result<(), reqwest::Error> {
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
                    handle_unauthorized();
                } _ => {
                    handle_others(response);
                }
            }
        Ok(())
    }

    pub fn get_scheduled_tasks(self) -> Result<Vec<TaskDetails>, reqwest::Error> {
        let response = simple_get(self.server_url, self.api_key, Vec::new());
        let mut details = Vec::new();
        match response.status() {
            StatusCode::OK => {
                let scheduled_tasks = response.json::<ScheduledTasksVec>()?;
                for task in scheduled_tasks {
                    details.push(TaskDetails::new(task.name, task.state, task.percent_complete, task.id));
                }
            } StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            } _ => {
                handle_others(response);
            }
        }

        Ok(details)
    }

    pub fn scan_library(self) {
        let response = simple_post(
            self.server_url, 
            self.api_key, 
            String::new());
        match response.status() {
            StatusCode::NO_CONTENT => {
                println!("Library scan initiated.");
            } StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            } _ => {
                handle_others(response);
            }
        }
    }
}

pub struct LogFile {
    server_url: String,
    api_key: String,
    logname: String
}

impl LogFile {
    pub fn new(endpoint: &str, server_url: &str, api_key: String, logname: String) -> LogFile {
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
                println!("{:#}", body?);
            } StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            } _ => {
                handle_others(response);
            }
        }
        Ok(())
    }
}