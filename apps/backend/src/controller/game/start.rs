use std::sync::Arc;

use crate::frame::Request;
#[cfg(not_test)]
use crate::game::game_player;
use crate::model::game::start::StartResponse;
use crate::service::game_service::GameService;
use crate::{
    controller::controller::PrintableController,
    frame::{RequestData, ResponseData},
    router::RequestContext,
    service::player_service::PlayerService,
};

use crate::controller::controller::Controller;

#[derive(Debug, Clone)]
pub struct StartController {
    player_service: Arc<PlayerService>,
    game_service: Arc<GameService>,
}

impl StartController {
    pub fn new(player_service: Arc<PlayerService>, game_service: Arc<GameService>) -> Self {
        Self {
            player_service,
            game_service,
        }
    }
}

impl PrintableController for StartController {}

impl Controller for StartController {
    fn handle_request(
        &self,
        req: Request,
        context: RequestContext,
    ) -> Result<ResponseData, Box<dyn std::error::Error + Send + Sync>> {
        match *req.get_data() {
            RequestData::StartGame => req,
            _ => panic!("invalid request"),
        };

        let player = match self.player_service.get_player(context.client_id) {
            Some(player) => player,
            None => return Err("Player not found".into()),
        };
        let lobby = match player.get_lobby() {
            Some(lobby) => lobby,
            None => return Err("Player not in lobby".into()),
        };
        let game = self.game_service.start_game(player.clone(), lobby)?;
        let game_player = match game.get_player(player.id) {
            Some(game_player) => game_player,
            None => return Err("find no player".into()),
        };

        Ok(ResponseData::StartGame(StartResponse {
            success: true,
            board: Some(crate::model::game::board::Board::from(
                &*game.get_board().lock().unwrap(),
            )),
            cards: Some(crate::model::game::cards::Cards::from(
                &game_player.get_cards(),
            )),
            current_player: Some(crate::model::player::player::Player::from(game_player)),
            next_player: Some(crate::model::player::player::Player::from(
                game.get_next_turn_player(),
            )),
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::service::lobby_service::{self, LobbyService};

    use super::*;

    #[test]
    fn handle_request_with_test_user_in_test_lobby_should_start_game(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let controller = StartController::new(
            Arc::new(PlayerService::new(
                Arc::new(LobbyService::new()),
                Arc::new(GameService::new()),
            )),
            Arc::new(GameService::new()),
        );
        let player = controller
            .player_service
            .add_player(0, String::from("test"));
        let lobby_service = Arc::new(lobby_service::LobbyService::new());
        let lobby = lobby_service.create_lobby(player.clone(), 4)?;
        let lobby_player = lobby.get_player(player.clone().id).unwrap();
        lobby_player.set_ready(true);
        let res = match controller.handle_request(
            Request::new(0, Arc::new(RequestData::StartGame)),
            RequestContext { client_id: 0 },
        )? {
            ResponseData::StartGame(res) => res,
            _ => panic!("invalid response"),
        };
        assert!(res.success);
        Ok(())
    }
}
