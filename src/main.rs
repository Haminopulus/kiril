use std::{collections::VecDeque, io::{stdout, Write}, time::Duration};

use async_std::task;
use mpris::{Metadata, PlaybackStatus, Player, PlayerFinder, TrackID};
use tokio::{self};

use crate::json::json_convert;
mod lrc;
mod json;

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
    let json: bool = true;
    let step: u32 = 2;

    loop {
        let player: Player = get_active_player(retry_dur).await;
        let mut lyrics: VecDeque<Lyric> = VecDeque::new();
        let mut lyrics_clone: VecDeque<Lyric> = VecDeque::new();

        let mut current_line: Lyric = VecDeque::new();
        let mut line_num: i32 = -1;

        let mut previous_pos = Duration::default();
        let mut current_pos: Duration;

        let mut previous_song: Option<TrackID> = None;
        let mut current_song: Option<TrackID>;

        let mut previous_word: TimeTag = (Duration::default(), String::default());
        let mut current_word: TimeTag = (Duration::default(), String::default());
        let mut word_num: u32 = 0;
        let mut newline: bool = true;
        let mut cover: String = "".into();

        loop {
            // go back to player search if player quit
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

            // if we searched backwards or changed song, get new lyrics
            if (previous_song != current_song) || (previous_pos > current_pos) {
                cover = urlencoding::decode(metadata.art_url()
                    .unwrap_or_default().strip_prefix("file://").unwrap_or_default())
                    .unwrap_or_default().into_owned();
                line_num = -1;
                word_num = 0;
                current_line = VecDeque::new();
                current_word = (Duration::default(), String::default());
                previous_word = (Duration::default(), String::default());
                previous_song = current_song;
                lyrics = match lrc::get_lyrics(&metadata) {
                    Some(lrcs) => lrcs,
                    None => {VecDeque::new()}
                };
                lyrics_clone = lyrics.clone();
                if json {
                    println!("{}", json_convert(&lyrics_clone, 0, word_num, step, &cover))
                }
            }
            previous_pos = current_pos;


            match player.get_playback_status() {
                Ok(status) => {
                    match status {
                        PlaybackStatus::Stopped | PlaybackStatus::Paused => {
                            task::sleep(retry_dur).await;
                            continue;
                        }
                        PlaybackStatus::Playing => {
                            if !lyrics.is_empty() {
                                // Line has been fully printed, switch to next line
                                if current_line.is_empty() && previous_word == current_word {
                                    current_line = lyrics.pop_front().unwrap();
                                    if json {
                                        newline = true;
                                    } else {
                                        print!("\n");
                                        stdout().flush().expect("IOError");
                                    }

                                }

                                // previous word was printed, set next word as current
                                if previous_word == current_word {
                                    current_word = current_line.pop_front().unwrap();
                                }

                                // if word timetag is already reached, print word
                                if current_word.0 < current_pos {
                                    previous_word = current_word.clone();
                                    word_num += 1;

                                    if json {
                                        if newline {
                                            newline = false;
                                            line_num += 1;
                                            word_num = 0;
                                        }
                                        println!("{}", json_convert(&lyrics_clone, line_num as u32, word_num, step, &cover))
                                    } else {
                                        print!("{}", current_word.1.trim()); 
                                        print!("{}", if current_word.1.trim().is_empty() {" "} else {""});
                                        stdout().flush().expect("IOError");
                                    }

                                    continue;
                                }

                                // if word is not yet reached, sleep until it is reached or
                                // retry_duration is
                                let next_time = current_word.0 - current_pos;
                                if next_time > retry_dur {
                                    task::sleep(retry_dur).await;
                                } else {
                                    task::sleep(next_time).await;
                                    let state = player.get_playback_status();
                                    if !(state.is_ok() 
                                        && state.unwrap_or(PlaybackStatus::Stopped) == PlaybackStatus::Playing) {
                                        continue;
                                    }
                                    previous_word = current_word.clone();
                                    word_num += 1;

                                    if json {
                                        if newline {
                                            newline = false;
                                            line_num += 1;
                                            word_num = 0;
                                        }
                                        println!("{}", json_convert(&lyrics_clone, line_num as u32, word_num, step, &cover))
                                    } else {
                                        print!("{}", current_word.1.trim());
                                        print!("{}", if current_word.1.trim().is_empty() {" "} else {""});
                                        stdout().flush().expect("IOError");
                                    }
                                }
                            } else {
                                task::sleep(retry_dur).await;
                            }
                        }
                    }
                },
                // failed to get playback state
                Err(_) => break
            };
        }
    }
}
