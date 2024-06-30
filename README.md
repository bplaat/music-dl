# Music Downloader
<div>
<img align="left" src="assets/icon.svg" width="96" height="96" />
<br/>
<p>
    A tool that can download complete albums from YouTube videos with Deezer metadata
</p>
<br/>
</div>

## Features
- Search albums, artists and tracks from Deezer
- Automatically find the right YouTube music videos
- Download them and set the correct metadata tags
- Then play them in your favorite music player!

## Screenshot
*TODO*

## Getting Started
- Install [yt-dlp](https://github.com/yt-dlp/yt-dlp#installation) and [ffmpeg](https://ffmpeg.org/download.html)
- Install the Rust toolchain with [rustup](https://rustup.rs/)
- Run `./meta.sh run`

## Command Line Interface
The app also has a cli interface, it because available when you provide at least one argument. Search and list album info with the `--list` argument:
```
./music-dl --list "Snail's House - Ordinary Songs 3"
```

Which returns the following output:
```
# Ordinary Songs 3 by Snail's House
Released at 2017-06-15 with 5 tracks
1. Good Day (2:36) by Snail's House
2. Bouquet (2:28) by Snail's House
3. Aloha (3:04) by Snail's House
4. あめあがりのうた (2:37) by Snail's House
5. Lullaby (3:31) by Snail's House
```

Download an album without the `--list` argument:
```
./music-dl "Snail's House - Ordinary Songs 3"
```

You can download all albums and EP's from a artist by using the `--artist` argument, you could use the `--singles` option to download also all its singles:
```
./music-dl "Snail's House - Ordinary Songs 3" --artist --singles
```

## License
Copyright &copy; 2021 - 2024, [Bastiaan van der Plaat](https://bplaat.nl/)

Licensed under the [MIT](LICENSE) license.
