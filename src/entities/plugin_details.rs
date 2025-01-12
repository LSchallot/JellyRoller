use comfy_table::{ContentArrangement, Table};

pub type PluginRootJson = Vec<PluginDetails>;

#[derive(Serialize, Deserialize)]
pub struct PluginDetails {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "ConfigurationFileName")]
    pub configuration_file_name: Option<String>,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "CanUninstall")]
    pub can_uninstall: bool,
    #[serde(rename = "HasImage")]
    pub has_image: bool,
    #[serde(rename = "Status")]
    pub status: String,
}

impl PluginDetails {
    pub fn csv_print(plugins: Vec<PluginDetails>) {
        for plugin in plugins {
            println!("{}, {}, {}, {}, {}, {}, {}, {}",
                plugin.name,
                plugin.version,
                plugin
                    .configuration_file_name
                    .unwrap_or_else(|| String::new()),
                plugin.description,
                plugin.id,
                plugin.can_uninstall,
                plugin.has_image,
                plugin.status,
            )
        }
    }
    
    pub fn json_print(plugins: &[PluginDetails]) {
        println!("{}", serde_json::to_string_pretty(&plugins).unwrap());
    }

    pub fn table_print(plugins: Vec<PluginDetails>) {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(120)
            .set_header(vec![
                "Plugin Name",
                "Version",
                "Config Filename",
                "Description",
                "Id",
                "Can Uninstall",
                "Image",
                "Status",
            ]);
        for plugin in plugins {
            table.add_row(vec![
                plugin.name,
                plugin.version,
                plugin
                    .configuration_file_name
                    .unwrap_or_else(|| String::new()),
                plugin.description,
                plugin.id,
                plugin.can_uninstall.to_string(),
                plugin.has_image.to_string(),
                plugin.status,
            ]);
        }
        println!("{table}");
    }
}
