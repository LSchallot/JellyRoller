use crate::entities::{task_details::TaskDetails, activity_details::ActivityDetails, media_details::MediaRoot};

use super::{ ServerInfo, DeviceDetails, DeviceRootJson, LibraryDetails, LibraryRootJson, LogDetails, MovieDetails, ImageType, responder::{ simple_get, simple_post, simple_post_image }, handle_unauthorized, handle_others };
use reqwest::{blocking::Client, StatusCode};
use serde_json::Value;

pub type LogFileVec = Vec<LogDetails>;
pub type ScheduledTasksVec = Vec<TaskDetails>;

// Currently used for server-info, restart-jellyfin, shutdown-jellyfin
pub fn get_server_info(server_info: ServerInfo) -> Result<(), Box<dyn std::error::Error>> {
    let response = simple_get(server_info.server_url, server_info.api_key, Vec::new());
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

//pub fn return_server_info(server_info: ServerInfo) -> Result<String, Box<dyn std::error::Error>> {
pub fn return_server_info(server_info: ServerInfo) -> String {
    let response = simple_get(server_info.server_url, server_info.api_key, Vec::new());
    match response.status() {
        StatusCode::OK => {
            let body: Value = response.json().unwrap();
            body.to_string()
        } _ => {
            handle_others(response);
            "".to_string()
        }
    }
}

pub fn restart_or_shutdown(server_info: ServerInfo) {
    let response = simple_post(server_info.server_url, server_info.api_key, String::new());
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

pub fn get_log_filenames(server_info: ServerInfo) -> Result<Vec<LogDetails>, Box<dyn std::error::Error>> {
    let response = simple_get(server_info.server_url, server_info.api_key, Vec::new());
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

pub fn get_devices(server_info: ServerInfo) -> Result<Vec<DeviceDetails>, Box<dyn std::error::Error>> {
    let response = simple_get(server_info.server_url, server_info.api_key, Vec::new());
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

pub fn get_libraries(server_info: ServerInfo) -> Result<Vec<LibraryDetails>, Box<dyn std::error::Error>> {
    let response = simple_get(server_info.server_url, server_info.api_key, Vec::new());
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

pub fn export_library(server_info: ServerInfo, user_id: &str) -> Result<MovieDetails, Box<dyn std::error::Error>> {
    let query = 
        vec![
            ("SortBy", "SortName,ProductionYear"),
            ("IncludeItemTypes", "Movie"),
            ("Recursive", "true"),
            ("fields", "Genres,DateCreated,Width,Height,Path")
        ];
    let response = simple_get(server_info.server_url.replace("{userId}", user_id), server_info.api_key, query);
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

pub fn get_activity(server_info: ServerInfo, limit: &str) -> Result<ActivityDetails, Box<dyn std::error::Error>> {
    let query = vec![("limit", limit)];
    let response = simple_get(server_info.server_url, server_info.api_key, query);
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

pub fn get_taskid_by_taskname(server_info: ServerInfo, taskname: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = simple_get(server_info.server_url, server_info.api_key, Vec::new());
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

pub fn execute_task_by_id(server_info: ServerInfo, taskname: &str, taskid: &str) {
    let response = simple_post(server_info.server_url.replace("{taskId}", taskid), server_info.api_key, String::new());
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

pub fn get_deviceid_by_username(server_info: ServerInfo, username: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let response = simple_get(server_info.server_url, server_info.api_key, Vec::new());
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

pub fn remove_device(server_info: ServerInfo, id: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let apikey = server_info.api_key;
    let response = client
        .delete(server_info.server_url)
        .header("Authorization", format!("MediaBrowser Token=\"{apikey}\""))
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

pub fn get_scheduled_tasks(server_info: ServerInfo) -> Result<Vec<TaskDetails>, reqwest::Error> {
    let response = simple_get(server_info.server_url, server_info.api_key, Vec::new());
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

pub fn scan_library(server_info: ServerInfo) {
    let response = simple_post(
        server_info.server_url, 
        server_info.api_key, 
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

pub fn update_image(server_info: ServerInfo, id: String, imagetype: &ImageType, img_base64: &String) {
    let response = simple_post_image(
        server_info.server_url.replace("{itemId}", id.as_str()).replace("{imageType}", imagetype.to_string().as_str()),
        server_info.api_key,
        img_base64.to_string());
    match response.status() {
        StatusCode::NO_CONTENT => {
            println!("Image successfully updated.");
        } StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        } _ => {
            handle_others(response);
        }
    }
}

pub fn get_search_results(server_info: ServerInfo, query: Vec<(&str, &str)>) -> Result<MediaRoot, Box< dyn std::error::Error>> {
    let response = simple_get(
        server_info.server_url,
        server_info.api_key,
        query
    );
    match response.status() {
        StatusCode::OK => {
            let media = response.json::<MediaRoot>()?;
            Ok(media)
        } StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
            std::process::exit(1);
        } _ => {
            handle_others(response);
            std::process::exit(1);
        }
    }
}
pub struct LogFile {
    server_info: ServerInfo,
    logname: String
}

impl LogFile {
    pub fn new(server_info: ServerInfo, logname: String) -> LogFile {
        LogFile { 
            server_info,
            logname
        }
    }

    pub fn get_logfile(self) -> Result<(), reqwest::Error> {
        let client = Client::new();
        let apikey = self.server_info.api_key;
        let response = client
            .get(self.server_info.server_url)
            .query(&[("name", self.logname)])
            .header("Authorization", format!("MediaBrowser Token=\"{apikey}\""))
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