use crate::algorithm::collect_polygon;
use crate::database::DatabaseFactory;
use crate::database::MapPlace;
use crate::database::MapPoint;
use iron::mime::Mime;
use iron::status;
use iron::Handler;
use iron::IronResult;
use iron::Request;
use iron::Response;
use serde_json;
use std::fmt::Display;
use std::io::Read;

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
    ids: Vec<i64>,
    unique: bool,
}

#[derive(Serialize)]
struct HandlerResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    places: Option<Vec<ResponsePlace>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

#[derive(Serialize)]
struct ResponsePlace {
    id: i64,
    name: String,
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

    fn ids(&self) -> &Vec<i64> {
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
            places: Some(places),
            message: None,
        }
    }

    fn error<D>(error: D) -> HandlerResponse
    where
        D: Display,
    {
        HandlerResponse {
            ok: false,
            places: None,
            message: Some(format!("{}.", error)),
        }
    }
}

impl ResponsePlace {
    pub fn new(id: i64, name: String, polygons: Vec<Vec<ResponsePoint>>) -> ResponsePlace {
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

impl From<MapPoint> for ResponsePoint {
    fn from(point: MapPoint) -> Self {
        ResponsePoint::new(point.lat() as f64 / 100000.0, point.lon() as f64 / 100000.0)
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
            response = HandlerResponse::error("Request must contain at least one id")
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
        let polygons = self
            .polygons()
            .iter()
            .map(|polygon| collect_polygon(polygon.links()))
            .filter_map(|links| links)
            .map(|links| links.into_iter().map(|point| point.into()).collect())
            .collect();

        ResponsePlace::new(id, name, polygons)
    }
}
