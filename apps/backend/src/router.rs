use crate::controller::controller::PrintableController;
use crate::frame::{Request, ResponseData};
use crate::operation::Operation;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

#[cfg(not(test))]
use crate::frame::Frame;
#[cfg(not(test))]
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub struct Router {
    controllers: Arc<RwLock<HashMap<Operation, Box<dyn PrintableController>>>>,
}

#[derive(Debug)]
pub struct RequestContext {
    pub client_id: u32,
    #[cfg(not(test))]
    pub sender: Sender<Frame>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            controllers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register_controller(
        &self,
        operation: Operation,
        controller: Box<dyn PrintableController>,
    ) -> &Self {
        self.controllers
            .write()
            .unwrap()
            .insert(operation, controller);
        self
    }

    pub fn route(
        &self,
        request: Request,
        context: RequestContext,
    ) -> Result<ResponseData, Box<dyn Error + Sync + Send>> {
        match self
            .controllers
            .read()
            .unwrap()
            .get(&Operation::try_from(request.get_data().as_ref())?)
        {
            Some(controller) => match controller.handle_request(request, context) {
                Ok(response) => Ok(response),
                Err(err) => Ok(ResponseData::Error(crate::model::error::error::Error {
                    message: err.to_string(),
                })),
            },
            None => Err(format!("no controller for request {:?}", request).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{
        controller::control::connect::ConnectController,
        service::{
            game_service::GameService, lobby_service::LobbyService, player_service::PlayerService,
        },
    };

    use super::*;

    #[test]
    fn register_controller_with_controller_and_operation_controller_should_be_added() {
        let router = Router::new();
        router.register_controller(
            Operation::Connect,
            Box::new(ConnectController::new(Arc::new(PlayerService::new(
                Arc::new(LobbyService::new()),
                Arc::new(GameService::new(HashSet::new())),
            )))),
        );
        assert!(router
            .controllers
            .read()
            .unwrap()
            .contains_key(&Operation::Connect));
    }
}
