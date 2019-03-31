use crate::backend::handler::util::handle_request;
use crate::backend::HandlerError;
use crate::config::ConfigRef;
use crate::database::DatabaseClient;
use crate::database::Geometry;
use crate::database::MultiLine;
use crate::database::MultiPolygon;
use iron::Handler;
use iron::IronResult;
use iron::Request as IronRequest;
use iron::Response as IronResponse;

pub struct ObjectHandler {
    config: ConfigRef,
}

impl ObjectHandler {
    pub fn new(config: ConfigRef) -> ObjectHandler {
        ObjectHandler { config }
    }
}

impl Handler for ObjectHandler {
    fn handle(&self, request: &mut IronRequest) -> IronResult<IronResponse> {
        handle_request(request, move |request: Request| {
            let format = &request.format;
            let format = self
                .config
                .formats()
                .get(format)
                .ok_or_else(|| HandlerError::new("Request must contain valid format name"))?;

            if request.ids.is_empty() {
                return Err(HandlerError::new("Request must contain at least one id"));
            }

            let client = DatabaseClient::new(
                &format,
                &request.host,
                request.port,
                &request.database,
                &request.role,
                &request.password,
            );
            let objects = match client.query(&request.ids) {
                Ok(objects) => objects,
                Err(error) => return Err(HandlerError::new(&format!("Database error - {}", error))),
            };

            let mut result = Vec::new();

            for (id, data) in objects {
                let object = match data.geometry() {
                    Geometry::MultiLine(lines) => ResponseObject::MultiLineString {
                        id,
                        names: data.names().into(),
                        lines: collect_lines(lines),
                    },
                    Geometry::MultiPolygon(polygons) => ResponseObject::MultiPolygon {
                        id,
                        names: data.names().into(),
                        polygons: collect_polygons(polygons),
                    },
                };

                result.push(object);
            }

            Ok(result)
        })
    }
}

fn collect_lines(multi_line: &MultiLine) -> Vec<Vec<ResponsePoint>> {
    multi_line
        .lines()
        .iter()
        .map(|line| {
            line.points()
                .iter()
                .map(|point| ResponsePoint {
                    lat: point.lat(),
                    lon: point.lon(),
                })
                .collect()
        })
        .collect()
}

fn collect_polygons(multi_polygon: &MultiPolygon) -> Vec<Vec<ResponsePoint>> {
    multi_polygon
        .polygons()
        .iter()
        .map(|line| {
            line.points()
                .iter()
                .map(|point| ResponsePoint {
                    lat: point.lat(),
                    lon: point.lon(),
                })
                .collect()
        })
        .collect()
}

#[derive(Deserialize)]
struct Request {
    host: String,
    port: i16,
    database: String,
    role: String,
    password: String,
    format: String,
    ids: Vec<i64>,
}

#[serde(tag = "type")]
#[derive(Serialize)]
enum ResponseObject {
    MultiLineString {
        id: i64,
        names: Vec<String>,
        lines: Vec<Vec<ResponsePoint>>,
    },
    MultiPolygon {
        id: i64,
        names: Vec<String>,
        polygons: Vec<Vec<ResponsePoint>>,
    },
}

#[derive(Serialize)]
struct ResponsePoint {
    lat: f32,
    lon: f32,
}
