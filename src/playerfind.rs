use std::time::Duration;
use mpris::{Player, PlayerFinder};
use async_std::task;

/// Wait for active player to be found and return it
pub async fn get_active_player(retry_dur: Duration) -> Player {
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
