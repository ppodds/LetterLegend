#[cfg(not(test))]
use crate::connection::Connection;
use crate::{game::game::Game, lobby::lobby::Lobby};
use core::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub name: String,
    #[cfg(not(test))]
    pub connection: Arc<tokio::sync::Mutex<Connection>>,
    lobby: Mutex<Option<Arc<Lobby>>>,
    game: Mutex<Option<Arc<Game>>>,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Player {}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Player {
    pub fn new(
        id: u32,
        name: String,
        #[cfg(not(test))] connection: Arc<tokio::sync::Mutex<Connection>>,
    ) -> Self {
        Player {
            id,
            name,
            #[cfg(not(test))]
            connection,
            lobby: Mutex::new(None),
            game: Mutex::new(None),
        }
    }

    pub fn get_lobby(&self) -> Option<Arc<Lobby>> {
        self.lobby.lock().unwrap().clone()
    }

    pub fn set_lobby(&self, lobby: Option<Arc<Lobby>>) {
        *self.lobby.lock().unwrap() = lobby;
    }

    pub fn get_game(&self) -> Option<Arc<Game>> {
        self.game.lock().unwrap().clone()
    }

    pub fn set_game(&self, game: Option<Arc<Game>>) {
        *self.game.lock().unwrap() = game;
    }
}
