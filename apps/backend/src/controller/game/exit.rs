use crate::frame::Request;
use crate::model::game::exit::ExitResponse;
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
pub struct ExitController {
    player_service: Arc<PlayerService>,
    game_service: Arc<GameService>,
}

impl ExitController {
    pub fn new(player_service: Arc<PlayerService>, game_service: Arc<GameService>) -> Self {
        Self {
            player_service,
            game_service,
        }
    }
}

impl PrintableController for ExitController {}

impl Controller for ExitController {
    fn handle_request(
        &self,
        req: Request,
        context: RequestContext,
    ) -> Result<ResponseData, Box<dyn std::error::Error + Send + Sync>> {
        let data = req.get_data();
        match data.as_ref() {
            RequestData::Exit => true,
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
        match game.get_player(player.id) {
            Some(game_player) => game_player,
            None => return Err("Player not found".into()),
        };
        match self.game_service.remove_player_from_game(player.clone()) {
            Ok(_) => Ok(ResponseData::Exit(ExitResponse { success: true })),
            Err(_) => Ok(ResponseData::Exit(ExitResponse { success: false })),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, error::Error};

    use crate::service::lobby_service::LobbyService;

    use super::*;

    #[tokio::test]
    async fn handle_request_with_test_player_in_game_exit_should_success(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let game_service = Arc::new(GameService::new(HashSet::new()));
        let lobby_service = Arc::new(LobbyService::new());
        let controller = ExitController::new(
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
        GameService::start_game(game_service, player.clone(), lobby)?;
        let res = controller.handle_request(
            Request::new(0, Arc::new(RequestData::Exit)),
            RequestContext {
                client_id: player.id,
            },
        )?;
        match res {
            ResponseData::Exit(data) => {
                let success = data.success;
                assert!(success)
            }
            _ => panic!("wrong response type"),
        }
        Ok(())
    }

    #[tokio::test]
    async fn handle_request_with_test_player_not_in_game_exit_should_failed(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let game_service = Arc::new(GameService::new(HashSet::new()));
        let lobby_service = Arc::new(LobbyService::new());
        let controller = ExitController::new(
            Arc::new(PlayerService::new(
                lobby_service.clone(),
                game_service.clone(),
            )),
            game_service,
        );
        let player = controller
            .player_service
            .add_player(0, String::from("test"));
        let lobby = lobby_service.create_lobby(player.clone(), 4)?;
        let lobby_player = lobby.clone().get_player(player.clone().id).unwrap();
        lobby_player.set_ready(true);
        assert!(controller
            .handle_request(
                Request::new(0, Arc::new(RequestData::Exit)),
                RequestContext {
                    client_id: player.id,
                },
            )
            .is_err());
        Ok(())
    }
}
