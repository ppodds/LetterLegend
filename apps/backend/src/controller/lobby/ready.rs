use std::sync::Arc;

use crate::frame::Request;
use crate::model::lobby::ready::ReadyResponse;
use crate::{
    controller::controller::PrintableController,
    frame::{RequestData, ResponseData},
    router::RequestContext,
    service::player_service::PlayerService,
};

use crate::controller::controller::Controller;

#[derive(Debug, Clone)]
pub struct ReadyController {
    player_service: Arc<PlayerService>,
}

impl ReadyController {
    pub fn new(player_service: Arc<PlayerService>) -> Self {
        Self { player_service }
    }
}

impl PrintableController for ReadyController {}

impl Controller for ReadyController {
    fn handle_request(
        &self,
        req: Request,
        context: RequestContext,
    ) -> Result<ResponseData, Box<dyn std::error::Error + Send + Sync>> {
        match *req.get_data() {
            RequestData::Ready => req,
            _ => panic!("invalid request"),
        };

        let player = match self.player_service.get_player(context.client_id) {
            Some(player) => player,
            None => return Err("Player not found".into()),
        };

        match player.get_lobby() {
            Some(lobby) => match lobby.get_player(player.id) {
                Some(lobby_player) => {
                    {
                        lobby_player.set_ready(!lobby_player.get_ready());
                    }
                    Ok(ResponseData::Ready(ReadyResponse { success: true }))
                }
                None => panic!("Player in lobby but LobbyPlayer not found"),
            },
            None => Err("Player not in lobby".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, error::Error};

    use crate::service::{game_service::GameService, lobby_service::LobbyService};

    use super::*;

    #[test]
    fn handle_request_with_test_user_in_test_lobby_should_ready(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let controller = ReadyController::new(Arc::new(PlayerService::new(
            Arc::new(LobbyService::new()),
            Arc::new(GameService::new(HashSet::new())),
        )));
        let lobby_service = LobbyService::new();
        let leader = controller
            .player_service
            .add_player(0, String::from("test"));
        let lobby = lobby_service.create_lobby(leader, 4)?;
        controller.handle_request(
            Request::new(0, Arc::new(RequestData::Ready)),
            RequestContext { client_id: 0 },
        )?;
        assert!(lobby.get_player(0).unwrap().get_ready());
        Ok(())
    }

    #[test]
    fn handle_request_with_test_user_in_test_lobby_should_not_ready(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let controller = ReadyController::new(Arc::new(PlayerService::new(
            Arc::new(LobbyService::new()),
            Arc::new(GameService::new(HashSet::new())),
        )));
        let lobby_service = LobbyService::new();
        let leader = controller
            .player_service
            .add_player(0, String::from("test"));
        let lobby = lobby_service.create_lobby(leader, 4)?;
        lobby.get_player(0).unwrap().set_ready(true);
        controller.handle_request(
            Request::new(0, Arc::new(RequestData::Ready)),
            RequestContext { client_id: 0 },
        )?;
        assert!(!lobby.get_player(0).unwrap().get_ready());
        Ok(())
    }
}
