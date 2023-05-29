use crate::{
    frame::{Request, ResponseData},
    router::RequestContext,
};
use std::error::Error;
use std::fmt::Debug;

pub trait Controller {
    fn handle_request(
        &self,
        req: Request,
        context: RequestContext,
    ) -> Result<ResponseData, Box<dyn Error + Send + Sync>>;
}

pub trait PrintableController: Controller + Debug + Send + Sync {}
