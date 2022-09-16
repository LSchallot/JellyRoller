use std::fs::{File, self};
use std::env;
use std::io::{self, Write, BufReader, BufRead};
use clap::{Parser, Subcommand, ValueEnum};

mod user_actions;
use user_actions::{ResetPass, UserAdd, UserAuth, UserDel, UserList};
mod system_actions;
use system_actions::{LogFile, ServerInfo};
mod plugin_actions;
use plugin_actions::PluginInfo;
mod responder;
mod entities;
use entities::user_details::{UserDetails, Policy};
use entities::device_details::{DeviceDetails, DeviceRootJson};
use entities::task_details::TaskDetails;
use entities::log_details::LogDetails;
use entities::library_details::{LibraryDetails, LibraryRootJson};
use entities::plugin_details::{PluginDetails, PluginRootJson};
use entities::activity_details::{ActivityDetails};
mod utils;
use utils::output_writer::export_data;

#[macro_use]
extern crate serde_derive;

//
// Global variables for API endpoints
//
const USER_POLICY: &str = "/Users/{userId}/Policy";
const USER_ID: &str = "/Users/{userId}";
const USERS: &str = "/Users";
const DEVICES: &str = "/Devices";

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
            status: "not configured".to_owned(),
            comfy: true,
            server_url: "Unknown".to_owned(),
            os: "Unknown".to_owned(),
            api_key: "Unknown".to_owned(),
            token: "Unknown".to_owned()
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
    /// Uses the supplied file to mass create new users.  
    AddUsers {
        /// File that contains the user information in "username,password" lines.
        #[clap(required = true, value_parser)]
        inputfile: String
    },
    /// Mass update users in the supplied file
    UpdateUsers {
        /// File that contains the user JSON information.
        #[clap(required = true, value_parser)]
        inputfile: String
    },
    /// Creates a report of either activity or available media items
    CreateReport {
        /// Type of report (activity or media)
        #[clap(required = true, arg_enum)]
        report_type: ReportType,
        /// Total number of records to return (defaults to 100)
        #[clap(required = false, short, long, default_value="100")]
        limit: String
        
    }
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum Detail {
    User,
    Server
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum ReportType {
    Activity,
    Media
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
            add_user(&cfg, username, password);
        },
        Commands::DeleteUser { username } => {
            let user_id = get_user_id(&cfg, &username);
            UserDel::remove(UserDel::new(user_id, &cfg.server_url, cfg.api_key))
                .expect("Unable to delete user.");
        },
        Commands::ListUsers { export, mut output, username} => {
            if username.is_empty() {
                let users: Vec<UserDetails> = 
                    match UserList::list_users(UserList::new(USERS, &cfg.server_url, cfg.api_key)) {
                        Err(_) => {
                            eprintln!("Unable to gather users.");
                            std::process::exit(0);
                        },
                        Ok(i) => i
                    };
                if export {
                    println!("Exporting all user information.....");
                    if output.is_empty() {
                        output = "exported-user-info.json".to_owned();
                    }
                    let data: String = 
                        match serde_json::to_string_pretty(&users) {
                            Err(_) => {
                                eprintln!("Unable to convert user information into JSON.");
                                std::process::exit(0);
                            },
                            Ok(i) => i
                        };
                    export_data(&data, output);
                } else {
                    UserDetails::json_print_users(&users);
                } 
            } else {
                let user_id = UserList::get_user_id(UserList::new(USERS, &cfg.server_url, cfg.api_key.clone()), &username);
                let user = gather_user_information(&cfg, &username, &user_id);
                if export {
                    println!("Exporting user information.....");
                    if output.is_empty() {
                        output = format!("exported-user-info-{}.json", username);
                    }
                    let data: String = 
                        match serde_json::to_string_pretty(&user) {
                            Err(_) => {
                                eprintln!("Unable to convert user information into JSON.");
                                std::process::exit(0);
                            },
                            Ok(i) => i
                        };
                    export_data(&data, output);
                } else {
                    UserDetails::json_print_user(&user);
                }
            }
        },
        Commands::ResetPassword { username, password } => {
            let user_id = UserList::get_user_id(UserList::new(USERS, &cfg.server_url, cfg.api_key.clone()), &username);
            match ResetPass::reset(ResetPass::new(&user_id, password, &cfg.server_url, &cfg.api_key)) {
                Err(_) => {
                    eprintln!("Unable to convert user information into JSON.");
                    std::process::exit(0);
                },
                Ok(i) => i
                        
            }
        },
        Commands::DisableUser { username } => {
            let id = get_user_id(&cfg, &username);
            let mut user_info = gather_user_information(&cfg, &username, &id);
            user_info.policy.is_disabled = true;
            UserList::update_user_config_bool(
                UserList::new(USER_POLICY, &cfg.server_url, cfg.api_key),
                &user_info.policy, 
                &id,
                &username)
                .expect("Unable to update user.");
        },
        Commands::EnableUser { username } => {
            let id = get_user_id(&cfg, &username);
            let mut user_info = gather_user_information(&cfg, &username, &id);
            user_info.policy.is_disabled = false;
            UserList::update_user_config_bool(
                UserList::new(USER_POLICY, &cfg.server_url, cfg.api_key),
                &user_info.policy, 
                &id,
                &username)
                .expect("Unable to update user.");
        },
        Commands::GrantAdmin { username } => {
            let id = get_user_id(&cfg, &username);
            let mut user_info = gather_user_information(&cfg, &username, &id);
            user_info.policy.is_administrator = true;
            UserList::update_user_config_bool(
                UserList::new(USER_POLICY, &cfg.server_url, cfg.api_key),
                &user_info.policy, 
                &id,
                &username)
                .expect("Unable to update user.");
        },
        Commands::RevokeAdmin { username } => {
            let id = get_user_id(&cfg, &username);
            let mut user_info = gather_user_information(&cfg, &username, &id);
            user_info.policy.is_administrator = false;
            UserList::update_user_config_bool(
                UserList::new(USER_POLICY, &cfg.server_url, cfg.api_key),
                &user_info.policy, 
                &id,
                &username)
                .expect("Unable to update user.");
        },
        Commands::AddUsers { inputfile } => {
            let reader = BufReader::new(File::open(inputfile).unwrap());
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        let vec: Vec<&str> = l.split(',').collect();
                        add_user(&cfg, vec[0].to_owned(), vec[1].to_owned());

                    },
                    Err(e) => println!("Unable to add user.  {e}")
                }
            }
        },
        Commands::UpdateUsers { inputfile } => {
            let data: String = 
                match fs::read_to_string(inputfile) {
                    Err(_) => {
                        eprintln!("Unable to process input file.");
                        std::process::exit(0);
                    },
                    Ok(i) => i
                };
            if data.starts_with('[') {
                let info: Vec<UserDetails> = 
                    match serde_json::from_str::<Vec<UserDetails>>(&data) {
                        Err(_) => {
                            eprintln!("Unable to convert user details JSON..");
                            std::process::exit(0);
                        },
                        Ok(i) => i
                    };
                for item in info {
                    match UserList::update_user_info(
                        UserList::new(USER_ID, &cfg.server_url, cfg.api_key.clone()),
                        &item.id,
                        &item
                    ) {
                        Ok(_) => {},
                        Err(e) => eprintln!("Unable to update user.  {e}")
                    };
                    
                }
            } else {
                let info: UserDetails = 
                    match serde_json::from_str::<UserDetails>(&data) {
                        Err(_) => {
                            eprintln!("Unable to convert user details JSON.");
                            std::process::exit(0);
                        },
                        Ok(i) => i
                    };
                let user_id = get_user_id(&cfg, &info.name);
                match UserList::update_user_info(
                    UserList::new(USER_ID, &cfg.server_url, cfg.api_key),
                    &user_id,
                    &info
                ) {
                    Ok(_) => {},
                    Err(e) => { eprintln!("Unable to update user.  {e}")}
                }
            }
        }

        // Server based commands
        Commands::ServerInfo {} => {
            ServerInfo::get_server_info(ServerInfo::new("/System/Info", &cfg.server_url, &cfg.api_key))
                .expect("Unable to gather server information.");
        },

        Commands::ListLogs { json } => {
            let logs = 
                match ServerInfo::get_log_filenames(ServerInfo::new("/System/Logs", &cfg.server_url, &cfg.api_key)) {
                    Err(_) => {
                        eprintln!("Unable to get get log filenames.");
                        std::process::exit(0);
                    },
                    Ok(i) => i
                };
            if json {
                LogDetails::json_print(&logs);
            } else {
                LogDetails::table_print(logs);
            }     
        },
        Commands::ShowLog { logfile } => {
            LogFile::get_logfile(LogFile::new("/System/Logs/Log", &cfg.server_url, cfg.api_key, logfile))
                .expect("Unable to retrieve the specified logfile.");
        },
        Commands::Reconfigure {} => {
            initial_config(cfg);
        },
        Commands::GetDevices { json } => {
            let devices: Vec<DeviceDetails> = 
                match ServerInfo::get_devices(ServerInfo::new(DEVICES, &cfg.server_url, &cfg.api_key)) {
                    Err(_) => {
                        eprintln!("Unable to get devices.");
                        std::process::exit(0);
                    },
                    Ok(i) => i
            };
            if json {
                DeviceDetails::json_print(&devices);
            } else {
                DeviceDetails::table_print(&devices);
            }
        },
        Commands::GetLibraries { json } => {
            let libraries: Vec<LibraryDetails> = 
             match ServerInfo::get_libraries(ServerInfo::new("/Library/VirtualFolders", &cfg.server_url, &cfg.api_key)) {
                Err(_) => {
                    eprintln!("Unable to get libraries.");
                    std::process::exit(0);
                },
                Ok(i) => i
             };
            if json {
                LibraryDetails::json_print(&libraries);
            } else {
                LibraryDetails::table_print(libraries);
            }
        },
        Commands::GetScheduledTasks { json } => {
            let tasks: Vec<TaskDetails> = 
                match ServerInfo::get_scheduled_tasks(ServerInfo::new("/ScheduledTasks", &cfg.server_url, &cfg.api_key)) {
                    Err(_) => {
                        eprintln!("Unable to get scheduled tasks.");
                        std::process::exit(0);
                    },
                    Ok(i) => i
                };
            
            if json {
                TaskDetails::json_print(&tasks);
            } else {
                TaskDetails::table_print(tasks);
            }
        },
        Commands::ExecuteTaskByName { task } => {
            let taskid: String = 
                match ServerInfo::get_taskid_by_taskname(ServerInfo::new("/ScheduledTasks", &cfg.server_url, &cfg.api_key), &task) {
                    Err(_) => {
                        eprintln!("Unable to get task id by taskname.");
                        std::process::exit(0);
                    },
                    Ok(i) => i
                };
            ServerInfo::execute_task_by_id(ServerInfo::new("/ScheduledTasks/Running/{taskId}", &cfg.server_url, &cfg.api_key), &task, &taskid);
        }
        Commands::ScanLibrary {} => {
            ServerInfo::scan_library(ServerInfo::new("/Library/Refresh", &cfg.server_url, &cfg.api_key));
        },
        Commands::RemoveDeviceByUsername { username } => {
            let filtered: Vec<String> = 
                match ServerInfo::get_deviceid_by_username(ServerInfo::new(DEVICES, &cfg.server_url, &cfg.api_key), &username) {
                    Err(_) => {
                        eprintln!("Unable to get device id by username.");
                        std::process::exit(0);
                    },
                    Ok(i) => i
                };
            for item in filtered {
                ServerInfo::remove_device(ServerInfo::new(DEVICES, &cfg.server_url, &cfg.api_key), &item)
                    .expect("Unable to delete specified id.");
            }
        },
        Commands::RestartJellyfin {} => {
            ServerInfo::restart_or_shutdown(ServerInfo::new("/System/Restart", &cfg.server_url, &cfg.api_key));
        },
        Commands::ShutdownJellyfin {} => {
            ServerInfo::restart_or_shutdown(ServerInfo::new("/System/Shutdown", &cfg.server_url, &cfg.api_key));
        },
        Commands::GetPlugins { json } => {
            let plugins: Vec<PluginDetails> = 
                match PluginInfo::get_plugins(PluginInfo::new("/Plugins", &cfg.server_url, cfg.api_key)) {
                    Err(_) => {
                        eprintln!("Unable to get plugin information.");
                        std::process::exit(0);
                    },
                    Ok(i) => i
                };
            if json {
                PluginDetails::json_print(&plugins);
            } else {
                PluginDetails::table_print(plugins);
            }
        },
        Commands::CreateReport { report_type, limit} => {
            match report_type {
                ReportType::Activity => {
                    let activities: ActivityDetails =
                        match ServerInfo::get_activity(ServerInfo::new("/System/ActivityLog/Entries", &cfg.server_url, &cfg.api_key), &limit) {
                            Err(e) => {
                                eprintln!("Unable to gather activity log entries, {e}");
                                std::process::exit(0);
                            },
                            Ok(i) => i
                        };
                    ActivityDetails::table_print(activities);
                },
                ReportType::Media => {
                    println!("Media");
                }
            }
        }
    }
    
    Ok(())
}

///
/// Retrieve the id for the specified user.  Most API calls require the id of the user rather than the username.
/// 
fn get_user_id(cfg: &AppConfig, username: &String) -> String {
    UserList::get_user_id(UserList::new("/Users", &cfg.server_url, cfg.api_key.clone()), username)
}

fn gather_user_information(cfg: &AppConfig, username: &String, id: &str) -> UserDetails {
    match UserList::get_user_information(UserList::new(USER_ID, &cfg.server_url, cfg.api_key.clone()), id) {
        Err(_) => {
            println!("Unable to get user id for {}", username);
            std::process::exit(0);
        },
        Ok(ul) => ul,
    }
}

///
/// Helper function to standardize the call for adding a user with a password.
/// 
fn add_user(cfg: &AppConfig, username: String, password: String) {
    UserAdd::create(UserAdd::new(username, password, &cfg.server_url, cfg.api_key.clone()))
                .expect("Unable to add user.");
}

///
/// Executed on initial run or when user wants to redo configuration.  Will attempt to auto-configure
/// the application prior to allowing customization by 
/// the user.
/// 
fn initial_config(mut cfg: AppConfig) {
    println!("[INFO] Attempting to determine Jellyfin information.....");
    cfg.os = env::consts::OS.to_owned();
    println!("[INFO] OS detected as {}.", cfg.os);
    
    print!("[INPUT] Please enter your Jellyfin URL:  ");
    io::stdout().flush().expect("Unable to get Jellyfin URL.");
    let mut server_url_input = String::new();
    io::stdin().read_line(&mut server_url_input)
        .expect("Could not read server url information");
    cfg.server_url = server_url_input.trim().to_owned();
    
    print!("[INPUT] Please enter your Jellyfin username:  ");
    io::stdout().flush().expect("Unable to get username.");
    let mut username = String::new();
    io::stdin().read_line(&mut username)
        .expect("[ERROR] Could not read Jellyfin username");
    let password = rpassword::prompt_password("Please enter your Jellyfin password: ").unwrap();
    println!("Attempting to authenticate user.");
    cfg.api_key = UserAuth::auth_user(UserAuth::new(&cfg.server_url, username.trim(), password))
        .expect("Unable to generate user auth token.  Please assure your configuration information was input correctly\n"); 

    cfg.status = "configured".to_owned();
    confy::store("jellyroller", cfg)
        .expect("[ERROR] Unable to store configuration.");
}