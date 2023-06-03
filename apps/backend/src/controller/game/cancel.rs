use crate::frame::Request;
use crate::game::card::Card;
use crate::model::game::cancel::CancelResponse;
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
pub struct CancelController {
    player_service: Arc<PlayerService>,
    game_service: Arc<GameService>,
}

impl CancelController {
    pub fn new(player_service: Arc<PlayerService>, game_service: Arc<GameService>) -> Self {
        Self {
            player_service,
            game_service,
        }
    }
}

impl PrintableController for CancelController {}

impl Controller for CancelController {
    fn handle_request(
        &self,
        req: Request,
        context: RequestContext,
    ) -> Result<ResponseData, Box<dyn std::error::Error + Send + Sync>> {
        let data = req.get_data();
        let req = match data.as_ref() {
            RequestData::Cancel(req) => req,
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

        let card =
            match game.get_board().lock().unwrap().tiles[req.x as usize][req.y as usize].clone() {
                Some(card) => card,
                None => return Err("Card not in the hand".into()),
            };
        if card.turn != game.get_turns() {
            return Err("card not place in this turn".into());
        }
        self.game_service.remove_selected_tile(req.x, req.y, game);
        let mut return_card = Card::new(card.char);
        return_card.used = false;
        game_player.return_cancel_card(return_card);
        Ok(ResponseData::Cancel(CancelResponse {
            success: true,
            cards: Some(crate::model::game::cards::Cards::from(
                &game_player.get_cards(),
            )),
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::{
        game::tile::Tile, model::game::cancel::CancelRequest, service::lobby_service::LobbyService,
    };

    use super::*;

    #[test]
    fn handle_request_with_test_card_not_place_in_this_turn_should_return_error(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let lobby_service = Arc::new(LobbyService::new());
        let game_service = Arc::new(GameService::new());
        let controller = CancelController::new(
            Arc::new(PlayerService::new(
                Arc::new(LobbyService::new()),
                game_service.clone(),
            )),
            game_service.clone(),
        );

        let player = controller
            .player_service
            .add_player(0, String::from("test"));
        let lobby = lobby_service.create_lobby(player.clone(), 4)?;
        let lobby_player = lobby.clone().get_player(player.clone().id).unwrap();
        lobby_player.set_ready(true);
        let game = game_service.start_game(player.clone(), lobby.clone())?;
        let tile = Tile::new('z', player, 1);
        controller
            .game_service
            .place_tile_on_board(game.clone(), tile, 1, 1);
        game.next_turn();
        assert!(controller
            .handle_request(
                Request::new(
                    0,
                    Arc::new(RequestData::Cancel(CancelRequest { x: 1, y: 1 }))
                ),
                RequestContext { client_id: 0 },
            )
            .is_err());
        Ok(())
    }

    #[test]
    fn handle_request_with_cancel_card_in_wrong_turn_should_return_err(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let lobby_service = Arc::new(LobbyService::new());
        let game_service = Arc::new(GameService::new());
        let controller = CancelController::new(
            Arc::new(PlayerService::new(
                lobby_service.clone(),
                game_service.clone(),
            )),
            game_service.clone(),
        );

        let player = controller
            .player_service
            .add_player(0, String::from("test"));
        let lobby = lobby_service.create_lobby(player.clone(), 4)?;
        let lobby_player = lobby.clone().get_player(player.clone().id).unwrap();
        lobby_player.set_ready(true);
        let game = game_service.start_game(player.clone(), lobby.clone())?;
        let tile = Tile::new(
            game.get_player_in_this_turn().get_cards()[0].char,
            player,
            game.get_turns(),
        );
        controller
            .game_service
            .place_tile_on_board(game.clone(), tile, 1, 1);
        game.next_turn();
        assert!(controller
            .handle_request(
                Request::new(
                    0,
                    Arc::new(RequestData::Cancel(CancelRequest { x: 1, y: 1 })),
                ),
                RequestContext { client_id: 0 },
            )
            .is_err());
        Ok(())
    }

    #[test]
    fn handle_request_with_cancel_card_should_return_to_cards(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let lobby_service = Arc::new(LobbyService::new());
        let game_service = Arc::new(GameService::new());
        let controller = CancelController::new(
            Arc::new(PlayerService::new(
                lobby_service.clone(),
                game_service.clone(),
            )),
            game_service.clone(),
        );

        let player = controller
            .player_service
            .add_player(0, String::from("test"));
        let lobby = lobby_service.create_lobby(player.clone(), 4)?;
        let lobby_player = lobby.clone().get_player(player.clone().id).unwrap();
        lobby_player.set_ready(true);
        let game = game_service.start_game(player.clone(), lobby.clone())?;
        let tile = Tile::new(
            game.get_player_in_this_turn().get_cards()[0].char,
            player,
            1,
        );
        controller
            .game_service
            .place_tile_on_board(game.clone(), tile, 1, 1);
        controller.handle_request(
            Request::new(
                0,
                Arc::new(RequestData::Cancel(CancelRequest { x: 1, y: 1 })),
            ),
            RequestContext { client_id: 0 },
        )?;
        assert!(game.get_player_in_this_turn().get_cards()[0].used == false);
        Ok(())
    }
}
