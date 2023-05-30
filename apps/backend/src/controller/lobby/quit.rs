use std::sync::Arc;

use crate::frame::Request;
use crate::model::lobby::quit::QuitResponse;
use crate::{
    controller::controller::PrintableController,
    frame::{RequestData, ResponseData},
    router::RequestContext,
    service::{lobby_service::LobbyService, player_service::PlayerService},
};

use crate::controller::controller::Controller;

#[derive(Debug, Clone)]
pub struct QuitController {
    player_service: Arc<PlayerService>,
    lobby_service: Arc<LobbyService>,
}

impl QuitController {
    pub fn new(player_service: Arc<PlayerService>, lobby_service: Arc<LobbyService>) -> Self {
        Self {
            player_service,
            lobby_service,
        }
    }
}

impl PrintableController for QuitController {}

impl Controller for QuitController {
    fn handle_request(
        &self,
        req: Request,
        context: RequestContext,
    ) -> Result<ResponseData, Box<dyn std::error::Error + Send + Sync>> {
        match *req.get_data() {
            RequestData::QuitLobby => req,
            _ => panic!("invalid request"),
        };
        let player = match self.player_service.get_player(context.client_id) {
            Some(player) => player,
            None => return Err("Player not found".into()),
        };
        self.lobby_service.remove_player_from_lobby(player)?;
        Ok(ResponseData::QuitLobby(QuitResponse { success: true }))
    }
}
