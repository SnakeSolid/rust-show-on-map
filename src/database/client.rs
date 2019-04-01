use super::DatabaseError;
use super::DatabaseResult;
use super::Geometry;
use super::NamesGeometry;
use super::Point;
use crate::config::FormatConfig;
use crate::config::FormatType;
use postgres::Connection;
use postgres::TlsMode;
use std::collections::HashMap;
use std::collections::HashSet;
use wkt::types::Coord as WktCoord;
use wkt::Geometry as WktGeometry;
use wkt::Wkt;

pub struct DatabaseClient<'a> {
    format: &'a FormatConfig,
    url: String,
}

impl<'a> DatabaseClient<'a> {
    pub fn new(
        format: &'a FormatConfig,
        host: &str,
        port: i16,
        database: &str,
        role: &str,
        password: &str,
    ) -> DatabaseClient<'a> {
        DatabaseClient {
            format,
            url: format!(
                "postgresql://{3}:{4}@{0}:{1}/{2}",
                host, port, database, role, password
            ),
        }
    }

    pub fn query(&self, ids: &Vec<i64>) -> DatabaseResult<HashMap<i64, NamesGeometry>> {
        if ids.is_empty() {
            return Err(DatabaseError::no_data());
        }

        let connection = Connection::connect(self.url.as_ref(), TlsMode::None)?;
        let names = self.get_names(&connection, ids)?;
        let geometry: HashMap<i64, Geometry> = match self.format.format_type() {
            FormatType::PlainLines => self.get_geometry_lines(&connection, ids)?,
            FormatType::PlainPolygons => self.get_geometry_polygons(&connection, ids)?,
            FormatType::Wkt => self.get_geometry_wkt(&connection, ids)?,
        };
        let mut result = HashMap::default();

        for (id, geometry) in geometry.into_iter() {
            let names = match names.get(&id).cloned() {
                Some(names) => names,
                None => vec![format!("#{}", id)],
            };

            result.insert(id, (names, geometry).into());
        }

        Ok(result)
    }

    fn get_names(
        &self,
        connection: &Connection,
        ids: &Vec<i64>,
    ) -> DatabaseResult<HashMap<i64, Vec<String>>> {
        let query = self.format.names_query();
        let rows = connection.query(query, &[ids])?;
        let mut result = HashMap::default();

        for row in rows.into_iter() {
            let id: i64 = row.get(0);
            let name: String = row.get(1);

            result.entry(id).or_insert_with(Vec::default).push(name);
        }

        Ok(result)
    }

    fn get_geometry_lines(
        &self,
        connection: &Connection,
        ids: &Vec<i64>,
    ) -> DatabaseResult<HashMap<i64, Geometry>> {
        let query = self.format.geometry_query();
        let rows = connection.query(query, &[ids])?;
        let mut object_lines: HashMap<_, HashSet<_>> = HashMap::default();
        let mut line_coords: HashMap<_, Vec<_>> = HashMap::default();

        for row in rows.into_iter() {
            let id: i64 = row.get(0);
            let line_id: i64 = row.get(1);
            let lat: f32 = row.get(2);
            let lon: f32 = row.get(3);
            let point = Point::new(lat, lon);

            object_lines
                .entry(id)
                .or_insert_with(HashSet::default)
                .insert(line_id);
            line_coords
                .entry(line_id)
                .or_insert_with(Vec::default)
                .push(point);
        }

        let mut result = HashMap::new();

        for (id, line_ids) in object_lines {
            let lines: Vec<Vec<Point>> = line_ids
                .iter()
                .map(|line_id| line_coords[&line_id].clone())
                .collect();

            result.insert(id, Geometry::MultiLine(lines.into()));
        }

        Ok(result)
    }

    fn get_geometry_polygons(
        &self,
        connection: &Connection,
        ids: &Vec<i64>,
    ) -> DatabaseResult<HashMap<i64, Geometry>> {
        let query = self.format.geometry_query();
        let rows = connection.query(query, &[ids])?;
        let mut object_polygons: HashMap<_, HashSet<_>> = HashMap::default();
        let mut polygon_coords: HashMap<_, Vec<_>> = HashMap::default();

        for row in rows.into_iter() {
            let id: i64 = row.get(0);
            let polygon_id: i64 = row.get(1);
            let lat: f32 = row.get(2);
            let lon: f32 = row.get(3);
            let point = Point::new(lat, lon);

            object_polygons
                .entry(id)
                .or_insert_with(HashSet::default)
                .insert(polygon_id);
            polygon_coords
                .entry(polygon_id)
                .or_insert_with(Vec::default)
                .push(point);
        }

        let mut result = HashMap::new();

        for (id, polygon_ids) in object_polygons {
            let polygons: Vec<Vec<Point>> = polygon_ids
                .iter()
                .map(|polygon_id| polygon_coords[&polygon_id].clone())
                .collect();

            result.insert(id, Geometry::MultiPolygon(polygons.into()));
        }

        Ok(result)
    }

    fn get_geometry_wkt(
        &self,
        connection: &Connection,
        ids: &Vec<i64>,
    ) -> DatabaseResult<HashMap<i64, Geometry>> {
        let query = self.format.geometry_query();
        let rows = connection.query(query, &[ids])?;
        let mut object_lines: HashMap<_, Vec<_>> = HashMap::default();
        let mut object_polygons: HashMap<_, Vec<_>> = HashMap::default();

        for row in rows.into_iter() {
            let id: i64 = row.get(0);
            let geometry_text: String = row.get(1);
            let wkt = Wkt::from_str(&geometry_text).unwrap();

            for geometry in wkt.items {
                match geometry {
                    WktGeometry::LineString(line) => {
                        object_lines
                            .entry(id)
                            .or_insert_with(Vec::default)
                            .push(wkt_line_to_points(&line.0));
                    }
                    WktGeometry::MultiLineString(lines) => {
                        let object_lines = object_lines.entry(id).or_insert_with(Vec::default);

                        for line in lines.0 {
                            object_lines.push(wkt_line_to_points(&line.0));
                        }
                    }
                    WktGeometry::Polygon(polygon) => {
                        let object_polygons =
                            object_polygons.entry(id).or_insert_with(Vec::default);

                        object_polygons.push(wkt_line_to_points(&polygon.0[0].0));
                    }
                    WktGeometry::MultiPolygon(polygons) => {
                        let object_polygons =
                            object_polygons.entry(id).or_insert_with(Vec::default);

                        for polygon in &polygons.0 {
                            object_polygons.push(wkt_line_to_points(&polygon.0[0].0));
                        }
                    }
                    _ => {
                        return Err(DatabaseError::unsupported_format(format_args!(
                            "{}",
                            geometry
                        )));
                    }
                }
            }
        }

        let mut result = HashMap::new();

        for (id, lines) in object_lines {
            result.insert(id, Geometry::MultiLine(lines.into()));
        }

        for (id, polygons) in object_polygons {
            result.insert(id, Geometry::MultiPolygon(polygons.into()));
        }

        Ok(result)
    }
}

fn wkt_line_to_points(line: &Vec<WktCoord>) -> Vec<Point> {
    line.iter()
        .map(|coord| Point::new(coord.y as f32, coord.x as f32))
        .collect()
}
