# BassieBAS's Album Downloader
A Python script / tool that downloads complete albums via the YouTube API and youtube-dl with the right metadata via the Deezer API

## Installation
You need to install the `mutagen` Python package:

```
pip install mutagen
```

Copy the `config.py.example` file to `config.py`:

```
cp config.py.example config.py
```

Generate YouTube Data v3 API keys add this to the `config.py` file

## Usage
You can search and list album info with the `--list` argument:

```
./dl.py "Ordinary Songs 3" --list
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

You can download an album without the `--list` argument and with the `--cover` argument you can also download the cover:

```
./dl.py "Ordinary Songs 3"
```
