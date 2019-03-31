use iron::status;
use iron::Handler;
use iron::IronResult;
use iron::Request;
use iron::Response;

pub struct EmptyHandler;

impl EmptyHandler {
    pub fn new() -> EmptyHandler {
        EmptyHandler {}
    }
}

impl Handler for EmptyHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::BadRequest, "No API entry point")))
    }
}
