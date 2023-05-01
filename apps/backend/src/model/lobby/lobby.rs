use std::sync::Arc;

include!(concat!(env!("OUT_DIR"), "/lobby.lobby.rs"));

impl From<Arc<crate::lobby::lobby::Lobby>> for Lobby {
    fn from(lobby: Arc<crate::lobby::lobby::Lobby>) -> Self {
        Lobby::from(lobby.as_ref())
    }
}

impl From<&crate::lobby::lobby::Lobby> for Lobby {
    fn from(lobby: &crate::lobby::lobby::Lobby) -> Self {
        let mut players = Vec::new();
        for player in lobby.get_players() {
            players.push(crate::model::player::player::Player::from(
                player.player.clone(),
            ));
        }
        Self {
            id: lobby.get_id(),
            players,
        }
    }
}
