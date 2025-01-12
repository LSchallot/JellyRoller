use comfy_table::{ContentArrangement, Table};
pub type PackageDetailsRoot = Vec<PackageDetails>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageDetails {
    pub name: String,
    pub description: String,
    pub overview: String,
    pub owner: String,
    pub category: String,
    pub guid: String,
    pub versions: Vec<Version>,
    #[serde(default)]
    pub image_url: String,
}

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

impl PackageDetails {
    pub fn csv_print(packages: Vec<PackageDetails>) {
        for package in packages {
            let mut version_output: String = "".to_string();
            for version in package.versions {
                version_output.push_str(version.version.as_str());
                version_output.push(' ');
            }
            println!("{}, {}, {}, {}, {}, {}, {}", 
                package.name,
                package.description,
                package.overview,
                package.owner,
                package.guid,
                package.category,
                version_output,
            );
        }
    }
    
    pub fn json_print(packages: &[PackageDetails]) {
        println!("{}", serde_json::to_string_pretty(&packages).unwrap());
    }

    pub fn table_print(packages: Vec<PackageDetails>) {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(120)
            .set_header(vec![
                "Name",
                "Description",
                "Overview",
                "Owner",
                "GUID",
                "Category",
                "Versions",
            ]);
        for package in packages {
            let mut version_output: String = "".to_string();
            for version in package.versions {
                version_output.push_str(version.version.as_str());
                version_output.push(' ');
            }
            table.add_row(vec![
                package.name,
                package.description,
                package.overview,
                package.owner,
                package.guid,
                package.category,
                version_output,
            ]);
        }
        println!("{table}");
    }
}
