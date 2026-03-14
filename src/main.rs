use std::{env, collections::VecDeque, io::{stdout, Write}, time::Duration};
use async_std::task;
use mpris::{Metadata, PlaybackStatus, Player, TrackID};
use tokio::{self};

use crate::json::json_convert;
mod lrc;
mod json;
mod playerfind;

type TimeTag = (Duration, String);
type Lyric = VecDeque<TimeTag>;

#[tokio::main]
async fn main() {
    let mut retry_dur = Duration::from_secs(2);
    let mut json: bool = false;
    let mut step: u32 = 2;

    let mut cargs = env::args();

    _ = cargs.next(); // program name
    while let Some(arg) = cargs.next() {
         match arg.as_str() {
            "--help" | "-h" => {
                println!("
Usage: kiril [ARGS]
Arguments:
 -h, --help                             Print this help message
 -j, --json                             Enable JSON output mode
 -r [SECS], --retry-duration [SECS]     Set the maximum sleep-duration between player state and metadata queries.
                                        high SECS leads to longer response time to changes but less CPU usage (default: 2)
 -s [u32], --step [u32]                 Set how many lines before and after the current line are sent inside the JSON 
                                        message (only important if -j arg is present)
                "); return}
            "--json" | "-j" => {json = true;}
            "--retry-duration" | "-r" => {
                match cargs.next() {
                    Some(next) => match next.parse::<u64>() {
                        Ok(secs) => retry_dur = Duration::from_secs(secs),
                        Err(_) => {println!("Expected retry duration in seconds, Exiting."); return;}
                    },
                    None => {println!("Expected retry duration, Exiting."); return;}
                }
            }
            "--step" | "-s" => {
                match cargs.next() {
                    Some(next) => match next.parse::<u32>() {
                        Ok(stp) => step = stp,
                        Err(_) => {println!("Expected step value as u32, Exiting."); return;}
                    },
                    None => {println!("Expected step value, Exiting."); return;}
                }
            }
            _ => {println!("Invalid Argument, see `kiril --help` for permitted args"); return;}
        }
    }

    // player search and variable initialization loop
    loop {
        let player: Player = playerfind::get_active_player(retry_dur).await;
        let mut lyrics: VecDeque<Lyric> = VecDeque::new();
        let mut lyrics_clone: VecDeque<Lyric> = VecDeque::new();

        let mut curr_line: Lyric = VecDeque::new();
        let mut line_num: i32 = -1;

        let mut prev_pos: Duration = Duration::default();
        let mut curr_pos: Duration;

        let mut prev_song: Option<TrackID> = None;
        let mut curr_song: Option<TrackID>;

        let mut prev_word: TimeTag = (Duration::default(), String::default());
        let mut curr_word: TimeTag = (Duration::default(), String::default());
        let mut word_num: u32 = 0;          // index of current word in line (only relevant for ELRC)
        let mut newline: bool = true;       // is the current line done playing

        // once player found, lyrics parse/play loop
        loop {
            // go back to player search if player quit or if DBusErrors occurr while getting metadata
            if !player.is_running() {
                break;
            }
            curr_pos = match player.get_position() { Ok(p) => p, Err(_) => break };

            let metadata: Metadata = match player.get_metadata() { Ok(m) => m, Err(_) => break };
            curr_song = metadata.track_id();

            // if we searched backwards or changed song, get new lyrics
            if (prev_song != curr_song) || (prev_pos > curr_pos) {
                // cover sent once here, afterwards only lyrics will be sent
                let cover = urlencoding::decode(metadata.art_url()
                    .unwrap_or_default().strip_prefix("file://").unwrap_or_default())
                    .unwrap_or_default().into_owned();
                line_num = -1;
                word_num = 0;
                curr_line = VecDeque::new();
                curr_word = (Duration::default(), String::default());
                prev_word = (Duration::default(), String::default());
                prev_song = curr_song;
                lyrics = match lrc::get_lyrics(&metadata) { Some(x) => x, None => {VecDeque::new()} };

                if json {
                    lyrics_clone = lyrics.clone(); // full lyrics, not modified when played
                    println!("{}", json_convert(&lyrics_clone, 0, word_num, step, cover))
                }
            }

            prev_pos = curr_pos;

            match player.get_playback_status() {
                Ok(status) => {
                    match status {
                        PlaybackStatus::Stopped | PlaybackStatus::Paused => {
                            task::sleep(retry_dur).await;
                            continue;
                        }
                        PlaybackStatus::Playing => {
                            if !lyrics.is_empty() {
                                // line has been fully printed, switch to next line
                                if curr_line.is_empty() && prev_word == curr_word {
                                    curr_line = match lyrics.pop_front() { Some(x) => x, None => break };
                                    if json {
                                        newline = true;
                                    } else {
                                        print!("\n");
                                        _ = stdout().flush();
                                    }
                                }

                                // previous word was printed, set next word as current
                                if prev_word == curr_word {
                                    curr_word = match curr_line.pop_front() { Some(x) => x, None => break };
                                }

                                // if word timetag is already reached, print word
                                if curr_word.0 < curr_pos {
                                    prev_word = curr_word.clone();
                                    word_num += 1;

                                    if json {
                                        if newline {
                                            newline = false;
                                            line_num += 1;
                                            word_num = 0;
                                        }
                                        println!("{}", json_convert(
                                            &lyrics_clone,
                                            line_num as u32,
                                            word_num, step,
                                            String::default()));
                                    } else {
                                        if !curr_word.1.trim().is_empty() {
                                            print!("{} ", curr_word.1.trim()); 
                                            let _ = stdout().flush();
                                        }
                                    }
                                    continue;
                                }

                                // if word is not yet reached, sleep until it is reached or
                                // retry_duration is
                                let next_time = curr_word.0 - curr_pos;
                                if next_time > retry_dur {
                                    task::sleep(retry_dur).await;
                                } else {
                                    task::sleep(next_time).await;
                                    let state = player.get_playback_status();
                                    if !(state.is_ok() 
                                        && state.unwrap_or(PlaybackStatus::Stopped) == PlaybackStatus::Playing) {
                                        continue;
                                    }
                                    prev_word = curr_word.clone();
                                    word_num += 1;

                                    if json {
                                        if newline {
                                            newline = false;
                                            line_num += 1;
                                            word_num = 0;
                                        }
                                        println!("{}", json_convert(
                                            &lyrics_clone,
                                            line_num as u32,
                                            word_num,
                                            step,
                                            String::default()))
                                    } else {
                                        if !curr_word.1.trim().is_empty() {
                                            print!("{} ", curr_word.1.trim());
                                            stdout().flush().expect("IOError");
                                        }
                                    }
                                }
                            // if lyrics is empty (instrumental or missing .lrc file...)
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
