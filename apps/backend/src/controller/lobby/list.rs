use std::sync::Arc;

use crate::model::lobby::list::ListResponse;
use crate::{
    controller::controller::PrintableController,
    frame::{Request, Response},
    router::RequestContext,
    service::lobby_service::LobbyService,
};

use crate::controller::controller::Controller;

#[derive(Debug, Clone)]
pub struct ListController {
    lobby_service: Arc<LobbyService>,
}

impl ListController {
    pub fn new(lobby_service: Arc<LobbyService>) -> Self {
        Self { lobby_service }
    }
}

impl PrintableController for ListController {}

impl Controller for ListController {
    fn handle_request(
        &self,
        req: Request,
        _: RequestContext,
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        match req {
            Request::ListLobby => req,
            _ => panic!("invalid request"),
        };
        Ok(Response::ListLobby(ListResponse {
            success: true,
            lobby_infos: Some(crate::model::lobby::list::LobbyInfos::from(
                self.lobby_service.get_lobbies(),
            )),
        }))
    }
}
