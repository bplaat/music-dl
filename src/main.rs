/*
 * Copyright (c) 2024, Bastiaan van der Plaat <bastiaan.v.d.plaat@gmail.com>
 *
 * SPDX-License-Identifier: MIT
 */

use std::collections::HashMap;
use std::env;
use std::io::Write;

use anyhow::Result;
use clap::Parser;

use crate::services::download::{DownloadEvent, DownloadService};
use crate::services::metadata::MetadataService;
use crate::structs::deezer::Album;
use crate::window::start_window;

mod services;
mod structs;
mod window;

fn list_album(metadata_service: &MetadataService, album_id: i64) -> Result<()> {
    let album = metadata_service.get_album(album_id)?;
    println!(
        "# {} by {}",
        album.title,
        album
            .artists
            .iter()
            .map(|artist| artist.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "Released at {} with {} tracks",
        album.release_date, album.nb_tracks
    );

    for (index, track) in album.tracks.data.iter().enumerate() {
        let track = metadata_service.get_track(track.id)?;
        println!(
            "{}. {} ({}:{:02}) by {}",
            index + 1,
            track.title,
            track.duration / 60,
            track.duration % 60,
            track
                .artists
                .iter()
                .map(|artist| artist.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    println!();
    Ok(())
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(help = "Search query")]
    pub query: Option<String>,
    #[arg(short, long, help = "Output folder")]
    pub output: Option<String>,
    #[arg(short, long, help = "Search by artist")]
    pub artist: bool,
    #[arg(short, long, help = "Also download artist singles")]
    pub singles: bool,
    #[arg(short, long, help = "List album contents")]
    pub list: bool,
    #[arg(short, long, help = "Search by Deezer id")]
    pub id: bool,
}

fn main() -> Result<()> {
    // Parse args
    let args = Args::parse();
    let output_path = if let Some(output) = args.output {
        output
    } else {
        format!(
            "{}/Music",
            env::var("HOME").expect("Can't find $HOME folder")
        )
    };

    // Start GUI window when no args
    let query = match args.query {
        Some(query) => query,
        None => return start_window(),
    };

    // Find album ids
    let metadata_service = MetadataService::new();
    let album_ids = if args.artist {
        let artist_id = if args.id {
            query.parse()?
        } else {
            let artists = metadata_service.seach_artist(&query)?;
            if artists.is_empty() {
                println!("No artist found");
                return Ok(());
            }
            artists[0].id
        };

        let albums = metadata_service.get_artist_albums(artist_id)?;
        if args.singles {
            albums.iter().map(|album| album.id).collect()
        } else {
            albums
                .iter()
                .filter(|album| {
                    (album.r#type == "album" || album.r#type == "ep")
                        && album.record_type != "single"
                })
                .map(|album| album.id)
                .collect()
        }
    } else {
        if args.id {
            vec![query.parse()?]
        } else {
            let albums = metadata_service.search_album(&query)?;
            if albums.is_empty() {
                println!("No album found");
                return Ok(());
            }
            vec![albums[0].id]
        }
    };

    // List albums
    if args.list {
        for album_id in album_ids {
            list_album(&metadata_service, album_id)?;
        }
    }
    // Or download albums
    else {
        // Queue albums to download
        let download_service = DownloadService::new(&output_path);
        for album_id in album_ids {
            download_service.queue_album(album_id);
        }

        // Listen to download events and print progress until done
        let mut albums: HashMap<i64, Album> = HashMap::new();
        let mut tracks_progress: HashMap<i64, f32> = HashMap::new();
        let print_progress = |albums: &HashMap<i64, Album>, tracks_progress: &HashMap<i64, f32>| {
            // Clear screen and move cursor to top
            let mut stdout = std::io::stdout();
            _ = stdout.write(b"\x1b[2J\x1b[1;1H");
            _ = stdout.flush();

            // Print album progress
            for (_, album) in albums {
                println!(
                    "{} by {}",
                    album.title,
                    album
                        .artists
                        .iter()
                        .map(|artist| artist.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                for (index, track) in album.tracks.data.iter().enumerate() {
                    let progress = tracks_progress.get(&track.id).unwrap_or(&0.0);
                    println!("{}. {} ({:.2}%)", index + 1, track.title, progress);
                }
            }
        };
        print_progress(&albums, &tracks_progress);
        loop {
            match download_service.receive_event() {
                DownloadEvent::QueueEmpty => break,
                DownloadEvent::AlbumDownloadStart { album_id } => {
                    let album = metadata_service.get_album(album_id)?;
                    albums.insert(album_id, album);
                }
                DownloadEvent::AlbumDownloadDone { album_id } => {
                    albums.remove(&album_id);
                }
                DownloadEvent::TrackDownloadStart { track_id } => {
                    tracks_progress.insert(track_id, 0.0);
                }
                DownloadEvent::TrackDownloadProgress { track_id, progress } => {
                    tracks_progress.insert(track_id, progress);
                }
                DownloadEvent::TrackDownloadDone { track_id } => {
                    tracks_progress.insert(track_id, 100.0);
                }
            }
            print_progress(&albums, &tracks_progress);
        }
    }
    Ok(())
}
