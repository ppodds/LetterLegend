#[cfg(not(test))]
use crate::frame::Frame;
#[cfg(not(test))]
use crate::frame::Response;
use crate::{game::game::Game, lobby::lobby::Lobby};
use core::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
#[cfg(not(test))]
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub name: String,
    #[cfg(not(test))]
    sender: Sender<Frame>,
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
    pub fn new(id: u32, name: String, #[cfg(not(test))] sender: Sender<Frame>) -> Self {
        Player {
            id,
            name,
            #[cfg(not(test))]
            sender,
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

    #[cfg(not(test))]
    pub async fn send_message(
        &self,
        res: Response,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<Frame>> {
        match self.sender.send(Frame::Response(res)).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
