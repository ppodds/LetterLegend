use std::sync::Arc;
use tokio::sync::Mutex;

include!(concat!(env!("OUT_DIR"), "/lobby.lobby.rs"));

impl Lobby {
    pub async fn from_lobby(lobby: Arc<Mutex<crate::lobby::lobby::Lobby>>) -> Self {
        let mut players = Vec::new();
        for player in lobby.lock().await.get_players().await {
            players.push(crate::model::player::player::Player::from_player(player).await);
        }
        Self {
            id: lobby.lock().await.get_id(),
            players,
        }
    }
}
