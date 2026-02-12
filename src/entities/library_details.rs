use comfy_table::{ContentArrangement, Table};

pub type LibraryRootJson = Vec<LibraryDetails>;

#[derive(Serialize, Deserialize)]
pub struct LibraryDetails {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "CollectionType")]
    pub collection_type: String,
    #[serde(rename = "LibraryOptions")]
    pub library_options: LibraryOptions,
    #[serde(rename = "ItemId")]
    pub item_id: String,
    #[serde(rename = "RefreshStatus")]
    pub refresh_status: String,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct LibraryOptions {
    #[serde(rename = "Enabled")]
    pub enabled: bool
}

impl LibraryDetails {
    pub fn new(
        name: String,
        collection_type: String,
        library_options: LibraryOptions,
        item_id: String,
        refresh_status: String,
    ) -> LibraryDetails {
        LibraryDetails {
            name,
            collection_type,
            library_options,
            item_id,
            refresh_status,
        }
    }

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
