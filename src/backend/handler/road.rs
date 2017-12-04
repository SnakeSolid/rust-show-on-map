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
use database::MapRoad;

pub struct RoadHandler {
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
    roads: Vec<ResponseRoad>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

#[derive(Serialize)]
struct ResponseRoad {
    id: i64,
    names: Vec<String>,
    lines: Vec<Vec<ResponsePoint>>,
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
    fn ok(roads: Vec<ResponseRoad>) -> HandlerResponse {
        HandlerResponse {
            ok: true,
            roads: roads,
            message: None,
        }
    }

    fn error<E>(error: E) -> HandlerResponse
    where
        E: Error,
    {
        warn!("{:?}", error);

        HandlerResponse {
            ok: false,
            roads: Vec::with_capacity(0),
            message: Some(error.description().into()),
        }
    }

    fn message<S>(message: S) -> HandlerResponse
    where
        S: Into<String>,
    {
        HandlerResponse {
            ok: false,
            roads: Vec::with_capacity(0),
            message: Some(message.into()),
        }
    }
}

impl ResponseRoad {
    pub fn new(id: i64, names: Vec<String>, lines: Vec<Vec<ResponsePoint>>) -> ResponseRoad {
        ResponseRoad { id, names, lines }
    }
}

impl ResponsePoint {
    pub fn new(lat: f64, lon: f64) -> ResponsePoint {
        ResponsePoint { lat, lon }
    }
}

impl RoadHandler {
    pub fn new(factory: DatabaseFactory) -> RoadHandler {
        RoadHandler { factory: factory }
    }
}

impl Handler for RoadHandler {
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
            let roads = if request.unique() {
                check_error!(
                    client.query_roads_unique(request.ids()),
                    content_type,
                    HandlerResponse::error
                )
            } else {
                check_error!(
                    client.query_roads(request.ids()),
                    content_type,
                    HandlerResponse::error
                )
            };
            let roads = roads.into_iter().map(|road| road.into()).collect();

            response = HandlerResponse::ok(roads);
        }

        let body = check_server_error!(serde_json::to_string(&response));

        Ok(Response::with((content_type, status::Ok, body)))
    }
}

impl Into<ResponseRoad> for MapRoad {
    fn into(self) -> ResponseRoad {
        let id = self.id();
        let names = self.names().clone();
        let lines = collect_points(self.links());

        ResponseRoad::new(id, names, lines)
    }
}

fn collect_points(map_links: &Vec<MapLink>) -> Vec<Vec<ResponsePoint>> {
    let mut result: Vec<Vec<ResponsePoint>> = Vec::default();

    for map_link in map_links {
        let mut result_line = Vec::default();

        for map_point in map_link.points() {
            let point = ResponsePoint::new(
                map_point.lat() as f64 / 100000.0,
                map_point.lon() as f64 / 100000.0,
            );

            result_line.push(point);
        }

        result.push(result_line);
    }

    result
}
