use std::sync::Arc;
use tokio::sync::Mutex;

include!(concat!(env!("OUT_DIR"), "/player.player.rs"));

impl Player {
    pub async fn from_player(player: Arc<Mutex<crate::player::Player>>) -> Self {
        let player = player.lock().await;
        Self {
            id: player.id,
            name: player.name.clone(),
        }
    }
}
