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
    pub name: String
}
//     #[serde(rename = "OriginalTitle")]
//     pub original_title: String,
//     #[serde(rename = "ServerId")]
//     pub server_id: String,
//     #[serde(rename = "Id")]
//     pub id: String,
//     #[serde(rename = "Etag")]
//     pub etag: String,
//     #[serde(rename = "SourceType")]
//     pub source_type: String,
//     #[serde(rename = "PlaylistItemId")]
//     pub playlist_item_id: String,
//     #[serde(rename = "DateCreated")]
//     pub date_created: String,
//     #[serde(rename = "DateLastMediaAdded")]
//     pub date_last_media_added: String,
//     #[serde(rename = "ExtraType")]
//     pub extra_type: String,
//     #[serde(rename = "AirsBeforeSeasonNumber")]
//     pub airs_before_season_number: i64,
//     #[serde(rename = "AirsAfterSeasonNumber")]
//     pub airs_after_season_number: i64,
//     #[serde(rename = "AirsBeforeEpisodeNumber")]
//     pub airs_before_episode_number: i64,
//     #[serde(rename = "CanDelete")]
//     pub can_delete: bool,
//     #[serde(rename = "CanDownload")]
//     pub can_download: bool,
//     #[serde(rename = "HasSubtitles")]
//     pub has_subtitles: bool,
//     #[serde(rename = "PreferredMetadataLanguage")]
//     pub preferred_metadata_language: String,
//     #[serde(rename = "PreferredMetadataCountryCode")]
//     pub preferred_metadata_country_code: String,
//     #[serde(rename = "SupportsSync")]
//     pub supports_sync: bool,
//     #[serde(rename = "Container")]
//     pub container: String,
//     #[serde(rename = "SortName")]
//     pub sort_name: String,
//     #[serde(rename = "ForcedSortName")]
//     pub forced_sort_name: String,
//     #[serde(rename = "Video3DFormat")]
//     pub video3dformat: String,
//     #[serde(rename = "PremiereDate")]
//     pub premiere_date: String,
//     #[serde(rename = "ExternalUrls")]
//     pub external_urls: Vec<ExternalUrl>,
//     #[serde(rename = "MediaSources")]
//     pub media_sources: Vec<MediaSource>,
//     #[serde(rename = "CriticRating")]
//     pub critic_rating: i64,
//     #[serde(rename = "ProductionLocations")]
//     pub production_locations: Vec<String>,
//     #[serde(rename = "Path")]
//     pub path: String,
//     #[serde(rename = "EnableMediaSourceDisplay")]
//     pub enable_media_source_display: bool,
//     #[serde(rename = "OfficialRating")]
//     pub official_rating: String,
//     #[serde(rename = "CustomRating")]
//     pub custom_rating: String,
//     #[serde(rename = "ChannelId")]
//     pub channel_id: String,
//     #[serde(rename = "ChannelName")]
//     pub channel_name: String,
//     #[serde(rename = "Overview")]
//     pub overview: String,
//     #[serde(rename = "Taglines")]
//     pub taglines: Vec<String>,
//     #[serde(rename = "Genres")]
//     pub genres: Vec<String>,
//     #[serde(rename = "CommunityRating")]
//     pub community_rating: i64,
//     #[serde(rename = "CumulativeRunTimeTicks")]
//     pub cumulative_run_time_ticks: i64,
//     #[serde(rename = "RunTimeTicks")]
//     pub run_time_ticks: i64,
//     #[serde(rename = "PlayAccess")]
//     pub play_access: String,
//     #[serde(rename = "AspectRatio")]
//     pub aspect_ratio: String,
//     #[serde(rename = "ProductionYear")]
//     pub production_year: i64,
//     #[serde(rename = "IsPlaceHolder")]
//     pub is_place_holder: bool,
//     #[serde(rename = "Number")]
//     pub number: String,
//     #[serde(rename = "ChannelNumber")]
//     pub channel_number: String,
//     #[serde(rename = "IndexNumber")]
//     pub index_number: i64,
//     #[serde(rename = "IndexNumberEnd")]
//     pub index_number_end: i64,
//     #[serde(rename = "ParentIndexNumber")]
//     pub parent_index_number: i64,
//     #[serde(rename = "RemoteTrailers")]
//     pub remote_trailers: Vec<RemoteTrailer>,
//     #[serde(rename = "ProviderIds")]
//     pub provider_ids: ProviderIds,
//     #[serde(rename = "IsHD")]
//     pub is_hd: bool,
//     #[serde(rename = "IsFolder")]
//     pub is_folder: bool,
//     #[serde(rename = "ParentId")]
//     pub parent_id: String,
//     #[serde(rename = "Type")]
//     pub type_field: String,
//     #[serde(rename = "People")]
//     pub people: Vec<People>,
//     #[serde(rename = "Studios")]
//     pub studios: Vec<Studio>,
//     #[serde(rename = "GenreItems")]
//     pub genre_items: Vec<GenreItem>,
//     #[serde(rename = "ParentLogoItemId")]
//     pub parent_logo_item_id: String,
//     #[serde(rename = "ParentBackdropItemId")]
//     pub parent_backdrop_item_id: String,
//     #[serde(rename = "ParentBackdropImageTags")]
//     pub parent_backdrop_image_tags: Vec<String>,
//     #[serde(rename = "LocalTrailerCount")]
//     pub local_trailer_count: i64,
//     #[serde(rename = "UserData")]
//     pub user_data: UserData,
//     #[serde(rename = "RecursiveItemCount")]
//     pub recursive_item_count: i64,
//     #[serde(rename = "ChildCount")]
//     pub child_count: i64,
//     #[serde(rename = "SeriesName")]
//     pub series_name: String,
//     #[serde(rename = "SeriesId")]
//     pub series_id: String,
//     #[serde(rename = "SeasonId")]
//     pub season_id: String,
//     #[serde(rename = "SpecialFeatureCount")]
//     pub special_feature_count: i64,
//     #[serde(rename = "DisplayPreferencesId")]
//     pub display_preferences_id: String,
//     #[serde(rename = "Status")]
//     pub status: String,
//     #[serde(rename = "AirTime")]
//     pub air_time: String,
//     #[serde(rename = "AirDays")]
//     pub air_days: Vec<String>,
//     #[serde(rename = "Tags")]
//     pub tags: Vec<String>,
//     #[serde(rename = "PrimaryImageAspectRatio")]
//     pub primary_image_aspect_ratio: i64,
//     #[serde(rename = "Artists")]
//     pub artists: Vec<String>,
//     #[serde(rename = "ArtistItems")]
//     pub artist_items: Vec<ArtistItem>,
//     #[serde(rename = "Album")]
//     pub album: String,
//     #[serde(rename = "CollectionType")]
//     pub collection_type: String,
//     #[serde(rename = "DisplayOrder")]
//     pub display_order: String,
//     #[serde(rename = "AlbumId")]
//     pub album_id: String,
//     #[serde(rename = "AlbumPrimaryImageTag")]
//     pub album_primary_image_tag: String,
//     #[serde(rename = "SeriesPrimaryImageTag")]
//     pub series_primary_image_tag: String,
//     #[serde(rename = "AlbumArtist")]
//     pub album_artist: String,
//     #[serde(rename = "AlbumArtists")]
//     pub album_artists: Vec<AlbumArtist>,
//     #[serde(rename = "SeasonName")]
//     pub season_name: String,
//     #[serde(rename = "MediaStreams")]
//     pub media_streams: Vec<MediaStream2>,
//     #[serde(rename = "VideoType")]
//     pub video_type: String,
//     #[serde(rename = "PartCount")]
//     pub part_count: i64,
//     #[serde(rename = "MediaSourceCount")]
//     pub media_source_count: i64,
//     #[serde(rename = "ImageTags")]
//     pub image_tags: ImageTags,
//     #[serde(rename = "BackdropImageTags")]
//     pub backdrop_image_tags: Vec<String>,
//     #[serde(rename = "ScreenshotImageTags")]
//     pub screenshot_image_tags: Vec<String>,
//     #[serde(rename = "ParentLogoImageTag")]
//     pub parent_logo_image_tag: String,
//     #[serde(rename = "ParentArtItemId")]
//     pub parent_art_item_id: String,
//     #[serde(rename = "ParentArtImageTag")]
//     pub parent_art_image_tag: String,
//     #[serde(rename = "SeriesThumbImageTag")]
//     pub series_thumb_image_tag: String,
//     #[serde(rename = "ImageBlurHashes")]
//     pub image_blur_hashes: ImageBlurHashes2,
//     #[serde(rename = "SeriesStudio")]
//     pub series_studio: String,
//     #[serde(rename = "ParentThumbItemId")]
//     pub parent_thumb_item_id: String,
//     #[serde(rename = "ParentThumbImageTag")]
//     pub parent_thumb_image_tag: String,
//     #[serde(rename = "ParentPrimaryImageItemId")]
//     pub parent_primary_image_item_id: String,
//     #[serde(rename = "ParentPrimaryImageTag")]
//     pub parent_primary_image_tag: String,
//     #[serde(rename = "Chapters")]
//     pub chapters: Vec<Chapter3>,
//     #[serde(rename = "LocationType")]
//     pub location_type: String,
//     #[serde(rename = "IsoType")]
//     pub iso_type: String,
//     #[serde(rename = "MediaType")]
//     pub media_type: String,
//     #[serde(rename = "EndDate")]
//     pub end_date: String,
//     #[serde(rename = "LockedFields")]
//     pub locked_fields: Vec<String>,
//     #[serde(rename = "TrailerCount")]
//     pub trailer_count: i64,
//     #[serde(rename = "MovieCount")]
//     pub movie_count: i64,
//     #[serde(rename = "SeriesCount")]
//     pub series_count: i64,
//     #[serde(rename = "ProgramCount")]
//     pub program_count: i64,
//     #[serde(rename = "EpisodeCount")]
//     pub episode_count: i64,
//     #[serde(rename = "SongCount")]
//     pub song_count: i64,
//     #[serde(rename = "AlbumCount")]
//     pub album_count: i64,
//     #[serde(rename = "ArtistCount")]
//     pub artist_count: i64,
//     #[serde(rename = "MusicVideoCount")]
//     pub music_video_count: i64,
//     #[serde(rename = "LockData")]
//     pub lock_data: bool,
//     #[serde(rename = "Width")]
//     pub width: i64,
//     #[serde(rename = "Height")]
//     pub height: i64,
//     #[serde(rename = "CameraMake")]
//     pub camera_make: String,
//     #[serde(rename = "CameraModel")]
//     pub camera_model: String,
//     #[serde(rename = "Software")]
//     pub software: String,
//     #[serde(rename = "ExposureTime")]
//     pub exposure_time: i64,
//     #[serde(rename = "FocalLength")]
//     pub focal_length: i64,
//     #[serde(rename = "ImageOrientation")]
//     pub image_orientation: String,
//     #[serde(rename = "Aperture")]
//     pub aperture: i64,
//     #[serde(rename = "ShutterSpeed")]
//     pub shutter_speed: i64,
//     #[serde(rename = "Latitude")]
//     pub latitude: i64,
//     #[serde(rename = "Longitude")]
//     pub longitude: i64,
//     #[serde(rename = "Altitude")]
//     pub altitude: i64,
//     #[serde(rename = "IsoSpeedRating")]
//     pub iso_speed_rating: i64,
//     #[serde(rename = "SeriesTimerId")]
//     pub series_timer_id: String,
//     #[serde(rename = "ProgramId")]
//     pub program_id: String,
//     #[serde(rename = "ChannelPrimaryImageTag")]
//     pub channel_primary_image_tag: String,
//     #[serde(rename = "StartDate")]
//     pub start_date: String,
//     #[serde(rename = "CompletionPercentage")]
//     pub completion_percentage: i64,
//     #[serde(rename = "IsRepeat")]
//     pub is_repeat: bool,
//     #[serde(rename = "EpisodeTitle")]
//     pub episode_title: String,
//     #[serde(rename = "ChannelType")]
//     pub channel_type: String,
//     #[serde(rename = "Audio")]
//     pub audio: String,
//     #[serde(rename = "IsMovie")]
//     pub is_movie: bool,
//     #[serde(rename = "IsSports")]
//     pub is_sports: bool,
//     #[serde(rename = "IsSeries")]
//     pub is_series: bool,
//     #[serde(rename = "IsLive")]
//     pub is_live: bool,
//     #[serde(rename = "IsNews")]
//     pub is_news: bool,
//     #[serde(rename = "IsKids")]
//     pub is_kids: bool,
//     #[serde(rename = "IsPremiere")]
//     pub is_premiere: bool,
//     #[serde(rename = "TimerId")]
//     pub timer_id: String,
//     #[serde(rename = "CurrentProgram")]
//     pub current_program: CurrentProgram,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ExternalUrl {
//     #[serde(rename = "Name")]
//     pub name: String,
//     #[serde(rename = "Url")]
//     pub url: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct MediaSource {
//     #[serde(rename = "Protocol")]
//     pub protocol: String,
//     #[serde(rename = "Id")]
//     pub id: String,
//     #[serde(rename = "Path")]
//     pub path: String,
//     #[serde(rename = "EncoderPath")]
//     pub encoder_path: String,
//     #[serde(rename = "EncoderProtocol")]
//     pub encoder_protocol: String,
//     #[serde(rename = "Type")]
//     pub type_field: String,
//     #[serde(rename = "Container")]
//     pub container: String,
//     #[serde(rename = "Size")]
//     pub size: i64,
//     #[serde(rename = "Name")]
//     pub name: String,
//     #[serde(rename = "IsRemote")]
//     pub is_remote: bool,
//     #[serde(rename = "ETag")]
//     pub etag: String,
//     #[serde(rename = "RunTimeTicks")]
//     pub run_time_ticks: i64,
//     #[serde(rename = "ReadAtNativeFramerate")]
//     pub read_at_native_framerate: bool,
//     #[serde(rename = "IgnoreDts")]
//     pub ignore_dts: bool,
//     #[serde(rename = "IgnoreIndex")]
//     pub ignore_index: bool,
//     #[serde(rename = "GenPtsInput")]
//     pub gen_pts_input: bool,
//     #[serde(rename = "SupportsTranscoding")]
//     pub supports_transcoding: bool,
//     #[serde(rename = "SupportsDirectStream")]
//     pub supports_direct_stream: bool,
//     #[serde(rename = "SupportsDirectPlay")]
//     pub supports_direct_play: bool,
//     #[serde(rename = "IsInfiniteStream")]
//     pub is_infinite_stream: bool,
//     #[serde(rename = "RequiresOpening")]
//     pub requires_opening: bool,
//     #[serde(rename = "OpenToken")]
//     pub open_token: String,
//     #[serde(rename = "RequiresClosing")]
//     pub requires_closing: bool,
//     #[serde(rename = "LiveStreamId")]
//     pub live_stream_id: String,
//     #[serde(rename = "BufferMs")]
//     pub buffer_ms: i64,
//     #[serde(rename = "RequiresLooping")]
//     pub requires_looping: bool,
//     #[serde(rename = "SupportsProbing")]
//     pub supports_probing: bool,
//     #[serde(rename = "VideoType")]
//     pub video_type: String,
//     #[serde(rename = "IsoType")]
//     pub iso_type: String,
//     #[serde(rename = "Video3DFormat")]
//     pub video3dformat: String,
//     #[serde(rename = "MediaStreams")]
//     pub media_streams: Vec<MediaStream>,
//     #[serde(rename = "MediaAttachments")]
//     pub media_attachments: Vec<MediaAttachment>,
//     #[serde(rename = "Formats")]
//     pub formats: Vec<String>,
//     #[serde(rename = "Bitrate")]
//     pub bitrate: i64,
//     #[serde(rename = "Timestamp")]
//     pub timestamp: String,
//     #[serde(rename = "RequiredHttpHeaders")]
//     pub required_http_headers: RequiredHttpHeaders,
//     #[serde(rename = "TranscodingUrl")]
//     pub transcoding_url: String,
//     #[serde(rename = "TranscodingSubProtocol")]
//     pub transcoding_sub_protocol: String,
//     #[serde(rename = "TranscodingContainer")]
//     pub transcoding_container: String,
//     #[serde(rename = "AnalyzeDurationMs")]
//     pub analyze_duration_ms: i64,
//     #[serde(rename = "DefaultAudioStreamIndex")]
//     pub default_audio_stream_index: i64,
//     #[serde(rename = "DefaultSubtitleStreamIndex")]
//     pub default_subtitle_stream_index: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct MediaStream {
//     #[serde(rename = "Codec")]
//     pub codec: String,
//     #[serde(rename = "CodecTag")]
//     pub codec_tag: String,
//     #[serde(rename = "Language")]
//     pub language: String,
//     #[serde(rename = "ColorRange")]
//     pub color_range: String,
//     #[serde(rename = "ColorSpace")]
//     pub color_space: String,
//     #[serde(rename = "ColorTransfer")]
//     pub color_transfer: String,
//     #[serde(rename = "ColorPrimaries")]
//     pub color_primaries: String,
//     #[serde(rename = "DvVersionMajor")]
//     pub dv_version_major: i64,
//     #[serde(rename = "DvVersionMinor")]
//     pub dv_version_minor: i64,
//     #[serde(rename = "DvProfile")]
//     pub dv_profile: i64,
//     #[serde(rename = "DvLevel")]
//     pub dv_level: i64,
//     #[serde(rename = "RpuPresentFlag")]
//     pub rpu_present_flag: i64,
//     #[serde(rename = "ElPresentFlag")]
//     pub el_present_flag: i64,
//     #[serde(rename = "BlPresentFlag")]
//     pub bl_present_flag: i64,
//     #[serde(rename = "DvBlSignalCompatibilityId")]
//     pub dv_bl_signal_compatibility_id: i64,
//     #[serde(rename = "Comment")]
//     pub comment: String,
//     #[serde(rename = "TimeBase")]
//     pub time_base: String,
//     #[serde(rename = "CodecTimeBase")]
//     pub codec_time_base: String,
//     #[serde(rename = "Title")]
//     pub title: String,
//     #[serde(rename = "VideoRange")]
//     pub video_range: String,
//     #[serde(rename = "VideoRangeType")]
//     pub video_range_type: String,
//     #[serde(rename = "VideoDoViTitle")]
//     pub video_do_vi_title: String,
//     #[serde(rename = "LocalizedUndefined")]
//     pub localized_undefined: String,
//     #[serde(rename = "LocalizedDefault")]
//     pub localized_default: String,
//     #[serde(rename = "LocalizedForced")]
//     pub localized_forced: String,
//     #[serde(rename = "LocalizedExternal")]
//     pub localized_external: String,
//     #[serde(rename = "DisplayTitle")]
//     pub display_title: String,
//     #[serde(rename = "NalLengthSize")]
//     pub nal_length_size: String,
//     #[serde(rename = "IsInterlaced")]
//     pub is_interlaced: bool,
//     #[serde(rename = "IsAVC")]
//     pub is_avc: bool,
//     #[serde(rename = "ChannelLayout")]
//     pub channel_layout: String,
//     #[serde(rename = "BitRate")]
//     pub bit_rate: i64,
//     #[serde(rename = "BitDepth")]
//     pub bit_depth: i64,
//     #[serde(rename = "RefFrames")]
//     pub ref_frames: i64,
//     #[serde(rename = "PacketLength")]
//     pub packet_length: i64,
//     #[serde(rename = "Channels")]
//     pub channels: i64,
//     #[serde(rename = "SampleRate")]
//     pub sample_rate: i64,
//     #[serde(rename = "IsDefault")]
//     pub is_default: bool,
//     #[serde(rename = "IsForced")]
//     pub is_forced: bool,
//     #[serde(rename = "Height")]
//     pub height: i64,
//     #[serde(rename = "Width")]
//     pub width: i64,
//     #[serde(rename = "AverageFrameRate")]
//     pub average_frame_rate: i64,
//     #[serde(rename = "RealFrameRate")]
//     pub real_frame_rate: i64,
//     #[serde(rename = "Profile")]
//     pub profile: String,
//     #[serde(rename = "Type")]
//     pub type_field: String,
//     #[serde(rename = "AspectRatio")]
//     pub aspect_ratio: String,
//     #[serde(rename = "Index")]
//     pub index: i64,
//     #[serde(rename = "Score")]
//     pub score: i64,
//     #[serde(rename = "IsExternal")]
//     pub is_external: bool,
//     #[serde(rename = "DeliveryMethod")]
//     pub delivery_method: String,
//     #[serde(rename = "DeliveryUrl")]
//     pub delivery_url: String,
//     #[serde(rename = "IsExternalUrl")]
//     pub is_external_url: bool,
//     #[serde(rename = "IsTextSubtitleStream")]
//     pub is_text_subtitle_stream: bool,
//     #[serde(rename = "SupportsExternalStream")]
//     pub supports_external_stream: bool,
//     #[serde(rename = "Path")]
//     pub path: String,
//     #[serde(rename = "PixelFormat")]
//     pub pixel_format: String,
//     #[serde(rename = "Level")]
//     pub level: i64,
//     #[serde(rename = "IsAnamorphic")]
//     pub is_anamorphic: bool,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct MediaAttachment {
//     #[serde(rename = "Codec")]
//     pub codec: String,
//     #[serde(rename = "CodecTag")]
//     pub codec_tag: String,
//     #[serde(rename = "Comment")]
//     pub comment: String,
//     #[serde(rename = "Index")]
//     pub index: i64,
//     #[serde(rename = "FileName")]
//     pub file_name: String,
//     #[serde(rename = "MimeType")]
//     pub mime_type: String,
//     #[serde(rename = "DeliveryUrl")]
//     pub delivery_url: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct RequiredHttpHeaders {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct RemoteTrailer {
//     #[serde(rename = "Url")]
//     pub url: String,
//     #[serde(rename = "Name")]
//     pub name: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ProviderIds {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct People {
//     #[serde(rename = "Name")]
//     pub name: String,
//     #[serde(rename = "Id")]
//     pub id: String,
//     #[serde(rename = "Role")]
//     pub role: String,
//     #[serde(rename = "Type")]
//     pub type_field: String,
//     #[serde(rename = "PrimaryImageTag")]
//     pub primary_image_tag: String,
//     #[serde(rename = "ImageBlurHashes")]
//     pub image_blur_hashes: ImageBlurHashes,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ImageBlurHashes {
//     #[serde(rename = "Primary")]
//     pub primary: Primary,
//     #[serde(rename = "Art")]
//     pub art: Art,
//     #[serde(rename = "Backdrop")]
//     pub backdrop: Backdrop,
//     #[serde(rename = "Banner")]
//     pub banner: Banner,
//     #[serde(rename = "Logo")]
//     pub logo: Logo,
//     #[serde(rename = "Thumb")]
//     pub thumb: Thumb,
//     #[serde(rename = "Disc")]
//     pub disc: Disc,
//     #[serde(rename = "Box")]
//     pub box_field: Box,
//     #[serde(rename = "Screenshot")]
//     pub screenshot: Screenshot,
//     #[serde(rename = "Menu")]
//     pub menu: Menu,
//     #[serde(rename = "Chapter")]
//     pub chapter: Chapter,
//     #[serde(rename = "BoxRear")]
//     pub box_rear: BoxRear,
//     #[serde(rename = "Profile")]
//     pub profile: Profile,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Primary {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Art {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Backdrop {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Banner {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Logo {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Thumb {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Disc {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Box {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Screenshot {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Menu {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Chapter {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct BoxRear {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Profile {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Studio {
//     #[serde(rename = "Name")]
//     pub name: String,
//     #[serde(rename = "Id")]
//     pub id: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct GenreItem {
//     #[serde(rename = "Name")]
//     pub name: String,
//     #[serde(rename = "Id")]
//     pub id: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct UserData {
//     #[serde(rename = "Rating")]
//     pub rating: i64,
//     #[serde(rename = "PlayedPercentage")]
//     pub played_percentage: i64,
//     #[serde(rename = "UnplayedItemCount")]
//     pub unplayed_item_count: i64,
//     #[serde(rename = "PlaybackPositionTicks")]
//     pub playback_position_ticks: i64,
//     #[serde(rename = "PlayCount")]
//     pub play_count: i64,
//     #[serde(rename = "IsFavorite")]
//     pub is_favorite: bool,
//     #[serde(rename = "Likes")]
//     pub likes: bool,
//     #[serde(rename = "LastPlayedDate")]
//     pub last_played_date: String,
//     #[serde(rename = "Played")]
//     pub played: bool,
//     #[serde(rename = "Key")]
//     pub key: String,
//     #[serde(rename = "ItemId")]
//     pub item_id: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ArtistItem {
//     #[serde(rename = "Name")]
//     pub name: String,
//     #[serde(rename = "Id")]
//     pub id: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct AlbumArtist {
//     #[serde(rename = "Name")]
//     pub name: String,
//     #[serde(rename = "Id")]
//     pub id: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct MediaStream2 {
//     #[serde(rename = "Codec")]
//     pub codec: String,
//     #[serde(rename = "CodecTag")]
//     pub codec_tag: String,
//     #[serde(rename = "Language")]
//     pub language: String,
//     #[serde(rename = "ColorRange")]
//     pub color_range: String,
//     #[serde(rename = "ColorSpace")]
//     pub color_space: String,
//     #[serde(rename = "ColorTransfer")]
//     pub color_transfer: String,
//     #[serde(rename = "ColorPrimaries")]
//     pub color_primaries: String,
//     #[serde(rename = "DvVersionMajor")]
//     pub dv_version_major: i64,
//     #[serde(rename = "DvVersionMinor")]
//     pub dv_version_minor: i64,
//     #[serde(rename = "DvProfile")]
//     pub dv_profile: i64,
//     #[serde(rename = "DvLevel")]
//     pub dv_level: i64,
//     #[serde(rename = "RpuPresentFlag")]
//     pub rpu_present_flag: i64,
//     #[serde(rename = "ElPresentFlag")]
//     pub el_present_flag: i64,
//     #[serde(rename = "BlPresentFlag")]
//     pub bl_present_flag: i64,
//     #[serde(rename = "DvBlSignalCompatibilityId")]
//     pub dv_bl_signal_compatibility_id: i64,
//     #[serde(rename = "Comment")]
//     pub comment: String,
//     #[serde(rename = "TimeBase")]
//     pub time_base: String,
//     #[serde(rename = "CodecTimeBase")]
//     pub codec_time_base: String,
//     #[serde(rename = "Title")]
//     pub title: String,
//     #[serde(rename = "VideoRange")]
//     pub video_range: String,
//     #[serde(rename = "VideoRangeType")]
//     pub video_range_type: String,
//     #[serde(rename = "VideoDoViTitle")]
//     pub video_do_vi_title: String,
//     #[serde(rename = "LocalizedUndefined")]
//     pub localized_undefined: String,
//     #[serde(rename = "LocalizedDefault")]
//     pub localized_default: String,
//     #[serde(rename = "LocalizedForced")]
//     pub localized_forced: String,
//     #[serde(rename = "LocalizedExternal")]
//     pub localized_external: String,
//     #[serde(rename = "DisplayTitle")]
//     pub display_title: String,
//     #[serde(rename = "NalLengthSize")]
//     pub nal_length_size: String,
//     #[serde(rename = "IsInterlaced")]
//     pub is_interlaced: bool,
//     #[serde(rename = "IsAVC")]
//     pub is_avc: bool,
//     #[serde(rename = "ChannelLayout")]
//     pub channel_layout: String,
//     #[serde(rename = "BitRate")]
//     pub bit_rate: i64,
//     #[serde(rename = "BitDepth")]
//     pub bit_depth: i64,
//     #[serde(rename = "RefFrames")]
//     pub ref_frames: i64,
//     #[serde(rename = "PacketLength")]
//     pub packet_length: i64,
//     #[serde(rename = "Channels")]
//     pub channels: i64,
//     #[serde(rename = "SampleRate")]
//     pub sample_rate: i64,
//     #[serde(rename = "IsDefault")]
//     pub is_default: bool,
//     #[serde(rename = "IsForced")]
//     pub is_forced: bool,
//     #[serde(rename = "Height")]
//     pub height: i64,
//     #[serde(rename = "Width")]
//     pub width: i64,
//     #[serde(rename = "AverageFrameRate")]
//     pub average_frame_rate: i64,
//     #[serde(rename = "RealFrameRate")]
//     pub real_frame_rate: i64,
//     #[serde(rename = "Profile")]
//     pub profile: String,
//     #[serde(rename = "Type")]
//     pub type_field: String,
//     #[serde(rename = "AspectRatio")]
//     pub aspect_ratio: String,
//     #[serde(rename = "Index")]
//     pub index: i64,
//     #[serde(rename = "Score")]
//     pub score: i64,
//     #[serde(rename = "IsExternal")]
//     pub is_external: bool,
//     #[serde(rename = "DeliveryMethod")]
//     pub delivery_method: String,
//     #[serde(rename = "DeliveryUrl")]
//     pub delivery_url: String,
//     #[serde(rename = "IsExternalUrl")]
//     pub is_external_url: bool,
//     #[serde(rename = "IsTextSubtitleStream")]
//     pub is_text_subtitle_stream: bool,
//     #[serde(rename = "SupportsExternalStream")]
//     pub supports_external_stream: bool,
//     #[serde(rename = "Path")]
//     pub path: String,
//     #[serde(rename = "PixelFormat")]
//     pub pixel_format: String,
//     #[serde(rename = "Level")]
//     pub level: i64,
//     #[serde(rename = "IsAnamorphic")]
//     pub is_anamorphic: bool,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ImageTags {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ImageBlurHashes2 {
//     #[serde(rename = "Primary")]
//     pub primary: Primary2,
//     #[serde(rename = "Art")]
//     pub art: Art2,
//     #[serde(rename = "Backdrop")]
//     pub backdrop: Backdrop2,
//     #[serde(rename = "Banner")]
//     pub banner: Banner2,
//     #[serde(rename = "Logo")]
//     pub logo: Logo2,
//     #[serde(rename = "Thumb")]
//     pub thumb: Thumb2,
//     #[serde(rename = "Disc")]
//     pub disc: Disc2,
//     #[serde(rename = "Box")]
//     pub box_field: Box2,
//     #[serde(rename = "Screenshot")]
//     pub screenshot: Screenshot2,
//     #[serde(rename = "Menu")]
//     pub menu: Menu2,
//     #[serde(rename = "Chapter")]
//     pub chapter: Chapter2,
//     #[serde(rename = "BoxRear")]
//     pub box_rear: BoxRear2,
//     #[serde(rename = "Profile")]
//     pub profile: Profile2,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Primary2 {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Art2 {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Backdrop2 {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Banner2 {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Logo2 {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Thumb2 {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Disc2 {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Box2 {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Screenshot2 {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Menu2 {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Chapter2 {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct BoxRear2 {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Profile2 {
//     pub property1: String,
//     pub property2: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Chapter3 {
//     #[serde(rename = "StartPositionTicks")]
//     pub start_position_ticks: i64,
//     #[serde(rename = "Name")]
//     pub name: String,
//     #[serde(rename = "ImagePath")]
//     pub image_path: String,
//     #[serde(rename = "ImageDateModified")]
//     pub image_date_modified: String,
//     #[serde(rename = "ImageTag")]
//     pub image_tag: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct CurrentProgram {
// }

impl MediaDetails {
    pub fn table_print(details: MediaDetails) {
        let mut table = Table::new();
        table  
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Name"]);
        for detail in details.items {
            table.add_row(vec![
                &detail.name
            ]);
        }
        println!("{table}");
    }
}