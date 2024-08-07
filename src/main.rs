use iced::widget::{button, column, container, row, text, scrollable, slider, Text};
use iced::{executor, theme, Application, Command, Element, Length, Settings, Theme};
use iced::widget::scrollable::Properties;
use std::path::PathBuf;

mod library;
mod player;

use library::{Library, Song};
use player::Player;

pub fn main() -> iced::Result {
    MusicPlayer::run(Settings::default())
}

struct MusicPlayer {
    library: Library,
    player: Player,
    current_song: Option<Song>,
    play_pause_text: String,
    volume: f32,
    selected_song_index: Option<usize>,
}

#[derive(Debug, Clone)]
enum Message {
    PlayPause,
    Stop,
    SongSelected(usize),
    VolumeChanged(f32),
}

impl Application for MusicPlayer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (MusicPlayer, Command<Message>) {
        let music_dir = PathBuf::from("data");
        let library = Library::new(&music_dir).expect("Failed to create library");
        let player = Player::new().expect("Failed to create player");

        (
            MusicPlayer {
                library,
                player,
                current_song: None,
                play_pause_text: "Play".to_string(),
                volume: 69.,
                selected_song_index: None,
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
                    } else if let Some(index) = self.selected_song_index {
                        if let Some(song) = self.library.get_sorted_songs().get(index) {
                            self.player
                                .play(song.file_path.to_str().unwrap())
                                .expect("Failed to play song");
                            self.current_song = Some(song.clone().clone());
                            self.play_pause_text = "Pause".to_string();
                        }
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
            Message::SongSelected(index) => {
                self.selected_song_index = Some(index);
                if let Some(song) = self.library.get_sorted_songs().get(index) {
                    self.player.stop();
                    self.player
                        .play(song.file_path.to_str().unwrap())
                        .expect("Failed to play song");
                    self.current_song = Some(song.clone().clone());
                    self.play_pause_text = "Pause".to_string();
                }
            }
            Message::VolumeChanged(new_volume) => {
                self.volume = new_volume;
                self.player.set_volume(new_volume / 100.);
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

        let volume_slider = slider(0.0..=100.0, self.volume, Message::VolumeChanged);
        let volume_text = text(format!("Volume: {:.0}%", self.volume));
        let volume_control = row![volume_text, volume_slider].spacing(20);

        let song_list = self.library.get_sorted_songs().iter().enumerate().fold(
            column![].spacing(9),
            |column, (i, song)| {
                column.push(
                    button(row!(
                        text(format!("{} - {}", song.artist, song.title)), 
                        text(format!("{} - {}", song.artist, song.title))
                    ))
                        .on_press(Message::SongSelected(i))
                        .style(if Some(i) == self.selected_song_index {
                            theme::Button::Primary
                        } else {
                            theme::Button::Secondary
                        }),
                )
            },
        );

        let content = column![
            //Text::new("Player").size(40),
            text(current_song_text),
            controls,
            volume_control,
            scrollable(song_list)
                .height(Length::FillPortion(3))
                .width(Length::Fill)
                .style(theme::Scrollable::Default) 
        ]
        .spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .padding(20)
            .into()
    }
}