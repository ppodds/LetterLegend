use std::sync::Arc;

use crate::{
    controller::controller::PrintableController,
    frame::{Request, Response},
    router::RequestContext,
    service::player_service::PlayerService,
};

use crate::controller::controller::Controller;
use crate::model::control::heartbeat::HeartbeatResponse;
#[derive(Debug, Clone)]
pub struct HeartbeatController {
    player_service: Arc<PlayerService>,
}

impl HeartbeatController {
    pub fn new(player_service: Arc<PlayerService>) -> Self {
        Self { player_service }
    }
}

impl PrintableController for HeartbeatController {}

impl Controller for HeartbeatController {
    fn handle_request(
        &self,
        req: Request,
        context: RequestContext,
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        assert!(match req {
            Request::Heartbeat => true,
            _ => false,
        });
        let player = match self.player_service.get_player(context.client_id) {
            Some(player) => player,
            None => return Err("Player not found".into()),
        };
        self.player_service.heartbeat(player)?;
        Ok(Response::Heartbeat(HeartbeatResponse { success: true }))
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::service::{game_service::GameService, lobby_service::LobbyService};

    use super::*;

    #[test]
    fn handle_request_with_user_not_exist_should_return_error() -> Result<(), Box<dyn Error>> {
        let controller = HeartbeatController::new(Arc::new(PlayerService::new(
            Arc::new(LobbyService::new()),
            Arc::new(GameService::new()),
        )));
        assert!(controller
            .handle_request(Request::Heartbeat, RequestContext { client_id: 0 })
            .is_err());
        Ok(())
    }
}
