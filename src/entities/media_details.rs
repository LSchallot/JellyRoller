use serde_derive::Deserialize;
use serde_derive::Serialize;
use comfy_table::{ Table, ContentArrangement };

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaDetails {
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "TotalRecordCount")]
    pub total_record_count: i64,
    #[serde(rename = "StartIndex")]
    pub start_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "DateCreated")]
    pub date_created: String,
    #[serde(rename = "HasSubtitles", default)]
    pub has_subtitles: bool,
    #[serde(rename = "PremiereDate", default)]
    pub premiere_date: String,
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "OfficialRating", default)]
    pub official_rating: String,
    #[serde(rename = "Genres", default)]
    pub genres: Vec<String>,
    #[serde(rename = "CommunityRating", default)]
    pub community_rating: f32,
    #[serde(rename = "RunTimeTicks", default)]
    pub run_time_ticks: i64,
    #[serde(rename = "ProductionYear", default)]
    pub production_year: i64,
    #[serde(rename = "Width", default)]
    pub width: i64,
    #[serde(rename = "Height", default)]
    pub height: i64,
}

impl MediaDetails {
    pub fn table_print(details: MediaDetails) {
        let mut table = Table::new();
        table  
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Name", "Date Added", "Premiere Date", "Release Year", "Genres", "Parental Rating", "Community Rating", 
                "Runtime (in minutes)", "Resolution", "Subtitles", "Path "]);
        for detail in details.items {
            let string = &detail.genres.iter().map(|x| x.to_string() + ";").collect::<String>();
            let string = string.trim_end_matches(",");
            let ticks = &detail.run_time_ticks / 10000000 / 60;
            let resolution = format!("{} * {}", &detail.width.to_string(), &detail.height.to_string());
            table.add_row(vec![
                &detail.name,
                &detail.date_created,
                &detail.premiere_date,
                &detail.production_year.to_string(),
                &string.to_string(),
                &detail.official_rating,
                &detail.community_rating.to_string(),
                &ticks.to_string(),
                &resolution,
                &detail.has_subtitles.to_string(),
                &detail.path
            ]);
        }
        println!("{table}");
    }
}