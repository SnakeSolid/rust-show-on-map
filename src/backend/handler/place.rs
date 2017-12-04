use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
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
use database::MapPoint;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
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
    pub fn new(id: i64, name: Option<String>, polygons: Vec<Vec<ResponsePoint>>) -> ResponsePlace {
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
        let mut polygons = Vec::default();

        for map_polygon in self.polygons() {
            let map_links = map_polygon.links();
            let polygon = collect_points(map_links);

            polygons.push(polygon);
        }

        ResponsePlace::new(id, Some(name), polygons)
    }
}

fn collect_points(map_links: &Vec<MapLink>) -> Vec<ResponsePoint> {
    let mut link_connections: HashMap<MapPoint, Vec<_>> = HashMap::default();

    // Collect adjacent links of polygon.
    for (i, map_link) in map_links.iter().enumerate() {
        let map_points = map_link.points();

        if let Some(map_point) = map_points.first() {
            link_connections
                .entry(map_point.clone())
                .or_insert_with(|| Vec::default())
                .push(i);
        }

        if let Some(map_point) = map_points.last() {
            link_connections
                .entry(map_point.clone())
                .or_insert_with(|| Vec::default())
                .push(i);
        }
    }

    let mut used_links: HashSet<usize> = HashSet::default();
    let mut result: Vec<ResponsePoint> = Vec::default();

    let mut current_link_id = 0;
    let current_link = &map_links[current_link_id];
    let mut current_point: &MapPoint = current_link.points().first().unwrap();

    // Loop over all links and connect adjacent links to single loop.
    loop {
        used_links.insert(current_link_id);

        let adjacent_link_id = match link_connections
            .get(&current_point)
            .expect("Link connections not contain border point")
            .iter()
            .filter(|id| !used_links.contains(id))
            .next() {
            Some(link_id) => link_id.clone(),
            None => break,
        };
        let adjacent_link = map_links.get(adjacent_link_id).expect(
            "Links not contain link id",
        );
        let adjacent_points = adjacent_link.points();
        let first_point = adjacent_points.first().expect("Link points is empty");

        if first_point == current_point {
            result.extend(adjacent_points.iter().map(|map_point| {
                let point = ResponsePoint::new(
                    map_point.lat() as f64 / 100000.0,
                    map_point.lon() as f64 / 100000.0,
                );

                point
            }));

            current_point = adjacent_points.last().expect("Link points is empty");
        } else {
            result.extend(adjacent_points.iter().rev().map(|map_point| {
                let point = ResponsePoint::new(
                    map_point.lat() as f64 / 100000.0,
                    map_point.lon() as f64 / 100000.0,
                );

                point
            }));

            current_point = adjacent_points.first().expect("Link points is empty");
        }

        current_link_id = adjacent_link_id;
    }

    // Collect all other links to output buffer, if polygon contains links out of loop.
    for (_, map_link) in map_links.iter().enumerate().filter(|&(i, _)| {
        !used_links.contains(&i)
    })
    {
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
