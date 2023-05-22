use std::sync::Arc;

include!(concat!(env!("OUT_DIR"), "/player.player.rs"));

impl From<Arc<crate::player::Player>> for Player {
    fn from(player: Arc<crate::player::Player>) -> Self {
        Self {
            id: player.id,
            name: player.name.clone(),
        }
    }
}

impl From<Arc<crate::game::game_player::GamePlayer>> for Player {
    fn from(player: Arc<crate::game::game_player::GamePlayer>) -> Self {
        Self {
            id: player.player.id,
            name: player.player.name.clone(),
        }
    }
}
