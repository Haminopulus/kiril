use std::{collections::VecDeque, io::{stdout, Write}, time::Duration};

use async_std::task;
use mpris::{Metadata, PlaybackStatus, Player, PlayerFinder, TrackID};
use tokio::{self};
mod lrc;

type TimeTag = (Duration, String);
type Lyric = VecDeque<TimeTag>;

/// Wait for active player to be found and return it
async fn get_active_player(retry_dur: Duration) -> Player {
    let finder: PlayerFinder = PlayerFinder::new().expect("DBusError");
    loop {
        let player = match finder.find_active() {
            Ok(pl) => pl,
            Err(_) => {
                task::sleep(retry_dur).await;
                continue;
            },
        };
        return player;
    }
}


#[tokio::main]
async fn main() {
    let retry_dur = Duration::from_secs(2);
    let mut playback_status: PlaybackStatus;

    loop {
        let player: Player = get_active_player(retry_dur).await;
        let mut lyrics: VecDeque<Lyric> = VecDeque::new();
        let mut current_line: Lyric = VecDeque::new();
        let mut previous_pos = Duration::default();
        let mut current_pos: Duration;
        let mut previous_song: Option<TrackID> = None;
        let mut current_song: Option<TrackID>;
        let mut previous_word: TimeTag = (Duration::default(), String::default());
        let mut current_word: TimeTag = (Duration::default(), String::default());

        loop {
            if !player.is_running() {
                break;
            }

            let metadata: Metadata = match player.get_metadata() {
                Ok(met) => met,
                Err(_) => break
            };

            current_song = metadata.track_id();
            current_pos = match player.get_position() {
                Ok(pos) => pos,
                Err(_) => break
                };

            if (previous_song != current_song) || (previous_pos > current_pos) {
                current_line = VecDeque::new();
                current_word = (Duration::default(), String::default());
                previous_word = (Duration::default(), String::default());
                previous_song = current_song;
                lyrics = match lrc::get_lyrics(&metadata) {
                    Some(lrcs) => lrcs,
                    None => {println!("No Lyrics!"); VecDeque::new()}
                };
            }

            previous_pos = current_pos;

            playback_status = match player.get_playback_status() {
                Ok(status) => status,
                Err(_) => break
            };
            match playback_status {
                PlaybackStatus::Stopped | PlaybackStatus::Paused => {
                    task::sleep(retry_dur).await;
                    continue;
                }
                PlaybackStatus::Playing => {
                    let word: TimeTag;
                    if !lyrics.is_empty() {
                        if current_line.is_empty() {
                            print!("\n");
                            stdout().flush().expect("IOError");
                            current_line = lyrics.pop_front().unwrap();
                        }
                        if previous_word == current_word {
                            word = current_line.pop_front().unwrap();
                            current_word = word.clone();
                        } else {
                            word = current_word.clone();
                        }

                        if word.0 < current_pos {
                            print!("{}", word.1.trim());
                            if !word.1.trim().is_empty() {print!(" ")}
                            previous_word = word.clone();
                            stdout().flush().expect("IOError");
                            continue;
                        }
                        let next_time = word.0 - current_pos;
                        if next_time > retry_dur {
                            task::sleep(retry_dur).await;
                        } else {
                            task::sleep(next_time).await;
                            print!("{}", word.1.trim());
                            if !word.1.trim().is_empty() {print!(" ")}
                            previous_word = word.clone();
                            stdout().flush().expect("IOError");
                        }
                    } else {
                        task::sleep(retry_dur).await;
                    }
                }
            }
        }
    }
}
