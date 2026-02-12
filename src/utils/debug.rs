use std::sync::atomic::{AtomicBool, Ordering};

/// Global flag for verbose/debug mode
static VERBOSE: AtomicBool = AtomicBool::new(false);

/// Enable verbose mode globally
pub fn set_verbose(enabled: bool) {
    VERBOSE.store(enabled, Ordering::SeqCst);
}

/// Check if verbose mode is enabled
pub fn is_verbose() -> bool {
    VERBOSE.load(Ordering::SeqCst)
}

/// Print a debug message if verbose mode is enabled
#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        if $crate::utils::debug::is_verbose() {
            eprintln!("[DEBUG] {}", format!($($arg)*));
        }
    };
}

/// Print request details if verbose mode is enabled
pub fn log_request(method: &str, url: &str, body: Option<&str>) {
    if is_verbose() {
        eprintln!("[DEBUG] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        eprintln!("[DEBUG] HTTP Request: {} {}", method, url);
        if let Some(b) = body {
            if !b.is_empty() {
                eprintln!("[DEBUG] Request Body:");
                // Pretty print if it's JSON, otherwise print as-is
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(b) {
                    eprintln!("{}", serde_json::to_string_pretty(&json).unwrap_or_else(|_| b.to_string()));
                } else {
                    eprintln!("{}", b);
                }
            }
        }
    }
}

/// Print response details if verbose mode is enabled
pub fn log_response(status: u16, status_text: &str, body: Option<&str>) {
    if is_verbose() {
        eprintln!("[DEBUG] Response Status: {} {}", status, status_text);
        if let Some(b) = body {
            if !b.is_empty() {
                eprintln!("[DEBUG] Response Body:");
                // Pretty print if it's JSON, otherwise print as-is (truncate if very long)
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(b) {
                    let pretty = serde_json::to_string_pretty(&json).unwrap_or_else(|_| b.to_string());
                    if pretty.len() > 2000 {
                        eprintln!("{}...\n[truncated, {} bytes total]", &pretty[..2000], pretty.len());
                    } else {
                        eprintln!("{}", pretty);
                    }
                } else if b.len() > 2000 {
                    eprintln!("{}...\n[truncated, {} bytes total]", &b[..2000], b.len());
                } else {
                    eprintln!("{}", b);
                }
            }
        }
        eprintln!("[DEBUG] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
}

