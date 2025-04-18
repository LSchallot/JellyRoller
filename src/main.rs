use base64::{engine::general_purpose, Engine as _};
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Shell};
use image::ImageFormat;
use std::env;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Cursor, Read, Write};
use std::{thread, time};

mod user_actions;
use user_actions::{UserAuth, UserList, UserWithPass};
mod system_actions;
use system_actions::*;
mod plugin_actions;
use plugin_actions::PluginInfo;
mod entities;
mod responder;
use entities::activity_details::ActivityDetails;
use entities::device_details::{DeviceDetails, DeviceRootJson};
use entities::library_details::{LibraryDetails, LibraryRootJson};
use entities::log_details::LogDetails;
use entities::media_details::MediaRoot;
use entities::movie_details::MovieDetails;
use entities::package_details::{PackageDetails, PackageDetailsRoot};
use entities::plugin_details::{PluginDetails, PluginRootJson};
use entities::repository_details::{RepositoryDetails, RepositoryDetailsRoot};
use entities::server_info::ServerInfo;
use entities::task_details::TaskDetails;
use entities::user_details::{Policy, UserDetails};
mod utils;
use utils::output_writer::export_data;
use utils::status_handler::{handle_others, handle_unauthorized};

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
        /// Print information as json.
        #[clap(long, required = false)]
        json: bool,
        /// Specify the output format
        #[clap(short = 'o', long, value_enum, default_value = "table")]
        output_format: OutputFormat,
    },
    /// Gets the libraries available to the configured user
    GetLibraries {
        /// Print information as json.
        #[clap(long, required = false)]
        json: bool,
        /// Specify the output format
        #[clap(short = 'o', long, value_enum, default_value = "table")]
        output_format: OutputFormat,
    },
    /// Lists all available packages
    GetPackages {
        /// Print information as json.
        #[clap(long, required = false)]
        json: bool,
        /// Specify the output format
        #[clap(short = 'o', long, value_enum, default_value = "table")]
        output_format: OutputFormat,
    },
    /// Returns a list of installed plugins
    GetPlugins {
        /// Print information as json.
        #[clap(long, required = false)]
        json: bool,
        /// Specify the output format
        #[clap(short = 'o', long, value_enum, default_value = "table")]
        output_format: OutputFormat,
    },
    /// Lists all current repositories
    GetRepositories {
        /// Print information as json.
        #[clap(long, required = false)]
        json: bool,
        /// Specify the output format
        #[clap(short = 'o', long, value_enum, default_value = "table")]
        output_format: OutputFormat,
    },
    /// Show all scheduled tasks and their status.
    GetScheduledTasks {
        /// Print information as json (DEPRECATED).
        #[clap(long, required = false)]
        json: bool,
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
        /// Print information as json.
        #[clap(long, required = false)]
        json: bool,
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
        /// Available columns: Name, Id, Type, Path, CriticRating, ProductionYear
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

    let mut cfg: AppConfig = if std::path::Path::new(current.as_path()).exists() {
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
        Commands::Initialize {
            username,
            password,
            server_url
        } => {
            println!("Configuring JellyRoller with supplied values.....");
            env::consts::OS.clone_into(&mut cfg.os);
            server_url.trim().clone_into(&mut cfg.server_url);
            cfg.api_key = UserAuth::auth_user(UserAuth::new(&cfg.server_url, username.trim(), password))
                .expect("Unable to generate user auth token.  Please assure your configuration information was input correctly\n");
            "configured".clone_into(&mut cfg.status);
            token_to_api(cfg);
        }
        
        //TODO: Create a simple_post variation that allows for query params.
        Commands::RegisterLibrary {
            name,
            collectiontype,
            filename,
        } => {
            let mut endpoint = String::from("/Library/VirtualFolders?CollectionType=");
            endpoint.push_str(collectiontype.to_string().as_str());
            endpoint.push_str("&refreshLibrary=true");
            endpoint.push_str("&name=");
            endpoint.push_str(name.as_str());
            let mut file = File::open(filename).expect("Unable to open file.");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Unable to read file.");
            register_library(
                ServerInfo::new(endpoint.as_str(), &cfg.server_url, &cfg.api_key),
                contents,
            )
        }

        Commands::GenerateReport {} => {
            let info = return_server_info(ServerInfo::new(
                "/System/Info",
                &cfg.server_url,
                &cfg.api_key,
            ));
            let json: serde_json::Value = serde_json::from_str(info.as_str()).expect("failed");
            println!(
                "\
                Please copy/paste the following information to any issue that is being opened:\n\
                JellyRoller Version: {}\n\
                JellyRoller OS: {}\n\
                Jellyfin Version: {}\n\
                Jellyfin Host OK: {}\n\
                Jellyfin Server Architecture: {}\
                ",
                env!("CARGO_PKG_VERSION"),
                env::consts::OS,
                json.get("Version")
                    .expect("Unable to extract Jellyfin version."),
                json.get("OperatingSystem")
                    .expect("Unable to extract Jellyfin OS information."),
                json.get("SystemArchitecture")
                    .expect("Unable to extract Jellyfin System Architecture.")
            );
        }

        Commands::UpdateMetadata { id, filename } => {
            // Read the JSON file and prepare it for upload.
            let json: String = fs::read_to_string(filename).unwrap();
            update_metadata(
                ServerInfo::new("/Items/{itemId}", &cfg.server_url, &cfg.api_key),
                id,
                json,
            );
        }
        Commands::UpdateImageByName {
            title,
            path,
            imagetype,
        } => {
            let search: MediaRoot =
                execute_search(&title, "all".to_string(), "".to_string(), false, &cfg);
            if search.total_record_count > 1 {
                eprintln!(
                    "Too many results found.  Updating by name requires a unique search term."
                );
                std::process::exit(1);
            }
            let img_base64 = image_to_base64(path);
            for item in search.items {
                update_image(
                    ServerInfo::new(
                        "/Items/{itemId}/Images/{imageType}",
                        &cfg.server_url,
                        &cfg.api_key,
                    ),
                    item.id,
                    &imagetype,
                    &img_base64,
                );
            }
        }
        Commands::UpdateImageById {
            id,
            path,
            imagetype,
        } => {
            let img_base64 = image_to_base64(path);
            update_image(
                ServerInfo::new(
                    "/Items/{itemId}/Images/{imageType}",
                    &cfg.server_url,
                    &cfg.api_key,
                ),
                id,
                &imagetype,
                &img_base64,
            );
        }

        // User based commands
        Commands::AddUser { username, password } => {
            add_user(&cfg, username, password);
        }
        Commands::DeleteUser { username } => {
            let user_id = get_user_id(&cfg, &username);
            let server_path = format!("{}/Users/{}", cfg.server_url, user_id);
            match UserWithPass::delete_user(UserWithPass::new(
                Some(username),
                None,
                None,
                server_path,
                cfg.api_key,
            )) {
                Err(_) => {
                    eprintln!("Unable to delete user.");
                    std::process::exit(1);
                }
                Ok(i) => i,
            }
        }
        Commands::ListUsers {
            export,
            mut output,
            username,
        } => {
            if username.is_empty() {
                let users: Vec<UserDetails> =
                    match UserList::list_users(UserList::new(USERS, &cfg.server_url, &cfg.api_key))
                    {
                        Err(_) => {
                            eprintln!("Unable to gather users.");
                            std::process::exit(1);
                        }
                        Ok(i) => i,
                    };
                if export {
                    println!("Exporting all user information.....");
                    if output.is_empty() {
                        "exported-user-info.json".clone_into(&mut output);
                    }
                    let data: String = match serde_json::to_string_pretty(&users) {
                        Err(_) => {
                            eprintln!("Unable to convert user information into JSON.");
                            std::process::exit(1);
                        }
                        Ok(i) => i,
                    };
                    export_data(&data, output);
                } else {
                    UserDetails::json_print_users(&users);
                }
            } else {
                let user_id = UserList::get_user_id(
                    UserList::new(USERS, &cfg.server_url, &cfg.api_key),
                    &username,
                );
                let user = gather_user_information(&cfg, &username, &user_id);
                if export {
                    println!("Exporting user information.....");
                    if output.is_empty() {
                        output = format!("exported-user-info-{}.json", username);
                    }
                    let data: String = match serde_json::to_string_pretty(&user) {
                        Err(_) => {
                            eprintln!("Unable to convert user information into JSON.");
                            std::process::exit(1);
                        }
                        Ok(i) => i,
                    };
                    export_data(&data, output);
                } else {
                    UserDetails::json_print_user(&user);
                }
            }
        }
        Commands::ResetPassword { username, password } => {
            // Get usename
            let user_id = UserList::get_user_id(
                UserList::new(USERS, &cfg.server_url, &cfg.api_key),
                &username,
            );
            // Setup the endpoint
            let server_path = format!("{}/Users/{}/Password", &cfg.server_url, user_id);
            match UserWithPass::resetpass(UserWithPass::new(
                None,
                Some(password),
                Some("".to_string()),
                server_path,
                cfg.api_key,
            )) {
                Err(_) => {
                    eprintln!("Unable to convert user information into JSON.");
                    std::process::exit(1);
                }
                Ok(i) => i,
            }
        }
        Commands::DisableUser { username } => {
            let id = get_user_id(&cfg, &username);
            let mut user_info = gather_user_information(&cfg, &username, &id);
            user_info.policy.is_disabled = true;
            UserList::update_user_config_bool(
                UserList::new(USER_POLICY, &cfg.server_url, &cfg.api_key),
                &user_info.policy,
                &id,
                &username,
            )
            .expect("Unable to update user.");
        }
        Commands::EnableUser { username } => {
            let id = get_user_id(&cfg, &username);
            let mut user_info = gather_user_information(&cfg, &username, &id);
            user_info.policy.is_disabled = false;
            UserList::update_user_config_bool(
                UserList::new(USER_POLICY, &cfg.server_url, &cfg.api_key),
                &user_info.policy,
                &id,
                &username,
            )
            .expect("Unable to update user.");
        }
        Commands::GrantAdmin { username } => {
            let id = get_user_id(&cfg, &username);
            let mut user_info = gather_user_information(&cfg, &username, &id);
            user_info.policy.is_administrator = true;
            UserList::update_user_config_bool(
                UserList::new(USER_POLICY, &cfg.server_url, &cfg.api_key),
                &user_info.policy,
                &id,
                &username,
            )
            .expect("Unable to update user.");
        }
        Commands::RevokeAdmin { username } => {
            let id = get_user_id(&cfg, &username);
            let mut user_info = gather_user_information(&cfg, &username, &id);
            user_info.policy.is_administrator = false;
            UserList::update_user_config_bool(
                UserList::new(USER_POLICY, &cfg.server_url, &cfg.api_key),
                &user_info.policy,
                &id,
                &username,
            )
            .expect("Unable to update user.");
        }
        Commands::AddUsers { inputfile } => {
            let reader = BufReader::new(File::open(inputfile).unwrap());
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        let vec: Vec<&str> = l.split(',').collect();
                        add_user(&cfg, vec[0].to_owned(), vec[1].to_owned());
                    }
                    Err(e) => println!("Unable to add user.  {e}"),
                }
            }
        }
        Commands::UpdateUsers { inputfile } => {
            let data: String = match fs::read_to_string(inputfile) {
                Err(_) => {
                    eprintln!("Unable to process input file.");
                    std::process::exit(1);
                }
                Ok(i) => i,
            };
            if data.starts_with('[') {
                let info: Vec<UserDetails> = match serde_json::from_str::<Vec<UserDetails>>(&data) {
                    Err(_) => {
                        eprintln!("Unable to convert user details JSON..");
                        std::process::exit(1);
                    }
                    Ok(i) => i,
                };
                for item in info {
                    match UserList::update_user_info(
                        UserList::new(USER_ID, &cfg.server_url, &cfg.api_key),
                        &item.id,
                        &item,
                    ) {
                        Ok(_) => {}
                        Err(e) => eprintln!("Unable to update user.  {e}"),
                    };
                }
            } else {
                let info: UserDetails = match serde_json::from_str::<UserDetails>(&data) {
                    Err(_) => {
                        eprintln!("Unable to convert user details JSON.");
                        std::process::exit(1);
                    }
                    Ok(i) => i,
                };
                let user_id = get_user_id(&cfg, &info.name);
                match UserList::update_user_info(
                    UserList::new(USER_ID, &cfg.server_url, &cfg.api_key),
                    &user_id,
                    &info,
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Unable to update user.  {e}")
                    }
                }
            }
        }

        // Server based commands
        Commands::GetPackages {
            json,
            output_format,
        } => {
            let packages =
                get_packages_info(ServerInfo::new("/Packages", &cfg.server_url, &cfg.api_key))
                    .unwrap();

            if json {
                json_deprecation();
                PackageDetails::json_print(&packages);
                std::process::exit(0)
            }

            match output_format {
                OutputFormat::Json => {
                    PackageDetails::json_print(&packages);
                }
                OutputFormat::Csv => {
                    PackageDetails::csv_print(packages);
                }
                _ => {
                    PackageDetails::table_print(packages);
                }
            }
        }

        Commands::GetRepositories {
            json,
            output_format,
        } => {
            let repos = get_repo_info(ServerInfo::new(
                "/Repositories",
                &cfg.server_url,
                &cfg.api_key,
            ))
            .unwrap();

            if json {
                json_deprecation();
                RepositoryDetails::json_print(&repos);
                std::process::exit(0)
            }

            match output_format {
                OutputFormat::Json => {
                    RepositoryDetails::json_print(&repos);
                }
                OutputFormat::Csv => {
                    RepositoryDetails::csv_print(repos);
                }
                _ => {
                    RepositoryDetails::table_print(repos);
                }
            }
        }

        Commands::RegisterRepository { name, path } => {
            let mut repos = get_repo_info(ServerInfo::new(
                "/Repositories",
                &cfg.server_url,
                &cfg.api_key,
            ))
            .unwrap();
            repos.push(RepositoryDetails::new(name, path, true));
            set_repo_info(
                ServerInfo::new("/Repositories", &cfg.server_url, &cfg.api_key),
                repos,
            );
        }

        Commands::InstallPackage {
            package,
            version,
            repository,
        } => {
            // Check if package name has spaces and replace them as needed
            let encoded = package.replace(" ", "%20");
            install_package(
                ServerInfo::new(
                    "/Packages/Installed/{package}",
                    &cfg.server_url,
                    &cfg.api_key,
                ),
                &encoded,
                &version,
                &repository,
            );
        }

        Commands::ServerInfo {} => {
            get_server_info(ServerInfo::new(
                "/System/Info",
                &cfg.server_url,
                &cfg.api_key,
            ))
            .expect("Unable to gather server information.");
        }
        Commands::ListLogs {
            json,
            output_format,
        } => {
            let logs = match get_log_filenames(ServerInfo::new(
                "/System/Logs",
                &cfg.server_url,
                &cfg.api_key,
            )) {
                Err(_) => {
                    eprintln!("Unable to get get log filenames.");
                    std::process::exit(1);
                }
                Ok(i) => i,
            };

            if json {
                json_deprecation();
                LogDetails::json_print(&logs);
                std::process::exit(0)
            }

            match output_format {
                OutputFormat::Json => {
                    LogDetails::json_print(&logs);
                }
                OutputFormat::Csv => {
                    LogDetails::csv_print(logs);
                }
                _ => {
                    LogDetails::table_print(logs);
                }
            }
        }
        Commands::ShowLog { logfile } => {
            LogFile::get_logfile(LogFile::new(
                ServerInfo::new("/System/Logs/Log", &cfg.server_url, &cfg.api_key),
                logfile,
            ))
            .expect("Unable to retrieve the specified logfile.");
        }
        Commands::Reconfigure {} => {
            initial_config(cfg);
        }
        Commands::GetDevices {
            active,
            json,
            output_format,
        } => {
            let devices: Vec<DeviceDetails> = match get_devices(
                ServerInfo::new(DEVICES, &cfg.server_url, &cfg.api_key),
                active,
            ) {
                Err(e) => {
                    eprintln!("Unable to get devices, {e}");
                    std::process::exit(1);
                }
                Ok(i) => i,
            };

            if json {
                json_deprecation();
                DeviceDetails::json_print(&devices);
                std::process::exit(0)
            }

            match output_format {
                OutputFormat::Json => {
                    DeviceDetails::json_print(&devices);
                }
                OutputFormat::Csv => {
                    DeviceDetails::csv_print(&devices);
                }
                _ => {
                    DeviceDetails::table_print(devices);
                }
            }
        }
        Commands::GetLibraries {
            json,
            output_format,
        } => {
            let libraries: Vec<LibraryDetails> = match get_libraries(ServerInfo::new(
                "/Library/VirtualFolders",
                &cfg.server_url,
                &cfg.api_key,
            )) {
                Err(_) => {
                    eprintln!("Unable to get libraries.");
                    std::process::exit(1);
                }
                Ok(i) => i,
            };

            if json {
                json_deprecation();
                LibraryDetails::json_print(&libraries);
                std::process::exit(0)
            }

            match output_format {
                OutputFormat::Json => {
                    LibraryDetails::json_print(&libraries);
                }
                OutputFormat::Csv => {
                    LibraryDetails::csv_print(libraries);
                }
                _ => {
                    LibraryDetails::table_print(libraries);
                }
            }
        }
        Commands::GetScheduledTasks {
            json,
            output_format,
        } => {
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

            if json {
                json_deprecation();
                TaskDetails::json_print(&tasks);
                std::process::exit(0);
            }

            match output_format {
                OutputFormat::Json => {
                    TaskDetails::json_print(&tasks);
                }
                OutputFormat::Csv => {
                    TaskDetails::csv_print(&tasks);
                }
                _ => {
                    TaskDetails::table_print(tasks);
                }
            }
        }
        Commands::ExecuteTaskByName { task } => {
            let taskid: String = match get_taskid_by_taskname(
                ServerInfo::new("/ScheduledTasks", &cfg.server_url, &cfg.api_key),
                &task,
            ) {
                Err(e) => {
                    eprintln!("Unable to get task id by taskname, {e}");
                    std::process::exit(1);
                }
                Ok(i) => i,
            };
            execute_task_by_id(
                ServerInfo::new(
                    "/ScheduledTasks/Running/{taskId}",
                    &cfg.server_url,
                    &cfg.api_key,
                ),
                &task,
                &taskid,
            );
        }
        Commands::ScanLibrary {
            library_id,
            scan_type,
        } => {
            if library_id == "all" {
                scan_library_all(ServerInfo::new(
                    "/Library/Refresh",
                    &cfg.server_url,
                    &cfg.api_key,
                ));
            } else {
                let query_info = match scan_type {
                    ScanType::NewUpdated => {
                        vec![
                            ("Recursive", "true"),
                            ("ImageRefreshMode", "Default"),
                            ("MetadataRefreshMode", "Default"),
                            ("ReplaceAllImages", "false"),
                            ("RegenerateTrickplay", "false"),
                            ("ReplaceAllMetadata", "false"),
                        ]
                    }
                    ScanType::MissingMetadata => {
                        vec![
                            ("Recursive", "true"),
                            ("ImageRefreshMode", "FullRefresh"),
                            ("MetadataRefreshMode", "FullRefresh"),
                            ("ReplaceAllImages", "false"),
                            ("RegenerateTrickplay", "false"),
                            ("ReplaceAllMetadata", "false"),
                        ]
                    }
                    ScanType::ReplaceMetadata => {
                        vec![
                            ("Recursive", "true"),
                            ("ImageRefreshMode", "FullRefresh"),
                            ("MetadataRefreshMode", "FullRefresh"),
                            ("ReplaceAllImages", "false"),
                            ("RegenerateTrickplay", "false"),
                            ("ReplaceAllMetadata", "true"),
                        ]
                    }
                    _ => std::process::exit(1),
                };
                scan_library(
                    ServerInfo::new("/Items/{library_id}/Refresh", &cfg.server_url, &cfg.api_key),
                    query_info,
                    library_id,
                );
            }
        }
        Commands::RemoveDeviceByUsername { username } => {
            let filtered: Vec<String> = match get_deviceid_by_username(
                ServerInfo::new(DEVICES, &cfg.server_url, &cfg.api_key),
                &username,
            ) {
                Err(_) => {
                    eprintln!("Unable to get device id by username.");
                    std::process::exit(1);
                }
                Ok(i) => i,
            };
            for item in filtered {
                remove_device(
                    ServerInfo::new(DEVICES, &cfg.server_url, &cfg.api_key),
                    &item,
                )
                .expect("Unable to delete specified id.");
            }
        }
        Commands::RestartJellyfin {} => {
            restart_or_shutdown(ServerInfo::new(
                "/System/Restart",
                &cfg.server_url,
                &cfg.api_key,
            ));
        }
        Commands::ShutdownJellyfin {} => {
            restart_or_shutdown(ServerInfo::new(
                "/System/Shutdown",
                &cfg.server_url,
                &cfg.api_key,
            ));
        }
        Commands::GetPlugins {
            json,
            output_format,
        } => {
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

            if json {
                json_deprecation();
                PluginDetails::json_print(&plugins);
                std::process::exit(0)
            }

            match output_format {
                OutputFormat::Json => {
                    PluginDetails::json_print(&plugins);
                }
                OutputFormat::Csv => {
                    PluginDetails::csv_print(plugins);
                }
                _ => {
                    PluginDetails::table_print(plugins);
                }
            }
        }
        Commands::CreateReport {
            report_type,
            limit,
            filename,
        } => match report_type {
            ReportType::Activity => {
                println!("Gathering Activity information.....");
                let activities: ActivityDetails = match get_activity(
                    ServerInfo::new("/System/ActivityLog/Entries", &cfg.server_url, &cfg.api_key),
                    &limit,
                ) {
                    Err(e) => {
                        eprintln!("Unable to gather activity log entries, {e}");
                        std::process::exit(1);
                    }
                    Ok(i) => i,
                };
                if !filename.is_empty() {
                    println!("Exporting Activity information to {}.....", &filename);
                    let csv = ActivityDetails::print_as_csv(activities);
                    export_data(&csv, filename);
                    println!("Export complete.");
                } else {
                    ActivityDetails::table_print(activities);
                }
            }
            ReportType::Movie => {
                let user_id: String = match UserList::get_current_user_information(UserList::new(
                    "/Users/Me",
                    &cfg.server_url,
                    &cfg.api_key,
                )) {
                    Err(e) => {
                        eprintln!("Unable to gather information about current user, {e}");
                        std::process::exit(1);
                    }
                    Ok(i) => i.id,
                };
                let movies: MovieDetails = match export_library(
                    ServerInfo::new("/Users/{userId}/Items", &cfg.server_url, &cfg.api_key),
                    &user_id,
                ) {
                    Err(e) => {
                        eprintln!("Unable to export library, {e}");
                        std::process::exit(1);
                    }
                    Ok(i) => i,
                };
                if !filename.is_empty() {
                    println!("Exporting Movie information to {}.....", &filename);
                    let csv = MovieDetails::print_as_csv(movies);
                    export_data(&csv, filename);
                    println!("Export complete.");
                } else {
                    MovieDetails::table_print(movies);
                }
            }
        },
        Commands::SearchMedia {
            term,
            mediatype,
            parentid,
            include_filepath,
            output_format,
            table_columns,
        } => {
            let search_result = execute_search(&term, mediatype, parentid, include_filepath, &cfg);

            let mut used_table_columns = table_columns.clone();

            if include_filepath {
                used_table_columns.push("Path".to_string());
            }

            match output_format {
                OutputFormat::Json => {
                    MediaRoot::json_print(search_result);
                }
                OutputFormat::Csv => {
                    MediaRoot::csv_print(search_result, &used_table_columns);
                }
                _ => {
                    MediaRoot::table_print(search_result, &used_table_columns);
                }
            }
        }
        Commands::Completions { shell } => {
            let cmd = &mut Cli::command();

            generate(shell, cmd, cmd.get_name().to_string(), &mut io::stdout());
        }
    }

    Ok(())
}

///
/// JSON flag deprecation message.
///
fn json_deprecation() {
    println!("|========= DEPRECATION WARNING ============|");
    println!("  The \"--json\" flag has been deprecated.");
    println!("  Please consider migrating to the");
    println!("  \"output_format\" argument");
    println!("|==========================================|");
    thread::sleep(time::Duration::from_millis(5000));
}

///
/// Executes a search with the passed parameters.
///
fn execute_search(
    term: &str,
    mediatype: String,
    parentid: String,
    include_filepath: bool,
    cfg: &AppConfig,
) -> MediaRoot {
    let mut query = vec![
        ("SortBy", "SortName,ProductionYear"),
        ("Recursive", "true"),
        ("searchTerm", term),
    ];
    if mediatype != "all" {
        query.push(("IncludeItemTypes", &mediatype));
    }

    if include_filepath {
        query.push(("fields", "Path"));
    }

    if !parentid.is_empty() {
        query.push(("parentId", &parentid));
    }

    match get_search_results(
        ServerInfo::new("/Items", &cfg.server_url, &cfg.api_key),
        query,
    ) {
        Err(e) => {
            eprintln!("Unable to execute search, {e}");
            std::process::exit(1);
        }
        Ok(i) => i,
    }
}

///
/// Retrieve the id for the specified user.  Most API calls require the id of the user rather than the username.
///
fn get_user_id(cfg: &AppConfig, username: &String) -> String {
    UserList::get_user_id(
        UserList::new("/Users", &cfg.server_url, &cfg.api_key),
        username,
    )
}

///
/// Gathers user information.
///
fn gather_user_information(cfg: &AppConfig, username: &String, id: &str) -> UserDetails {
    match UserList::get_user_information(UserList::new(USER_ID, &cfg.server_url, &cfg.api_key), id)
    {
        Err(_) => {
            println!("Unable to get user id for {}", username);
            std::process::exit(1);
        }
        Ok(ul) => ul,
    }
}

///
/// Helper function to standardize the call for adding a user with a password.
///
fn add_user(cfg: &AppConfig, username: String, password: String) {
    let server_path = format!("{}/Users/New", cfg.server_url);
    match UserWithPass::create_user(UserWithPass::new(
        Some(username),
        Some(password),
        None,
        server_path,
        cfg.api_key.clone(),
    )) {
        Err(_) => {
            println!("Unable to create user");
            std::process::exit(1);
        }
        Ok(i) => i,
    }
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
/// Due to an issue with api key processing in Jellyfin, JellyRoller was initially relied on using auto tokens to communicate.
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
/// Function that converts an image into a base64 png image.
///
fn image_to_base64(path: String) -> String {
    let base_img = image::open(path).unwrap();
    let mut image_data: Vec<u8> = Vec::new();
    base_img
        .write_to(&mut Cursor::new(&mut image_data), ImageFormat::Png)
        .unwrap();
    general_purpose::STANDARD.encode(image_data)
}

///
/// Custom implementation to convert the ImageType enum into Strings
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
