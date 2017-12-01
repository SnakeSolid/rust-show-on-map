use std::error::Error;

use iron::Handler;
use iron::IronResult;
use iron::mime::Mime;
use iron::Request;
use iron::Response;
use iron::status;
use serde_json;

use database::DatabaseFactory;

pub struct ShapeHandler {
    factory: DatabaseFactory,
}

#[derive(Serialize)]
struct HandlerResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    admin_places: Vec<ResponseShape>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    roads: Vec<ResponseShape>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

#[derive(Serialize)]
struct ResponseShape {
    id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    polygons: Vec<Vec<ResponsePoint>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    lines: Vec<Vec<ResponsePoint>>,
}

#[derive(Serialize)]
struct ResponsePoint {
    lat: f64,
    lon: f64,
}

impl HandlerResponse {
    fn ok(admin_places: Vec<ResponseShape>, roads: Vec<ResponseShape>) -> HandlerResponse {
        HandlerResponse {
            ok: true,
            admin_places: admin_places,
            roads: roads,
            message: None,
        }
    }

    fn error<S>(error: S) -> HandlerResponse
    where
        S: Into<String>,
    {
        HandlerResponse {
            ok: false,
            admin_places: Vec::with_capacity(0),
            roads: Vec::with_capacity(0),
            message: Some(error.into()),
        }
    }
}

impl ShapeHandler {
    pub fn new(factory: DatabaseFactory) -> ShapeHandler {
        ShapeHandler { factory: factory }
    }
}

impl Handler for ShapeHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let content_type: Mime = check_text!("application/json".parse(), "MIME type parsing error");
        let response = HandlerResponse::error("Dummy response");
        let body = check_error!(serde_json::to_string(&response));

        Ok(Response::with((content_type, status::Ok, body)))
    }
}
