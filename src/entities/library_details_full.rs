use serde_derive::Deserialize;
use serde_derive::Serialize;
use super::library_options::LibraryOptions;

pub type LibraryDetailsFullVec = Vec<LibraryDetailsFull>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryDetailsFull {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Locations")]
    pub locations: Vec<String>,
    #[serde(rename = "CollectionType")]
    pub collection_type: String,
    #[serde(rename = "LibraryOptions")]
    pub library_options: LibraryOptions,
    #[serde(rename = "ItemId")]
    pub item_id: String,
    #[serde(rename = "PrimaryImageItemId")]
    pub primary_image_item_id: String,
    #[serde(rename = "RefreshProgress", default)]
    pub refresh_progress: f64,
    #[serde(rename = "RefreshStatus")]
    pub refresh_status: String,
}