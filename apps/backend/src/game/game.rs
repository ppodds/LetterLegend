use std::{
    collections::{linked_list::LinkedList, HashMap},
    sync::{Arc, Mutex},
};

use super::{board::Board, game_player::GamePlayer};
use crate::player::Player;

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    turn: Mutex<u32>,
    players: Mutex<HashMap<u32, Arc<GamePlayer>>>,
    turn_queue: Mutex<LinkedList<Arc<GamePlayer>>>,
    board: Arc<Mutex<Board>>,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Game {
    pub fn new(id: u32, players: Vec<Arc<Player>>) -> Self {
        let mut map = HashMap::new();
        let mut queue = LinkedList::new();

        for player in players {
            map.insert(player.id, Arc::new(GamePlayer::new(player.clone())));
        }

        for game_player in map.clone() {
            queue.push_back(game_player.1);
        }
        Self {
            id,
            turn: Mutex::new(1),
            players: Mutex::new(map),
            turn_queue: Mutex::new(queue),
            board: Arc::new(Mutex::new(Board::new())),
        }
    }

    pub fn get_board(&self) -> Arc<Mutex<Board>> {
        self.board.clone()
    }

    pub fn remove_player(&self, player: Arc<Player>) -> Option<Arc<GamePlayer>> {
        self.players.lock().unwrap().remove(&player.id)
    }

    pub fn get_player(&self, id: u32) -> Option<Arc<GamePlayer>> {
        Some(self.players.lock().unwrap().get(&id)?.clone())
    }

    pub fn get_players(&self) -> Vec<Arc<GamePlayer>> {
        self.players
            .lock()
            .unwrap()
            .values()
            .map(|player| player.clone())
            .collect()
    }

    pub fn get_turns(&self) -> u32 {
        self.turn.lock().unwrap().clone()
    }

    pub fn next_turn(&self) -> u32 {
        *self.turn.lock().unwrap() += 1;
        let pop_player = self.turn_queue.lock().unwrap().pop_front().unwrap();
        pop_player.set_has_shuffled(false);
        self.turn_queue.lock().unwrap().push_back(pop_player);
        *self.turn.lock().unwrap()
    }

    pub fn get_player_in_this_turn(&self) -> Arc<GamePlayer> {
        self.turn_queue.lock().unwrap().front().unwrap().clone()
    }

    pub fn get_next_turn_player(&self) -> Arc<GamePlayer> {
        if self.get_players().len() == 1 {
            return self.get_player_in_this_turn();
        }
        self.turn_queue
            .lock()
            .unwrap()
            .iter()
            .nth(1)
            .unwrap()
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;

    #[test]
    fn next_turn_without_parameter_should_return_2() -> Result<(), Box<dyn Error + Sync + Send>> {
        let game = Game::new(
            0,
            vec![
                Arc::new(Player::new(0, String::from("test"))),
                Arc::new(Player::new(1, String::from("test1"))),
            ],
        );
        game.next_turn();
        assert_eq!(*game.turn.lock().unwrap(), 2);
        Ok(())
    }

    #[test]
    fn next_turn_has_shuffled_reset() -> Result<(), Box<dyn Error + Sync + Send>> {
        let game = Game::new(
            0,
            vec![
                Arc::new(Player::new(0, String::from("test"))),
                Arc::new(Player::new(1, String::from("test1"))),
            ],
        );
        let person_now = game.get_player_in_this_turn();
        person_now.set_has_shuffled(true);
        game.next_turn();
        assert_eq!(person_now.get_has_shuffled(), false);
        Ok(())
    }

    #[test]
    fn next_turn_without_parameter_should_return_first_person_when_his_second_round(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let game = Game::new(
            0,
            vec![
                Arc::new(Player::new(0, String::from("test"))),
                Arc::new(Player::new(1, String::from("test1"))),
            ],
        );
        game.next_turn();
        game.next_turn();
        let person_now = game.get_player_in_this_turn();
        let person_first = game.get_players();
        assert_eq!(*person_first.get(0).unwrap(), person_now);
        Ok(())
    }

    #[test]
    fn next_turn_without_parameter_player_in_this_turn_should_changed(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let game = Game::new(
            0,
            vec![
                Arc::new(Player::new(0, String::from("test"))),
                Arc::new(Player::new(1, String::from("test1"))),
            ],
        );
        let person_first = game.get_player_in_this_turn();
        game.next_turn();
        let person_second = game.get_player_in_this_turn();
        assert_ne!(person_second.player.id, person_first.player.id);
        Ok(())
    }
}
