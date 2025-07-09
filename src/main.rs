#![warn(clippy::all)]
#![warn(clippy::pedantic)]
// #![warn(clippy::restriction)]
// #![warn(clippy::nursery)]
#![warn(clippy::cargo)]

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Shell};
use std::env;
use std::fmt;
use std::io::{self, Write};

mod user_actions;
use user_actions::{UserAuth, UserList, UserWithPass};

mod system_actions;
use system_actions::{LogFile, restart_or_shutdown, get_server_info};

mod plugin_actions;
mod responder;

mod entities;
use entities::activity_details::ActivityDetails;
use entities::device_details::{DeviceDetails, DeviceRootJson};
use entities::library_details::{LibraryDetails, LibraryRootJson};
use entities::log_details::LogDetails;
use entities::movie_details::MovieDetails;
use entities::package_details::{PackageDetails, PackageDetailsRoot};
use entities::plugin_details::{PluginDetails, PluginRootJson};
use entities::repository_details::{RepositoryDetailsRoot};
use entities::server_info::ServerInfo;
use entities::user_details::{Policy, UserDetails};

mod utils;
use utils::status_handler::{handle_others, handle_unauthorized};

// All public functions in the below use statements are used within this file, so just get them all.
mod commands;
use commands::log_commands::{command_create_report, command_generate_report, command_list_logs};
use commands::media_commands::{command_get_libraries, command_register_libarary, command_scan_library, command_search_media, command_update_metadata, command_update_image_by_name, command_update_image_by_id};
use commands::server_commands::{command_execute_task_by_name, command_get_devices, command_get_packages, command_get_plugins, command_get_repositories, command_get_scheduled_tasks, command_initialize, command_install_package, command_register_repository};
use commands::user_commands::{command_add_user, command_add_users, command_delete_user, command_disable_user, command_enable_user, command_grant_admin, command_list_users, command_remove_device_by_username, command_reset_password, command_revoke_admin, command_update_users};

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
    token: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            status: "not configured".to_owned(),
            comfy: true,
            server_url: "Unknown".to_owned(),
            os: "Unknown".to_owned(),
            api_key: "Unknown".to_owned(),
            token: "Unknown".to_owned(),
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
    /// Uses the supplied file to mass create new users.  
    AddUsers {
        /// File that contains the user information in "username,password" lines.
        #[clap(required = true, value_parser)]
        inputfile: String,
    },
    /// Generate Shell completions
    Completions {
        #[clap(required = true, value_parser)]
        shell: Shell,
    },
    /// Creates a report of either activity or available movie items
    CreateReport {
        /// Type of report (activity or movie)
        #[clap(required = true)]
        report_type: ReportType,
        /// Total number of records to return (defaults to 100)
        #[clap(required = false, short, long, default_value = "100")]
        limit: String,
        /// Output filename
        #[clap(required = false, short, long, default_value = "")]
        filename: String,
    },
    /// Deletes an existing user.
    #[clap(arg_required_else_help = true)]
    DeleteUser {
        /// User to remove.
        #[clap(required = true, value_parser)]
        username: String,
    },
    /// Disable a user.
    DisableUser {
        #[clap(required = true, value_parser)]
        username: String,
    },
    /// Enable a user.
    EnableUser {
        #[clap(required = true, value_parser)]
        username: String,
    },
    /// Executes a scheduled task by name.
    ExecuteTaskByName {
        #[clap(required = true, value_parser)]
        task: String,
    },
    /// Generate a report for an issue.
    GenerateReport {},
    /// Show all devices.
    GetDevices {
        /// Only show devices active in the last hour
        #[clap(long, required = false)]
        active: bool,
        /// Specify the output format
        #[clap(short = 'o', long, value_enum, default_value = "table")]
        output_format: OutputFormat,
    },
    /// Gets the libraries available to the configured user
    GetLibraries {
        /// Specify the output format
        #[clap(short = 'o', long, value_enum, default_value = "table")]
        output_format: OutputFormat,
    },
    /// Lists all available packages
    GetPackages {
        /// Specify the output format
        #[clap(short = 'o', long, value_enum, default_value = "table")]
        output_format: OutputFormat,
    },
    /// Returns a list of installed plugins
    GetPlugins {
        /// Specify the output format
        #[clap(short = 'o', long, value_enum, default_value = "table")]
        output_format: OutputFormat,
    },
    /// Lists all current repositories
    GetRepositories {
        /// Specify the output format
        #[clap(short = 'o', long, value_enum, default_value = "table")]
        output_format: OutputFormat,
    },
    /// Show all scheduled tasks and their status.
    GetScheduledTasks {
        /// Specify the output format
        #[clap(short = 'o', long, value_enum, default_value = "table")]
        output_format: OutputFormat,
    },
    /// Grants the specified user admin rights.
    GrantAdmin {
        #[clap(required = true, value_parser)]
        username: String,
    },
    /// Perform a silent initialization.
    Initialize {
        /// Username for API key creation
        #[clap(required = true, long = "username")]
        username: String,
        /// Password for user
        #[clap(required = true, long = "password")]
        password: String,
        /// URL of server
        #[clap(required = true, long = "url")]
        server_url: String
    },
    /// Installs the specified package
    InstallPackage {
        /// Package to install
        #[clap(short = 'p', long = "package", required = true)]
        package: String,
        /// Version to install
        #[clap(short = 'v', long = "version", required = false, default_value = "")]
        version: String,
        /// Repository to install from
        #[clap(short = 'r', long = "repository", required = false, default_value = "")]
        repository: String,
    },
    /// Displays the available system logs.
    ListLogs {
        /// Specify the output format
        #[clap(short = 'o', long, value_enum, default_value = "table")]
        output_format: OutputFormat,
    },
    /// Lists the current users with basic information.
    ListUsers {
        /// Exports the user list information to a file
        #[clap(short, long)]
        export: bool,
        /// Path for the file export
        #[clap(short, long, default_value = "")]
        output: String,
        /// Username to gather information about
        #[clap(short, long, default_value = "")]
        username: String,
    },
    /// Reconfigure the connection information.
    Reconfigure {},
    /// Registers a new library.
    RegisterLibrary {
        /// Name of the new library
        #[clap(required = true, short = 'n', long)]
        name: String,
        /// Collection Type of the new library
        #[clap(required = true, short = 'c', long)]
        collectiontype: CollectionType,
        /// Path to file that contains the JSON for the library
        #[clap(required = true, short = 'f', long)]
        filename: String,
    },
    /// Registers a new Plugin Repository
    RegisterRepository {
        /// Name of the new repository
        #[clap(required = true, short = 'n', long = "name")]
        name: String,
        /// URL of the new repository
        #[clap(required = true, short = 'u', long = "url")]
        path: String,
    },
    /// Removes all devices associated with the specified user.
    RemoveDeviceByUsername {
        #[clap(required = true, value_parser)]
        username: String,
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
    /// Revokes admin rights from the specified user.
    RevokeAdmin {
        #[clap(required = true, value_parser)]
        username: String,
    },
    /// Restarts Jellyfin
    RestartJellyfin {},
    /// Start a library scan.
    ScanLibrary {
        /// Library ID
        #[clap(required = false, value_parser, default_value = "all")]
        library_id: String,
        /// Type of scan
        #[clap(required = false, default_value = "all")]
        scan_type: ScanType,
    },
    /// Executes a search of your media
    SearchMedia {
        /// Search term
        #[clap(required = true, short, long)]
        term: String,
        /// Filter for media type
        #[clap(required = false, short, long, default_value = "all")]
        mediatype: String,
        #[clap(required = false, short, long, default_value = "")]
        parentid: String,
        #[clap(short = 'o', long, value_enum, default_value = "table")]
        output_format: OutputFormat,
        /// By default, the server does not include file paths in the search results. Setting this
        /// will tell the server to include the file path in the search results.
        #[clap(short = 'f', long, required = false)]
        include_filepath: bool,
        /// Available columns: `Name`, `Id`, `Type`, `Path`, `CriticRating`, `ProductionYear`
        #[clap(short = 'c', long, value_parser, num_args = 0.., value_delimiter = ',', default_value = "Name,ID,Type")]
        table_columns: Vec<String>,
    },
    /// Displays the server information.
    ServerInfo {},
    /// Displays the requested logfile.
    ShowLog {
        /// Name of the logfile to show.
        #[clap(required = true, value_parser)]
        logfile: String,
    },
    /// Shuts down Jellyfin
    ShutdownJellyfin {},
    /// Updates image of specified file by id
    UpdateImageById {
        /// Attempt to update based on item id.
        #[clap(required = true, short = 'i', long)]
        id: String,
        /// Path to the image that will be used.
        #[clap(required = true, short, long)]
        path: String,
        #[clap(required = true, short = 'I', long)]
        imagetype: ImageType,
    },
    /// Updates image of specified file by name
    UpdateImageByName {
        /// Attempt to update based on title.  Requires unique search term.
        #[clap(required = true, short, long)]
        title: String,
        /// Path to the image that will be used.
        #[clap(required = true, short, long)]
        path: String,
        #[clap(required = true, short, long)]
        imagetype: ImageType,
    },
    /// Updates metadata of specified id with metadata provided by specified file
    UpdateMetadata {
        /// ID of the file to update
        #[clap(required = true, short = 'i', long)]
        id: String,
        /// File that contains the metadata to upload to the server
        #[clap(required = true, short = 'f', long)]
        filename: String,
    },
    /// Mass update users in the supplied file
    UpdateUsers {
        /// File that contains the user JSON information.
        #[clap(required = true, value_parser)]
        inputfile: String,
    }
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum CollectionType {
    Movies,
    TVShows,
    Music,
    MusicVideos,
    HomeVideos,
    BoxSets,
    Books,
    Mixed,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum Detail {
    User,
    Server,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum ImageType {
    Primary,
    Art,
    Backdrop,
    Banner,
    Logo,
    Thumb,
    Disc,
    Box,
    Screenshot,
    Menu,
    BoxRear,
    Profile,
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    Json,
    Csv,
    Table,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum ReportType {
    Activity,
    Movie,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum ScanType {
    NewUpdated,
    MissingMetadata,
    ReplaceMetadata,
    All,
}

fn main() -> Result<(), confy::ConfyError> {
    let mut current = env::current_exe().unwrap();
    current.pop();
    current.push("jellyroller.config");

    let cfg: AppConfig = if std::path::Path::new(current.as_path()).exists() {
        confy::load_path(current.as_path())?
    } else {
        confy::load("jellyroller", "jellyroller")?
    };

    // Due to an oddity with confy and clap, manually check for help flag.
    let args: Vec<String> = env::args().collect();
    if !(args.contains(&"initialize".to_string()) || args.contains(&"-h".to_string()) || args.contains(&"--help".to_string())) {
        if cfg.status == "not configured" {
            println!("Application is not configured!");
            initial_config(cfg);
            std::process::exit(0);
        } else if cfg.token == "Unknown" {
            println!("[INFO] Username/Password detected.  Reconfiguring to use API key.");
            token_to_api(cfg.clone());
        }
    }

    let args = Cli::parse();

    match args.command {
        // Log Commands
        Commands::CreateReport { report_type, limit, filename } => command_create_report(&cfg, &report_type, &limit, filename),
        Commands::GenerateReport {} => command_generate_report(&cfg),
        Commands::ListLogs { output_format } => command_list_logs(&cfg, &output_format),
        Commands::ShowLog { logfile } => LogFile::get_logfile(LogFile::new(ServerInfo::new("/System/Logs/Log", &cfg.server_url, &cfg.api_key),logfile,)).expect("Unable to retrieve the specified logfile."),
        
        // Media Commands
        Commands::GetLibraries { output_format } => command_get_libraries(&cfg, &output_format),
        Commands::RegisterLibrary { name, collectiontype, filename } => command_register_libarary(&cfg, &name, &collectiontype, filename),
        Commands::ScanLibrary { library_id, scan_type } => command_scan_library(&cfg, &library_id, &scan_type),
        Commands::SearchMedia { term, mediatype, parentid, output_format, include_filepath, table_columns } => command_search_media(&cfg, &term, &mediatype, &parentid, &output_format, include_filepath, &table_columns),
        Commands::UpdateMetadata { id, filename } => command_update_metadata(&cfg, &id, filename),
        Commands::UpdateImageByName {title, path, imagetype} => command_update_image_by_name(&cfg, &title, path, &imagetype),
        Commands::UpdateImageById { id, path, imagetype } => command_update_image_by_id(&cfg, &id, path, &imagetype),
        
        // Server Commands
        Commands::ExecuteTaskByName { task } => command_execute_task_by_name(&cfg, &task),
        Commands::GetDevices { active, output_format} => command_get_devices(&cfg, active, &output_format, DEVICES),
        Commands::GetPackages { output_format } => command_get_packages(&cfg, &output_format),
        Commands::GetPlugins { output_format} => command_get_plugins(cfg, &output_format),
        Commands::GetRepositories { output_format } => command_get_repositories(&cfg, &output_format),
        Commands::GetScheduledTasks { output_format } => command_get_scheduled_tasks(&cfg, &output_format),
        Commands::Initialize { username, password, server_url } => command_initialize(cfg, &username, password, &server_url),
        Commands::InstallPackage { package, version, repository} => command_install_package(&cfg, &package, &version, &repository),
        Commands::Reconfigure {} => initial_config(cfg),
        Commands::RegisterRepository { name, path } => command_register_repository(&cfg, name, path),
        Commands::RestartJellyfin {} => restart_or_shutdown(ServerInfo::new("/System/Restart",&cfg.server_url,&cfg.api_key,)),
        Commands::ServerInfo {} => get_server_info(ServerInfo::new("/System/Info", &cfg.server_url, &cfg.api_key,)).expect("Unable to gather server information."),        
        Commands::ShutdownJellyfin {} => restart_or_shutdown(ServerInfo::new("/System/Shutdown",&cfg.server_url,&cfg.api_key,)),

        // User commands
        Commands::AddUser { username, password } => command_add_user(&cfg, username, password),
        Commands::AddUsers { inputfile } => command_add_users(&cfg, inputfile),
        Commands::DeleteUser { username } => command_delete_user(cfg, username),
        Commands::DisableUser { username } => command_disable_user(&cfg, &username, USER_POLICY, USER_ID),
        Commands::EnableUser { username } => command_enable_user(&cfg, &username, USER_POLICY, USER_ID),
        Commands::GrantAdmin { username } => command_grant_admin(&cfg, &username, USER_POLICY, USER_ID),
        Commands::ListUsers { export, output, username } => command_list_users(&cfg, export, output, &username, USERS, USER_ID),
        Commands::RemoveDeviceByUsername { username } => command_remove_device_by_username(&cfg, &username, DEVICES),
        Commands::ResetPassword { username, password } => command_reset_password(cfg, &username, password, USERS),
        Commands::RevokeAdmin { username } => command_revoke_admin(&cfg, &username, USER_POLICY, USER_ID),
        Commands::UpdateUsers { inputfile } => command_update_users(&cfg, inputfile, USER_ID),
        
        // Other
        Commands::Completions { shell } => {
            let cmd = &mut Cli::command();
            generate(shell, cmd, cmd.get_name().to_string(), &mut io::stdout());
        }
    }

    Ok(())
}

///
/// Executed on initial run or when user wants to redo configuration.  Will attempt to auto-configure
/// the application prior to allowing customization by
/// the user.
///
fn initial_config(mut cfg: AppConfig) {
    println!("[INFO] Attempting to determine Jellyfin information.....");
    env::consts::OS.clone_into(&mut cfg.os);
    println!("[INFO] OS detected as {}.", cfg.os);

    print!("[INPUT] Please enter your Jellyfin URL:  ");
    io::stdout().flush().expect("Unable to get Jellyfin URL.");
    let mut server_url_input = String::new();
    io::stdin()
        .read_line(&mut server_url_input)
        .expect("Could not read server url information");
    server_url_input.trim().clone_into(&mut cfg.server_url);

    print!("[INPUT] Please enter your Jellyfin username:  ");
    io::stdout().flush().expect("Unable to get username.");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("[ERROR] Could not read Jellyfin username");
    let password = rpassword::prompt_password("Please enter your Jellyfin password: ").unwrap();
    println!("[INFO] Attempting to authenticate user.");
    cfg.api_key = UserAuth::auth_user(UserAuth::new(&cfg.server_url, username.trim(), password))
        .expect("Unable to generate user auth token.  Please assure your configuration information was input correctly\n");

    "configured".clone_into(&mut cfg.status);
    token_to_api(cfg);
}

///
/// Due to an issue with api key processing in Jellyfin, `JellyRoller` was initially relied on using auto tokens to communicate.
/// Now that the issue has been fixed, the auto tokens need to be converted to an API key.  The single purpose of this function
/// is to handle the conversion with no input required from the user.
///
fn token_to_api(mut cfg: AppConfig) {
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

///
/// Custom implementation to convert the `ImageType` enum into Strings
/// for easy comparison.
///
impl fmt::Display for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageType::Primary => write!(f, "Primary"),
            ImageType::Art => write!(f, "Art"),
            ImageType::Backdrop => write!(f, "Backdrop"),
            ImageType::Banner => write!(f, "Banner"),
            ImageType::Logo => write!(f, "Logo"),
            ImageType::Thumb => write!(f, "Thumb"),
            ImageType::Disc => write!(f, "Disc"),
            ImageType::Box => write!(f, "Box"),
            ImageType::Screenshot => write!(f, "Screenshot"),
            ImageType::Menu => write!(f, "Menu"),
            ImageType::BoxRear => write!(f, "BoxRear"),
            ImageType::Profile => write!(f, "Profile"),
        }
    }
}

///
/// Custom implementation to convert collectiontype enum into Strings
///
impl fmt::Display for CollectionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CollectionType::Movies => write!(f, "movies"),
            CollectionType::TVShows => write!(f, "tvshows"),
            CollectionType::Music => write!(f, "music"),
            CollectionType::MusicVideos => write!(f, "musicvideos"),
            CollectionType::HomeVideos => write!(f, "homevideos"),
            CollectionType::BoxSets => write!(f, "boxsets"),
            CollectionType::Books => write!(f, "books"),
            CollectionType::Mixed => write!(f, "mixed"),
        }
    }
}
