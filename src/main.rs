use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use id3::{Tag, TagLike};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct Song {
    file_path: PathBuf,
    artist: String,
    album: String,
    title: String,
    track_number: Option<u32>,
    duration: Option<u32>,
    genre: Option<String>,
    year: Option<i32>,
}

struct Library {
    songs: HashMap<PathBuf, Song>,
    song_list: Vec<PathBuf>,
}

impl Library {
    fn new() -> Self {
        Library {
            songs: HashMap::new(),
            song_list: Vec::new(),
        }
    }

    fn add_song(&mut self, song: Song) {
        let path = song.file_path.clone();
        self.songs.insert(path.clone(), song);
        self.song_list.push(path);
    }

    fn search_by_title(&self, query: &str) -> Vec<&Song> {
        self.songs
            .values()
            .filter(|song| song.title.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    fn sort_by_album(&mut self) {
        self.song_list.sort_by(|a, b| {
            let song_a = self.songs.get(a).unwrap();
            let song_b = self.songs.get(b).unwrap();
            song_a.album.cmp(&song_b.album)
                .then(song_a.track_number.cmp(&song_b.track_number))
        });
    }

    fn get_sorted_songs(&self) -> Vec<&Song> {
        self.song_list.iter().map(|path| self.songs.get(path).unwrap()).collect()
    }
}

fn parse_mp3_directory(dir_path: &Path) -> Result<Library, Box<dyn std::error::Error>> {
    let mut library = Library::new();

    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "mp3") {
            if let Ok(tag) = Tag::read_from_path(path) {
                let song = Song {
                    file_path: path.to_path_buf(),
                    artist: tag.artist().unwrap_or("Unknown Artist").to_string(),
                    album: tag.album().unwrap_or("Unknown Album").to_string(),
                    title: tag.title().unwrap_or("Unknown Title").to_string(),
                    track_number: tag.track(),
                    duration: None, // ID3 doesn't provide duration, we'll need to use another crate for this
                    genre: tag.genre().map(String::from),
                    year: tag.year(),
                };
                library.add_song(song);
            }
        }
    }

    Ok(library)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let music_dir = Path::new("data");
    let mut library = parse_mp3_directory(music_dir)?;
    
    println!("Parsed {} songs", library.songs.len());

    // Example of searching by title
    let search_query = "love";
    let search_results = library.search_by_title(search_query);
    println!("Search results for '{}': ", search_query);
    for song in search_results {
        println!("- {} by {}", song.title, song.artist);
    }

    // Example of sorting by album
    library.sort_by_album();
    println!("\nSongs sorted by album:");
    for song in library.get_sorted_songs() {
        println!("- {} - {} (Album: {})", song.artist, song.title, song.album);
    }
    
    Ok(())
}