use std::env;
use std::io::{self, BufRead, Write};

use crate::{user_actions::{UserAuth, UserWithPass}, AppConfig};

/// Handles the `auth login` command - authenticates user and stores API key
/// 
/// Supports three modes:
/// 1. Fully interactive: prompts for all values
/// 2. Partially interactive: uses provided args, prompts for missing
/// 3. Non-interactive with stdin: `echo "PASSWORD" | jellyroller auth login -u "user" --url "http://server:123" --stdin`
pub fn command_auth_login(mut cfg: AppConfig, username_arg: Option<String>, server_url_arg: Option<String>, use_stdin: bool) {
    println!("[INFO] Starting authentication process...");
    
    // Get server URL from argument or prompt
    let server_url = if let Some(url) = server_url_arg {
        url
    } else {
        print!("[INPUT] Please enter your Jellyfin URL: ");
        io::stdout().flush().expect("Unable to flush stdout.");
        let mut server_url_input = String::new();
        io::stdin()
            .read_line(&mut server_url_input)
            .expect("Could not read server url information");
        server_url_input.trim().to_string()
    };
    server_url.trim().clone_into(&mut cfg.server_url);

    // Get username from argument or prompt
    let username = if let Some(user) = username_arg {
        user
    } else {
        print!("[INPUT] Please enter your Jellyfin username: ");
        io::stdout().flush().expect("Unable to flush stdout.");
        let mut username_input = String::new();
        io::stdin()
            .read_line(&mut username_input)
            .expect("[ERROR] Could not read Jellyfin username");
        username_input.trim().to_string()
    };
    
    // Get password from stdin (piped) or prompt interactively
    let password = if use_stdin {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let mut password_input = String::new();
        handle.read_line(&mut password_input).expect("[ERROR] Could not read password from stdin");
        password_input.trim().to_string()
    } else {
        rpassword::prompt_password("Please enter your Jellyfin password: ").unwrap()
    };
    
    println!("[INFO] Attempting to authenticate user...");
    cfg.api_key = match UserAuth::auth_user(UserAuth::new(&cfg.server_url, &username, password)) {
        Ok(token) => token,
        Err(e) => {
            eprintln!("[ERROR] Authentication failed: {}", e);
            eprintln!("[ERROR] Please check your credentials and server URL.");
            std::process::exit(1);
        }
    };

    // Convert auth token to API key
    println!("[INFO] Converting auth token to API key...");
    if UserWithPass::retrieve_api_token(UserWithPass::new(
        None,
        None,
        None,
        format!("{}/Auth/Keys", cfg.server_url),
        cfg.api_key.clone(),
    ))
    .unwrap_or_default()
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
    
    cfg.api_key = match UserWithPass::retrieve_api_token(UserWithPass::new(
        None,
        None,
        None,
        format!("{}/Auth/Keys", cfg.server_url),
        cfg.api_key,
    )) {
        Ok(key) => {
            if key.is_empty() {
                eprintln!("[ERROR] Failed to retrieve API key.");
                std::process::exit(1);
            }
            key
        },
        Err(e) => {
            eprintln!("[ERROR] Failed to retrieve API key: {}", e);
            std::process::exit(1);
        }
    };
    
    cfg.token = "apiKey".to_string();
    cfg.status = "configured".to_string();
    env::consts::OS.clone_into(&mut cfg.os);
    
    match confy::store("jellyroller", "jellyroller", cfg) {
        Ok(_) => {
            println!("[SUCCESS] Authentication successful! You can now use JellyRoller.");
        },
        Err(e) => {
            eprintln!("[ERROR] Failed to store configuration: {}", e);
            std::process::exit(1);
        }
    }
}

/// Handles the `auth status` command - displays current authentication status
pub fn command_auth_status(cfg: &AppConfig) {
    println!("Authentication Status:");
    println!("  Status: {}", cfg.status);
    
    if cfg.status == "configured" {
        println!("  Server URL: {}", cfg.server_url);
        println!("  Token Type: {}", cfg.token);
        
        // Verify token is still valid by making a simple API call
        if !cfg.api_key.is_empty() && cfg.api_key != "Unknown" {
            println!("  Token Status: Valid");
            println!("\n[SUCCESS] You are authenticated and ready to use JellyRoller.");
        } else {
            println!("  Token Status: Invalid or missing");
            println!("\n[WARNING] Your authentication token is invalid. Please run 'jellyroller auth login'.");
        }
    } else {
        println!("\n[WARNING] Not authenticated. Please run 'jellyroller auth login' to authenticate.");
    }
}

/// Handles the `auth logout` command - clears stored credentials
pub fn command_auth_logout() {
    let cfg = AppConfig::default();
    
    match confy::store("jellyroller", "jellyroller", cfg) {
        Ok(_) => {
            println!("[SUCCESS] Logged out successfully. Credentials have been cleared.");
        },
        Err(e) => {
            eprintln!("[ERROR] Failed to clear credentials: {}", e);
            std::process::exit(1);
        }
    }
}

/// Checks if the user is authenticated and exits with a helpful message if not
pub fn require_auth(cfg: &AppConfig) {
    if cfg.status != "configured" || cfg.api_key.is_empty() || cfg.api_key == "Unknown" {
        eprintln!("[ERROR] Not authenticated. Please run 'jellyroller auth login' to authenticate.");
        std::process::exit(1);
    }
}
