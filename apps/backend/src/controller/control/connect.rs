use std::sync::Arc;

use crate::{
    controller::controller::PrintableController,
    frame::{Request, Response},
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
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        let req = match req {
            Request::Connect(req) => req,
            _ => panic!("invalid request"),
        };
        match self.player_service.get_player(context.client_id) {
            Some(_) => return Err("client already connected".into()),
            None => self.player_service.add_player(
                context.client_id,
                req.name,
                #[cfg(not(test))]
                context.sender,
            ),
        };

        Ok(Response::Connect(ConnectResponse { success: true }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::control::connect::ConnectRequest;
    use std::error::Error;

    #[test]
    fn handle_request_with_test_user_should_create_test_user(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let controller = ConnectController::new(Arc::new(PlayerService::new()));
        controller.handle_request(
            Request::Connect(ConnectRequest {
                name: String::from("test"),
            }),
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
        let controller = ConnectController::new(Arc::new(PlayerService::new()));
        controller.handle_request(
            Request::Connect(ConnectRequest {
                name: String::from("test"),
            }),
            RequestContext { client_id: 0 },
        )?;
        assert!(controller
            .handle_request(
                Request::Connect(ConnectRequest {
                    name: String::from("test"),
                }),
                RequestContext { client_id: 0 },
            )
            .is_err());
        Ok(())
    }
}
