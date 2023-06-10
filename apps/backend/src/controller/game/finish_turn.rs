use crate::frame::Request;
use crate::model::game::finish_turn::FinishTurnResponse;
use crate::service::game_service::GameService;
use crate::{
    controller::controller::PrintableController,
    frame::{RequestData, ResponseData},
    router::RequestContext,
    service::player_service::PlayerService,
};
use std::sync::Arc;

use crate::controller::controller::Controller;

#[derive(Debug, Clone)]
pub struct FinishTurnController {
    player_service: Arc<PlayerService>,
    game_service: Arc<GameService>,
}

impl FinishTurnController {
    pub fn new(player_service: Arc<PlayerService>, game_service: Arc<GameService>) -> Self {
        Self {
            player_service,
            game_service,
        }
    }
}

impl PrintableController for FinishTurnController {}

impl Controller for FinishTurnController {
    fn handle_request(
        &self,
        req: Request,
        context: RequestContext,
    ) -> Result<ResponseData, Box<dyn std::error::Error + Send + Sync>> {
        let data = req.get_data();
        match data.as_ref() {
            RequestData::FinishTurn => true,
            _ => panic!("invalid request"),
        };

        let player = match self.player_service.get_player(context.client_id) {
            Some(player) => player,
            None => return Err("Player not found".into()),
        };

        let game = match player.get_game() {
            Some(game) => game,
            None => return Err("Player not in a game".into()),
        };
        let request_game_player = match game.get_player(player.id) {
            Some(game_player) => game_player,
            None => return Err("Player not found".into()),
        };
        if request_game_player != game.get_player_in_this_turn() {
            return Err("Player not in his turn".into());
        }
        let next_turn_player = match game.get_next_turn_player() {
            Some(player) => Some(crate::model::player::player::Player::from(player)),
            None => None,
        };
        match self
            .game_service
            .validate_board_and_finish_turn(game.clone())
        {
            Ok(words) => Ok(ResponseData::FinishTurn(FinishTurnResponse {
                success: true,
                current_player: Some(crate::model::player::player::Player::from(
                    game.get_player_in_this_turn(),
                )),
                next_player: next_turn_player,
                cards: Some(crate::model::game::cards::Cards::from(
                    &request_game_player.get_cards(),
                )),
                words: Some(crate::model::game::words::Words::from(&words)),
            })),
            Err(_) => Ok(ResponseData::FinishTurn(FinishTurnResponse {
                success: false,
                current_player: None,
                next_player: None,
                cards: None,
                words: None,
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::{collections::HashSet, error::Error};

    use crate::service::lobby_service::{self, LobbyService};

    use super::*;

    #[tokio::test]
    async fn player_in_his_turn_finish_turn_should_success(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let game_service = Arc::new(GameService::new(HashSet::new()));
        let controller = FinishTurnController::new(
            Arc::new(PlayerService::new(
                Arc::new(LobbyService::new()),
                game_service.clone(),
            )),
            game_service,
        );
        let player = controller
            .player_service
            .add_player(0, String::from("test"));
        let lobby_service = Arc::new(lobby_service::LobbyService::new());
        let lobby = lobby_service.create_lobby(player.clone(), 4)?;
        let lobby_player = lobby.clone().get_player(player.clone().id).unwrap();
        lobby_player.set_ready(true);
        let game = controller.game_service.start_game(player, lobby)?;
        let player_now = game.get_player_in_this_turn();
        assert!(controller
            .handle_request(
                Request::new(0, Arc::new(RequestData::FinishTurn)),
                RequestContext {
                    client_id: player_now.player.id
                },
            )
            .is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn player_not_in_his_turn_finish_turn_should_fail(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let lobby_service = Arc::new(LobbyService::new());
        let game_service = Arc::new(GameService::new(HashSet::new()));
        let player_service = Arc::new(PlayerService::new(
            lobby_service.clone(),
            game_service.clone(),
        ));
        let controller = FinishTurnController::new(player_service.clone(), game_service.clone());
        let player = controller
            .player_service
            .add_player(0, String::from("test"));
        let player1 = controller
            .player_service
            .add_player(1, String::from("test1"));
        let lobby = lobby_service.create_lobby(player.clone(), 4)?;
        lobby_service.add_player_to_lobby(player1.clone(), lobby.clone())?;
        let lobby_player = lobby.clone().get_player(player.clone().id).unwrap();
        let lobby_player1 = lobby.clone().get_player(player1.clone().id).unwrap();
        lobby_player.set_ready(true);
        lobby_player1.set_ready(true);
        let game = controller.game_service.start_game(player, lobby)?;
        let player_now = game.get_player_in_this_turn();
        assert!(controller
            .handle_request(
                Request::new(0, Arc::new(RequestData::FinishTurn)),
                RequestContext {
                    client_id: match player_now.player.id {
                        0 => 1,
                        1 => 0,
                        _ => panic!("invalid test case"),
                    }
                },
            )
            .is_err());
        Ok(())
    }
}
