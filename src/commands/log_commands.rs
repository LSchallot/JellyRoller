use std::env;

use crate::{ AppConfig, ReportType, ActivityDetails, UserList, MovieDetails, OutputFormat, utils::output_writer::export_data, system_actions::{get_activity, export_library, return_server_info, get_log_filenames}, entities::server_info::ServerInfo, entities::log_details::LogDetails};

pub fn command_generate_report(cfg: AppConfig) {
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

pub fn command_list_logs(cfg: AppConfig, output_format: OutputFormat) {
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

pub fn command_create_report(cfg: AppConfig, report_type: ReportType, limit: String, filename: String) {
    match report_type {
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
    }
}