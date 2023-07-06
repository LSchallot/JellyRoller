use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDetails {
    #[serde(rename = "Items")]
    pub items: Vec<TokenItem>,
    #[serde(rename = "TotalRecordCount")]
    pub total_record_count: i64,
    #[serde(rename = "StartIndex")]
    pub start_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenItem {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    #[serde(rename = "AppName")]
    pub app_name: String,
    #[serde(rename = "AppVersion")]
    pub app_version: String,
    #[serde(rename = "DeviceName")]
    pub device_name: String,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "IsActive")]
    pub is_active: bool,
    #[serde(rename = "DateCreated")]
    pub date_created: String,
    #[serde(rename = "DateRevoked", default)]
    pub date_revoked: String,
    #[serde(rename = "DateLastActivity", default)]
    pub date_last_activity: String,
    #[serde(rename = "UserName", default)]
    pub user_name: String,
}
