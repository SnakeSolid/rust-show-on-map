use iron::Iron;
use mount::Mount;
use router::Router;
use staticfile::Static;

use database::DatabaseFactory;

use super::error::BackendError;
use super::handler::EmptyHandler;
use super::handler::PlaceHandler;
use super::handler::RoadHandler;

pub fn start_backend(
    factory: DatabaseFactory,
    bind_address: &str,
    bind_port: u16,
) -> Result<(), BackendError> {
    let mut router = Router::new();
    router.post("/place", PlaceHandler::new(factory.clone()), "place");
    router.post("/road", RoadHandler::new(factory), "road");
    router.get("/", EmptyHandler::new(), "empty");

    let mut mount = Mount::new();
    mount.mount("/static", Static::new("public/static"));
    mount.mount("/api/v1", router);
    mount.mount("/", Static::new("public/index.html"));

    info!("Starting WEB server {}:{}", bind_address, bind_port);

    Iron::new(mount).http((bind_address, bind_port))?;

    Ok(())
}
