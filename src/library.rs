use std::path::{Path, PathBuf};
use std::collections::HashMap;
use id3::{Tag, TagLike};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Song {
    pub file_path: PathBuf,
    pub artist: String,
    pub album: String,
    pub title: String,
    pub track_number: Option<u32>,
}

pub struct Library {
    pub songs: HashMap<PathBuf, Song>,
    pub song_list: Vec<PathBuf>,
}

impl Library {
    pub fn new(dir_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut library = Library {
            songs: HashMap::new(),
            song_list: Vec::new(),
        };

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
                    };
                    library.add_song(song);
                }
            }
        }

        Ok(library)
    }

    fn add_song(&mut self, song: Song) {
        let path = song.file_path.clone();
        self.songs.insert(path.clone(), song);
        self.song_list.push(path);
    }

    pub fn get_sorted_songs(&self) -> Vec<&Song> {
        self.song_list.iter().map(|path| self.songs.get(path).unwrap()).collect()
    }

    pub fn search_by_title(&self, query: &str) -> Vec<&Song> {
        self.songs
            .values()
            .filter(|song| song.title.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    pub fn sort_by_album(&mut self) {
        self.song_list.sort_by(|a, b| {
            let song_a = self.songs.get(a).unwrap();
            let song_b = self.songs.get(b).unwrap();
            song_a.album.cmp(&song_b.album)
                .then(song_a.track_number.cmp(&song_b.track_number))
        });
    }
}