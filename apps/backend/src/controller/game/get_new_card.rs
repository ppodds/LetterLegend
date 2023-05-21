use crate::model::game::get_new_card::GetNewCardResponse;
use crate::service::game_service::GameService;
use crate::{
    controller::controller::PrintableController,
    frame::{Request, Response},
    router::RequestContext,
    service::player_service::PlayerService,
};
use std::sync::Arc;

use crate::controller::controller::Controller;
use crate::model::game::card::Card;

#[derive(Debug, Clone)]
pub struct GetNewCardController {
    player_service: Arc<PlayerService>,
    game_service: Arc<GameService>,
}

impl GetNewCardController {
    pub fn new(player_service: Arc<PlayerService>, game_service: Arc<GameService>) -> Self {
        Self {
            player_service,
            game_service,
        }
    }
}

impl PrintableController for GetNewCardController {}

impl Controller for GetNewCardController {
    fn handle_request(
        &self,
        req: Request,
        context: RequestContext,
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        match req {
            Request::GetNewCard => req,
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
        let game_player = match game.get_player(player.id) {
            Some(game_player) => game_player,
            None => return Err("Player not found".into()),
        };
        let turn_player = game.get_player_in_this_turn();
        if turn_player != game_player {
            return Err("Player get new card when not his turn".into());
        };
        if turn_player.get_has_shuffled() {
            return Err("Player has shuffled in this turn".into());
        }
        let cards = self.game_service.shuffle(
            #[cfg(not(test))]
            game,
            turn_player,
        )?;
        Ok(Response::GetNewCard(GetNewCardResponse {
            success: true,
            cards: cards.iter().map(|char| Card::from(char)).collect(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::error::Error;

    use crate::service::lobby_service;

    use super::*;

    #[test]
    fn handle_request_with_test_user_is_not_his_round_should_return_error(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let controller =
            GetNewCardController::new(Arc::new(PlayerService::new()), Arc::new(GameService::new()));
        let player = controller
            .player_service
            .add_player(0, String::from("test"));
        let player1 = controller
            .player_service
            .add_player(1, String::from("test1"));
        let lobby_service = Arc::new(lobby_service::LobbyService::new());
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
                Request::GetNewCard,
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

    #[test]
    fn handle_request_with_test_user_has_shuffled_shuffles_again_should_return_error(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let controller =
            GetNewCardController::new(Arc::new(PlayerService::new()), Arc::new(GameService::new()));
        let player = controller
            .player_service
            .add_player(0, String::from("test"));
        let lobby_service = Arc::new(lobby_service::LobbyService::new());
        let lobby = lobby_service.create_lobby(player.clone(), 4)?;
        let lobby_player = lobby.clone().get_player(player.clone().id).unwrap();
        lobby_player.set_ready(true);
        controller.game_service.start_game(player, lobby)?;
        controller.handle_request(Request::GetNewCard, RequestContext { client_id: 0 })?;
        assert!(controller
            .handle_request(Request::GetNewCard, RequestContext { client_id: 0 })
            .is_err());
        Ok(())
    }
}
