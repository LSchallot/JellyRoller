use std::env;

use reqwest::StatusCode;
use prop_reader::PropReader;

use crate::{ entities::{backup_details::BackupDetails, device_details::DeviceDetails, package_details::PackageDetails, plugin_details::PluginDetails, repository_details::RepositoryDetails, server_info::ServerInfo, task_details::TaskDetails}, plugin_actions::PluginInfo, responder::{simple_get, simple_post}, system_actions::{ execute_task_by_id, get_backups_info, get_devices, get_packages_info, get_repo_info, get_scheduled_tasks, get_taskid_by_taskname, install_package, set_repo_info }, user_actions::{ UserAuth, UserWithPass }, utils::status_handler::{handle_others, handle_unauthorized}, AppConfig, OutputFormat};


pub fn command_initialize(mut cfg: AppConfig, username: &str, password: String, server_url: &str) {
    println!("Configuring JellyRoller with supplied values.....");
    env::consts::OS.clone_into(&mut cfg.os);
    server_url.trim().clone_into(&mut cfg.server_url);
    cfg.api_key = UserAuth::auth_user(UserAuth::new(&cfg.server_url, username.trim(), password))
                .expect("Unable to generate user auth token.  Please assure your configuration information was input correctly\n");
            "configured".clone_into(&mut cfg.status);
            token_to_api(cfg);
}

pub fn command_get_devices(cfg: &AppConfig, active: bool, output_format: &OutputFormat, devices_endpoint: &str) {
    let devices: Vec<DeviceDetails> = match get_devices(
        ServerInfo::new(devices_endpoint, &cfg.server_url, &cfg.api_key),
        active,
    ) {
        Err(e) => {
            eprintln!("Unable to get devices, {e}");
            std::process::exit(1);
        }
        Ok(i) => i,
    };

    match output_format {
        OutputFormat::Json => {
            DeviceDetails::json_print(&devices);
        }
        OutputFormat::Csv => {
            DeviceDetails::csv_print(&devices);
        }
        OutputFormat::Table => {
            DeviceDetails::table_print(devices);
        }
    }
}

pub fn command_execute_task_by_name(cfg: &AppConfig, task: &str) {
    let taskid: String = match get_taskid_by_taskname(
        ServerInfo::new("/ScheduledTasks", &cfg.server_url, &cfg.api_key),
        task,
    ) {
        Err(e) => {
            eprintln!("Unable to get task id by taskname, {e}");
            std::process::exit(1);
        }
        Ok(i) => i,
    };
    execute_task_by_id(
        &ServerInfo::new(
            "/ScheduledTasks/Running/{taskId}",
            &cfg.server_url,
            &cfg.api_key,
        ),
        task,
        &taskid,
    );
}

pub fn command_get_packages(cfg: &AppConfig, output_format: &OutputFormat) {
    let packages =
        get_packages_info(ServerInfo::new("/Packages", &cfg.server_url, &cfg.api_key))
            .unwrap();

    match output_format {
        OutputFormat::Json => {
            PackageDetails::json_print(&packages);
        }
        OutputFormat::Csv => {
            PackageDetails::csv_print(packages);
        }
        OutputFormat::Table => {
            PackageDetails::table_print(packages);
        }
    }
}

pub fn command_get_plugins(cfg: AppConfig, output_format: &OutputFormat) {
    let plugins: Vec<PluginDetails> = match PluginInfo::get_plugins(PluginInfo::new(
        "/Plugins",
        &cfg.server_url,
        cfg.api_key,
    )) {
        Err(_) => {
            eprintln!("Unable to get plugin information.");
            std::process::exit(1);
        }
        Ok(i) => i,
    };

    match output_format {
        OutputFormat::Json => {
            PluginDetails::json_print(&plugins);
        }
        OutputFormat::Csv => {
            PluginDetails::csv_print(plugins);
        }
        OutputFormat::Table => {
            PluginDetails::table_print(plugins);
        }
    }
}

pub fn command_get_repositories(cfg: &AppConfig, output_format: &OutputFormat) {
    let repos = get_repo_info(ServerInfo::new(
        "/Repositories",
        &cfg.server_url,
        &cfg.api_key,
    ))
    .unwrap();

    match output_format {
        OutputFormat::Json => {
            RepositoryDetails::json_print(&repos);
        }
        OutputFormat::Csv => {
            RepositoryDetails::csv_print(repos);
        }
        OutputFormat::Table => {
            RepositoryDetails::table_print(repos);
        }
    }
}

pub fn command_get_scheduled_tasks(cfg: &AppConfig, output_format: &OutputFormat) {
    let tasks: Vec<TaskDetails> = match get_scheduled_tasks(ServerInfo::new(
        "/ScheduledTasks",
        &cfg.server_url,
        &cfg.api_key,
    )) {
        Err(e) => {
            eprintln!("Unable to get scheduled tasks, {e}");
            std::process::exit(1);
        }
        Ok(i) => i,
    };

    match output_format {
        OutputFormat::Json => {
            TaskDetails::json_print(&tasks);
        }
        OutputFormat::Csv => {
            TaskDetails::csv_print(&tasks);
        }
        OutputFormat::Table => {
            TaskDetails::table_print(tasks);
        }
    }
}

pub fn command_install_package(cfg: &AppConfig, package: &str, version: &str, repository: &str) {
     // Check if package name has spaces and replace them as needed
    let encoded = package.replace(' ', "%20");
    install_package(
        &ServerInfo::new(
            "/Packages/Installed/{package}",
            &cfg.server_url,
            &cfg.api_key,
        ),
        &encoded,
        version,
        repository,
    );
}

pub fn command_register_repository(cfg: &AppConfig, name: String, path: String) {
    let mut repos = get_repo_info(ServerInfo::new(
        "/Repositories",
        &cfg.server_url,
        &cfg.api_key,
    ))
    .unwrap();
    repos.push(RepositoryDetails::new(name, path, true));
    set_repo_info(
        ServerInfo::new("/Repositories", &cfg.server_url, &cfg.api_key),
        &repos,
    );
}

pub fn command_create_backup(cfg: &AppConfig) {
    let server_info = ServerInfo::new("/Backup/Create", &cfg.server_url, &cfg.api_key);
    let body= "{\"Metadata\": true,\"Trickplay\": true,\"Subtitles\": true,\"Database\": true}";
    let response = simple_post(
        server_info.server_url,
        &cfg.api_key,
        body.to_string(),
        "application/json"
    );
    match response.status() {
        StatusCode::OK => {
            println!("Success");
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }
}

pub fn command_apply_backup(cfg: &AppConfig, filename: &str) {
    let server_info = ServerInfo::new("/Backup/Restore", &cfg.server_url, &cfg.api_key);
    let body = format!("{{\"ArchiveFileName\": \"{filename}\"}}");
    let response = simple_post(
        server_info.server_url,
        &cfg.api_key,
        body.to_string(),
        "application/json"
    );
    match response.status() {
        StatusCode::OK => {
            println!("Success");
        }
        StatusCode::UNAUTHORIZED => {
            handle_unauthorized();
        }
        _ => {
            handle_others(&response);
        }
    }
}

pub fn command_get_backups(cfg: &AppConfig, output_format: &OutputFormat, backups_endpoint: &str) {
    let backups = match get_backups_info(ServerInfo::new(
        backups_endpoint,
        &cfg.server_url,
        &cfg.api_key
    )) {
        Err(e) => {
            eprintln!("Unable to get backups, {e}");
            std::process::exit(1);
        }
        Ok(i) => i
    };
    
    match output_format {
        OutputFormat::Json =>{
            BackupDetails::json_print(&backups);
        }
        OutputFormat::Csv => {
            BackupDetails::csv_print(backups);
        }
        OutputFormat::Table => {
            BackupDetails::table_print(backups);
        }
    }
    
}
/// All of the following calls are POST
/// 
/// Call /Startup/Configuration with JSON body of:
/// * `MetadataCountryCode`
/// * `PreferredMetadataLanguage`
/// * `UICulture`
/// 
/// Call /Startup/User with JSON body of:
/// * Name
/// * Password
/// 
/// Call /Startup/RemoteAccess with JSON body of:
/// * `EnableAutomaticPortMapping`
/// * `EnableRemoteAccess`
/// 
/// Call /Startup/Complete
/// * No configuration items needed 
pub fn command_server_setup(server_url: String, filename: String) {
    let server_config = PropReader::new(&filename);

    // Setup and execute the /Startup/Configuration call
    let mut body = format!(
        "{{\"MetadataCountryCode\": \"{}\",
        \"PreferredMetadataLanguage\": \"{}\",
        \"UICulture\": \"{}\"}}", 
        server_config.get("MetadataCountryCode"),
        server_config.get("PreferredMetadataLanguage"),
        server_config.get("UICulture")
    );

    let mut response = simple_post(
        format!("{server_url}/Startup/Configuration"),
        "",
        body.to_string(),
        "application/json"
    );
    match response.status() {
        StatusCode::NO_CONTENT => {
            println!("Configuration successfully submitted.");
        }
        _ => {
            handle_others(&response);
        }
    }

    // Jellyfin seemingly requires that a call to /Startup/User via GET is required before registering the first user.
    simple_get(format!("{server_url}/Startup/User"), "", Vec::new());

    // Setup and execute the /Startup/User call
    body = format!(
        "{{\"Name\":\"{}\",\"Password\":\"{}\"}}", 
        server_config.get("Name"),
        server_config.get("Password")
    );

    response = simple_post(
        format!("{server_url}/Startup/User"),
        "",
        body.to_string(),
        "application/json"
    );

    match response.status() {
        StatusCode::NO_CONTENT => {
            println!("Initial user successfully submitted.");
        }
        _ => {
            handle_others(&response);
        }
    }

    // Setup and execute the /Setup/RemoteAccess call
    body = format!(
        "{{\"EnableAutomaticPortMapping\": {},
        \"EnableRemoteAccess\": {}}}", 
        server_config.get("EnableAutomaticPortMapping"),
        server_config.get("EnableRemoteAccess")
    );

    response = simple_post(
        format!("{server_url}/Startup/RemoteAccess"),
        "",
        body.to_string(),
        "application/json"
    );

    match response.status() {
        StatusCode::NO_CONTENT => {
            println!("Initial remote access successfully submitted.");
        }
        _ => {
            handle_others(&response);
        }
    }

    // Execute a call to /Startup/Complete to flag that the startup wizard has been completed
    response = simple_post(
        format!("{server_url}/Startup/Complete"),
        "",
        String::new(),
        "application/json"
    );

    match response.status() {
        StatusCode::NO_CONTENT => {
            println!("Startup wizard completed successfully.");
        }
        _ => {
            handle_others(&response);
        }
    }

}

/* 
    The following section contains additional
    functions that are used to support the server_commands
    base functions.
*/

pub fn token_to_api(mut cfg: AppConfig) {
    println!("[INFO] Attempting to auto convert user auth token to API key.....");
    // Check if api key already exists
    if UserWithPass::retrieve_api_token(UserWithPass::new(
        None,
        None,
        None,
        format!("{}/Auth/Keys", cfg.server_url),
        cfg.api_key.clone(),
    ))
    .unwrap()
    .is_empty()
    {
        UserWithPass::create_api_token(UserWithPass::new(
            None,
            None,
            None,
            format!("{}/Auth/Keys", cfg.server_url),
            cfg.api_key.clone(),
        ));
    }
    cfg.api_key = UserWithPass::retrieve_api_token(UserWithPass::new(
        None,
        None,
        None,
        format!("{}/Auth/Keys", cfg.server_url),
        cfg.api_key,
    ))
    .unwrap();
    cfg.token = "apiKey".to_string();
    confy::store("jellyroller", "jellyroller", cfg)
        .expect("[ERROR] Unable to store updated configuration.");
    println!("[INFO] Auth token successfully converted to API key.");
}