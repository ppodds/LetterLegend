use std::sync::Arc;

use crate::{
    controller::controller::PrintableController,
    frame::{Request, RequestData, ResponseData},
    model::control::connect::ConnectResponse,
    router::RequestContext,
    service::player_service::PlayerService,
};

use crate::controller::controller::Controller;

#[derive(Debug, Clone)]
pub struct ConnectController {
    player_service: Arc<PlayerService>,
}

impl ConnectController {
    pub fn new(player_service: Arc<PlayerService>) -> Self {
        Self { player_service }
    }
}

impl PrintableController for ConnectController {}

impl Controller for ConnectController {
    fn handle_request(
        &self,
        req: Request,
        context: RequestContext,
    ) -> Result<ResponseData, Box<dyn std::error::Error + Send + Sync>> {
        let data = req.get_data();
        let req = match data.as_ref() {
            RequestData::Connect(req) => req,
            _ => panic!("invalid request"),
        };
        let player = match self.player_service.get_player(context.client_id) {
            Some(_) => return Err("client already connected".into()),
            None => self.player_service.add_player(
                context.client_id,
                req.name.clone(),
                #[cfg(not(test))]
                context.sender,
            ),
        };

        Ok(ResponseData::Connect(ConnectResponse {
            success: true,
            player: Some(crate::model::player::player::Player::from(player)),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        model::control::connect::ConnectRequest,
        service::{game_service::GameService, lobby_service::LobbyService},
    };
    use std::{collections::HashSet, error::Error};

    #[test]
    fn handle_request_with_test_user_should_create_test_user(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let controller = ConnectController::new(Arc::new(PlayerService::new(
            Arc::new(LobbyService::new()),
            Arc::new(GameService::new(HashSet::new())),
        )));
        controller.handle_request(
            Request::new(
                0,
                Arc::new(RequestData::Connect(ConnectRequest {
                    name: String::from("test"),
                })),
            ),
            RequestContext { client_id: 0 },
        )?;
        let player = controller.player_service.get_player(0).unwrap();
        assert_eq!(player.id, 0);
        assert_eq!(player.name, String::from("test"));
        Ok(())
    }

    #[test]
    fn handle_request_with_test_user_who_already_connected_should_return_error(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let controller = ConnectController::new(Arc::new(PlayerService::new(
            Arc::new(LobbyService::new()),
            Arc::new(GameService::new(HashSet::new())),
        )));
        controller.handle_request(
            Request::new(
                0,
                Arc::new(RequestData::Connect(ConnectRequest {
                    name: String::from("test"),
                })),
            ),
            RequestContext { client_id: 0 },
        )?;
        assert!(controller
            .handle_request(
                Request::new(
                    0,
                    Arc::new(RequestData::Connect(ConnectRequest {
                        name: String::from("test"),
                    }))
                ),
                RequestContext { client_id: 0 },
            )
            .is_err());
        Ok(())
    }
}
