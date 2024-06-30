/*
 * Copyright (c) 2024, Bastiaan van der Plaat <bastiaan.v.d.plaat@gmail.com>
 *
 * SPDX-License-Identifier: MIT
 */

use std::io::BufRead;

use anyhow::Result;

use crate::structs::deezer::{Album, AlbumList, AlbumSmall, ArtistList, ArtistSmall, Track};
use crate::structs::youtube::Video;

const DOWNLOAD_THREAD_COUNT: usize = 8;
const TRACK_DURATION_SLACK: i64 = 5;

fn escape_path(path: String) -> String {
    path.replace("<", "_")
        .replace(">", "_")
        .replace(":", "_")
        .replace("\"", "_")
        .replace("'", "_")
        .replace("/", "_")
        .replace("\\", "_")
        .replace("|", "_")
        .replace("?", "_")
        .replace("*", "_")
}

pub enum DownloadEvent {
    QueueEmpty,
    AlbumDownloadStart { album_id: i64 },
    AlbumDownloadDone { album_id: i64 },
    TrackDownloadStart { track_id: i64 },
    TrackDownloadProgress { track_id: i64, progress: f32 },
    TrackDownloadDone { track_id: i64 },
}

pub struct DownloadService {
    agent: ureq::Agent,
    output_path: String,
    rx: std::sync::mpsc::Receiver<DownloadEvent>,
    tx: std::sync::mpsc::Sender<DownloadEvent>,
}

impl DownloadService {
    pub fn new(output_path: &str) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            agent: ureq::Agent::new(),
            output_path: output_path.to_string(),
            rx,
            tx,
        }
    }

    pub fn receive_event(&self) -> DownloadEvent {
        self.rx.recv().expect("Can't read from channel")
    }

    pub fn queue_album(&self, album_id: i64) {
        // let album = album_info(&self.agent, album_id)?;
        // for (index, track) in album.tracks.data.iter().enumerate() {
        //     let track = track_info(agent, track.id)?;

        //     let search_queries = vec![
        //         format!("{} - {}", album.artists[0].name, track.title),
        //         format!(
        //             "{} - {} - {}",
        //             album.artists[0].name, album.title, track.title
        //         ),
        //         format!("{} - {}", album.title, track.title),
        //     ];
        //     for search_query in search_queries {
        //         let mut search_process = std::process::Command::new("yt-dlp")
        //             .arg("--dump-json")
        //             .arg(format!("ytsearch25:{}", search_query))
        //             .stdout(std::process::Stdio::piped())
        //             .spawn()?;

        //         let stdout = search_process.stdout.as_mut().unwrap();
        //         let mut reader = std::io::BufReader::new(stdout);
        //         for line in reader.lines() {
        //             let video = serde_json::from_str::<Video>(&line?)?;

        //             if track.duration >= video.duration - TRACK_DURATION_SLACK
        //                 && track.duration <= video.duration + TRACK_DURATION_SLACK
        //             {
        //                 search_process.kill()?;

        //                 // Download video
        //                 let path = format!(
        //                     "{}/{}/{}/{} - {} - {} - {}.m4a",
        //                     output_path,
        //                     escape_path(album.artists[0].name),
        //                     escape_path(album.title),
        //                     escape_path(album.artists[0].name),
        //                     escape_path(album.title),
        //                     index + 1,
        //                     escape_path(track.title)
        //                 );
        //                 let mut download_process = std::process::Command::new("yt-dlp")
        //                     .arg("--newline")
        //                     .arg("-f")
        //                     .arg("bestaudio[ext=m4a]")
        //                     .arg(format!("https://www.youtube.com/watch?v={}", video.id))
        //                     .arg("-o")
        //                     .arg(&path)
        //                     .stdout(std::process::Stdio::piped())
        //                     .spawn()?;
        //                 download_process.wait()?;

        //                 // Update metadata

        //                 let mut tag = mp4ameta::Tag::read_from_path(path)?;
        //                 tag.set_title(track.title);
        //                 tag.set_album(album.title);
        //                 tag.set_artist(
        //                     album
        //                         .artists
        //                         .iter()
        //                         .map(|artist| artist.name.as_str())
        //                         .collect::<Vec<_>>()
        //                         .join(", "),
        //                 );
        //                 tag.set_album_artist(
        //                     album
        //                         .artists
        //                         .iter()
        //                         .map(|artist| artist.name.as_str())
        //                         .collect::<Vec<_>>()
        //                         .join(", "),
        //                 );
        //                 tag.set_year(album.release_date.split('-').next().unwrap().parse()?);

        //                 // tag.set_disc_number(disc_number);
        //                 // tag.set_track_number(index + 1);

        //                 tag.set_genre(
        //                     album
        //                         .genres
        //                         .data
        //                         .iter()
        //                         .map(|genre| genre.name.as_str())
        //                         .collect::<Vec<_>>()
        //                         .join(", "),
        //                 );
        //                 // tag.set_cover(
        //                 //     std::fs::read(&cover_file_path)?
        //                 //         .into_iter()
        //                 //         .map(|byte| byte as u8)
        //                 //         .collect(),
        //                 //     mp4ameta::ImageFormat::Jpeg,
        //                 // );
        //                 tag.write_to_path(path)?;

        //                 // file = mutagen.mp4.MP4(path)
        //                 // file['\xa9nam'] = track['title']
        //                 // file['\xa9alb'] = album['title']
        //                 // albumArtists = [ artist['name'] for artist in album['contributors'] ]
        //                 // file['\xa9ART'] = ', '.join(albumArtists + [ artist['name'] for artist in track['contributors'] if artist['name'] not in albumArtists ])
        //                 // file['aART'] = ', '.join(albumArtists)
        //                 // file['\xa9day'] = album['release_date'].split('-')[0]
        //                 // file['trkn'] = [ (index, album['nb_tracks']) ]
        //                 // file['\xa9gen'] = ', '.join([ genre['name'] for genre in album['genres']['data'] ])
        //                 // with open(coverFilePath, 'rb') as coverFile:
        //                 //     file['covr'] = [ mutagen.mp4.MP4Cover(coverFile.read(), imageformat=mutagen.mp4.MP4Cover.FORMAT_JPEG) ]
        //                 // file.save()

        //                 break;
        //             }
        //         }
        //     }
        // }

        // Ok(())
    }
}
