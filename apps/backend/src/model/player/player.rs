use std::sync::Arc;

include!(concat!(env!("OUT_DIR"), "/player.player.rs"));

impl Player {
    pub async fn from_player(player: Arc<crate::player::Player>) -> Self {
        Self {
            id: player.id,
            name: player.name.clone(),
        }
    }
}
