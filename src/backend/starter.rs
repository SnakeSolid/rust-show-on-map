use super::EmptyHandler;
use super::FormatHandler;
use super::ObjectHandler;
use crate::config::ConfigRef;
use iron::Iron;
use mount::Mount;
use router::Router;
use staticfile::Static;

pub fn start_backend(config: ConfigRef, address: &str, port: u16) {
    let mut router = Router::new();
    router
        .post("/format", FormatHandler::new(config.clone()), "format")
        .post("/object", ObjectHandler::new(config.clone()), "object")
        .get("/", EmptyHandler::new(), "empty");

    let mut mount = Mount::new();
    mount
        .mount("/api/v1", router)
        .mount("/static", Static::new("public/static"))
        .mount("/", Static::new("public/index.html"));

    println!("Starting HTTP server on {}:{}", address, port);

    match Iron::new(mount).http((address, port)) {
        Ok(_) => {}
        Err(err) => error!("Failed to start HTTP server: {}", err),
    }
}
