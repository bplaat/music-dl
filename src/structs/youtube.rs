/*
 * Copyright (c) 2024, Bastiaan van der Plaat <bastiaan.v.d.plaat@gmail.com>
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: String,
    pub title: String,
    pub formats: Vec<Format>,
    pub thumbnails: Vec<Thumbnail>,
    pub thumbnail: String,
    pub description: String,
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    #[serde(rename = "channel_url")]
    pub channel_url: String,
    pub duration: i64,
    #[serde(rename = "view_count")]
    pub view_count: i64,
    #[serde(rename = "average_rating")]
    pub average_rating: Value,
    #[serde(rename = "age_limit")]
    pub age_limit: i64,
    #[serde(rename = "webpage_url")]
    pub webpage_url: String,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    #[serde(rename = "playable_in_embed")]
    pub playable_in_embed: bool,
    #[serde(rename = "live_status")]
    pub live_status: String,
    #[serde(rename = "release_timestamp")]
    pub release_timestamp: Value,
    #[serde(rename = "_format_sort_fields")]
    pub format_sort_fields: Vec<String>,
    #[serde(rename = "automatic_captions")]
    pub automatic_captions: AutomaticCaptions,
    pub subtitles: Subtitles,
    pub album: String,
    pub artists: Vec<String>,
    pub track: String,
    #[serde(rename = "release_date")]
    pub release_date: Value,
    #[serde(rename = "release_year")]
    pub release_year: i64,
    #[serde(rename = "comment_count")]
    pub comment_count: i64,
    pub chapters: Value,
    pub heatmap: Vec<Heatmap>,
    #[serde(rename = "like_count")]
    pub like_count: i64,
    pub channel: String,
    #[serde(rename = "channel_follower_count")]
    pub channel_follower_count: i64,
    #[serde(rename = "channel_is_verified")]
    pub channel_is_verified: bool,
    pub uploader: String,
    #[serde(rename = "uploader_id")]
    pub uploader_id: Value,
    #[serde(rename = "uploader_url")]
    pub uploader_url: Value,
    #[serde(rename = "upload_date")]
    pub upload_date: String,
    pub timestamp: i64,
    pub creators: Vec<String>,
    #[serde(rename = "alt_title")]
    pub alt_title: String,
    pub availability: String,
    #[serde(rename = "original_url")]
    pub original_url: String,
    #[serde(rename = "webpage_url_basename")]
    pub webpage_url_basename: String,
    #[serde(rename = "webpage_url_domain")]
    pub webpage_url_domain: String,
    pub extractor: String,
    #[serde(rename = "extractor_key")]
    pub extractor_key: String,
    #[serde(rename = "playlist_count")]
    pub playlist_count: i64,
    pub playlist: String,
    #[serde(rename = "playlist_id")]
    pub playlist_id: String,
    #[serde(rename = "playlist_title")]
    pub playlist_title: String,
    #[serde(rename = "playlist_uploader")]
    pub playlist_uploader: Value,
    #[serde(rename = "playlist_uploader_id")]
    pub playlist_uploader_id: Value,
    #[serde(rename = "n_entries")]
    pub n_entries: i64,
    #[serde(rename = "playlist_index")]
    pub playlist_index: i64,
    #[serde(rename = "__last_playlist_index")]
    pub last_playlist_index: i64,
    #[serde(rename = "playlist_autonumber")]
    pub playlist_autonumber: i64,
    #[serde(rename = "display_id")]
    pub display_id: String,
    pub fulltitle: String,
    #[serde(rename = "duration_string")]
    pub duration_string: String,
    #[serde(rename = "is_live")]
    pub is_live: bool,
    #[serde(rename = "was_live")]
    pub was_live: bool,
    pub artist: String,
    pub creator: String,
    #[serde(rename = "requested_subtitles")]
    pub requested_subtitles: Value,
    #[serde(rename = "_has_drm")]
    pub has_drm: Value,
    pub epoch: i64,
    #[serde(rename = "requested_formats")]
    pub requested_formats: Vec<RequestedFormat>,
    pub format: String,
    #[serde(rename = "format_id")]
    pub format_id: String,
    pub ext: String,
    pub protocol: String,
    pub language: Value,
    #[serde(rename = "format_note")]
    pub format_note: String,
    #[serde(rename = "filesize_approx")]
    pub filesize_approx: i64,
    pub tbr: f64,
    pub width: i64,
    pub height: i64,
    pub resolution: String,
    pub fps: i64,
    #[serde(rename = "dynamic_range")]
    pub dynamic_range: String,
    pub vcodec: String,
    pub vbr: f64,
    #[serde(rename = "stretched_ratio")]
    pub stretched_ratio: Value,
    #[serde(rename = "aspect_ratio")]
    pub aspect_ratio: f64,
    pub acodec: String,
    pub abr: f64,
    pub asr: i64,
    #[serde(rename = "audio_channels")]
    pub audio_channels: i64,
    #[serde(rename = "_filename")]
    pub filename: String,
    #[serde(rename = "filename")]
    pub filename2: String,
    #[serde(rename = "_type")]
    pub r#type: String,
    #[serde(rename = "_version")]
    pub version: Version,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    #[serde(rename = "format_id")]
    pub format_id: String,
    #[serde(rename = "format_note")]
    pub format_note: Option<String>,
    pub ext: String,
    pub protocol: String,
    pub acodec: Option<String>,
    pub vcodec: String,
    pub url: String,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub fps: Option<f64>,
    pub rows: Option<i64>,
    pub columns: Option<i64>,
    #[serde(default)]
    pub fragments: Vec<Fragment>,
    pub resolution: String,
    #[serde(rename = "aspect_ratio")]
    pub aspect_ratio: Option<f64>,
    #[serde(rename = "filesize_approx")]
    pub filesize_approx: Option<i64>,
    #[serde(rename = "http_headers")]
    pub http_headers: HttpHeaders,
    #[serde(rename = "audio_ext")]
    pub audio_ext: String,
    #[serde(rename = "video_ext")]
    pub video_ext: String,
    pub vbr: Option<f64>,
    pub abr: Option<f64>,
    pub tbr: Option<f64>,
    pub format: String,
    #[serde(rename = "manifest_url")]
    pub manifest_url: Option<String>,
    pub language: Value,
    pub preference: Value,
    pub quality: Option<f64>,
    #[serde(rename = "has_drm")]
    pub has_drm: Option<bool>,
    #[serde(rename = "source_preference")]
    pub source_preference: Option<i64>,
    pub asr: Option<i64>,
    pub filesize: Option<i64>,
    #[serde(rename = "audio_channels")]
    pub audio_channels: Option<i64>,
    #[serde(rename = "language_preference")]
    pub language_preference: Option<i64>,
    #[serde(rename = "dynamic_range")]
    pub dynamic_range: Option<String>,
    pub container: Option<String>,
    #[serde(rename = "downloader_options")]
    pub downloader_options: Option<DownloaderOptions>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fragment {
    pub url: String,
    pub duration: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpHeaders {
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(rename = "Accept")]
    pub accept: String,
    #[serde(rename = "Accept-Language")]
    pub accept_language: String,
    #[serde(rename = "Sec-Fetch-Mode")]
    pub sec_fetch_mode: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloaderOptions {
    #[serde(rename = "http_chunk_size")]
    pub http_chunk_size: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub url: String,
    pub preference: i64,
    pub id: String,
    pub height: Option<i64>,
    pub width: Option<i64>,
    pub resolution: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomaticCaptions {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subtitles {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Heatmap {
    #[serde(rename = "start_time")]
    pub start_time: f64,
    #[serde(rename = "end_time")]
    pub end_time: f64,
    pub value: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestedFormat {
    pub asr: Option<i64>,
    pub filesize: i64,
    #[serde(rename = "format_id")]
    pub format_id: String,
    #[serde(rename = "format_note")]
    pub format_note: String,
    #[serde(rename = "source_preference")]
    pub source_preference: i64,
    pub fps: Option<i64>,
    #[serde(rename = "audio_channels")]
    pub audio_channels: Option<i64>,
    pub height: Option<i64>,
    pub quality: f64,
    #[serde(rename = "has_drm")]
    pub has_drm: bool,
    pub tbr: f64,
    #[serde(rename = "filesize_approx")]
    pub filesize_approx: i64,
    pub url: String,
    pub width: Option<i64>,
    pub language: Value,
    #[serde(rename = "language_preference")]
    pub language_preference: i64,
    pub preference: Value,
    pub ext: String,
    pub vcodec: String,
    pub acodec: String,
    #[serde(rename = "dynamic_range")]
    pub dynamic_range: Option<String>,
    pub container: String,
    #[serde(rename = "downloader_options")]
    pub downloader_options: DownloaderOptions2,
    pub protocol: String,
    pub resolution: String,
    #[serde(rename = "aspect_ratio")]
    pub aspect_ratio: Option<f64>,
    #[serde(rename = "http_headers")]
    pub http_headers: HttpHeaders2,
    #[serde(rename = "video_ext")]
    pub video_ext: String,
    #[serde(rename = "audio_ext")]
    pub audio_ext: String,
    pub abr: f64,
    pub vbr: f64,
    pub format: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloaderOptions2 {
    #[serde(rename = "http_chunk_size")]
    pub http_chunk_size: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpHeaders2 {
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(rename = "Accept")]
    pub accept: String,
    #[serde(rename = "Accept-Language")]
    pub accept_language: String,
    #[serde(rename = "Sec-Fetch-Mode")]
    pub sec_fetch_mode: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub version: String,
    #[serde(rename = "current_git_head")]
    pub current_git_head: Value,
    #[serde(rename = "release_git_head")]
    pub release_git_head: String,
    pub repository: String,
}
