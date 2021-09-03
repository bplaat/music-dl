#!/bin/env python
# AlbumDownloader - A Python script / tool that downloads complete albums via YouTube with the right metadata via the Deezer API
# Usage: ./dl.py "Ordinary Songs 3"
# Usage: ./dl.py "Ordinary Songs 3" --list

import config, json, mutagen, os, sys, threading, urllib.parse, urllib.request

def escapePath(path):
    path = path.replace('<', '_').replace('>', '_').replace(':', '_').replace('\"', '_').replace('/', '_')
    return path.replace('\\', '_').replace('|', '_').replace('?', '_').replace('*', '_')

trackQueue = None
def downloadThread(name):
    print('Download thread ' + name + ' starting...')
    currentYouTubeApiKeyIndex = 0
    while len(trackQueue) > 0:
        track = trackQueue.pop(0)
        track = json.load(urllib.request.urlopen('https://api.deezer.com/track/' + str(track['id'])))

        # Search for YouTube video's for track
        tryAgain = True
        pageToken = None
        while tryAgain:
            try:
                print('Search for YouTube video with query: ' + album['artist']['name'] + ' - ' + track['title'] + (pageToken != None and ' - ' + pageToken or ''))
                videos = json.load(urllib.request.urlopen('https://youtube.googleapis.com/youtube/v3/search?part=snippet&maxResults=50&type=video&q=' + urllib.parse.quote_plus(album['artist']['name'] + ' - ' + track['title']) +
                    (pageToken != None and '&pageToken=' + pageToken or '') + '&key=' + config.YOUTUBE_API_KEYS[currentYouTubeApiKeyIndex]))
                for video in videos['items']:
                    # Check video duration to be more or less equal to real track duration
                    video = json.load(urllib.request.urlopen('https://youtube.googleapis.com/youtube/v3/videos?part=snippet,contentDetails&id=' + video['id']['videoId'] + '&key=' + config.YOUTUBE_API_KEYS[currentYouTubeApiKeyIndex]))['items'][0]
                    duration = video['contentDetails']['duration'][2:]
                    seconds = int(duration.split('M')[0]) * 60 + int(duration[:-1].split('M')[1])
                    print('Found video of ' + str(seconds) + ' seconds must be ' + str(track['duration']) + ' seconds')
                    if track['duration'] >= seconds - config.TRACK_DURATION_SLACK and track['duration'] <= seconds + config.TRACK_DURATION_SLACK:
                        # Download track video with youtube-dl add correct metadata and rename to right name
                        os.system('youtube-dl --newline -f bestaudio[ext=m4a] -o temp' + name + '.m4a ' + video['id'])
                        file = mutagen.File('temp' + name + '.m4a')
                        file['\xa9nam'] = track['title']
                        file['\xa9alb'] = album['title']
                        file['\xa9ART'] = ', '.join([ artist['name'] for artist in track['contributors']])
                        file['aART'] = albumArtists
                        file['\xa9day'] = album['release_date'].split('-')[0]
                        file['trkn'] = [(track['track_position'], album['nb_tracks'])]
                        file.save()
                        os.rename('temp' + name + '.m4a', folder + '/' + folder + ' - ' + str(track['track_position']) + ' - ' + escapePath(track['title']) + '.m4a')
                        tryAgain = False
                        break
                pageToken = videos['nextPageToken']
            except:
                if currentYouTubeApiKeyIndex == len(config.YOUTUBE_API_KEYS) - 1:
                    print('Out of YouTube API keys!')
                    exit(1)
                else:
                    print('YouTube API key ' + str(currentYouTubeApiKeyIndex) + ' is used up go to next one...')
                    currentYouTubeApiKeyIndex += 1
    print('Download thread ' + name + ' finished...')

if len(sys.argv) >= 2:
    justList = False
    if len(sys.argv) >= 3 and sys.argv[2] == '--list':
        justList = True

    # Search for album with Deezer API
    albums = json.load(urllib.request.urlopen('https://api.deezer.com/search/album?q=' + urllib.parse.quote_plus(sys.argv[1])))['data']
    if len(albums) > 0:
        # When album is found create folder and download cover image
        album = json.load(urllib.request.urlopen('https://api.deezer.com/album/' + str(albums[0]['id'])))
        albumArtists = ', '.join([ artist['name'] for artist in album['contributors']])
        if justList:
            print('# ' + album['title'] + ' by ' + albumArtists)
            print('Released at ' + album['release_date'] + ' with ' + str(album['nb_tracks']) + ' tracks')
            for track in album['tracks']['data']:
                track = json.load(urllib.request.urlopen('https://api.deezer.com/track/' + str(track['id'])))
                trackArtists = ', '.join([ artist['name'] for artist in track['contributors']])
                print('%d. %s (%d:%02d) by %s' % (track['track_position'], track['title'], track['duration'] / 60, track['duration'] % 60, trackArtists))
        else:
            folder = album['artist']['name'] + ' - ' + album['title']
            os.makedirs(folder, exist_ok=True)
            urllib.request.urlretrieve(album['cover_xl'], folder + '/cover.jpg')

            # Create download threads and wait
            trackQueue = album['tracks']['data']
            for i in range(config.DOWNLOAD_THREAD_COUNT):
                thread = threading.Thread(target=downloadThread, args=(str(i),))
                thread.start()
    else:
        print('No album found!')
else:
    print('No album name given!')
