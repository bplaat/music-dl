/*
 * Copyright (c) 2024, Bastiaan van der Plaat <bastiaan.v.d.plaat@gmail.com>
 *
 * SPDX-License-Identifier: MIT
 */

use anyhow::Result;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use ureq::{Agent, AgentBuilder};

use crate::structs::deezer::{Album, AlbumList, AlbumSmall, ArtistList, ArtistSmall, Track};

pub struct MetadataService {
    agent: Agent,
}

impl MetadataService {
    pub fn new() -> Self {
        let agent = AgentBuilder::new()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:127.0) Gecko/20100101 Firefox/127.0")
            .build();
        Self { agent }
    }

    pub fn search_album(&self, query: &str) -> Result<Vec<AlbumSmall>> {
        Ok(self
            .agent
            .get(&format!(
                "https://api.deezer.com/search/album?q={}",
                percent_encode(query.as_bytes(), NON_ALPHANUMERIC).to_string()
            ))
            .call()?
            .into_json::<AlbumList>()?
            .data)
    }

    pub fn seach_artist(&self, query: &str) -> Result<Vec<ArtistSmall>> {
        Ok(self
            .agent
            .get(&format!(
                "https://api.deezer.com/search/artist?q={}",
                percent_encode(query.as_bytes(), NON_ALPHANUMERIC).to_string()
            ))
            .call()?
            .into_json::<ArtistList>()?
            .data)
    }

    pub fn get_artist_albums(&self, artist_id: i64) -> Result<Vec<AlbumSmall>> {
        Ok(self
            .agent
            .get(&format!(
                "https://api.deezer.com/artist/{}/albums",
                artist_id
            ))
            .call()?
            .into_json::<AlbumList>()?
            .data)
    }

    pub fn get_album(&self, album_id: i64) -> Result<Album> {
        Ok(self
            .agent
            .get(&format!("https://api.deezer.com/album/{}", album_id))
            .call()?
            .into_json::<Album>()?)
    }

    pub fn get_track(&self, track_id: i64) -> Result<Track> {
        Ok(self
            .agent
            .get(&format!("https://api.deezer.com/track/{}", track_id))
            .call()?
            .into_json::<Track>()?)
    }
}
