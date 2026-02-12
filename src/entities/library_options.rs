use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryOptionsRoot {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "LibraryOptions")]
    pub library_options: LibraryOptions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryOptions {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "EnablePhotos")]
    pub enable_photos: bool,
    #[serde(rename = "EnableRealtimeMonitor")]
    pub enable_realtime_monitor: bool,
    #[serde(rename = "EnableLUFSScan")]
    pub enable_lufsscan: bool,
    #[serde(rename = "EnableChapterImageExtraction")]
    pub enable_chapter_image_extraction: bool,
    #[serde(rename = "ExtractChapterImagesDuringLibraryScan")]
    pub extract_chapter_images_during_library_scan: bool,
    #[serde(rename = "EnableTrickplayImageExtraction")]
    pub enable_trickplay_image_extraction: bool,
    #[serde(rename = "ExtractTrickplayImagesDuringLibraryScan")]
    pub extract_trickplay_images_during_library_scan: bool,
    #[serde(rename = "PathInfos")]
    pub path_infos: Vec<PathInfo>,
    #[serde(rename = "SaveLocalMetadata")]
    pub save_local_metadata: bool,
    #[serde(rename = "EnableInternetProviders")]
    pub enable_internet_providers: bool,
    #[serde(rename = "EnableAutomaticSeriesGrouping")]
    pub enable_automatic_series_grouping: bool,
    #[serde(rename = "EnableEmbeddedTitles")]
    pub enable_embedded_titles: bool,
    #[serde(rename = "EnableEmbeddedExtrasTitles")]
    pub enable_embedded_extras_titles: bool,
    #[serde(rename = "EnableEmbeddedEpisodeInfos")]
    pub enable_embedded_episode_infos: bool,
    #[serde(rename = "AutomaticRefreshIntervalDays")]
    pub automatic_refresh_interval_days: i64,
    #[serde(rename = "PreferredMetadataLanguage")]
    pub preferred_metadata_language: String,
    #[serde(rename = "MetadataCountryCode")]
    pub metadata_country_code: String,
    #[serde(rename = "SeasonZeroDisplayName")]
    pub season_zero_display_name: String,
    #[serde(rename = "MetadataSavers")]
    pub metadata_savers: Vec<String>,
    #[serde(rename = "DisabledLocalMetadataReaders")]
    pub disabled_local_metadata_readers: Vec<String>,
    #[serde(rename = "LocalMetadataReaderOrder")]
    pub local_metadata_reader_order: Vec<String>,
    #[serde(rename = "DisabledSubtitleFetchers")]
    pub disabled_subtitle_fetchers: Vec<String>,
    #[serde(rename = "SubtitleFetcherOrder")]
    pub subtitle_fetcher_order: Vec<String>,
    #[serde(rename = "DisabledMediaSegmentProviders")]
    pub disabled_media_segment_providers: Vec<String>,
    #[serde(rename = "MediaSegmentProviderOrder")]
    pub media_segment_provider_order: Vec<String>,
    #[serde(rename = "SkipSubtitlesIfEmbeddedSubtitlesPresent")]
    pub skip_subtitles_if_embedded_subtitles_present: bool,
    #[serde(rename = "SkipSubtitlesIfAudioTrackMatches")]
    pub skip_subtitles_if_audio_track_matches: bool,
    #[serde(rename = "SubtitleDownloadLanguages")]
    pub subtitle_download_languages: Vec<String>,
    #[serde(rename = "RequirePerfectSubtitleMatch")]
    pub require_perfect_subtitle_match: bool,
    #[serde(rename = "SaveSubtitlesWithMedia")]
    pub save_subtitles_with_media: bool,
    #[serde(rename = "SaveLyricsWithMedia")]
    pub save_lyrics_with_media: bool,
    #[serde(rename = "SaveTrickplayWithMedia")]
    pub save_trickplay_with_media: bool,
    #[serde(rename = "DisabledLyricFetchers")]
    pub disabled_lyric_fetchers: Vec<String>,
    #[serde(rename = "LyricFetcherOrder")]
    pub lyric_fetcher_order: Vec<String>,
    #[serde(rename = "PreferNonstandardArtistsTag")]
    pub prefer_nonstandard_artists_tag: bool,
    #[serde(rename = "UseCustomTagDelimiters")]
    pub use_custom_tag_delimiters: bool,
    #[serde(rename = "CustomTagDelimiters")]
    pub custom_tag_delimiters: Vec<String>,
    #[serde(rename = "DelimiterWhitelist")]
    pub delimiter_whitelist: Vec<String>,
    #[serde(rename = "AutomaticallyAddToCollection")]
    pub automatically_add_to_collection: bool,
    #[serde(rename = "AllowEmbeddedSubtitles")]
    pub allow_embedded_subtitles: String,
    #[serde(rename = "TypeOptions")]
    pub type_options: Vec<TypeOption>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathInfo {
    #[serde(rename = "Path")]
    pub path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeOption {
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "MetadataFetchers")]
    pub metadata_fetchers: Vec<String>,
    #[serde(rename = "MetadataFetcherOrder")]
    pub metadata_fetcher_order: Vec<String>,
    #[serde(rename = "ImageFetchers")]
    pub image_fetchers: Vec<String>,
    #[serde(rename = "ImageFetcherOrder")]
    pub image_fetcher_order: Vec<String>,
    #[serde(rename = "ImageOptions")]
    pub image_options: Vec<ImageOption>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageOption {
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "Limit")]
    pub limit: i64,
    #[serde(rename = "MinWidth")]
    pub min_width: i64,
}
