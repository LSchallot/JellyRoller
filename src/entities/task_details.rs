use comfy_table::{ Table, ContentArrangement };

#[derive(Serialize, Deserialize)]
pub struct TaskDetails {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "CurrentProgressPercentage")]
    pub percent_complete: Option<String>,
    #[serde(rename = "Id")]
    pub id: String
}

impl TaskDetails {
    pub fn new(name: String, state: String, percent_complete: Option<String>, id: String) -> TaskDetails {
        TaskDetails{
            name,
            state,
            percent_complete,
            id
        }
    }

    pub fn json_print(tasks: &[TaskDetails]) {
        println!("{}", serde_json::to_string_pretty(&tasks).unwrap());
    }

    pub fn table_print(tasks: Vec<TaskDetails>) {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(120)
            .set_header(vec!["Task Name", "State", "% Complete", "Id"]);
        for task in tasks {
            table.add_row(vec![task.name, task.state, task.percent_complete.unwrap_or_else(|| "".to_owned()), task.id]);
        }
        println!("{table}");
    }
}