use std::{
    collections::{HashMap, HashSet},
    error::Error,
    sync::{Arc, Mutex},
};

#[cfg(not(test))]
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

use crate::{
    game::{card::Card, game::Game, game_player::GamePlayer, tile::Tile},
    lobby::lobby::Lobby,
    player::Player,
};

#[cfg(not(test))]
use crate::frame::{Response, ResponseData};
#[cfg(not(test))]
use crate::model::game::broadcast::GameEvent;
#[cfg(not(test))]
use crate::model::game::cards::Cards;
#[cfg(not(test))]
use crate::model::lobby::broadcast::{LobbyBroadcast, LobbyEvent};
#[cfg(not(test))]
use crate::model::{game::broadcast::GameBroadcast, state::State};

#[derive(Debug)]
pub struct GameService {
    next_game_id: Mutex<u32>,
    games: Mutex<HashMap<u32, Arc<Game>>>,
    wordlist: HashSet<String>,
}

impl GameService {
    #[cfg(not(test))]
    pub async fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let mut wordlist = HashSet::new();
        let file = File::open("assets/wordlist.txt").await?;
        let mut reader = BufReader::new(file).lines();
        while let Some(line) = reader.next_line().await? {
            wordlist.insert(line);
        }
        Ok(Self {
            next_game_id: Mutex::new(0),
            games: Mutex::new(HashMap::new()),
            wordlist,
        })
    }

    #[cfg(test)]
    pub fn new(wordlist: HashSet<String>) -> Self {
        Self {
            next_game_id: Mutex::new(0),
            games: Mutex::new(HashMap::new()),
            wordlist,
        }
    }

    pub fn start_game(
        &self,
        player: Arc<Player>,
        lobby: Arc<Lobby>,
    ) -> Result<Arc<Game>, Box<dyn Error + Send + Sync>> {
        if player != lobby.leader {
            return Err("Only leader can start game".into());
        }
        let mut check = true;
        for player in lobby.get_players() {
            if !player.get_ready() {
                check = false;
                break;
            }
        }
        if !check {
            return Err("Not all players are ready".into());
        }
        let game = {
            let mut next_id = self.next_game_id.lock().unwrap();
            let game = Arc::new(Game::new(
                *next_id,
                lobby
                    .get_players()
                    .iter()
                    .map(|x| x.player.clone())
                    .collect(),
            ));
            self.games.lock().unwrap().insert(*next_id, game.clone());
            *next_id += 1;
            game
        };
        for game_player in game.get_players() {
            game_player.player.set_game(Some(game.clone()));
            if game_player.player == player {
                continue;
            }
            #[cfg(not(test))]
            {
                let game = game.clone();
                tokio::spawn(async move {
                    if let Err(e) = game_player
                        .clone()
                        .player
                        .send_message(Response::new(
                            State::LobbyBroadcast as u32,
                            Arc::new(ResponseData::LobbyBroadcast(LobbyBroadcast {
                                event: LobbyEvent::Start as i32,
                                lobby: None,
                                cards: Some(Cards::from(&game_player.get_cards())),
                                current_player: Some(crate::model::player::player::Player::from(
                                    game.get_player_in_this_turn(),
                                )),
                                next_player: Some(crate::model::player::player::Player::from(
                                    game.get_next_turn_player(),
                                )),
                            })),
                        ))
                        .await
                    {
                        eprintln!("Error sending lobby broadcast: {}", e);
                    }
                });
            }
        }
        Ok(game)
    }

    pub fn get_game(&self, id: u32) -> Option<Arc<Game>> {
        self.games.lock().unwrap().get(&id).cloned()
    }

    pub fn remove_player_from_game(
        &self,
        player: Arc<Player>,
    ) -> Result<Arc<GamePlayer>, Box<dyn Error + Send + Sync>> {
        let game = match player.clone().get_game() {
            Some(game) => game,
            None => return Err("Player is not in a game".into()),
        };
        let game_player = match game.remove_player(player.clone()) {
            Some(game) => game,
            None => return Err("Player is not in the game".into()),
        };
        let is_game_destroy = game.get_players().len() == 0;
        #[cfg(not(test))]
        {
            for game_player in game.get_players() {
                let game = game.clone();
                tokio::spawn(async move {
                    if let Err(e) = game_player
                        .player
                        .send_message(Response::new(
                            State::GameBroadcast as u32,
                            Arc::new(ResponseData::GameBroadcast(GameBroadcast {
                                event: GameEvent::Leave as i32,
                                board: None,
                                players: Some(crate::model::player::players::Players::from(
                                    &game.get_players(),
                                )),
                                current_player: None,
                                next_player: None,
                            })),
                        ))
                        .await
                    {
                        eprintln!("Error sending game broadcast: {}", e);
                    }
                });
            }
        }
        player.set_game(None);
        if is_game_destroy {
            self.remove_game(game)?;
        }
        Ok(game_player)
    }

    pub fn remove_game(&self, game: Arc<Game>) -> Result<Arc<Game>, Box<dyn Error + Send + Sync>> {
        match self.games.lock().unwrap().remove(&game.id) {
            Some(game) => {
                for game_player in game.get_players() {
                    game_player.player.set_game(None);
                }
                Ok(game)
            }
            None => Err("Game not found".into()),
        }
    }

    pub fn get_gamees(&self) -> Vec<Arc<Game>> {
        self.games.lock().unwrap().values().cloned().collect()
    }

    pub fn place_tile_on_board(&self, game: Arc<Game>, tile: Tile, x: usize, y: usize) {
        game.get_board().lock().unwrap().tiles[x][y] = Some(tile);
        #[cfg(not(test))]
        {
            for game_player in game.get_players() {
                if game_player == game.get_player_in_this_turn() {
                    continue;
                }

                let board = game.get_board().clone();
                tokio::spawn(async move {
                    let t = Some(crate::model::game::board::Board::from(
                        &*board.lock().unwrap(),
                    ));
                    if let Err(e) = game_player
                        .player
                        .send_message(Response::new(
                            State::GameBroadcast as u32,
                            Arc::new(ResponseData::GameBroadcast(GameBroadcast {
                                event: GameEvent::PlaceTile as i32,
                                board: t,
                                players: None,
                                current_player: None,
                                next_player: None,
                            })),
                        ))
                        .await
                    {
                        eprintln!("Error sending game broadcast: {}", e);
                    }
                });
            }
        }
    }

    pub fn finish_turn(&self, game: Arc<Game>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !game.get_board().lock().unwrap().validate(&self.wordlist) {
            return Err("invalid word".into());
        }
        game.get_player_in_this_turn().get_new_card();
        game.next_turn();
        #[cfg(not(test))]
        {
            for game_player in game.get_players() {
                if game_player == game.get_player_in_this_turn() {
                    continue;
                }
                let game = game.clone();
                tokio::spawn(async move {
                    if let Err(e) = game_player
                        .player
                        .send_message(Response::new(
                            State::GameBroadcast as u32,
                            Arc::new(ResponseData::GameBroadcast(GameBroadcast {
                                event: GameEvent::FinishTurn as i32,
                                board: None,
                                players: None,
                                current_player: Some(crate::model::player::player::Player::from(
                                    game.get_player_in_this_turn(),
                                )),
                                next_player: Some(crate::model::player::player::Player::from(
                                    game.get_next_turn_player(),
                                )),
                            })),
                        ))
                        .await
                    {
                        eprintln!("Error sending game broadcast: {}", e);
                    }
                });
            }
        }
        Ok(())
    }

    pub fn remove_selected_tile(&self, x: u32, y: u32, game: Arc<Game>) {
        {
            game.get_board().lock().unwrap().tiles[x as usize][y as usize] = None;
        }
        #[cfg(not(test))]
        {
            let board = game.get_board().clone();
            for game_player in game.get_players() {
                if game_player == game.get_player_in_this_turn() {
                    continue;
                }
                let _game = game.clone();
                let t = Some(crate::model::game::board::Board::from(
                    &*board.lock().unwrap(),
                ));
                tokio::spawn(async move {
                    if let Err(e) = game_player
                        .player
                        .send_message(Response::new(
                            State::GameBroadcast as u32,
                            Arc::new(ResponseData::GameBroadcast(GameBroadcast {
                                event: GameEvent::PlaceTile as i32,
                                board: t,
                                players: None,
                                current_player: None,
                                next_player: None,
                            })),
                        ))
                        .await
                    {
                        eprintln!("Error sending game broadcast: {}", e);
                    }
                });
            }
        }
    }

    pub fn shuffle(
        &self,
        #[cfg(not(test))] game: Arc<Game>,
        game_player: Arc<GamePlayer>,
    ) -> Result<Vec<Card>, Box<dyn Error + Send + Sync>> {
        if game_player.get_has_shuffled() {
            return Err("Player has shuffled in this turn".into());
        }
        let cards = game_player.get_new_card();
        #[cfg(not(test))]
        {
            for game_player in game.get_players() {
                if game_player == game.get_player_in_this_turn() {
                    continue;
                }

                tokio::spawn(async move {
                    if let Err(e) = game_player
                        .player
                        .send_message(Response::new(
                            State::GameBroadcast as u32,
                            Arc::new(ResponseData::GameBroadcast(GameBroadcast {
                                event: GameEvent::Shuffle as i32,
                                board: None,
                                players: None,
                                current_player: None,
                                next_player: None,
                            })),
                        ))
                        .await
                    {
                        eprintln!("Error sending game broadcast: {}", e);
                    }
                });
            }
        }
        Ok(cards)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_game_with_test_player_in_test_lobby_should_start_game(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = GameService::new(HashSet::new());
        let leader = Arc::new(Player::new(0, "test".to_string()));
        let lobby = Arc::new(Lobby::new(0, 4, leader.clone()));
        lobby.get_player(leader.id).unwrap().set_ready(true);
        assert!(game_service.start_game(leader.clone(), lobby).is_ok());
        Ok(())
    }

    #[test]
    fn start_game_with_test_player_not_in_lobby_should_return_error(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = GameService::new(HashSet::new());
        let leader = Arc::new(Player::new(0, "test1".to_string()));
        let player = Arc::new(Player::new(1, "test2".to_string()));
        assert!(game_service
            .start_game(player, Arc::new(Lobby::new(0, 4, leader)))
            .is_err());
        Ok(())
    }

    #[test]
    fn start_game_with_not_leader_should_return_error() -> Result<(), Box<dyn Error + Send + Sync>>
    {
        let game_service = GameService::new(HashSet::new());
        let leader = Arc::new(Player::new(0, "test".to_string()));
        let lobby = Arc::new(Lobby::new(0, 4, leader.clone()));
        let player = Arc::new(Player::new(1, "test2".to_string()));
        lobby.add_player(player.clone())?;
        assert!(game_service.start_game(player, lobby).is_err());
        Ok(())
    }

    #[test]
    fn start_game_with_players_not_ready_should_return_error(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = GameService::new(HashSet::new());
        let leader = Arc::new(Player::new(0, "test".to_string()));
        let lobby = Arc::new(Lobby::new(0, 4, leader.clone()));
        assert!(game_service.start_game(leader, lobby).is_err());
        Ok(())
    }

    #[test]
    fn finish_turn_when_board_is_broken_should_return_error(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = GameService::new(HashSet::new());
        let player = Arc::new(Player::new(0, String::from("test1")));
        let player1 = Arc::new(Player::new(1, String::from("test2")));
        let lobby = Arc::new(Lobby::new(0, 4, player.clone()));
        lobby.add_player(player1)?;
        lobby.get_player(0).unwrap().set_ready(true);
        lobby.get_player(1).unwrap().set_ready(true);
        let game = game_service.start_game(player.clone(), lobby)?;
        {
            game.get_board().lock().unwrap().tiles[0][0] = Some(Tile::new('a', player.clone(), 1));
        }
        assert!(game_service.finish_turn(game).is_err());
        Ok(())
    }

    #[test]
    fn finish_turn_when_board_is_legal_should_finish_turn(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = GameService::new(HashSet::new());
        let player = Arc::new(Player::new(0, String::from("test1")));
        let player1 = Arc::new(Player::new(1, String::from("test2")));
        let lobby = Arc::new(Lobby::new(0, 4, player.clone()));
        lobby.add_player(player1)?;
        lobby.get_player(0).unwrap().set_ready(true);
        lobby.get_player(1).unwrap().set_ready(true);
        let game = game_service.start_game(player.clone(), lobby)?;
        assert!(game_service.finish_turn(game).is_ok());
        Ok(())
    }

    #[test]
    fn get_game_with_game_id_should_return_game() -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = GameService::new(HashSet::new());
        game_service
            .games
            .lock()
            .unwrap()
            .insert(0, Arc::new(Game::new(0, Vec::new())));
        assert!(game_service.get_game(0).is_some());
        Ok(())
    }

    #[test]
    fn remove_game_with_test_game_should_be_remove() -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = GameService::new(HashSet::new());
        let game = Arc::new(Game::new(0, Vec::new()));
        game_service.games.lock().unwrap().insert(0, game.clone());
        game_service.remove_game(game)?;
        assert!(game_service.games.lock().unwrap().is_empty());
        Ok(())
    }

    #[test]
    fn remove_game_with_test_player_and_with_test_game_test_player_game_should_be_none(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = GameService::new(HashSet::new());
        let player = Arc::new(Player::new(0, "test".to_string()));
        let game = Arc::new(Game::new(0, vec![player.clone()]));
        game_service.games.lock().unwrap().insert(0, game.clone());
        game_service.remove_game(game)?;
        assert!(player.get_game().is_none());
        Ok(())
    }

    #[test]
    fn get_games_with_test_games_should_return_test_games(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = GameService::new(HashSet::new());
        let game1 = Arc::new(Game::new(0, Vec::new()));
        let game2 = Arc::new(Game::new(1, Vec::new()));
        game_service.games.lock().unwrap().insert(0, game1.clone());
        game_service.games.lock().unwrap().insert(1, game2.clone());
        let games = game_service.get_gamees();
        assert!(games.contains(&game1));
        assert!(games.contains(&game2));
        assert!(games.len() == 2);
        Ok(())
    }

    #[test]
    fn remove_player_from_game_with_test_player_should_remove_the_player(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = GameService::new(HashSet::new());
        let player = Arc::new(Player::new(0, String::from("test1")));
        let game = Arc::new(Game::new(
            0,
            vec![
                player.clone(),
                Arc::new(Player::new(1, String::from("test2"))),
            ],
        ));
        player.set_game(Some(game.clone()));
        game_service.games.lock().unwrap().insert(0, game.clone());
        game_service.remove_player_from_game(player.clone())?;
        assert!(game.get_player(0).is_none());
        Ok(())
    }

    #[test]
    fn remove_player_from_game_with_test_player_and_game_players_amount_equal_0_should_destroy_game(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = GameService::new(HashSet::new());
        let player = Arc::new(Player::new(0, String::from("test1")));
        let game = Arc::new(Game::new(0, vec![player.clone()]));
        player.set_game(Some(game.clone()));
        game_service.games.lock().unwrap().insert(0, game.clone());
        game_service.remove_player_from_game(player.clone())?;
        assert!(game_service.games.lock().unwrap().len() == 0);
        Ok(())
    }

    #[test]
    fn remove_player_from_game_with_test_player_should_return_test_user(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = GameService::new(HashSet::new());
        let player = Arc::new(Player::new(0, String::from("test1")));
        let game = Arc::new(Game::new(0, vec![player.clone()]));
        player.set_game(Some(game.clone()));
        game_service.games.lock().unwrap().insert(0, game.clone());
        let game_player = game_service.remove_player_from_game(player.clone())?;
        assert_eq!(game_player.player, player);
        Ok(())
    }
}
