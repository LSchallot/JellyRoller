use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type RepositoryDetailsRoot = Vec<RepositoryDetails>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryDetails {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Url")]
    pub url: String,
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

impl RepositoryDetails {
    pub fn new (name: String, url: String, enabled: bool) -> RepositoryDetails {
        RepositoryDetails {
            name,
            url,
            enabled
        }
    }
}