use comfy_table::{ContentArrangement, Table};

#[derive(Serialize, Deserialize)]
pub struct TaskDetails {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "CurrentProgressPercentage", default)]
    //pub percent_complete: Option<String>,
    pub percent_complete: f32,
    #[serde(rename = "Id")]
    pub id: String,
}

impl TaskDetails {
    pub fn new(name: String, state: String, percent_complete: f32, id: String) -> TaskDetails {
        TaskDetails {
            name,
            state,
            percent_complete,
            id,
        }
    }

    pub fn json_print(tasks: &[TaskDetails]) {
        println!("{}", serde_json::to_string_pretty(&tasks).unwrap());
    }

    pub fn csv_print(tasks: &[TaskDetails]) {
        for task in tasks {
            let mut per_comp: String = "".to_string();
            if task.percent_complete > 0.0 {
                per_comp = task.percent_complete.to_string();
            }
            println!("{}, {}, {}, {}", task.name, task.state, per_comp, task.id);
        }
    }
    pub fn table_print(tasks: Vec<TaskDetails>) {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(120)
            .set_header(vec!["Task Name", "State", "% Complete", "Id"]);
        for task in tasks {
            let mut per_comp: String = "".to_string();
            if task.percent_complete > 0.0 {
                per_comp = task.percent_complete.to_string();
            }
            // table.add_row(vec![task.name, task.state, task.percent_complete.unwrap_or_else(|| "".to_owned()), task.id]);
            table.add_row(vec![task.name, task.state, per_comp, task.id]);
        }
        println!("{table}");
    }
}
