use comfy_table::{ Table, ContentArrangement };

#[derive(Serialize, Deserialize)]
pub struct TaskDetails {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "CurrentProgressPercentage")]
    pub percent_complete: String
}

impl TaskDetails {
    pub fn new(name: String, state: String, percent_complete: String) -> TaskDetails {
        TaskDetails{
            name,
            state,
            percent_complete
        }
    }

    pub fn json_print(tasks: Vec<TaskDetails>) {
        println!("{}", serde_json::to_string_pretty(&tasks).unwrap());
    }

    pub fn table_print(tasks: Vec<TaskDetails>) {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Username", "Admin", "Disabled"]);
        for task in tasks {
            table.add_row(vec![task.name, task.state, task.percent_complete]);
        }
        println!("{table}");
    }
}