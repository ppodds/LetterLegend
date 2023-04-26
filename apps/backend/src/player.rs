#[cfg(not(test))]
use crate::connection::Connection;
use core::hash::{Hash, Hasher};
#[cfg(not(test))]
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub name: String,
    #[cfg(not(test))]
    pub connection: Arc<tokio::sync::Mutex<Connection>>,
    pub lobby_id: Mutex<Option<u32>>,
    pub game_id: Mutex<Option<u32>>,
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
            lobby_id: Mutex::new(None),
            game_id: Mutex::new(None),
        }
    }
}
