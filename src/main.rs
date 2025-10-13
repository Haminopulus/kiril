use std::{collections::VecDeque, time::Duration};

use async_std::task;
use mpris::{Player, PlayerFinder};
use tokio::{self, time::{sleep_until, Instant}};
use crate::lrc;


struct Lyric {
    text: String,
    time: Duration,
}
impl Lyric {
    fn new(txt: &str, dur: Duration) -> Lyric {
        Lyric { text: String::from(txt), time: dur }
    }
    fn empty() -> Lyric {
        return Lyric::new("", Duration::new(0,0))
    }
}

/// Wait for active player to be found and return it
async fn get_active_player(retry_duration: Duration) -> Player {
    let finder: PlayerFinder = PlayerFinder::new().expect("DBusError");
    loop {
        let player = match finder.find_active() {
            Ok(pl) => pl,
            Err(_) => {
                task::sleep(retry_duration).await;
                continue;
            },
        };
        return player;
    }
}


#[tokio::main]
async fn main() {
    let lyrics: VecDeque<Lyric>;
    let retry_dur = Duration::from_secs(5);
    let player: Player = get_active_player(retry_dur).await;
    loop {
        // check for Playerevents
        todo!();
        // get the next line
        if !lyrics.is_empty() {
            let next_lyric: Lyric = lyrics.pop_front().unwrap();
            // sleep till next line
            sleep_until(Instant::now() + next_lyric.time).await;
        };
    }
}



//let player = PlayerFinder::new()?
//        .find_active()
//        .expect("No active players found!");
//    let metadata = player.get_metadata().unwrap();
//    println!("Artist: {}, Track Name: {}, URL: {}", 
//        metadata.artists().unwrap()[0],
//        metadata.title().unwrap(),
//        decode(metadata.url().unwrap()).expect("FromUTF8Error"))


