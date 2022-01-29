package main

import (
	"encoding/json"
	"fmt"
	"flag"
	"io"
	"io/ioutil"
	"log"
	"net/http"
	"net/url"
	"os"
	"strings"
	"time"
	"github.com/nsf/termbox-go"
)

func fetch(url string) []byte {
	resp, err := http.Get(url)
	if err != nil { log.Fatalln(err) }
	defer resp.Body.Close()

	body, err := ioutil.ReadAll(resp.Body)
	if err != nil { log.Fatalln(err) }
	return body
}

func downloadFile(url string, path string) {
	file, err := os.Create(path)
	if err != nil { log.Fatalln(err) }
	defer file.Close()

	resp, err := http.Get(url)
	if err != nil { log.Fatalln(err) }
	defer resp.Body.Close()

	_, err = io.Copy(file, resp.Body)
	if err != nil { log.Fatalln(err) }
}

func downloadAlbum(id int) {
	// Fetch album
	album := DeezerAlbum{}
	err := json.Unmarshal(fetch(fmt.Sprintf("https://api.deezer.com/album/%d", id)), &album)
	if err != nil { log.Fatalln(err) }

	// Create output folders
	err = os.Mkdir("out", 0755)
	// if err != nil { log.Fatalln(err) }
	err = os.Mkdir("out/" + album.Artist.Name, 0755)
	// if err != nil { log.Fatalln(err) }
	err = os.Mkdir("out/" + album.Artist.Name + "/" + album.Title, 0755)
	// if err != nil { log.Fatalln(err) }

	// Download album cover
	downloadFile(album.CoverXl,  "out/" + album.Artist.Name + "/" + album.Title + "/cover.jpg")

	err = termbox.Init()
	if err != nil { log.Fatalln(err) }
	defer termbox.Close()

	termbox.HideCursor()

	width, height := termbox.Size()

	termbox.SetChar(width / 2, height / 2, 'H')
	termbox.Flush()

	time.Sleep(time.Second)
}

func listAlbum(id int) {
	// Fetch album
	album := DeezerAlbum{}
	err := json.Unmarshal(fetch(fmt.Sprintf("https://api.deezer.com/album/%d", id)), &album)
	if err != nil { log.Fatalln(err) }

	// Print album stats
	albumArtists := strings.Builder{}
	for i, artist := range album.Contributors {
		albumArtists.WriteString(artist.Name)
		if i < len(album.Contributors) - 1 {
			albumArtists.WriteString(", ")
		}
	}
	fmt.Printf("# %s by %s\n", album.Title, albumArtists.String())
	fmt.Printf("Released at %s with %d tracks\n", album.ReleaseDate, album.NbTracks)

	// Print album track stats
	for i, trackIncomplete := range album.Tracks.Data {
		track := DeezerTrack{}
		err := json.Unmarshal(fetch(fmt.Sprintf("https://api.deezer.com/track/%d", trackIncomplete.ID)), &track)
		if err != nil { log.Fatalln(err) }

		trackArtists := strings.Builder{}
		for i, artist := range track.Contributors {
			trackArtists.WriteString(artist.Name)
			if i < len(track.Contributors) - 1 {
				trackArtists.WriteString(", ")
			}
		}
		fmt.Printf("%d. %s (%d:%02d) by %s\n", i + 1, track.Title, track.Duration / 60, track.Duration % 60, trackArtists.String())
	}
}

func main() {
	// Parse args
	var list bool
	flag.BoolVar(&list, "l", false, "Just list / print the music metadata")
	flag.Parse()

	values := flag.Args()
	if len(values) == 0 {
		fmt.Println("Usage: albumdownloader [options] query")
		flag.PrintDefaults()
		return
	}
	query := values[0]

	// Search for album
	albums := DeezerAlbumsSearch{}
	err := json.Unmarshal(fetch("https://api.deezer.com/search/album?q=" + url.QueryEscape(query)), &albums)
	if err != nil { log.Fatalln(err) }

	// Do something with album
	if list {
		listAlbum(albums.Data[0].ID)
	} else {
		downloadAlbum(albums.Data[0].ID)
	}
}
