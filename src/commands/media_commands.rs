use base64::{engine::general_purpose, Engine as _};
use image::ImageFormat;
use std::fs::{self,File};
use std::io::{Read, Cursor};
use crate::{AppConfig, ImageType, OutputFormat, ScanType, system_actions::{get_libraries, get_search_results, register_library, update_metadata, update_image, scan_library, scan_library_all}, CollectionType, entities::library_details::LibraryDetails, entities::server_info::ServerInfo, entities::media_details::MediaRoot};

pub fn command_register_libarary(cfg: AppConfig, name: String, collectiontype: CollectionType, filename: String) {
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

pub fn command_update_metadata(cfg: AppConfig, id: String, filename: String) {
    // Read the JSON file and prepare it for upload.
    let json: String = fs::read_to_string(filename).unwrap();
    update_metadata(
        ServerInfo::new("/Items/{itemId}", &cfg.server_url, &cfg.api_key),
        id,
        json,
    );
}

pub fn command_update_image_by_name(cfg: AppConfig, title: String, path: String, imagetype: ImageType) {
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

pub fn command_update_image_by_id(cfg: AppConfig, id: String, path: String, imagetype: ImageType) {
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

pub fn command_get_libraries(cfg: AppConfig, output_format: OutputFormat) {
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
        _ => {
            LibraryDetails::table_print(libraries);
        }
    }
}

pub fn command_scan_library(cfg: AppConfig, library_id: String, scan_type: ScanType) {
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

pub fn command_search_media(cfg:AppConfig, term: String, mediatype: String, parentid: String, output_format: OutputFormat, include_filepath: bool, table_columns: Vec<String>) {
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