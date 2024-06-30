/*
 * Copyright (c) 2024, Bastiaan van der Plaat <bastiaan.v.d.plaat@gmail.com>
 *
 * SPDX-License-Identifier: MIT
 */

use anyhow::Result;
use rust_embed::Embed;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::{
    http::{self, header::CONTENT_TYPE, Request, Response},
    WebViewBuilder,
};

use crate::structs::deezer::{Album, AlbumList, Artist, ArtistList};

#[derive(Embed)]
#[folder = "assets/"]
struct Assets;

enum UserEvent {}

pub fn start_window() -> Result<()> {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();

    let window = WindowBuilder::new()
        .with_title("Music Downloader")
        .with_inner_size(tao::dpi::LogicalSize::new(1280, 720))
        .build(&event_loop)?;

    let proxy = event_loop.create_proxy();
    let handler = move |req: Request<String>| {
        let body = req.body();
        match body.as_str() {
            _ if body.starts_with("search") => {
                let query = body.replace("search:", "");

                // Start search task in thread

                // _ = proxy.send_event(UserEvent::Search(query));
            }
            _ => {}
        }
    };

    let webview = WebViewBuilder::new(&window)
        .with_custom_protocol("wry".into(), move |request| {
            match get_wry_response(request) {
                Ok(r) => r.map(Into::into),
                Err(e) => http::Response::builder()
                    .header(CONTENT_TYPE, "text/plain")
                    .status(500)
                    .body(e.to_string().as_bytes().to_vec())
                    .unwrap()
                    .map(Into::into),
            }
        })
        .with_ipc_handler(handler)
        .with_new_window_req_handler(|url| {
            webbrowser::open(&url).expect("Can't open url in browser");
            false
        })
        .with_url("wry://localhost/")
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            // Event::UserEvent(UserEvent::Search(query)) => {
            //     _ = webview.evaluate_script("window.app.searchResult('hoi')");
            // }
            _ => {}
        }
    });
}

fn get_wry_response(request: Request<Vec<u8>>) -> Result<http::Response<Vec<u8>>> {
    let path = request.uri().path();
    let path = if path == "/" {
        "index.html"
    } else {
        &path[1..]
    };

    let mimetype = if path.ends_with(".html") || path == "/" {
        "text/html"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".js") {
        "application/javascript"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else {
        unimplemented!();
    };

    let content = Assets::get(path).ok_or_else(|| anyhow::anyhow!("Asset not found: {}", path))?;

    Response::builder()
        .header(CONTENT_TYPE, mimetype)
        .body(content.data.to_vec())
        .map_err(Into::into)
}
