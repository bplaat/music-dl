#!/bin/env python
# AlbumDownloader - A Python script / tool that downloads complete albums via YouTube with the right metadata via the Deezer API
# Usage: ./dl.py "Ordinary Songs 3"
# Usage: ./dl.py "Ordinary Songs 3" --list
# Usage: ./dl.py "Ordinary Songs 3" -o ".." --list

import argparse, config, json, mutagen, os, tempfile, threading, urllib.parse, urllib.request, re

def escapePath(path):
    path = path.replace('<', '_').replace('>', '_').replace(':', '_').replace('\"', '_').replace('/', '_')
    return path.replace('\\', '_').replace('|', '_').replace('?', '_').replace('*', '_')

def parseDuration(duration):
    seconds = 0

    hoursMatch = re.search(r'\d+H', duration)
    if hoursMatch != None:
        seconds += int(duration[hoursMatch.start():hoursMatch.end() - 1]) * 3600

    minutesMatch = re.search(r'\d+M', duration)
    if minutesMatch != None:
        seconds += int(duration[minutesMatch.start():minutesMatch.end() - 1]) * 60

    secondsMatch = re.search(r'\d+S', duration)
    if secondsMatch != None:
        seconds += int(duration[secondsMatch.start():secondsMatch.end() - 1])

    return seconds

args = None
album = None
albumArtists = None
folder = None
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
                print('Search for YouTube video with query: ' + album['artist']['name'] + ' - ' + album['title'] + ' - ' + track['title'] + (pageToken != None and ' - ' + pageToken or ''))
                videos = json.load(urllib.request.urlopen('https://youtube.googleapis.com/youtube/v3/search?part=snippet&maxResults=50&type=video&q=' + urllib.parse.quote_plus(album['artist']['name'] + ' - ' + album['title'] + ' - ' + track['title']) +
                    (pageToken != None and '&pageToken=' + pageToken or '') + '&key=' + config.YOUTUBE_API_KEYS[currentYouTubeApiKeyIndex]))
                for video in videos['items']:
                    # Check video duration to be more or less equal to real track duration
                    video = json.load(urllib.request.urlopen('https://youtube.googleapis.com/youtube/v3/videos?part=snippet,contentDetails&id=' + video['id']['videoId'] + '&key=' + config.YOUTUBE_API_KEYS[currentYouTubeApiKeyIndex]))['items'][0]
                    seconds = parseDuration(video['contentDetails']['duration'])
                    print('Found video of ' + str(seconds) + ' seconds must be ' + str(track['duration']) + ' seconds')
                    if track['duration'] >= seconds - config.TRACK_DURATION_SLACK and track['duration'] <= seconds + config.TRACK_DURATION_SLACK:
                        # Download track video with youtube-dl add correct metadata and rename to right name
                        _, temp_file_path = tempfile.mkstemp()
                        temp_file_path += '.m4a'
                        os.system('youtube-dl --newline -f bestaudio[ext=m4a] -o "' + temp_file_path + '" "https://www.youtube.com/watch?v=' + video['id'] + '"')
                        file = mutagen.File(temp_file_path)
                        file['\xa9nam'] = track['title']
                        file['\xa9alb'] = album['title']
                        file['\xa9ART'] = ', '.join([ artist['name'] for artist in track['contributors']])
                        file['aART'] = albumArtists
                        file['\xa9day'] = album['release_date'].split('-')[0]
                        file['trkn'] = [(track['track_position'], album['nb_tracks'])]
                        file.save()
                        os.rename(temp_file_path, folder + '/' + album['artist']['name'] + ' - ' + album['title'] + ' - ' + str(track['track_position']) + ' - ' + escapePath(track['title']) + '.m4a')
                        tryAgain = False
                        break
                pageToken = videos['nextPageToken']
            except urllib.error.HTTPError:
                if currentYouTubeApiKeyIndex == len(config.YOUTUBE_API_KEYS) - 1:
                    print('Out of YouTube API keys!')
                    exit(1)
                else:
                    print('YouTube API key ' + str(currentYouTubeApiKeyIndex) + ' is used up go to next one...')
                    currentYouTubeApiKeyIndex += 1
            except Exception as e:
                print(e)
    print('Download thread ' + name + ' finished...')

def main():
    global album, albumArtists, args, folder, trackQueue

    # Parse arguments
    parser = argparse.ArgumentParser(description='A Python script / tool that downloads complete albums via the YouTube API and youtube-dl with the right metadata via the Deezer API')
    parser.add_argument('album')
    parser.add_argument('-o', '--output', default=os.path.expanduser('~') + '/Music', help='The output directory')
    parser.add_argument('-l', '--list', action='store_true', help='Just list the music metadata')

    args = parser.parse_args()

    # Search for album with Deezer API
    albums = json.load(urllib.request.urlopen('https://api.deezer.com/search/album?q=' + urllib.parse.quote_plus(args.album)))['data']
    if len(albums) > 0:
        # When album is found create folder and download cover image
        album = json.load(urllib.request.urlopen('https://api.deezer.com/album/' + str(albums[0]['id'])))
        albumArtists = ', '.join([ artist['name'] for artist in album['contributors']])
        if args.list:
            print('# ' + album['title'] + ' by ' + albumArtists)
            print('Released at ' + album['release_date'] + ' with ' + str(album['nb_tracks']) + ' tracks')
            for track in album['tracks']['data']:
                track = json.load(urllib.request.urlopen('https://api.deezer.com/track/' + str(track['id'])))
                trackArtists = ', '.join([ artist['name'] for artist in track['contributors']])
                print('%d. %s (%d:%02d) by %s' % (track['track_position'], track['title'], track['duration'] / 60, track['duration'] % 60, trackArtists))
        else:
            folder = args.output + '/' + album['artist']['name'] + ' - ' + album['title']
            os.makedirs(folder, exist_ok=True)
            urllib.request.urlretrieve(album['cover_xl'], folder + '/cover.jpg')

            # Create download threads and wait
            trackQueue = album['tracks']['data']
            for i in range(config.DOWNLOAD_THREAD_COUNT):
                thread = threading.Thread(target=downloadThread, args=(str(i),))
                thread.start()
    else:
        print('No album found!')

if __name__ == "__main__":
    main()
