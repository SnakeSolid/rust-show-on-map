use super::error::BackendError;
use super::handler::EmptyHandler;
use super::handler::PlaceHandler;
use super::handler::RoadHandler;
use crate::database::DatabaseFactory;
use iron::Iron;
use mount::Mount;
use router::Router;
use staticfile::Static;

pub fn start_backend(
    factory: DatabaseFactory,
    bind_address: &str,
    bind_port: u16,
) -> Result<(), BackendError> {
    let mut router = Router::new();
    router
        .post("/place", PlaceHandler::new(factory.clone()), "place")
        .post("/road", RoadHandler::new(factory), "road")
        .get("/", EmptyHandler::new(), "empty");

    let mut mount = Mount::new();
    mount
        .mount("/api/v1", router)
        .mount("/static", Static::new("public/static"))
        .mount("/", Static::new("public/index.html"));

    println!("Starting WEB server {}:{}", bind_address, bind_port);

    Iron::new(mount).http((bind_address, bind_port))?;

    Ok(())
}
