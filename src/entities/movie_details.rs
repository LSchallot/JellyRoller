use comfy_table::{ContentArrangement, Table};
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieDetails {
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
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

impl MovieDetails {
    pub fn table_print(movies: MovieDetails) {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                "Name",
                "Date Added",
                "Premiere Date",
                "Release Year",
                "Genres",
                "Parental Rating",
                "Community Rating",
                "Runtime (in minutes)",
                "Resolution",
                "Subtitles",
                "Path ",
            ]);
        for movie in movies.items {
            table.add_row(vec![
                &movie.name,
                &movie.date_created,
                &movie.premiere_date,
                &movie.production_year.to_string(),
                &Self::genres_to_string(&movie),
                &movie.official_rating,
                &movie.community_rating.to_string(),
                &Self::ticks_to_minutes(&movie.run_time_ticks).to_string(),
                &Self::format_resolution(movie.width.to_string(), movie.height.to_string()),
                &movie.has_subtitles.to_string(),
                &movie.path,
            ]);
        }
        println!("{table}");
    }

    pub fn print_as_csv(movies: MovieDetails) -> String {
        let mut data: String = "Name,Date Added,Premiere Date,Release Year,Genres,Parental Rating,Community Rating,Runtime (in minutes),Resolution,Subtitles,Path\n".to_owned();
        for movie in movies.items {
            let piece = format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                movie.name,
                movie.date_created,
                movie.premiere_date,
                movie.production_year,
                Self::genres_to_string(&movie),
                movie.official_rating,
                movie.community_rating,
                Self::ticks_to_minutes(&movie.run_time_ticks),
                Self::format_resolution(movie.width.to_string(), movie.height.to_string()),
                movie.has_subtitles,
                movie.path
            );
            data.push_str(&piece);
        }
        data
    }

    fn ticks_to_minutes(ticks: &i64) -> i64 {
        ticks / 10000000 / 60
    }

    fn genres_to_string(movie: &Item) -> String {
        let string = &movie
            .genres
            .iter()
            .map(|x| x.to_string() + ";")
            .collect::<String>();
        string.trim_end_matches(',').to_string()
    }

    fn format_resolution(width: String, height: String) -> String {
        format!("{} * {}", width, height)
    }
}
