use crate::{
    controller::controller::PrintableController,
    frame::{Request, Response},
    router::RequestContext,
};

use crate::controller::controller::Controller;
use crate::model::control::heartbeat::HeartbeatResponse;
#[derive(Debug, Clone)]
pub struct HeartbeatController {}

impl HeartbeatController {
    pub fn new() -> Self {
        Self {}
    }
}

impl PrintableController for HeartbeatController {}

impl Controller for HeartbeatController {
    fn handle_request(
        &self,
        req: Request,
        _: RequestContext,
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        assert!(match req {
            Request::Heartbeat => true,
            _ => false,
        });
        Ok(Response::Heartbeat(HeartbeatResponse { success: true }))
    }
}
