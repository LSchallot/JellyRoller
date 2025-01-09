use comfy_table::{ContentArrangement, Table};

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
    pub fn new(name: String, url: String, enabled: bool) -> RepositoryDetails {
        RepositoryDetails { name, url, enabled }
    }

    pub fn csv_print(repos: Vec<RepositoryDetails>) {
        for repo in repos {
            println!("{}, {}, {}",
                repo.name, 
                repo.url,
                repo.enabled.to_string(),
            )
        }
    }

    pub fn json_print(repos: &[RepositoryDetails]) {
        println!("{}", serde_json::to_string_pretty(&repos).unwrap());
    }

    pub fn table_print(repos: Vec<RepositoryDetails>) {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(120)
            .set_header(vec![
                "Plugin Name",
                "Version",
                "Config Filename",
            ]);
        for repo in repos {
            table.add_row(vec![repo.name, repo.url, repo.enabled.to_string()]);
        }
        println!("{table}");
    }
}
