use std::{collections::VecDeque, time::Duration};

use async_std::task;
use mpris::{PlaybackStatus, Player, PlayerFinder, ProgressTick, ProgressTracker};
use tokio::{self, time::{sleep_until, Instant}};
mod lrc;

type TimeTag = (String, Duration);
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
async  fn main() {
    let mut lyrics: VecDeque<Lyric> = VecDeque::new();
    let retry_dur = Duration::from_secs(2);

    loop {
        let player: Player = get_active_player(retry_dur).await;
        println!("DEBUG: active player {} discovered", player.identity());
        let mut tracker: ProgressTracker = player.track_progress(10).unwrap();
        let ProgressTick {progress, ..} = tracker.tick();
        let mut song = progress.metadata().track_id();

        loop {
            let ProgressTick {progress, progress_changed, player_quit, ..} = tracker.tick();

            if player_quit {break}

            // happens on song change
            if progress_changed {
                let current_song = progress.metadata().track_id();
                if song != current_song {
                    song = current_song;
                    lyrics = match lrc::get_lyrics(progress.metadata()) {
                        Some(lrcs) => lrcs,
                        None => VecDeque::new()
                    }
                }
            }

            match progress.playback_status() {
                PlaybackStatus::Stopped | PlaybackStatus::Paused => {
                        task::sleep(retry_dur).await;
                }
                PlaybackStatus::Playing => {
                        // get the next line
                        if !lyrics.is_empty() {
                            let position: Duration = progress.position();
                            let next_line: Lyric = lyrics.pop_front().unwrap();
                            let mut next_time: Instant = Instant::now();
                                for word in next_line {
                                    next_time = next_time + (word.1 - position);
                                    sleep_until(next_time).await;
                                    print!("{}", word.0)
                                }
                                print!("\n")
                        } else {
                            task::sleep(retry_dur).await;
                        }
                }
            }
        }
    }
}
