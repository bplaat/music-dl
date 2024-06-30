/*
 * Copyright (c) 2024, Bastiaan van der Plaat <bastiaan.v.d.plaat@gmail.com>
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

// Album
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumList {
    pub data: Vec<AlbumSmall>,
    pub total: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumSmall {
    pub id: i64,
    pub title: String,
    pub cover: String,
    #[serde(rename = "cover_small")]
    pub cover_small: String,
    #[serde(rename = "cover_medium")]
    pub cover_medium: String,
    #[serde(rename = "cover_big")]
    pub cover_big: String,
    #[serde(rename = "cover_xl")]
    pub cover_xl: String,
    #[serde(rename = "record_type")]
    pub record_type: String,
    #[serde(rename = "explicit_lyrics")]
    pub explicit_lyrics: bool,
    pub r#type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub cover: String,
    #[serde(rename = "cover_small")]
    pub cover_small: String,
    #[serde(rename = "cover_medium")]
    pub cover_medium: String,
    #[serde(rename = "cover_big")]
    pub cover_big: String,
    #[serde(rename = "cover_xl")]
    pub cover_xl: String,
    pub genres: GenreList,
    #[serde(rename = "nb_tracks")]
    pub nb_tracks: i64,
    pub duration: i64,
    #[serde(rename = "release_date")]
    pub release_date: String,
    #[serde(rename = "record_type")]
    pub record_type: String,
    #[serde(rename = "explicit_lyrics")]
    pub explicit_lyrics: bool,
    #[serde(rename = "contributors")]
    pub artists: Vec<ArtistSmall>,
    pub r#type: String,
    pub tracks: TrackList,
}

// Artist
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistList {
    pub data: Vec<ArtistSmall>,
    pub total: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistSmall {
    pub id: i64,
    pub name: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: i64,
    pub name: String,
    pub picture: String,
    #[serde(rename = "picture_small")]
    pub picture_small: String,
    #[serde(rename = "picture_medium")]
    pub picture_medium: String,
    #[serde(rename = "picture_big")]
    pub picture_big: String,
    #[serde(rename = "picture_xl")]
    pub picture_xl: String,
    pub r#type: String,
}

// Genre
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenreList {
    pub data: Vec<Genre>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    pub id: i64,
    pub name: String,
    pub r#type: String,
}

// Track
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackList {
    pub data: Vec<TrackSmall>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackSmall {
    pub id: i64,
    pub title: String,
    pub duration: i64,
    #[serde(rename = "explicit_lyrics")]
    pub explicit_lyrics: bool,
    pub r#type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: i64,
    pub title: String,
    pub duration: i64,
    #[serde(rename = "track_position")]
    pub track_position: i64,
    #[serde(rename = "disk_number")]
    pub disk_number: i64,
    #[serde(rename = "release_date")]
    pub release_date: String,
    #[serde(rename = "explicit_lyrics")]
    pub explicit_lyrics: bool,
    pub bpm: f64,
    #[serde(rename = "contributors")]
    pub artists: Vec<ArtistSmall>,
    #[serde(rename = "type")]
    pub r#type: String,
}
