use std::env;
use std::io::{self, Write};
use clap::{Parser, Subcommand, ValueEnum};

mod user_actions;
use user_actions::*;
mod system_actions;
use system_actions::*;
mod plugin_actions;
use plugin_actions::*;
mod responder;
mod entities;
use entities::user_details::{UserDetails, Policy};
use entities::device_details::{DeviceDetails, DeviceRootJson};
use entities::task_details::TaskDetails;
use entities::log_details::LogDetails;
use entities::library_details::{LibraryDetails, LibraryRootJson};
use entities::plugin_details::{PluginDetails, PluginRootJson};
mod utils;
use utils::output_writer::*;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    status: String,
    comfy: bool,
    server_url: String,
    os: String,
    api_key: String,
    token: String
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            status: "not configured".to_string(),
            comfy: true,
            server_url: "Unknown".to_string(),
            os: "Unknown".to_string(),
            api_key: "Unknown".to_string(),
            token: "Unknown".to_string()
        }
    }
}

/// CLAP CONFIGURATION
/// CLI controller for Jellyfin
#[derive(Debug, Parser)] // requires `derive` feature
#[clap(name = "jellyroller", author, version)]
#[clap(about = "A CLI controller for managing Jellyfin", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Creates a new user
    #[clap(arg_required_else_help = true)]
    AddUser {
        /// Username to create.
        #[clap(required = true, value_parser)]
        username: String,
        /// Password for created user.
        #[clap(required = true, value_parser)]
        password: String,
    },
    /// Deletes an existing user.
    #[clap(arg_required_else_help = true)]
    DeleteUser {
        /// User to remove.
        #[clap(required = true, value_parser)]
        username: String,
    },
    /// Lists the current users with basic information.
    ListUsers {
        /// Exports the user list information to a file
        #[clap(short, long)]
        export: bool,
        /// Path for the file export
        #[clap(short, long, default_value="")]
        output: String,
        /// Username to gather information about
        #[clap(short, long, default_value="")]
        username: String
    },
    /// Resets a user's password.
    #[clap(arg_required_else_help = true)]
    ResetPassword {
        /// User to be modified.
        #[clap(required = true, value_parser)]
        username: String,
        /// What to reset the specified user's password to.
        #[clap(required = true, value_parser)]
        password: String,
    },
    /// Displays the server information.
    ServerInfo {},
    /// Displays the available system logs.
    ListLogs{
        /// Print information as json.
        #[clap(long, required = false)]
        json: bool
    },
    /// Displays the requested logfile.
    ShowLog {
        /// Name of the logfile to show.
        #[clap(required = true, value_parser)]
        logfile: String
    },
    /// Reconfigure the connection information.
    Reconfigure {},
    /// Show all active devices.
    GetDevices {
        /// Print information as json.
        #[clap(long, required = false)]
        json: bool
    },
    /// Removes all devices associated with the specified user.
    RemoveDeviceByUsername {
        #[clap(required = true, value_parser)]
        username: String
    },
    /// Show all scheduled tasks and their status.
    GetScheduledTasks {
        /// Print information as json.
        #[clap(long, required = false)]
        json: bool
    },
    /// Executes a scheduled task by name.
    ExecuteTaskByName {
        #[clap(required = true, value_parser)]
        task: String
    },
    /// Start a library scan.
    ScanLibrary {},
    /// Disable a user.
    DisableUser {
        #[clap(required = true, value_parser)]
        username: String
    },
    /// Enable a user.
    EnableUser {
        #[clap(required = true, value_parser)]
        username: String
    },
    /// Grants the specified user admin rights.
    GrantAdmin {
        #[clap(required = true, value_parser)]
        username: String
    },
    /// Revokes admin rights from the specified user.
    RevokeAdmin {
        #[clap(required = true, value_parser)]
        username: String
    },
    /// Restarts Jellyfin
    RestartJellyfin {},
    /// Shuts down Jellyfin
    ShutdownJellyfin {},
    /// Gets the libraries available to the configured user
    GetLibraries {
        /// Print information as json.
        #[clap(long, required = false)]
        json: bool
    },
    /// Returns a list of installed plugins
    GetPlugins {
       /// Print information as json.
       #[clap(long, required = false)]
       json: bool 
    },
    // Returns the specified user's information (JSON only)
    // GetUserDetails {
    //     /// Username to lookup information for.
    //     #[clap(required = true, value_parser)]
    //     username: String
    // },
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum Detail {
    User,
    Server
}

fn main() -> Result<(), confy::ConfyError> {
    
    let cfg: AppConfig = confy::load("jellyroller")?;
    if cfg.status == "not configured" {
        println!("Application is not configured!");
        initial_config(cfg);
        std::process::exit(0);
    }
    let args = Cli::parse();
    match args.command {
        // User based commands
        Commands::AddUser { username, password } => {
            UserAdd::create(UserAdd::new(username, password, cfg.server_url, cfg.api_key))
                .expect("Unable to add user.");
        },
        Commands::DeleteUser { username } => {
            let user_id = get_user_id(&cfg, &username);
            UserDel::remove(UserDel::new(user_id, cfg.server_url, cfg.api_key))
                .expect("Unable to delete user.");
        },
        Commands::ListUsers { export, mut output, username} => {
            if username.is_empty() {
                let users = UserList::list_users(UserList::new("/Users".to_string(), cfg.server_url, cfg.api_key)).unwrap();
                if !export {
                    UserDetails::json_print_users(users);
                } else {
                    println!("Exporting all user information.....");
                    if output.is_empty() {
                        output = "exported-user-info.json".to_string();
                    }
                    let data = serde_json::to_string_pretty(&users).unwrap();
                    export_data(data, output);
                } 
            } else {
                let user_id = UserList::get_user_id(UserList::new("/Users".to_string(), cfg.server_url.to_string(), cfg.api_key.to_string()), &username);
                let user = UserList::get_user_information(UserList::new("/Users/{userId}".to_string(), cfg.server_url, cfg.api_key), user_id).unwrap();
                if !export {
                    UserDetails::json_print_user(user);
                } else {
                    println!("Exporting user information.....");
                    if output.is_empty() {
                        output = format!("exported-user-info-{}.json", username);
                    }
                    let data = serde_json::to_string_pretty(&user).unwrap();
                    export_data(data, output);
                }
            }
        },
        Commands::ResetPassword { username, password } => {
            let user_id = UserList::get_user_id(UserList::new("/Users".to_string(), cfg.server_url.to_string(), cfg.api_key.to_string()), &username);
            ResetPass::reset(ResetPass::new(user_id, password, cfg.server_url, cfg.api_key))
                .expect("Unable to reset user password.");
        },
        Commands::DisableUser { username } => {
            let id = get_user_id(&cfg, &username);
            let mut user_info = UserList::get_user_information(UserList::new("/Users/{userId}".to_string(), cfg.server_url.to_string(), cfg.api_key.to_string()), id.to_string()).unwrap();
            user_info.policy.is_disabled = true;
            UserList::update_user_config_bool(
                UserList::new("/Users/{userId}/Policy".to_string(), cfg.server_url, cfg.api_key),
                user_info.policy, 
                id,
                username)
                .expect("Unable to update user.");
        },
        Commands::EnableUser { username } => {
            let id = get_user_id(&cfg, &username);
            let mut user_info = UserList::get_user_information(UserList::new("/Users/{userId}".to_string(), cfg.server_url.to_string(), cfg.api_key.to_string()), id.to_string()).unwrap();
            user_info.policy.is_disabled = false;
            UserList::update_user_config_bool(
                UserList::new("/Users/{userId}/Policy".to_string(), cfg.server_url, cfg.api_key),
                user_info.policy, 
                id,
                username)
                .expect("Unable to update user.");
        },
        Commands::GrantAdmin { username } => {
            let id = get_user_id(&cfg, &username);
            let mut user_info = UserList::get_user_information(UserList::new("/Users/{userId}".to_string(), cfg.server_url.to_string(), cfg.api_key.to_string()), id.to_string()).unwrap();
            user_info.policy.is_administrator = true;
            UserList::update_user_config_bool(
                UserList::new("/Users/{userId}/Policy".to_string(), cfg.server_url, cfg.api_key),
                user_info.policy, 
                id,
                username)
                .expect("Unable to update user.");
        },
        Commands::RevokeAdmin { username } => {
            let id = get_user_id(&cfg, &username);
            let mut user_info = UserList::get_user_information(UserList::new("/Users/{userId}".to_string(), cfg.server_url.to_string(), cfg.api_key.to_string()), id.to_string()).unwrap();
            user_info.policy.is_administrator = false;
            UserList::update_user_config_bool(
                UserList::new("/Users/{userId}/Policy".to_string(), cfg.server_url, cfg.api_key),
                user_info.policy, 
                id,
                username)
                .expect("Unable to update user.");
        },
        // Commands::GetUserDetails { username } => {
        //     let id = get_user_id(&cfg, &username);
        //     let user_info = UserList::get_user_information(UserList::new("/Users/{userId}".to_string(), cfg.server_url, cfg.api_key), id.to_string()).unwrap(); 
        // },

        // Server based commands
        Commands::ServerInfo {} => {
            ServerInfo::get_server_info(ServerInfo::new("/System/Info".to_string(), cfg.server_url, cfg.api_key))
                .expect("Unable to gather server information.");
        },

        Commands::ListLogs { json } => {
            let logs = ServerInfo::get_log_filenames(ServerInfo::new("/System/Logs".to_string(), cfg.server_url, cfg.api_key)).unwrap();
            if json {
                LogDetails::json_print(logs);
            } else {
                LogDetails::table_print(logs);
            }     
        },
        Commands::ShowLog { logfile } => {
            LogFile::get_logfile(LogFile::new("/System/Logs/Log".to_string(), cfg.server_url, cfg.api_key, logfile))
                .expect("Unable to retrieve the specified logfile.");
        },
        Commands::Reconfigure {} => {
            initial_config(cfg);
        },
        Commands::GetDevices { json } => {
            let devices = ServerInfo::get_devices(ServerInfo::new("/Devices".to_string(), cfg.server_url, cfg.api_key)).unwrap();
            if json {
                DeviceDetails::json_print(devices);
            } else {
                DeviceDetails::table_print(devices);
            }
        },
        Commands::GetLibraries { json } => {
            let libraries = ServerInfo::get_libraries(ServerInfo::new("/Library/VirtualFolders".to_string(), cfg.server_url, cfg.api_key)).unwrap();
            if json {
                LibraryDetails::json_print(libraries);
            } else {
                LibraryDetails::table_print(libraries);
            }
        },
        Commands::GetScheduledTasks { json } => {
            let tasks = ServerInfo::get_scheduled_tasks(ServerInfo::new("/ScheduledTasks".to_string(), cfg.server_url, cfg.api_key)).unwrap();
            if json {
                TaskDetails::json_print(tasks);
            } else {
                TaskDetails::table_print(tasks);
            }
        },
        Commands::ExecuteTaskByName { task } => {
            let taskid = ServerInfo::get_taskid_by_taskname(ServerInfo::new("/ScheduledTasks".to_string(), cfg.server_url.to_string(), cfg.api_key.to_string()), task.to_string()).unwrap();
            ServerInfo::execute_task_by_id(ServerInfo::new("/ScheduledTasks/Running/{taskId}".to_string(), cfg.server_url, cfg.api_key), task, taskid)
                .expect("Unable to start scheduled task.");
        }
        Commands::ScanLibrary {} => {
            ServerInfo::scan_library(ServerInfo::new("/Library/Refresh".to_string(), cfg.server_url, cfg.api_key))
                .expect("Unable to start library scan.");
        },
        Commands::RemoveDeviceByUsername { username } => {
            let filtered = ServerInfo::get_deviceid_by_username(ServerInfo::new("/Devices".to_string(), cfg.server_url.to_string(), cfg.api_key.to_string()), username).unwrap();
            for item in filtered {
                ServerInfo::remove_device(ServerInfo::new("/Devices".to_string(), cfg.server_url.to_string(), cfg.api_key.to_string()), item)
                    .expect("Unable to delete specified id.");
            }
        },
        Commands::RestartJellyfin {} => {
            ServerInfo::restart_or_shutdown(ServerInfo::new("/System/Restart".to_string(), cfg.server_url, cfg.api_key))
                .expect("Unable to restart Jellyfin.");
        },
        Commands::ShutdownJellyfin {} => {
            ServerInfo::restart_or_shutdown(ServerInfo::new("/System/Shutdown".to_string(), cfg.server_url, cfg.api_key))
                .expect("Unable to stop Jellyfin.");
        },
        Commands::GetPlugins { json } => {
            let plugins = PluginInfo::get_plugins(PluginInfo::new("/Plugins".to_string(), cfg.server_url, cfg.api_key)).unwrap();
            if json {
                PluginDetails::json_print(plugins);
            } else {
                PluginDetails::table_print(plugins);
            }
        }
        
    }
    
    Ok(())
}

///
/// Retrieve the id for the specified user.  Most API calls require the id of the user rather than the username.
/// 
fn get_user_id(cfg: &AppConfig, username: &String) -> String {
    UserList::get_user_id(UserList::new("/Users".to_string(), cfg.server_url.to_string(), cfg.api_key.to_string()), username)
}

///
/// Executed on initial run or when user wants to redo configuration.  Will attempt to auto-configure
/// the application prior to allowing customization by 
/// the user.
/// 
fn initial_config(mut cfg: AppConfig) {
    println!("[INFO] Attempting to determine Jellyfin information.....");
    cfg.os = env::consts::OS.to_string();
    println!("[INFO] OS detected as {}.", cfg.os);
    
    print!("[INPUT] Please enter your Jellyfin URL:  ");
    io::stdout().flush().unwrap();
    let mut server_url_input = String::new();
    io::stdin().read_line(&mut server_url_input)
        .expect("Could not read server url information");
    cfg.server_url = server_url_input.trim().to_string();
    
    print!("[INPUT] Please enter your Jellyfin username:  ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username)
        .expect("[ERROR] Could not read Jellyfin username");
    let password = rpassword::prompt_password("Please enter your Jellyfin password: ").unwrap();
    println!("Attempting to authenticate user.");
    cfg.api_key = UserAuth::auth_user(UserAuth::new(cfg.server_url.to_string(), username.trim().to_string(), password))
        .expect("Unable to generate user auth token.  Please assure your configuration information was input correctly\n"); 

    cfg.status = "configured".to_string();
    confy::store("jellyroller", cfg)
        .expect("[ERROR] Unable to store configuration.");
}