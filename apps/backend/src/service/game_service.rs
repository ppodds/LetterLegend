use std::{
    collections::{HashMap, HashSet},
    error::Error,
    sync::{Arc, Mutex},
    time::Duration,
};

#[cfg(not(test))]
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};
use tokio::{task, time::sleep};

use crate::{
    game::{
        card::Card,
        game::{Game, END_GAME_TURN},
        game_player::GamePlayer,
        tile::Tile,
    },
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
        game_service: Arc<GameService>,
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
            let mut next_id = game_service.next_game_id.lock().unwrap();
            let game = Arc::new(Game::new(
                *next_id,
                lobby
                    .get_players()
                    .iter()
                    .map(|x| x.player.clone())
                    .collect(),
            ));
            game_service
                .games
                .lock()
                .unwrap()
                .insert(*next_id, game.clone());
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
                                next_player: match game.get_next_turn_player() {
                                    Some(game_player) => Some(
                                        crate::model::player::player::Player::from(game_player),
                                    ),
                                    None => None,
                                },
                            })),
                        ))
                        .await
                    {
                        eprintln!("Error sending lobby broadcast: {}", e);
                    }
                });
            }
        }
        GameService::start_countdown(game_service, game.clone());
        Ok(game)
    }

    pub fn get_game(&self, id: u32) -> Option<Arc<Game>> {
        self.games.lock().unwrap().get(&id).cloned()
    }

    /**
     * Finish turn. Return true if the game is ended.
     */
    fn finish_turn(
        game_service: Arc<GameService>,
        game: Arc<Game>,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let player_in_this_turn = game.get_player_in_this_turn();
        player_in_this_turn.get_new_card();
        game.cancel_timeout_task();
        game.next_turn();
        game.backup_board();
        if game.get_turns() > END_GAME_TURN {
            game_service.clone().remove_game(game.clone())?;
            #[cfg(not(test))]
            GameService::boardcast_game_end(game);
            return Ok(true);
        }
        Ok(false)
    }

    #[cfg(not(test))]
    fn send_finish_turn_broadcast(
        game: Arc<Game>,
        words: &Vec<String>,
        origin_player: Arc<GamePlayer>,
        send_to_origin_player: bool,
    ) {
        for game_player in game.get_players() {
            if game_player == origin_player && !send_to_origin_player {
                continue;
            }
            let game = game.clone();
            let board = Some(crate::model::game::board::Board::from({
                &game.get_board().lock().unwrap().clone()
            }));
            let words = Some(crate::model::game::words::Words::from(words));
            let origin_player = origin_player.clone();
            tokio::spawn(async move {
                if let Err(e) = game_player
                    .player
                    .send_message(Response::new(
                        State::GameBroadcast as u32,
                        Arc::new(ResponseData::GameBroadcast(GameBroadcast {
                            event: GameEvent::FinishTurn as i32,
                            board,
                            players: None,
                            current_player: Some(crate::model::player::player::Player::from(
                                game.get_player_in_this_turn(),
                            )),
                            next_player: match game.get_next_turn_player() {
                                Some(game_player) => {
                                    Some(crate::model::player::player::Player::from(game_player))
                                }
                                None => None,
                            },
                            words,
                            cards: match game_player == origin_player {
                                true => Some(crate::model::game::cards::Cards::from(
                                    &origin_player.get_cards(),
                                )),
                                false => None,
                            },
                        })),
                    ))
                    .await
                {
                    eprintln!("Error sending game broadcast: {}", e);
                }
            });
        }
    }

    fn start_countdown(game_service: Arc<GameService>, game: Arc<Game>) {
        let game_bak = game.clone();
        let task = Arc::new(task::spawn(async move {
            sleep(Duration::from_secs(30)).await;
            let _origin_player = game.get_player_in_this_turn();
            match GameService::timeout_finish_turn(game_service, game.clone()) {
                Ok(_words) => {
                    #[cfg(not(test))]
                    GameService::send_finish_turn_broadcast(
                        game.clone(),
                        &_words,
                        _origin_player,
                        true,
                    );
                }
                Err(e) => eprintln!("encounter error when finish turn: {}", e),
            }
        }));
        game_bak.set_timeout_task(task);
    }

    pub fn timeout_finish_turn(
        game_service: Arc<GameService>,
        game: Arc<Game>,
    ) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        let words = {
            game.get_board()
                .lock()
                .unwrap()
                .validate(&game_service.wordlist)
        };
        let words = match words {
            Some(words) => words,
            None => {
                game.restore_board();
                Vec::new()
            }
        };
        let _origin_player = game.get_player_in_this_turn();
        game.get_player_in_this_turn().set_has_shuffled(false);
        if !GameService::finish_turn(game_service.clone(), game.clone())? {
            GameService::start_countdown(game_service, game.clone());
        }
        Ok(words)
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
                                words: None,
                                cards: None,
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
                                words: None,
                                cards: None,
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

    #[cfg(not(test))]
    fn boardcast_game_end(game: Arc<Game>) {
        for game_player in game.get_players() {
            tokio::spawn(async move {
                if let Err(e) = game_player
                    .player
                    .send_message(Response::new(
                        State::GameBroadcast as u32,
                        Arc::new(ResponseData::GameBroadcast(GameBroadcast {
                            event: GameEvent::Destroy as i32,
                            board: None,
                            players: None,
                            current_player: None,
                            next_player: None,
                            words: None,
                            cards: None,
                        })),
                    ))
                    .await
                {
                    eprintln!("Error sending game broadcast: {}", e);
                }
            });
        }
    }

    pub fn validate_board_and_finish_turn(
        game_service: Arc<GameService>,
        game: Arc<Game>,
    ) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        let words = match game
            .get_board()
            .lock()
            .unwrap()
            .validate(&game_service.wordlist)
        {
            Some(words) => words,
            None => return Err("invalid word".into()),
        };
        let _origin_player = game.get_player_in_this_turn();
        if !GameService::finish_turn(game_service.clone(), game.clone())? {
            GameService::start_countdown(game_service, game.clone());
        }
        #[cfg(not(test))]
        GameService::send_finish_turn_broadcast(game.clone(), &words, _origin_player, false);
        Ok(words)
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
                                words: None,
                                cards: None,
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
                                words: None,
                                cards: None,
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

    #[tokio::test]
    async fn start_game_with_test_player_in_test_lobby_should_start_game(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = Arc::new(GameService::new(HashSet::new()));
        let leader = Arc::new(Player::new(0, "test".to_string()));
        let lobby = Arc::new(Lobby::new(0, 4, leader.clone()));
        lobby.get_player(leader.id).unwrap().set_ready(true);
        assert!(GameService::start_game(game_service, leader.clone(), lobby).is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn start_game_with_test_player_not_in_lobby_should_return_error(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = Arc::new(GameService::new(HashSet::new()));
        let leader = Arc::new(Player::new(0, "test1".to_string()));
        let player = Arc::new(Player::new(1, "test2".to_string()));
        assert!(
            GameService::start_game(game_service, player, Arc::new(Lobby::new(0, 4, leader)))
                .is_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn start_game_with_not_leader_should_return_error(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = Arc::new(GameService::new(HashSet::new()));
        let leader = Arc::new(Player::new(0, "test".to_string()));
        let lobby = Arc::new(Lobby::new(0, 4, leader.clone()));
        let player = Arc::new(Player::new(1, "test2".to_string()));
        lobby.add_player(player.clone())?;
        assert!(GameService::start_game(game_service, player, lobby).is_err());
        Ok(())
    }

    #[tokio::test]
    async fn start_game_with_players_not_ready_should_return_error(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = Arc::new(GameService::new(HashSet::new()));
        let leader = Arc::new(Player::new(0, "test".to_string()));
        let lobby = Arc::new(Lobby::new(0, 4, leader.clone()));
        assert!(GameService::start_game(game_service, leader, lobby).is_err());
        Ok(())
    }

    #[tokio::test]
    async fn validate_board_and_finish_turn_when_board_is_broken_should_return_error(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = Arc::new(GameService::new(HashSet::new()));
        let player = Arc::new(Player::new(0, String::from("test1")));
        let player1 = Arc::new(Player::new(1, String::from("test2")));
        let lobby = Arc::new(Lobby::new(0, 4, player.clone()));
        lobby.add_player(player1)?;
        lobby.get_player(0).unwrap().set_ready(true);
        lobby.get_player(1).unwrap().set_ready(true);
        let game = GameService::start_game(game_service.clone(), player.clone(), lobby)?;
        {
            game.get_board().lock().unwrap().tiles[0][0] = Some(Tile::new('a', player.clone(), 1));
        }
        assert!(GameService::validate_board_and_finish_turn(game_service, game).is_err());
        Ok(())
    }

    #[tokio::test]
    async fn validate_board_and_finish_turn_when_game_end_should_remove_game(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = Arc::new(GameService::new(HashSet::new()));
        let player = Arc::new(Player::new(0, String::from("test1")));
        let player1 = Arc::new(Player::new(1, String::from("test2")));
        let lobby = Arc::new(Lobby::new(0, 4, player.clone()));
        lobby.add_player(player1)?;
        lobby.get_player(0).unwrap().set_ready(true);
        lobby.get_player(1).unwrap().set_ready(true);
        let game = GameService::start_game(game_service.clone(), player.clone(), lobby)?;
        game.set_turn(END_GAME_TURN + 1);
        GameService::validate_board_and_finish_turn(game_service.clone(), game)?;
        assert_eq!(game_service.get_gamees().len(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn validate_board_and_finish_turn_when_game_not_end_should_not_remove_game(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = Arc::new(GameService::new(HashSet::new()));
        let player = Arc::new(Player::new(0, String::from("test1")));
        let player1 = Arc::new(Player::new(1, String::from("test2")));
        let lobby = Arc::new(Lobby::new(0, 4, player.clone()));
        lobby.add_player(player1)?;
        lobby.get_player(0).unwrap().set_ready(true);
        lobby.get_player(1).unwrap().set_ready(true);
        let game = GameService::start_game(game_service.clone(), player.clone(), lobby)?;
        GameService::validate_board_and_finish_turn(game_service.clone(), game)?;
        assert_eq!(game_service.get_gamees().len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn validate_board_and_finish_turn_when_board_is_legal_should_success(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = Arc::new(GameService::new(HashSet::new()));
        let player = Arc::new(Player::new(0, String::from("test1")));
        let player1 = Arc::new(Player::new(1, String::from("test2")));
        let lobby = Arc::new(Lobby::new(0, 4, player.clone()));
        lobby.add_player(player1)?;
        lobby.get_player(0).unwrap().set_ready(true);
        lobby.get_player(1).unwrap().set_ready(true);
        let game = GameService::start_game(game_service.clone(), player.clone(), lobby)?;
        assert!(GameService::validate_board_and_finish_turn(game_service, game).is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn timeout_finish_turn_when_times_up_should_success(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = Arc::new(GameService::new(HashSet::new()));
        let player = Arc::new(Player::new(0, String::from("test1")));
        let player1 = Arc::new(Player::new(1, String::from("test2")));
        let lobby = Arc::new(Lobby::new(0, 4, player.clone()));
        lobby.add_player(player1)?;
        lobby.get_player(0).unwrap().set_ready(true);
        lobby.get_player(1).unwrap().set_ready(true);
        let game = GameService::start_game(game_service.clone(), player, lobby)?;
        let turn_before = game.clone().get_turns();
        GameService::finish_turn(game_service, game.clone())?;
        assert!(turn_before + 1 == game.get_turns());
        Ok(())
    }

    #[tokio::test]
    async fn get_game_with_game_id_should_return_game() -> Result<(), Box<dyn Error + Send + Sync>>
    {
        let game_service = GameService::new(HashSet::new());
        game_service
            .games
            .lock()
            .unwrap()
            .insert(0, Arc::new(Game::new(0, Vec::new())));
        assert!(game_service.get_game(0).is_some());
        Ok(())
    }

    #[tokio::test]
    async fn remove_game_with_test_game_should_be_remove(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = GameService::new(HashSet::new());
        let game = Arc::new(Game::new(0, Vec::new()));
        game_service.games.lock().unwrap().insert(0, game.clone());
        game_service.remove_game(game)?;
        assert!(game_service.games.lock().unwrap().is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn remove_game_with_test_player_and_with_test_game_test_player_game_should_be_none(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = GameService::new(HashSet::new());
        let player = Arc::new(Player::new(0, "test".to_string()));
        let game = Arc::new(Game::new(0, vec![player.clone()]));
        game_service.games.lock().unwrap().insert(0, game.clone());
        game_service.remove_game(game)?;
        assert!(player.get_game().is_none());
        Ok(())
    }

    #[tokio::test]
    async fn get_games_with_test_games_should_return_test_games(
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

    #[tokio::test]
    async fn remove_player_from_game_with_test_player_should_remove_the_player(
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

    #[tokio::test]
    async fn remove_player_from_game_with_test_player_and_game_players_amount_equal_0_should_destroy_game(
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

    #[tokio::test]
    async fn remove_player_from_game_with_test_player_should_return_test_user(
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

    #[tokio::test]
    async fn finish_turn_without_parameter_should_backup_board(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = Arc::new(GameService::new(HashSet::new()));
        let player = Arc::new(Player::new(0, String::from("test1")));
        let game = Arc::new(Game::new(0, vec![player.clone()]));
        player.set_game(Some(game.clone()));
        game_service.games.lock().unwrap().insert(0, game.clone());
        game.get_board().lock().unwrap().tiles[0][0] = Some(Tile::new('a', player, 1));
        GameService::finish_turn(game_service, game.clone())?;
        assert!(game.get_board_backup().tiles[0][0].is_some());
        Ok(())
    }
}
