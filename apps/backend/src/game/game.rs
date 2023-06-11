use std::{
    collections::{linked_list::LinkedList, HashMap},
    sync::{Arc, Mutex},
};

use super::{board::Board, game_player::GamePlayer};
use crate::player::Player;
pub const END_GAME_TURN: u32 = 16;
use tokio::task::JoinHandle;
#[derive(Debug)]
pub struct Game {
    pub id: u32,
    turn: Mutex<u32>,
    players: Mutex<HashMap<u32, Arc<GamePlayer>>>,
    turn_queue: Mutex<LinkedList<Arc<GamePlayer>>>,
    board: Arc<Mutex<Board>>,
    board_backup: Mutex<Board>,
    timeout: Mutex<Option<Arc<JoinHandle<()>>>>,
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
            board_backup: Mutex::new(Board::new()),
            timeout: Mutex::new(None),
        }
    }

    pub fn set_timeout_task(&self, task: Arc<JoinHandle<()>>) {
        *self.timeout.lock().unwrap() = Some(task);
    }

    pub fn cancel_timeout_task(&self) -> bool {
        let mut task_mutex = self.timeout.lock().unwrap();
        match task_mutex.as_ref() {
            Some(task) => {
                task.abort();
                *task_mutex = None;
                true
            }
            None => false,
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

    pub fn backup_board(&self) {
        *self.board_backup.lock().unwrap() = self.board.lock().unwrap().clone();
    }

    pub fn restore_board(&self) {
        *self.board.lock().unwrap() = self.board_backup.lock().unwrap().clone();
    }

    #[cfg(test)]
    pub fn get_board_backup(&self) -> Board {
        self.board_backup.lock().unwrap().clone()
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

    pub fn get_next_turn_player(&self) -> Option<Arc<GamePlayer>> {
        if self.get_turns() == END_GAME_TURN {
            return None;
        } else if self.get_players().len() == 1 {
            return Some(self.get_player_in_this_turn());
        }
        Some(
            self.turn_queue
                .lock()
                .unwrap()
                .iter()
                .nth(1)
                .unwrap()
                .clone(),
        )
    }

    #[cfg(test)]
    pub fn set_turn(&self, turn: u32) {
        *self.turn.lock().unwrap() = turn;
    }
}

#[cfg(test)]
mod tests {
    use tokio::time::{sleep, Duration};

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
    fn get_next_turn_player_only_one_person_without_parameter_should_return_previous_player(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let player0 = Arc::new(Player::new(0, String::from("test")));
        let game = Game::new(0, vec![player0.clone()]);
        let players = game.get_players();
        let next_player = game.get_next_turn_player().unwrap();
        assert_eq!(next_player.player, players.get(0).unwrap().player);
        Ok(())
    }

    #[test]
    fn get_next_turn_player_only_one_person_exceed_end_game_turn_without_parameter_should_return_none(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let player0 = Arc::new(Player::new(0, String::from("test")));
        let game = Game::new(0, vec![player0.clone()]);
        game.set_turn(END_GAME_TURN);
        assert!(game.get_next_turn_player().is_none());
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

    #[tokio::test]
    async fn cancel_timeout_task_with_exist_timeout_task_should_cancel_the_task_and_return_true(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let player = Arc::new(Player::new(0, String::from("test")));
        let game = Game::new(0, vec![player.clone()]);
        let task = Arc::new(tokio::spawn(async {
            sleep(Duration::from_secs(1)).await;
        }));
        game.set_timeout_task(task.clone());
        assert!(game.cancel_timeout_task());
        sleep(Duration::from_millis(10)).await;
        assert!(task.is_finished());
        Ok(())
    }

    #[tokio::test]
    async fn cancel_timeout_task_with_none_should_return_false(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let player = Arc::new(Player::new(0, String::from("test")));
        let game = Game::new(0, vec![player.clone()]);
        assert!(!game.cancel_timeout_task());
        Ok(())
    }
}
