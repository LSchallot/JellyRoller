use comfy_table::{ContentArrangement, Table};

#[derive(Serialize, Deserialize)]
pub struct LogDetails {
    #[serde(rename = "DateCreated")]
    pub date_created: String,
    #[serde(rename = "DateModified")]
    pub date_modified: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Size")]
    pub size: i32,
}

impl LogDetails {
    pub fn new(date_created: String, date_modified: String, name: String, size: i32) -> LogDetails {
        LogDetails {
            date_created,
            date_modified,
            name,
            size,
        }
    }

    pub fn csv_print(logs: Vec<LogDetails>) {
        for log in logs {
            println!("{}, {}, {}, {}",
                log.name, 
                log.size,
                log.date_created,
                log.date_modified,
            );
        }
    }

    pub fn json_print(logs: &[LogDetails]) {
        println!("{}", serde_json::to_string_pretty(&logs).unwrap());
    }

    pub fn table_print(logs: Vec<LogDetails>) {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(120)
            .set_header(vec!["Log Name", "Size", "Date Created", "Last Modified"]);
        for log in logs {
            table.add_row(vec![
                log.name,
                log.size.to_string(),
                log.date_created,
                log.date_modified,
            ]);
        }
        println!("{table}");
    }
}
