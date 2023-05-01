use std::sync::Arc;

use crate::model::game::start::StartResponse;
use crate::service::game_service::GameService;
use crate::{
    controller::controller::PrintableController,
    frame::{Request, Response},
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
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        match req {
            Request::StartGame => req,
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
        let game = self.game_service.start_game(player, lobby)?;
        Ok(Response::StartGame(StartResponse {
            success: true,
            board: Some(crate::model::game::board::Board::from(
                game.get_board().lock().unwrap().clone(),
            )),
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::service::lobby_service;

    use super::*;

    #[test]
    fn handle_request_with_test_user_in_test_lobby_should_start_game(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let controller =
            StartController::new(Arc::new(PlayerService::new()), Arc::new(GameService::new()));
        let player = controller
            .player_service
            .add_player(0, String::from("test"));
        let lobby_service = Arc::new(lobby_service::LobbyService::new());
        let lobby = lobby_service.create_lobby(player.clone(), 4)?;
        let lobby_player = lobby.get_player(player.clone().id).unwrap();
        lobby_player.set_ready(true);
        let res =
            match controller.handle_request(Request::StartGame, RequestContext { client_id: 0 })? {
                Response::StartGame(res) => res,
                _ => panic!("invalid response"),
            };
        assert!(res.success);
        Ok(())
    }
}
