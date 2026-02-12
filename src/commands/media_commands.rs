use std::fs::{self,File};
use std::io::Read;
use crate::system_actions::{get_libraries_full, update_library};
use crate::{AppConfig, ImageType, OutputFormat, ScanType, system_actions::{get_libraries, get_search_results, register_library, update_metadata, update_image, scan_library, scan_library_all}, CollectionType, entities::library_options::LibraryOptionsRoot, entities::library_details::LibraryDetails, entities::server_info::ServerInfo, entities::media_details::MediaRoot, utils::common::image_to_base64,};

pub fn command_register_libarary(cfg: &AppConfig, name: &str, collectiontype: &CollectionType, filename: String) {
    let mut endpoint = String::from("/Library/VirtualFolders?CollectionType=");
    endpoint.push_str(collectiontype.to_string().as_str());
    endpoint.push_str("&refreshLibrary=true");
    endpoint.push_str("&name=");
    endpoint.push_str(name);
    let mut file = File::open(filename).expect("Unable to open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file.");
    register_library(
        ServerInfo::new(endpoint.as_str(), &cfg.server_url, &cfg.api_key),
        contents,
    );
}

pub fn command_update_metadata(cfg: &AppConfig, id: &str, filename: String) {
    // Read the JSON file and prepare it for upload.
    let json: String = fs::read_to_string(filename).unwrap();
    update_metadata(
        &ServerInfo::new("/Items/{itemId}", &cfg.server_url, &cfg.api_key),
        id,
        json,
    );
}

pub fn command_update_image_by_name(cfg: &AppConfig, title: &str, path: String, imagetype: &ImageType) {
    let search: MediaRoot =
        execute_search(title, "all", "", false, cfg);
    if search.total_record_count > 1 {
        eprintln!(
            "Too many results found.  Updating by name requires a unique search term."
        );
        std::process::exit(1);
    }
    let img_base64 = image_to_base64(path);
    for item in search.items {
        update_image(
            &ServerInfo::new(
                "/Items/{itemId}/Images/{imageType}",
                &cfg.server_url,
                &cfg.api_key,
            ),
            &item.id,
            imagetype,
            &img_base64,
        );
    }
}

pub fn command_update_image_by_id(cfg: &AppConfig, id: &str, path: String, imagetype: &ImageType) {
    let img_base64 = image_to_base64(path);
    update_image(
        &ServerInfo::new(
            "/Items/{itemId}/Images/{imageType}",
            &cfg.server_url,
            &cfg.api_key,
        ),
        id,
        imagetype,
        &img_base64,
    );
}

pub fn command_get_libraries(cfg: &AppConfig, output_format: &OutputFormat) {
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

    match output_format {
        OutputFormat::Json => {
            LibraryDetails::json_print(&libraries);
        }
        OutputFormat::Csv => {
            LibraryDetails::csv_print(libraries);
        }
        OutputFormat::Table => {
            LibraryDetails::table_print(libraries);
        }
    }
}

pub fn command_scan_library(cfg: &AppConfig, library_id: &str, scan_type: &ScanType) {
    if library_id == "all" {
        scan_library_all(ServerInfo::new(
            "/Library/Refresh",
            &cfg.server_url,
            &cfg.api_key,
        ));
    } else {
        let query_info  = match scan_type {
            ScanType::NewUpdated => {
                [
                    ("Recursive", "true"),
                    ("ImageRefreshMode", "Default"),
                    ("MetadataRefreshMode", "Default"),
                    ("ReplaceAllImages", "false"),
                    ("RegenerateTrickplay", "false"),
                    ("ReplaceAllMetadata", "false"),
                ]
            }
            ScanType::MissingMetadata => {
                [
                    ("Recursive", "true"),
                    ("ImageRefreshMode", "FullRefresh"),
                    ("MetadataRefreshMode", "FullRefresh"),
                    ("ReplaceAllImages", "false"),
                    ("RegenerateTrickplay", "false"),
                    ("ReplaceAllMetadata", "false"),
                ]
            }
            ScanType::ReplaceMetadata => {
                [
                    ("Recursive", "true"),
                    ("ImageRefreshMode", "FullRefresh"),
                    ("MetadataRefreshMode", "FullRefresh"),
                    ("ReplaceAllImages", "false"),
                    ("RegenerateTrickplay", "false"),
                    ("ReplaceAllMetadata", "true"),
                ]
            }
            ScanType::All => std::process::exit(1), // Handled elsewhere.
        };
        scan_library(
            &ServerInfo::new("/Items/{library_id}/Refresh", &cfg.server_url, &cfg.api_key),
            &query_info,
            library_id,
        );
    }
}

pub fn command_search_media(cfg: &AppConfig, term: &str, mediatype: &str, parentid: &str, output_format: &OutputFormat, include_filepath: bool, table_columns: &[String]) {
    let search_result = execute_search(term, mediatype, parentid, include_filepath, cfg);

    let mut used_table_columns = table_columns.to_owned();

    if include_filepath {
        used_table_columns.push("Path".to_string());
    }

    match output_format {
        OutputFormat::Json => {
            MediaRoot::json_print(&search_result);
        }
        OutputFormat::Csv => {
            MediaRoot::csv_print(search_result, &used_table_columns);
        }
        OutputFormat::Table => {
            MediaRoot::table_print(search_result, &used_table_columns);
        }
    }
}

pub fn command_library_enable_disable(cfg: &AppConfig, library: String, status: bool) {
    let response = get_libraries_full(ServerInfo::new(
            "/Library/VirtualFolders",
            &cfg.server_url,
            &cfg.api_key,
        ));
    for item in response.unwrap() {
        if library.to_uppercase() == item.name.to_uppercase() {
            let mut update: LibraryOptionsRoot = LibraryOptionsRoot { id: item.item_id, library_options: item.library_options };
            update.library_options.enabled = status;
            update_library(ServerInfo::new(
                "/Library/VirtualFolders/LibraryOptions",
                &cfg.server_url,
                &cfg.api_key,
            ), update);
        }
    }
}

/* 
    The following section contains additional
    functions that are used to support the media_commands
    base functions.
*/

///
/// Executes a search with the passed parameters.
///
fn execute_search(
    term: &str,
    mediatype: &str,
    parentid: &str,
    include_filepath: bool,
    cfg: &AppConfig,
) -> MediaRoot {
    let mut query = vec![
        ("SortBy", "SortName,ProductionYear"),
        ("Recursive", "true"),
        ("searchTerm", term),
    ];
    if mediatype != "all" {
        query.push(("IncludeItemTypes", mediatype));
    }

    if include_filepath {
        query.push(("fields", "Path"));
    }

    if !parentid.is_empty() {
        query.push(("parentId", parentid));
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