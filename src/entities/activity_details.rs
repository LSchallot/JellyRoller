use serde_derive::Deserialize;
use serde_derive::Serialize;

use comfy_table::{ContentArrangement, Table};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct ActivityDetails {
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "TotalRecordCount")]
    pub total_record_count: i64,
    #[serde(rename = "StartIndex")]
    pub start_index: i64,
}

#[allow(clippy::struct_field_names)]
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Overview", default)]
    pub overview: String,
    #[serde(rename = "ShortOverview", default)]
    pub short_overview: String,
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "ItemId", default)]
    pub item_id: String,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "UserPrimaryImageTag", default)]
    pub user_primary_image_tag: String,
    #[serde(rename = "Severity")]
    pub severity: String,
}

impl ActivityDetails {
    pub fn table_print(activities: ActivityDetails) {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                "Date",
                "User",
                "Type",
                "Severity",
                "Name",
                "ShortOverview",
                "Overview",
            ]);
        for activity in activities.items {
            table.add_row(vec![
                &activity.date,
                &activity.id.to_string(),
                &activity.type_field,
                &activity.severity,
                &activity.name,
                &activity.short_overview,
                &activity.overview,
            ]);
        }
        println!("{table}");
    }

    pub fn print_as_csv(activities: ActivityDetails) -> String {
        // first print the headers
        let mut data: String = "Date,User,Type,Severity,Name,ShortOverview,Overview\n".to_owned();
        for activity in activities.items {
            let piece = format!(
                "{},{},{},{},{},{},{}\n",
                &activity.date,
                &activity.id.to_string(),
                &activity.type_field,
                &activity.severity,
                &activity.name,
                &activity.short_overview,
                &activity.overview
            );
            data.push_str(&piece);
        }
        data
    }
}
