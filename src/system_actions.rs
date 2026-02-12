use crate::{ReportType, entities::{
    activity_details::ActivityDetails, backup_details::{BackupDetails, BackupRootJson}, library_details_full::{LibraryDetailsFullVec}, library_options::LibraryOptionsRoot, media_details::MediaRoot, repository_details::RepositoryDetails, task_details::TaskDetails
}};

use super::{
    handle_others, handle_unauthorized,
    responder::{simple_get, simple_post, simple_post_image, simple_post_with_query},
    DeviceDetails, DeviceRootJson, ImageType, LibraryDetails, LibraryRootJson, LogDetails,
    MovieDetails, PackageDetails, PackageDetailsRoot, RepositoryDetailsRoot, ServerInfo,
};
use chrono::{DateTime, Duration};
use reqwest::{blocking::Client, StatusCode};
use serde_json::Value;

pub type LogFileVec = Vec<LogDetails>;
pub type ScheduledTasksVec = Vec<TaskDetails>;

// Currently used for server-info, restart-jellyfin, shutdown-jellyfin
pub fn get_server_info(server_info: ServerInfo) -> Result<(), Box<dyn std::error::Error>> {
    let response = simple_get(server_info.server_url, &server_info.api_key, Vec::new());
    match response.status() {
        StatusCode::OK => {
            let body: Value = response.json()?;
            println!("{body:#}");
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }

    Ok(())
}

pub fn get_backups_info(server_info: ServerInfo) -> Result<Vec<BackupDetails>, Box<dyn std::error::Error>> {
    let response = simple_get(server_info.server_url, &server_info.api_key, Vec::new());
    let mut backups = Vec::new();
    match response.status() {
        StatusCode::OK => {
            backups = response.json::<BackupRootJson>()?;
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }
    Ok(backups)
}

pub fn get_repo_info(
    server_info: ServerInfo,
) -> Result<Vec<RepositoryDetails>, Box<dyn std::error::Error>> {
    let response = simple_get(server_info.server_url, &server_info.api_key, Vec::new());
    let mut repos = Vec::new();
    match response.status() {
        StatusCode::OK => {
            repos = response.json::<RepositoryDetailsRoot>()?;
        }
        _ => handle_others(&response),
    }
    Ok(repos)
}

pub fn set_repo_info(server_info: ServerInfo, repos: &[RepositoryDetails]) {
    simple_post(
        server_info.server_url,
        &server_info.api_key,
        serde_json::to_string(&repos).unwrap(),
    );
}

pub fn get_packages_info(
    server_info: ServerInfo,
) -> Result<Vec<PackageDetails>, Box<dyn std::error::Error>> {
    let response = simple_get(server_info.server_url, &server_info.api_key, Vec::new());
    let mut packages = Vec::new();
    match response.status() {
        StatusCode::OK => {
            packages = response.json::<PackageDetailsRoot>()?;
        }
        _ => handle_others(&response),
    }
    Ok(packages)
}

pub fn install_package(server_info: &ServerInfo, package: &str, version: &str, repository: &str) {
    let query = &[("version", version), ("repository", repository)];
    let response = simple_post_with_query(
        server_info.server_url.replace("{package}", package),
        &server_info.api_key,
        String::new(),
        query,
    );
    match response.status() {
        StatusCode::NO_CONTENT => {
            println!("Package successfully installed.");
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }
}

pub fn return_server_info(server_info: ServerInfo) -> String {
    let response = simple_get(server_info.server_url, &server_info.api_key, Vec::new());
    if response.status() == StatusCode::OK {
        let body: Value = response.json().unwrap();
        body.to_string()
    } else {
        handle_others(&response);
        String::new()
    }
}

pub fn restart_or_shutdown(server_info: ServerInfo) {
    let response = simple_post(server_info.server_url, &server_info.api_key, String::new());
    match response.status() {
        StatusCode::NO_CONTENT => {
            println!("Command successful.");
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }
}

pub fn get_log_filenames(
    server_info: ServerInfo,
) -> Result<Vec<LogDetails>, Box<dyn std::error::Error>> {
    let response = simple_get(server_info.server_url, &server_info.api_key, Vec::new());
    let mut details = Vec::new();
    match response.status() {
        StatusCode::OK => {
            let logs = response.json::<LogFileVec>()?;
            for log in logs {
                details.push(LogDetails::new(
                    log.date_created,
                    log.date_modified,
                    log.name,
                    log.size / 1024,
                ));
            }
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }

    Ok(details)
}

pub fn get_devices(
    server_info: ServerInfo,
    active: bool,
) -> Result<Vec<DeviceDetails>, Box<dyn std::error::Error>> {
    let response = simple_get(server_info.server_url, &server_info.api_key, Vec::new());
    let mut details = Vec::new();
    match response.status() {
        StatusCode::OK => {
            let json = response.text()?;
            let devices = serde_json::from_str::<DeviceRootJson>(&json)?;
            let cutofftime = chrono::offset::Utc::now() - Duration::seconds(960);
            for device in devices.items {
                let datetime = DateTime::parse_from_rfc3339(&device.lastactivity).unwrap();
                if active {
                    if cutofftime < datetime {
                        details.push(DeviceDetails::new(
                            device.id,
                            device.name,
                            device.lastusername,
                            device.lastactivity,
                        ));
                    }
                } else {
                    details.push(DeviceDetails::new(
                        device.id,
                        device.name,
                        device.lastusername,
                        device.lastactivity,
                    ));
                }
            }
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }

    Ok(details)
}

pub fn get_libraries(
    server_info: ServerInfo,
) -> Result<Vec<LibraryDetails>, Box<dyn std::error::Error>> {
    let response = simple_get(server_info.server_url, &server_info.api_key, Vec::new());
    let mut details = Vec::new();
    match response.status() {
        StatusCode::OK => {
            let json = response.text()?;
            let libraries = serde_json::from_str::<LibraryRootJson>(&json)?;
            for library in libraries {
                details.push(LibraryDetails::new(
                    library.name,
                    library.collection_type,
                    library.library_options,
                    library.item_id,
                    library.refresh_status,
                ));
            }
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }
    Ok(details)
}

pub fn get_libraries_full(server_info: ServerInfo) -> Result<LibraryDetailsFullVec, Box<dyn std::error::Error>> {
    let response = simple_get(
        server_info.server_url.clone(),
        &server_info.api_key,
        Vec::new(),
    );
    if response.status() == StatusCode::OK {
        let libraries = response.json::<LibraryDetailsFullVec>()?;
        Ok(libraries)
    } else {
        handle_others(&response);
        std::process::exit(1)
    }
}

pub fn update_library(server_info: ServerInfo, library_options: LibraryOptionsRoot) {
    let response = simple_post(
            server_info.server_url, 
            &server_info.api_key, 
            serde_json::to_string(&library_options).unwrap());
    if response.status() == StatusCode::NO_CONTENT {
        println!("Library updated successfully.");
    } else {
        handle_others(&response);
        std::process::exit(1)
    }
}

pub fn export_library(
    server_info: &ServerInfo, report_type: &ReportType,
) -> Result<MovieDetails, Box<dyn std::error::Error>> {
    let binding = report_type.to_string();
    let query = vec![
        ("SortBy", "SortName,ProductionYear"),
        ("IncludeItemTypes", binding.as_str()),
        ("Recursive", "true"),
        ("fields", "Genres,DateCreated,Width,Height,Path"),
    ];
    let response = simple_get(
        server_info.server_url.clone(),
        &server_info.api_key,
        query,
    );

    if response.status() == StatusCode::OK {
        let details = response.json::<MovieDetails>()?;
        Ok(details)
    } else {
        handle_others(&response);
        std::process::exit(1)
    }
}

pub fn get_activity(
    server_info: ServerInfo,
    limit: &str,
) -> Result<ActivityDetails, Box<dyn std::error::Error>> {
    let query = vec![("limit", limit)];
    let response = simple_get(server_info.server_url, &server_info.api_key, query);
    if response.status() == StatusCode::OK {
        let activities = response.json::<ActivityDetails>()?;
        Ok(activities)
    } else {
        handle_others(&response);
        std::process::exit(1);
    }
}

pub fn get_taskid_by_taskname(
    server_info: ServerInfo,
    taskname: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let response = simple_get(server_info.server_url, &server_info.api_key, Vec::new());
    match response.status() {
        StatusCode::OK => {
            let tasks = response.json::<ScheduledTasksVec>()?;
            for task in tasks {
                if task.name.to_lowercase() == taskname.to_lowercase() {
                    return Ok(task.id);
                }
            }
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }
    Ok(String::new())
}

pub fn execute_task_by_id(server_info: &ServerInfo, taskname: &str, taskid: &str) {
    let response = simple_post(
        server_info.server_url.replace("{taskId}", taskid),
        &server_info.api_key,
        String::new(),
    );
    match response.status() {
        StatusCode::NO_CONTENT => {
            println!("Task \"{taskname}\" initiated.");
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }
}

pub fn get_deviceid_by_username(
    server_info: ServerInfo,
    username: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let response = simple_get(server_info.server_url, &server_info.api_key, Vec::new());
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
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }

    Ok(filtered)
}

pub fn remove_device(server_info: ServerInfo, id: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let apikey = &server_info.api_key;
    let response = client
        .delete(server_info.server_url)
        .header("Authorization", format!("MediaBrowser Token=\"{apikey}\""))
        .query(&[("id", &id)])
        .send()?;
    match response.status() {
        StatusCode::NO_CONTENT => {
            println!("\t Removes device with id = {id}.");
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }
    Ok(())
}

pub fn get_scheduled_tasks(server_info: ServerInfo) -> Result<Vec<TaskDetails>, reqwest::Error> {
    let response = simple_get(server_info.server_url, &server_info.api_key, Vec::new());
    let mut details = Vec::new();
    match response.status() {
        StatusCode::OK => {
            let scheduled_tasks = response.json::<ScheduledTasksVec>()?;
            for task in scheduled_tasks {
                details.push(TaskDetails::new(
                    task.name,
                    task.state,
                    task.percent_complete,
                    task.id,
                ));
            }
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }

    Ok(details)
}

pub fn scan_library(server_info: &ServerInfo, scan_options: &[(&str, &str)], library_id: &str) {
    let response = simple_post_with_query(
        server_info
            .server_url
            .replace("{library_id}", library_id),
        &server_info.api_key,
        String::new(),
        scan_options,
    );
    match response.status() {
        StatusCode::NO_CONTENT => {
            println!("Library scan initiated.");
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }
}

pub fn scan_library_all(server_info: ServerInfo) {
    let response = simple_post(server_info.server_url, &server_info.api_key, String::new());
    match response.status() {
        StatusCode::NO_CONTENT => {
            println!("Library scan initiated.");
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }
}

pub fn register_library(server_info: ServerInfo, json_contents: String) {
    let response = simple_post(server_info.server_url, &server_info.api_key, json_contents);
    match response.status() {
        StatusCode::NO_CONTENT => {
            println!("Library successfully added.");
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }
}

pub fn update_image(
    server_info: &ServerInfo,
    id: &str,
    imagetype: &ImageType,
    img_base64: &String,
) {
    let response = simple_post_image(
        server_info
            .server_url
            .replace("{itemId}", id)
            .replace("{imageType}", imagetype.to_string().as_str()),
        &server_info.api_key,
        img_base64.to_string(),
    );
    match response.status() {
        StatusCode::NO_CONTENT => {
            println!("Image successfully updated.");
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }
}

pub fn update_metadata(server_info: &ServerInfo, id: &str, json: String) {
    let response = simple_post(
        server_info.server_url.replace("{itemId}", id),
        &server_info.api_key,
        json,
    );
    match response.status() {
        StatusCode::NO_CONTENT => {
            println!("Metadata successfully updated.");
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }
}

pub fn get_search_results(
    server_info: ServerInfo,
    query: Vec<(&str, &str)>,
) -> Result<MediaRoot, Box<dyn std::error::Error>> {
    let response = simple_get(server_info.server_url, &server_info.api_key, query);
    match response.status() {
        StatusCode::OK => {
            let media = response.json::<MediaRoot>()?;
            Ok(media)
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
            std::process::exit(1);
        }
        _ => {
            handle_others(&response);
            std::process::exit(1);
        }
    }
}
pub struct LogFile {
    server_info: ServerInfo,
    logname: String,
}

impl LogFile {
    pub fn new(server_info: ServerInfo, logname: String) -> LogFile {
        LogFile {
            server_info,
            logname,
        }
    }

    pub fn get_logfile(self) -> Result<(), reqwest::Error> {
        
        let client = Client::new();
        let apikey = &self.server_info.api_key;
        let response = client
            .get(self.server_info.server_url)
            .query(&[("name", self.logname)])
            .header("Authorization", format!("MediaBrowser Token=\"{apikey}\""))
            .send()?;
        match response.status() {
            StatusCode::OK => {
                let body = response.text();
                println!("{:#}", body?);
            }
            StatusCode::UNAUTHORIZED => {
                handle_unauthorized();
            }
            _ => {
                handle_others(&response);
            }
        }
        Ok(())
    }
}
