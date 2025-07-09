use std::io::{BufRead, BufReader};
use std::fs::{self, File};
use crate::{AppConfig, 
    utils::output_writer::export_data, 
    system_actions::{remove_device, get_deviceid_by_username}, 
    user_actions::{UserList, UserWithPass}, 
    entities::{user_details::UserDetails, server_info::ServerInfo}};

#[derive(Clone, Debug, PartialEq)]
enum UserMods {
    Admin,
    Active
}

pub fn command_delete_user(cfg: AppConfig, username: String) {
    let user_id = get_user_id(&cfg, &username);
    let server_path = format!("{}/Users/{user_id}", cfg.server_url);
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

pub fn command_list_users(cfg: &AppConfig, export: bool, mut output: String, username: &str, users_endpoint: &str, user_id_endpoint: &str) {
    if username.is_empty() {
        let users: Vec<UserDetails> =
            match UserList::list_users(UserList::new(users_endpoint, &cfg.server_url, &cfg.api_key))
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
            UserList::new(users_endpoint, &cfg.server_url, &cfg.api_key),
            username,
        );
        let user = gather_user_information(cfg, username, &user_id, user_id_endpoint);
        if export {
            println!("Exporting user information.....");
            if output.is_empty() {
                output = format!("exported-user-info-{username}.json");
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

pub fn command_reset_password(cfg: AppConfig, username: &str, password: String, users_endpoint: &str) {
    // Get usename
    let user_id = UserList::get_user_id(
        UserList::new(users_endpoint, &cfg.server_url, &cfg.api_key),
        username,
    );
    // Setup the endpoint
    let server_path = format!("{}/Users/{user_id}/Password", &cfg.server_url);
    match UserWithPass::resetpass(UserWithPass::new(
        None,
        Some(password),
        Some(String::new()),
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

pub fn command_disable_user(cfg: &AppConfig, username: &str, user_policy_endpoint: &str, user_id_endpoint: &str) {
    modify_user(cfg, username, user_policy_endpoint, user_id_endpoint, &UserMods::Active, false);
}

pub fn command_enable_user(cfg: &AppConfig, username: &str, user_policy_endpoint: &str, user_id_endpoint: &str) {
    modify_user(cfg, username, user_policy_endpoint, user_id_endpoint, &UserMods::Active, true);
}

pub fn command_grant_admin(cfg: &AppConfig, username: &str, user_policy_endpoint: &str, user_id_endpoint: &str) {
    modify_user(cfg, username, user_policy_endpoint, user_id_endpoint, &UserMods::Admin, true);
}

pub fn command_revoke_admin(cfg: &AppConfig, username: &str, user_policy_endpoint: &str, user_id_endpoint: &str) {
    modify_user(cfg, username, user_policy_endpoint, user_id_endpoint, &UserMods::Admin, false);
}

pub fn command_add_user(cfg: &AppConfig, username: String, password: String) {
    add_user(cfg, username, password);
}

pub fn command_add_users(cfg: &AppConfig, inputfile: String) {
    let reader = BufReader::new(File::open(inputfile).unwrap());
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let vec: Vec<&str> = l.split(',').collect();
                add_user(cfg, vec[0].to_owned(), vec[1].to_owned());
            }
            Err(e) => println!("Unable to add user.  {e}"),
        }
    }
}

pub fn command_update_users(cfg: &AppConfig, inputfile: String, passed_user_id: &str) {
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
            if let Err(e) = UserList::update_user_info(
                UserList::new(passed_user_id, &cfg.server_url, &cfg.api_key),
                &item.id,
                &item
            ) {
                eprintln!("Unable to update user.  {e}");
            }
        }
    } else {
        let info: UserDetails = match serde_json::from_str::<UserDetails>(&data) {
            Err(_) => {
                eprintln!("Unable to convert user details JSON.");
                std::process::exit(1);
            }
            Ok(i) => i,
        };
        let user_id = get_user_id(cfg, &info.name);
        
        if let Err(e) = UserList::update_user_info(
            UserList::new(passed_user_id, &cfg.server_url, &cfg.api_key),
            &user_id,
            &info,
        ) {
            eprintln!("Unable to update user.  {e}");
        }
    }
}

pub fn command_remove_device_by_username(cfg: &AppConfig, username: &str, devices_endpoint: &str) {
    let filtered: Vec<String> = match get_deviceid_by_username(
        ServerInfo::new(devices_endpoint, &cfg.server_url, &cfg.api_key),
        username,
    ) {
        Err(_) => {
            eprintln!("Unable to get device id by username.");
            std::process::exit(1);
        }
        Ok(i) => i,
    };
    for item in filtered {
        remove_device(
            ServerInfo::new(devices_endpoint, &cfg.server_url, &cfg.api_key),
            &item,
        )
        .expect("Unable to delete specified id.");
    }
}

/* 
    The following section contains additional
    functions that are used to support the media_commands
    base functions.
*/

///
/// Retrieve the id for the specified user.  Most API calls require the id of the user rather than the username.
///
fn get_user_id(cfg: &AppConfig, username: &str) -> String {
    UserList::get_user_id(
        UserList::new("/Users", &cfg.server_url, &cfg.api_key),
        username,
    )
}

///
/// Gathers user information.
///
fn gather_user_information(cfg: &AppConfig, username: &str, id: &str, user_id: &str) -> UserDetails {
    match UserList::get_user_information(UserList::new(user_id, &cfg.server_url, &cfg.api_key), id)
    {
        Err(_) => {
            println!("Unable to get user id for {username}");
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
/// Function to modify user information
/// 
fn modify_user(cfg: &AppConfig, username: &str, user_policy_endpoint: &str, user_id_endpoint: &str, mod_type: &UserMods, mod_flag: bool) {
    let id = get_user_id(cfg, username);
    let mut user_info = gather_user_information(cfg, username, &id, user_id_endpoint);
    match mod_type {
        UserMods::Admin => user_info.policy.is_administrator = mod_flag,
        UserMods::Active => user_info.policy.is_disabled = mod_flag
    }
    UserList::update_user_config_bool(
        UserList::new(user_policy_endpoint, &cfg.server_url, &cfg.api_key),
        &user_info.policy,
        &id,
        username,
    ).expect("Unable to update user.");
}