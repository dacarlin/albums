use iced::widget::{button, column, container, row, text};
use iced::{executor, Application, Command, Element, Settings, Theme};
use std::path::PathBuf;

mod library;
mod player;

use library::{Library, Song};
use player::Player;

pub fn main() -> iced::Result {
    Mp3Player::run(Settings::default())
}

struct Mp3Player {
    library: Library,
    player: Player,
    current_song: Option<Song>,
    play_pause_text: String,
}

#[derive(Debug, Clone)]
enum Message {
    PlayPause,
    Stop,
}

impl Application for Mp3Player {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Mp3Player, Command<Message>) {
        let music_dir = PathBuf::from("data");
        let mut library = Library::new(&music_dir).expect("Failed to create library");

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

        let player = Player::new().expect("Failed to create player");

        (
            Mp3Player {
                library,
                player,
                current_song: None,
                play_pause_text: "Play".to_string(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("MP3 Player")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PlayPause => {
                if self.player.is_paused() {
                    if let Some(song) = &self.current_song {
                        self.player.resume();
                        self.play_pause_text = "Pause".to_string();
                    } else if let Some(first_song) = self.library.get_sorted_songs().first() {
                        println!("ho!");
                        self.player
                            .play(first_song.file_path.to_str().unwrap())
                            .expect("Failed to play song");
                        self.current_song = Some(first_song.clone().clone());
                        self.play_pause_text = "Pause".to_string();
                    }
                } else {
                    self.player.pause();
                    self.play_pause_text = "Play".to_string();
                }
            }
            Message::Stop => {
                self.player.stop();
                self.current_song = None;
                self.play_pause_text = "Play".to_string();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let play_pause_button = button(text(&self.play_pause_text)).on_press(Message::PlayPause);
        let stop_button = button(text("Stop")).on_press(Message::Stop);

        let controls = row![play_pause_button, stop_button].spacing(10);

        let current_song_text = if let Some(song) = &self.current_song {
            format!("Now playing: {} - {}", song.artist, song.title)
        } else {
            "No song playing".to_string()
        };

        let content = column![
            text(current_song_text),
            controls,
        ]
        .spacing(20);

        container(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}