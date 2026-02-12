use comfy_table::{ContentArrangement, Table};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use super::library_options::LibraryOptions;

pub type LibraryDetailsVec = Vec<LibraryDetails>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryDetails {
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

impl LibraryDetails {
    pub fn csv_print(libraries: Vec<LibraryDetails>) {
        for library in libraries {
            println!("{}, {}, {}, {}", library.name, library.collection_type, library.item_id, library.refresh_status);
        }
    }

    pub fn json_print(libraries: &[LibraryDetails]) {
        println!("{}", serde_json::to_string_pretty(&libraries).unwrap());
    }

    pub fn table_print(libraries: Vec<LibraryDetails>) {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(120)
            .set_header(vec![
                "Library Name",
                "Collection Type",
                "Library Id",
                "Refresh Status",
                "Enabled",
            ]);
        for library in libraries {
            table.add_row(vec![
                library.name,
                library.collection_type,
                library.item_id,
                library.refresh_status,
                library.library_options.enabled.to_string(),
            ]);
        }
        println!("{table}");
    }
}