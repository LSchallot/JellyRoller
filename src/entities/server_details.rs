use serde_derive::Deserialize;
use serde_derive::Serialize;
use super::version::Version;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerDetails {
    #[serde(rename = "LocalAddress")]
    pub local_address: String,
    #[serde(rename = "ServerName")]
    pub server_name: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "ProductName")]
    pub product_name: String,
    #[serde(rename = "OperatingSystem")]
    pub operating_system: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "StartupWizardCompleted")]
    pub startup_wizard_completed: bool,
    #[serde(rename = "OperatingSystemDisplayName")]
    pub operating_system_display_name: String,
    #[serde(rename = "PackageName")]
    pub package_name: String,
    #[serde(rename = "HasPendingRestart")]
    pub has_pending_restart: bool,
    #[serde(rename = "IsShuttingDown")]
    pub is_shutting_down: bool,
    #[serde(rename = "SupportsLibraryMonitor")]
    pub supports_library_monitor: bool,
    #[serde(rename = "WebSocketPortNumber")]
    pub web_socket_port_number: i64,
    #[serde(rename = "CompletedInstallations")]
    pub completed_installations: Vec<CompletedInstallation>,
    #[serde(rename = "CanSelfRestart")]
    pub can_self_restart: bool,
    #[serde(rename = "CanLaunchWebBrowser")]
    pub can_launch_web_browser: bool,
    #[serde(rename = "ProgramDataPath")]
    pub program_data_path: String,
    #[serde(rename = "WebPath")]
    pub web_path: String,
    #[serde(rename = "ItemsByNamePath")]
    pub items_by_name_path: String,
    #[serde(rename = "CachePath")]
    pub cache_path: String,
    #[serde(rename = "LogPath")]
    pub log_path: String,
    #[serde(rename = "InternalMetadataPath")]
    pub internal_metadata_path: String,
    #[serde(rename = "TranscodingTempPath")]
    pub transcoding_temp_path: String,
    #[serde(rename = "CastReceiverApplications")]
    pub cast_receiver_applications: Vec<CastReceiverApplication>,
    #[serde(rename = "HasUpdateAvailable")]
    pub has_update_available: bool,
    #[serde(rename = "EncoderLocation")]
    pub encoder_location: String,
    #[serde(rename = "SystemArchitecture")]
    pub system_architecture: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletedInstallation {
    #[serde(rename = "Guid")]
    pub guid: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Changelog")]
    pub changelog: String,
    #[serde(rename = "SourceUrl")]
    pub source_url: String,
    #[serde(rename = "Checksum")]
    pub checksum: String,
    #[serde(rename = "PackageInfo")]
    pub package_info: PackageInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageInfo {
    pub name: String,
    pub description: String,
    pub overview: String,
    pub owner: String,
    pub category: String,
    pub guid: String,
    pub versions: Vec<Version>,
    pub image_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CastReceiverApplication {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
}
