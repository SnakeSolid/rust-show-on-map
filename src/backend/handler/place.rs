use std::error::Error;
use std::io::Read;

use iron::Handler;
use iron::IronResult;
use iron::mime::Mime;
use iron::Request;
use iron::Response;
use iron::status;
use serde_json;

use database::DatabaseFactory;
use database::MapLink;
use database::MapPlace;

pub struct PlaceHandler {
    factory: DatabaseFactory,
}

#[derive(Deserialize)]
struct HandlerRequest {
    host: String,
    port: i16,
    database: String,
    role: String,
    password: String,
    ids: Vec<i32>,
    unique: bool,
}

#[derive(Serialize)]
struct HandlerResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    places: Vec<ResponsePlace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

#[derive(Serialize)]
struct ResponsePlace {
    id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    polygons: Vec<Vec<ResponsePoint>>,
}

#[derive(Serialize)]
struct ResponsePoint {
    lat: f64,
    lon: f64,
}

impl HandlerRequest {
    fn host(&self) -> &str {
        &self.host
    }

    fn port(&self) -> i16 {
        self.port
    }

    fn database(&self) -> &str {
        &self.database
    }

    fn role(&self) -> &str {
        &self.role
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn ids(&self) -> &Vec<i32> {
        &self.ids
    }

    fn unique(&self) -> bool {
        self.unique
    }
}

impl HandlerResponse {
    fn ok(places: Vec<ResponsePlace>) -> HandlerResponse {
        HandlerResponse {
            ok: true,
            places: places,
            message: None,
        }
    }

    fn error<E>(error: E) -> HandlerResponse
    where
        E: Error,
    {
        HandlerResponse {
            ok: false,
            places: Vec::with_capacity(0),
            message: Some(error.description().into()),
        }
    }

    fn message<S>(message: S) -> HandlerResponse
    where
        S: Into<String>,
    {
        HandlerResponse {
            ok: false,
            places: Vec::with_capacity(0),
            message: Some(message.into()),
        }
    }
}

impl ResponsePlace {
    pub fn new(id: i32, name: Option<String>, polygons: Vec<Vec<ResponsePoint>>) -> ResponsePlace {
        ResponsePlace { id, name, polygons }
    }
}

impl ResponsePoint {
    pub fn new(lat: f64, lon: f64) -> ResponsePoint {
        ResponsePoint { lat, lon }
    }
}

impl PlaceHandler {
    pub fn new(factory: DatabaseFactory) -> PlaceHandler {
        PlaceHandler { factory: factory }
    }
}

impl Handler for PlaceHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let content_type: Mime = check_text!("application/json".parse(), "MIME type parsing error");
        let mut body = String::default();

        check_error!(
            request.body.read_to_string(&mut body),
            content_type,
            HandlerResponse::error
        );

        let request: HandlerRequest = check_error!(
            serde_json::from_str(&body),
            content_type,
            HandlerResponse::error
        );
        let response;

        if request.ids().is_empty() {
            response = HandlerResponse::message("Request must contain at least one id.")
        } else {
            let client = self.factory.client(
                request.host(),
                request.port(),
                request.database(),
                request.role(),
                request.password(),
            );
            let places = if request.unique() {
                check_error!(
                    client.query_places_unique(request.ids()),
                    content_type,
                    HandlerResponse::error
                )
            } else {
                check_error!(
                    client.query_places(request.ids()),
                    content_type,
                    HandlerResponse::error
                )
            };
            let places = places.into_iter().map(|place| place.into()).collect();

            response = HandlerResponse::ok(places);
        }

        let body = check_server_error!(serde_json::to_string(&response));

        Ok(Response::with((content_type, status::Ok, body)))
    }
}

impl Into<ResponsePlace> for MapPlace {
    fn into(self) -> ResponsePlace {
        let id = self.id();
        let name = self.name().clone();
        let mut polygons = Vec::default();

        for map_polygon in self.polygons() {
            let map_links = map_polygon.links();
            let polygon = match map_links.len() {
                1 => collect_points_all(map_links),
                2 => collect_points_all(map_links),
                _ => collect_points_all(map_links),
            };

            polygons.push(polygon);
        }

        ResponsePlace::new(id, Some(name), polygons)
    }
}

fn collect_points_all(map_links: &Vec<MapLink>) -> Vec<ResponsePoint> {
    let mut result = Vec::default();

    for map_link in map_links {
        for map_point in map_link.points() {
            let point = ResponsePoint::new(
                map_point.lat() as f64 / 100000.0,
                map_point.lon() as f64 / 100000.0,
            );

            result.push(point);
        }
    }

    result
}
