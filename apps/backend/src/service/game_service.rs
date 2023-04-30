use std::{error::Error, sync::Arc};

use crate::{
    game::{game::Game, games::Games},
    player::Player,
};

#[derive(Debug, Clone)]
pub struct GameService {
    games: Arc<Games>,
}

impl GameService {
    pub fn new() -> Self {
        Self {
            games: Arc::new(Games::new()),
        }
    }

    pub fn start_game(
        &self,
        player: Arc<Player>,
    ) -> Result<Arc<Game>, Box<dyn Error + Send + Sync>> {
        let lobby = match player.get_lobby() {
            Some(lobby) => lobby,
            None => return Err("Player not in lobby".into()),
        };

        if player != lobby.leader {
            return Err("Only leader can start game".into());
        }

        Ok(self.games.create_game(
            lobby
                .get_players()
                .iter()
                .map(|x| x.player.clone())
                .collect(),
        ))
    }

    pub fn get_game(&self, id: u32) -> Option<Arc<Game>> {
        self.games.get_game(id)
    }
}
