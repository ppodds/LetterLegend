use std::sync::Arc;

include!(concat!(env!("OUT_DIR"), "/lobby.lobby.rs"));

impl Lobby {
    pub async fn from_lobby(lobby: Arc<crate::lobby::lobby::Lobby>) -> Self {
        let mut players = Vec::new();
        for player in lobby.get_players().await {
            players.push(
                crate::model::player::player::Player::from_player(player.player.clone()).await,
            );
        }
        Self {
            id: lobby.get_id(),
            players,
        }
    }
}
