use serde_derive::Deserialize;
use serde_derive::Serialize;

use comfy_table::{ContentArrangement, Table};

pub type BackupRootJson = Vec<BackupDetails>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupDetails {
    #[serde(rename = "ServerVersion")]
    pub server_version: String,
    #[serde(rename = "BackupEngineVersion")]
    pub backup_engine_version: String,
    #[serde(rename = "DateCreated")]
    pub date_created: String,
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "Options")]
    pub options: Options,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    #[serde(rename = "Metadata")]
    pub metadata: bool,
    #[serde(rename = "Trickplay")]
    pub trickplay: bool,
    #[serde(rename = "Subtitles")]
    pub subtitles: bool,
    #[serde(rename = "Database")]
    pub database: bool,
}

impl BackupDetails {
    pub fn csv_print(backups: Vec<BackupDetails>) {
        for backup in backups {
            println!("{}, {}, {}, {}",
                backup.server_version,
                backup.backup_engine_version,
                backup.date_created,
                backup.path
        );
        }
    }

    pub fn json_print(backups: &[BackupDetails]) {
        println!("{}", serde_json::to_string_pretty(&backups).unwrap());
    }

    pub fn table_print(backups: Vec<BackupDetails>) {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(120)
            .set_header(vec![
                "Backup Server Version",
                "Backup Engine Version",
                "Date Created",
                "Path"
            ]);
        for backup in backups {
            table.add_row(vec![
                backup.server_version,
                backup.backup_engine_version,
                backup.date_created,
                backup.path
            ]);
        }
        println!("{table}");
    }
}

