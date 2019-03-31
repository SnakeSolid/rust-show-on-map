use crate::backend::handler::util::handle_empty;
use crate::config::ConfigRef;
use iron::Handler;
use iron::IronResult;
use iron::Request as IronRequest;
use iron::Response as IronResponse;

pub struct FormatHandler {
    config: ConfigRef,
}

impl FormatHandler {
    pub fn new(config: ConfigRef) -> FormatHandler {
        FormatHandler { config }
    }
}

impl Handler for FormatHandler {
    fn handle(&self, _request: &mut IronRequest) -> IronResult<IronResponse> {
        handle_empty(move || {
            let result: Vec<_> = self.config.formats().keys().cloned().collect();

            Ok(result)
        })
    }
}
