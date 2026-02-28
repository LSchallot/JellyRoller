#[allow(clippy::struct_field_names)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub version: String,
    #[serde(rename = "VersionNumber")]
    pub version_number: String,
    pub changelog: String,
    pub target_abi: String,
    pub source_url: String,
    pub checksum: String,
    pub timestamp: String,
    pub repository_name: String,
    pub repository_url: String,
}