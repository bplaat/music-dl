#!/usr/bin/env python3
# AlbumDownloader - A Python script / tool that downloads complete albums via yt-dlp with the right metadata from the Deezer API
# Usage: ./dl.py "Ordinary Songs 3"
# Usage: ./dl.py "Ordinary Songs 3" --list
# Usage: ./dl.py "Ordinary Songs 3" -o ".." --list --cover

import argparse, json, math, os, subprocess, tempfile, sys, threading, urllib.parse, urllib.request, re
from mutagen.mp4 import MP4, MP4Cover

TRACK_DURATION_SLACK = 5

size = os.get_terminal_size()

def escapePath(path):
    path = path.replace('<', '_').replace('>', '_').replace(':', '_').replace('\"', '_').replace('/', '_')
    return path.replace('\\', '_').replace('|', '_').replace('?', '_').replace('*', '_')

cursorY = 0
def printLine(y, line):
    global cursorY
    if (y <= cursorY):
        sys.stdout.write('\033[F' * (cursorY - y))
    if (y > cursorY):
        sys.stdout.write('\033[E' * (y - cursorY))
    sys.stdout.write('\033[K' + line + '\n')
    cursorY = y + 1

def cut(string, width):
    if len(string) > width - 4:
        return string[:width - 4] + '... '
    return string

album = None
folderPath = None
coverFilePath = None
def downloadThread(index, track, searchAttempt, searchQuery):
    track = json.load(urllib.request.urlopen('https://api.deezer.com/track/' + str(track['id'])))

    # Search video
    with subprocess.Popen(['yt-dlp', '--dump-json', 'ytsearch25:' + searchQuery], stdout=subprocess.PIPE) as searchProcess:
        while searchProcess.poll() is None:
            line = searchProcess.stdout.readline().decode()
            try:
                videoJson = json.loads(line)
                if track['duration'] >= videoJson['duration'] - TRACK_DURATION_SLACK and track['duration'] <= videoJson['duration'] + TRACK_DURATION_SLACK:
                    searchProcess.terminate()

                    # Download video audio
                    _, temp_file_path = tempfile.mkstemp()
                    temp_file_path += '.m4a'
                    with subprocess.Popen(['yt-dlp', '--newline', '-f', 'bestaudio[ext=m4a]', 'https://www.youtube.com/watch?v=' + videoJson['id'], '-o', temp_file_path], stdout=subprocess.PIPE) as downloadProcess:
                        while downloadProcess.poll() is None:
                            line = downloadProcess.stdout.readline().decode()
                            procents = re.findall(r'[\d\.]+%', line)
                            if len(procents) > 0:
                                procent = float(procents[0][:-1]) / 100
                                width = math.floor(size.columns / 2) - 2
                                filled = math.ceil(width * procent)
                                leftColumn = ('%0' + str(len(str(album['nb_tracks']))) + 'd. %s (%d:%02d)') % (index, track['title'], track['duration'] / 60, track['duration'] % 60)
                                middleColumn = 'Downloading video...'
                                printLine(index, '%s%s%s%s[%s%s]' % (
                                    cut(leftColumn, math.ceil(size.columns / 4)), ' ' * (math.ceil(size.columns / 4) - len(leftColumn)),
                                    cut(middleColumn, math.floor(size.columns / 4)), ' ' * (math.floor(size.columns / 4) - len(middleColumn)),
                                    '#' * filled, '-' * (width - filled)
                                ))

                    # Correct video metadata
                    file = MP4(temp_file_path)
                    file['\xa9nam'] = track['title']
                    file['\xa9alb'] = album['title']
                    albumArtists = [ artist['name'] for artist in album['contributors'] ]
                    file['\xa9ART'] = ', '.join(albumArtists + [ artist['name'] for artist in track['contributors'] if artist['name'] not in albumArtists ])
                    file['aART'] = ', '.join(albumArtists)
                    file['\xa9day'] = album['release_date'].split('-')[0]
                    file['trkn'] = [ (index, album['nb_tracks']) ]
                    file['\xa9gen'] = ', '.join([ genre['name'] for genre in album['genres']['data'] ])
                    with open(coverFilePath, 'rb') as coverFile:
                        file["covr"] = [ MP4Cover(coverFile.read(), imageformat=MP4Cover.FORMAT_JPEG) ]
                    file.save()

                    # Rename / move video to right path
                    os.rename(temp_file_path, folderPath + '/' + escapePath(album['artist']['name'] + ' - ' + album['title'] + ' - ' + (('%0' + str(len(str(album['nb_tracks']))) + 'd') % track['track_position']) + ' - ' + track['title'] + '.m4a'))

                    leftColumn = ('%0' + str(len(str(album['nb_tracks']))) + 'd. %s (%d:%02d)') % (index, track['title'], track['duration'] / 60, track['duration'] % 60)
                    middleColumn = 'Done'
                    printLine(index, '%s%s%s%s[%s]' % (
                        cut(leftColumn, math.ceil(size.columns / 4)), ' ' * (math.ceil(size.columns / 4) - len(leftColumn)),
                        cut(middleColumn, math.floor(size.columns / 4)), ' ' * (math.floor(size.columns / 4) - len(middleColumn)),
                        '#' * (math.floor(size.columns / 2) - 2)
                    ))
                    break
            except:
                searchProcess.terminate()

                # Try another search query or fail
                if searchAttempt == 1:
                    downloadThread(index, track, 2, album['artist']['name'] + ' - ' + album['title'] + ' - ' + track['title'])
                elif searchAttempt == 2:
                    downloadThread(index, track, 3, album['title'] + ' - ' + track['title'])
                else:
                    leftColumn = ('%0' + str(len(str(album['nb_tracks']))) + 'd. %s (%d:%02d)') % (index, track['title'], track['duration'] / 60, track['duration'] % 60)
                    middleColumn = 'Can\'t find video'
                    printLine(index, '%s%s%s%s[%s]' % (
                        cut(leftColumn, math.ceil(size.columns / 4)), ' ' * (math.ceil(size.columns / 4) - len(leftColumn)),
                        cut(middleColumn, math.floor(size.columns / 4)), ' ' * (math.floor(size.columns / 4) - len(middleColumn)),
                        '-' * (math.floor(size.columns / 2) - 2)
                    ))
                return

def main():
    global album, folderPath, coverFilePath

    # Parse arguments
    parser = argparse.ArgumentParser(description='A Python script / tool that downloads complete albums via the YouTube API and yt-dlp with the right metadata via the Deezer API')
    parser.add_argument('album')
    parser.add_argument('-o', '--output', default=os.path.expanduser('~') + '/Music', help='The output directory')
    parser.add_argument('-l', '--list', action='store_true', help='Just list the music metadata')
    parser.add_argument('-c', '--cover', action='store_true', help='Save album cover as a seperated file')
    args = parser.parse_args()

    # Search for album with Deezer API
    albums = json.load(urllib.request.urlopen('https://api.deezer.com/search/album?q=' + urllib.parse.quote_plus(args.album)))['data']
    if len(albums) == 0:
        print('No album found!')
        exit()

    # When album is found create folder
    album = json.load(urllib.request.urlopen('https://api.deezer.com/album/' + str(albums[0]['id'])))
    albumArtists = [ artist['name'] for artist in album['contributors'] ]
    if args.list:
        print('# ' + album['title'] + ' by ' + ', '.join(albumArtists))
        print('Released at ' + album['release_date'] + ' with ' + str(album['nb_tracks']) + ' tracks')
        index = 1
        for track in album['tracks']['data']:
            track = json.load(urllib.request.urlopen('https://api.deezer.com/track/' + str(track['id'])))
            trackArtists = ', '.join(albumArtists + [ artist['name'] for artist in track['contributors'] if artist['name'] not in albumArtists ])
            print('%d. %s (%d:%02d) by %s' % (index, track['title'], track['duration'] / 60, track['duration'] % 60, trackArtists))
            index += 1
        exit()

    # Create album download folder
    folderPath = args.output + '/' + escapePath(album['artist']['name'] + ' - ' + album['title'])
    os.makedirs(folderPath, exist_ok=True)

    # Download album cover
    if args.cover:
        coverFilePath = folderPath + '/cover.jpg'
    else:
        _, coverFilePath = tempfile.mkstemp()
    urllib.request.urlretrieve(album['cover_xl'], coverFilePath)

    # Create download threads
    printLine(0, '# ' + album['title'] + ' by ' + ', '.join(albumArtists))
    index = 1
    for track in album['tracks']['data']:
        leftColumn = ('%0' + str(len(str(album['nb_tracks']))) + 'd. %s (%d:%02d)') % (index, track['title'], track['duration'] / 60, track['duration'] % 60)
        middleColumn = 'Searching video...'
        printLine(index, '%s%s%s%s[%s]' % (
            cut(leftColumn, math.ceil(size.columns / 4)), ' ' * (math.ceil(size.columns / 4) - len(leftColumn)),
            cut(middleColumn, math.floor(size.columns / 4)), ' ' * (math.floor(size.columns / 4) - len(middleColumn)),
            '-' * (math.floor(size.columns / 2) - 2)
        ))

        thread = threading.Thread(target=downloadThread, args=[index, track, 1, album['artist']['name'] + ' - ' + track['title']])
        thread.start()

        index += 1

if __name__ == '__main__':
    main()
