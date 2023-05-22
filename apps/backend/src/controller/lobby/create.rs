use std::sync::Arc;

use crate::model::lobby::create::CreateResponse;
use crate::{
    controller::controller::PrintableController,
    frame::{Request, Response},
    router::RequestContext,
    service::{lobby_service::LobbyService, player_service::PlayerService},
};

use crate::controller::controller::Controller;

#[derive(Debug, Clone)]
pub struct CreateController {
    player_service: Arc<PlayerService>,
    lobby_service: Arc<LobbyService>,
}

impl CreateController {
    pub fn new(player_service: Arc<PlayerService>, lobby_service: Arc<LobbyService>) -> Self {
        Self {
            player_service,
            lobby_service,
        }
    }
}

impl PrintableController for CreateController {}

impl Controller for CreateController {
    fn handle_request(
        &self,
        req: Request,
        context: RequestContext,
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        let req = match req {
            Request::CreateLobby(req) => req,
            _ => panic!("invalid request"),
        };
        let leader = match self.player_service.get_player(context.client_id) {
            Some(player) => player,
            None => return Err("Player not found".into()),
        };
        let lobby = self.lobby_service.create_lobby(leader, req.max_players)?;

        Ok(Response::CreateLobby(CreateResponse {
            success: true,
            lobby: Some(crate::model::lobby::lobby::Lobby::from(lobby)),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::lobby::create::CreateRequest, service::game_service::GameService};
    use std::error::Error;

    #[test]
    fn handle_request_with_test_user_should_create_lobby(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let player_service = Arc::new(PlayerService::new(
            Arc::new(LobbyService::new()),
            Arc::new(GameService::new()),
        ));
        player_service.add_player(0, String::from("test"));
        let controller = CreateController::new(player_service, Arc::new(LobbyService::new()));
        let res = match controller.handle_request(
            Request::CreateLobby(CreateRequest { max_players: 4 }),
            RequestContext { client_id: 0 },
        )? {
            Response::CreateLobby(res) => res,
            _ => panic!("invalid response"),
        };
        assert_eq!(res.success, true);
        assert_eq!(res.lobby.unwrap().id, 0);
        Ok(())
    }

    #[test]
    fn handle_request_with_test_user_and_invaild_max_players_should_return_error(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let player_service = Arc::new(PlayerService::new(
            Arc::new(LobbyService::new()),
            Arc::new(GameService::new()),
        ));
        player_service.add_player(0, String::from("test"));
        let controller = CreateController::new(player_service, Arc::new(LobbyService::new()));
        assert!(controller
            .handle_request(
                Request::CreateLobby(CreateRequest { max_players: 3 }),
                RequestContext { client_id: 0 },
            )
            .is_err());
        assert!(controller
            .handle_request(
                Request::CreateLobby(CreateRequest { max_players: 9 }),
                RequestContext { client_id: 0 },
            )
            .is_err());
        Ok(())
    }

    #[test]
    fn handle_request_with_not_exist_user_should_return_error(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let player_service = Arc::new(PlayerService::new(
            Arc::new(LobbyService::new()),
            Arc::new(GameService::new()),
        ));
        let controller = CreateController::new(player_service, Arc::new(LobbyService::new()));
        assert!(controller
            .handle_request(
                Request::CreateLobby(CreateRequest { max_players: 4 }),
                RequestContext { client_id: 0 },
            )
            .is_err());
        Ok(())
    }

    #[test]
    fn handle_request_with_test_user_should_contains_test_user(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let player_service = Arc::new(PlayerService::new(
            Arc::new(LobbyService::new()),
            Arc::new(GameService::new()),
        ));
        let player = player_service.add_player(0, String::from("test"));
        let controller = CreateController::new(player_service, Arc::new(LobbyService::new()));
        let res = match controller.handle_request(
            Request::CreateLobby(CreateRequest { max_players: 4 }),
            RequestContext { client_id: 0 },
        )? {
            Response::CreateLobby(res) => res,
            _ => panic!("invalid response"),
        };
        assert_eq!(
            res.lobby.unwrap().players[0],
            crate::model::player::player::Player::from(player)
        );
        Ok(())
    }
}
