use crate::player::Player;
use std::{
    cmp::Reverse,
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
    time::Instant,
};

#[cfg(not(test))]
use crate::connection::Connection;
use priority_queue::PriorityQueue;

type ClientMap = Arc<Mutex<HashMap<u32, Arc<Player>>>>;

#[derive(Debug, Clone)]
pub struct PlayerService {
    player_timeout_queue: Arc<Mutex<PriorityQueue<u32, Reverse<Instant>>>>,
    online_player_map: ClientMap,
}

impl PlayerService {
    pub fn new() -> Self {
        Self {
            player_timeout_queue: Arc::new(Mutex::new(PriorityQueue::new())),
            online_player_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_player(&self, client_id: u32) -> Option<Arc<Player>> {
        match self.online_player_map.lock().unwrap().get(&client_id) {
            Some(player) => Some(player.clone()),
            None => None,
        }
    }

    pub fn add_player(
        &self,
        client_id: u32,
        name: String,
        #[cfg(not(test))] connection: Arc<tokio::sync::Mutex<Connection>>,
    ) -> Arc<Player> {
        let player = Arc::new(Player::new(
            client_id,
            name,
            #[cfg(not(test))]
            connection,
        ));
        self.online_player_map
            .lock()
            .unwrap()
            .insert(client_id, player.clone());
        self.player_timeout_queue
            .lock()
            .unwrap()
            .push(client_id, Reverse(Instant::now()));
        player
    }

    pub fn get_players(&self) -> Vec<Arc<Player>> {
        self.online_player_map
            .lock()
            .unwrap()
            .iter()
            .map(|x| x.1.clone())
            .collect()
    }

    pub fn remove_player(
        &self,
        player: Arc<Player>,
    ) -> Result<Arc<Player>, Box<dyn Error + Send + Sync>> {
        match self.online_player_map.lock().unwrap().remove(&player.id) {
            Some(player) => {
                self.player_timeout_queue.lock().unwrap().remove(&player.id);
                if let Some(lobby) = player.clone().get_lobby() {
                    lobby.remove_player(player.clone());
                };
                if let Some(game) = player.clone().get_game() {
                    game.remove_player(player.clone());
                };
                Ok(player)
            }
            None => Err("Player not found".into()),
        }
    }

    pub fn heartbeat(&self, player: Arc<Player>) -> Result<(), Box<dyn Error + Send + Sync>> {
        match self
            .player_timeout_queue
            .lock()
            .unwrap()
            .change_priority(&player.id, Reverse(Instant::now()))
        {
            Some(_) => Ok(()),
            None => Err("Player not found")?,
        }
    }

    pub fn kick_timeout_users(&self) -> Result<u32, Box<dyn Error + Send + Sync>> {
        let queue = self.player_timeout_queue.clone();
        let mut timeout_players = Vec::new();
        {
            let mut queue = queue.lock().unwrap();
            while let Some(item) = queue.pop() {
                if item.1 .0.elapsed().as_secs() < 30 {
                    queue.push(item.0, item.1);
                    break;
                }
                timeout_players.push(item.0);
            }
        }
        for timeout_player in &timeout_players {
            self.remove_player(match self.get_player(*timeout_player) {
                Some(player) => player,
                None => return Err("Player not found")?,
            })?;
        }
        Ok(timeout_players.len() as u32)
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, time::Duration};

    use super::*;

    #[test]
    fn add_player_with_test_user_online_player_map_should_include_test_user(
    ) -> Result<(), Box<dyn Error>> {
        let service = PlayerService::new();
        service.add_player(0, String::from("test"));
        assert!(service.online_player_map.lock().unwrap().get(&0).is_some());
        Ok(())
    }

    #[test]
    fn add_player_with_test_user_player_timeout_queue_should_include_test_user(
    ) -> Result<(), Box<dyn Error>> {
        let service = PlayerService::new();
        service.add_player(0, String::from("test"));
        assert!(service
            .player_timeout_queue
            .lock()
            .unwrap()
            .get(&0)
            .is_some());
        Ok(())
    }

    #[test]
    fn kick_timeout_users_with_a_timeout_user_timeout_users_should_be_kicked(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let service = PlayerService::new();
        service
            .player_timeout_queue
            .lock()
            .unwrap()
            .push(0, Reverse(Instant::now() - Duration::from_secs(60)));
        service
            .online_player_map
            .lock()
            .unwrap()
            .insert(0, Arc::new(Player::new(0, String::from("test"))));
        assert_eq!(service.kick_timeout_users()?, 1);
        Ok(())
    }

    #[test]
    fn kick_timeout_users_with_two_timeout_users_and_a_normal_user_timeout_users_should_be_kicked(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let service = PlayerService::new();

        for i in 0..3 {
            service.player_timeout_queue.lock().unwrap().push(
                i,
                match i == 2 {
                    true => Reverse(Instant::now()),
                    false => Reverse(Instant::now() - Duration::from_secs(60)),
                },
            );
            service
                .online_player_map
                .lock()
                .unwrap()
                .insert(i, Arc::new(Player::new(i, String::from("test"))));
        }

        assert_eq!(service.kick_timeout_users()?, 2);
        Ok(())
    }
}
