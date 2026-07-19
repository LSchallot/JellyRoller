use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickConnectDetails {
    #[serde(rename = "Authenticated")]
    pub authenticated: bool,
    #[serde(rename = "Secret")]
    pub secret: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    #[serde(rename = "DeviceName")]
    pub device_name: String,
    #[serde(rename = "AppName")]
    pub app_name: String,
    #[serde(rename = "AppVersion")]
    pub app_version: String,
    #[serde(rename = "DateAdded")]
    pub date_added: String,
}