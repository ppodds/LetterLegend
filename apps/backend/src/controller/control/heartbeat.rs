use crate::{
    controller::controller::PrintableController,
    frame::{Request, RequestData, ResponseData},
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
    ) -> Result<ResponseData, Box<dyn std::error::Error + Send + Sync>> {
        assert!(match *req.get_data() {
            RequestData::Heartbeat => true,
            _ => false,
        });
        Ok(ResponseData::Heartbeat(HeartbeatResponse { success: true }))
    }
}
