use std::sync::Arc;

use crate::model::lobby::join::JoinResponse;
use crate::{
    controller::controller::PrintableController,
    frame::{Request, Response},
    router::RequestContext,
    service::{lobby_service::LobbyService, player_service::PlayerService},
};

use crate::controller::controller::Controller;

#[derive(Debug, Clone)]
pub struct JoinController {
    player_service: Arc<PlayerService>,
    lobby_service: Arc<LobbyService>,
}

impl JoinController {
    pub fn new(player_service: Arc<PlayerService>, lobby_service: Arc<LobbyService>) -> Self {
        Self {
            player_service,
            lobby_service,
        }
    }
}

impl PrintableController for JoinController {}

impl Controller for JoinController {
    fn handle_request(
        &self,
        req: Request,
        context: RequestContext,
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        let req = match req {
            Request::JoinLobby(req) => req,
            _ => panic!("invalid request"),
        };
        let player = match self.player_service.get_player(context.client_id) {
            Some(player) => player,
            None => return Err("Player not found".into()),
        };
        let lobby = match self.lobby_service.get_lobby(req.lobby_id) {
            Some(lobby) => lobby,
            None => return Err("Lobby not found".into()),
        };
        self.lobby_service
            .add_player_to_lobby(player, lobby.clone())?;
        Ok(Response::JoinLobby(JoinResponse {
            success: true,
            lobby: Some(crate::model::lobby::lobby::Lobby::from(lobby)),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::lobby::join::JoinRequest;
    use crate::player::Player;
    use crate::service::game_service::GameService;
    use std::error::Error;

    #[test]
    fn handle_request_with_test_user_should_join_lobby() -> Result<(), Box<dyn Error + Send + Sync>>
    {
        let player_service = Arc::new(PlayerService::new(
            Arc::new(LobbyService::new()),
            Arc::new(GameService::new()),
        ));
        let leader = player_service.add_player(0, String::from("test1"));
        player_service.add_player(1, String::from("test2"));
        let lobby_service = Arc::new(LobbyService::new());
        lobby_service.create_lobby(leader, 4)?;
        let controller = JoinController::new(player_service, lobby_service);
        let res = match controller.handle_request(
            Request::JoinLobby(JoinRequest { lobby_id: 0 }),
            RequestContext { client_id: 1 },
        )? {
            Response::JoinLobby(res) => res,
            _ => panic!("invalid response"),
        };
        assert_eq!(res.success, true);
        assert_eq!(res.lobby.clone().unwrap().id, 0);
        assert_eq!(res.lobby.unwrap().players.len(), 2);
        Ok(())
    }

    #[test]
    fn handle_request_with_not_exist_user_and_test_lobby_should_return_error(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let player_service = Arc::new(PlayerService::new(
            Arc::new(LobbyService::new()),
            Arc::new(GameService::new()),
        ));
        player_service.add_player(0, String::from("test1"));
        let lobby_service = Arc::new(LobbyService::new());
        lobby_service.create_lobby(Arc::new(Player::new(0, String::from("test"))), 4)?;
        let controller = JoinController::new(player_service, lobby_service);
        assert!(controller
            .handle_request(
                Request::JoinLobby(JoinRequest { lobby_id: 0 }),
                RequestContext { client_id: 1 },
            )
            .is_err());
        Ok(())
    }

    #[test]
    fn handle_request_with_test_user_and_not_exist_lobby_should_return_error(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let player_service = Arc::new(PlayerService::new(
            Arc::new(LobbyService::new()),
            Arc::new(GameService::new()),
        ));
        player_service.add_player(0, String::from("test"));
        let lobby_service = Arc::new(LobbyService::new());
        let controller = JoinController::new(player_service, lobby_service);
        assert!(controller
            .handle_request(
                Request::JoinLobby(JoinRequest { lobby_id: 0 }),
                RequestContext { client_id: 0 },
            )
            .is_err());
        Ok(())
    }

    #[test]
    fn handle_request_with_not_exist_user_and_not_exist_lobby_should_return_error(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let player_service = Arc::new(PlayerService::new(
            Arc::new(LobbyService::new()),
            Arc::new(GameService::new()),
        ));
        let controller = JoinController::new(player_service, Arc::new(LobbyService::new()));
        assert!(controller
            .handle_request(
                Request::JoinLobby(JoinRequest { lobby_id: 0 }),
                RequestContext { client_id: 0 },
            )
            .is_err());
        Ok(())
    }
}
