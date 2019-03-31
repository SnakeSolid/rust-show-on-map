use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Write;
use std::sync::Arc;

use postgres::Connection;
use postgres::TlsMode;

use super::DatabaseConfig;
use super::DatabaseError;
use super::MapLink;
use super::MapPlace;
use super::MapPoint;
use super::MapPolygon;
use super::MapRoad;

pub struct DatabaseClient {
    config: Arc<DatabaseConfig>,
    url: String,
}

impl DatabaseClient {
    pub fn new(
        config: Arc<DatabaseConfig>,
        host: &str,
        port: i16,
        database: &str,
        role: &str,
        password: &str,
    ) -> DatabaseClient {
        DatabaseClient {
            config: config,
            url: format!(
                "postgresql://{3}:{4}@{0}:{1}/{2}",
                host, port, database, role, password
            ),
        }
    }

    pub fn query_places(&self, ids: &Vec<i64>) -> Result<Vec<MapPlace>, DatabaseError> {
        if ids.is_empty() {
            return Err(DatabaseError::no_data());
        }

        let ids_string = join(',', ids);
        let connection = Connection::connect(self.url.clone(), TlsMode::None)?;
        let names = self.query_place_names(&connection, &ids_string)?;
        let geometry = self.query_place_geometry(&connection, &ids_string)?;
        let mut result = Vec::default();

        for (id, name) in names {
            let place = collect_place(id, name, id, &geometry);

            result.push(place);
        }

        Ok(result)
    }

    pub fn query_roads(&self, ids: &Vec<i64>) -> Result<Vec<MapRoad>, DatabaseError> {
        if ids.is_empty() {
            return Err(DatabaseError::no_data());
        }

        let ids_string = join(',', ids);
        let connection = Connection::connect(self.url.clone(), TlsMode::None)?;
        let all_names = self.query_road_names(&connection, &ids_string)?;
        let geometry = self.query_road_geometry(&connection, &ids_string)?;
        let mut result = Vec::default();

        for (id, names) in all_names {
            let road = collect_road(id, names, id, &geometry);

            result.push(road);
        }

        Ok(result)
    }

    pub fn query_places_unique(&self, ids: &Vec<i64>) -> Result<Vec<MapPlace>, DatabaseError> {
        if ids.is_empty() {
            return Err(DatabaseError::no_data());
        }

        let ids_string = join(',', ids);
        let connection = Connection::connect(self.url.clone(), TlsMode::None)?;
        let unique = self.query_place_ids(&connection, &ids_string)?;

        if unique.is_empty() {
            return Err(DatabaseError::no_data());
        }

        let ids: Vec<_> = unique.keys().collect();
        let ids_string = join(',', ids);
        let names = self.query_place_names(&connection, &ids_string)?;
        let geometry = self.query_place_geometry(&connection, &ids_string)?;
        let mut result = Vec::default();

        for (id, name) in names {
            let unique_id = unique.get(&id).map(|&id| id as i64).unwrap_or(id);
            let place = collect_place(unique_id, name, id, &geometry);

            result.push(place);
        }

        Ok(result)
    }

    pub fn query_roads_unique(&self, ids: &Vec<i64>) -> Result<Vec<MapRoad>, DatabaseError> {
        if ids.is_empty() {
            return Err(DatabaseError::no_data());
        }

        let ids_string = join(',', ids);
        let connection = Connection::connect(self.url.clone(), TlsMode::None)?;
        let unique = self.query_road_ids(&connection, &ids_string)?;

        if unique.is_empty() {
            return Err(DatabaseError::no_data());
        }

        let ids: Vec<_> = unique.keys().collect();
        let ids_string = join(',', ids);
        let all_names = self.query_road_names(&connection, &ids_string)?;
        let geometry = self.query_road_geometry(&connection, &ids_string)?;
        let mut result = Vec::default();

        for (id, names) in all_names {
            let unique_id = unique.get(&id).map(|&id| id as i64).unwrap_or(id);
            let place = collect_road(unique_id, names, id, &geometry);

            result.push(place);
        }

        Ok(result)
    }

    fn query_place_ids(
        &self,
        connection: &Connection,
        ids: &str,
    ) -> Result<HashMap<i64, i32>, DatabaseError> {
        let query = self.config.unique_place_ids();
        let rows = connection.query(&query.replace("{ids}", &ids), &[])?;
        let mut result = HashMap::default();

        for row in rows.into_iter() {
            let place_id: i64 = row.get(0);
            let unique_id: i32 = row.get(1);

            result.insert(place_id, unique_id);
        }

        Ok(result)
    }

    fn query_road_ids(
        &self,
        connection: &Connection,
        ids: &str,
    ) -> Result<HashMap<i64, i32>, DatabaseError> {
        let query = self.config.unique_road_ids();
        let rows = connection.query(&query.replace("{ids}", &ids), &[])?;
        let mut result = HashMap::default();

        for row in rows.into_iter() {
            let road_id: i64 = row.get(0);
            let unique_id: i32 = row.get(1);

            result.insert(road_id, unique_id);
        }

        Ok(result)
    }

    fn query_place_geometry(
        &self,
        connection: &Connection,
        ids: &str,
    ) -> Result<HashMap<i64, HashMap<i32, HashMap<i32, Vec<MapPoint>>>>, DatabaseError> {
        let query = self.config.points_for_places();
        let rows = connection.query(&query.replace("{ids}", &ids), &[])?;
        let mut result = HashMap::default();

        for row in rows.into_iter() {
            let place_id: i64 = row.get(0);
            let face_id: i32 = row.get(1);
            let link_id: i32 = row.get(2);
            let lat: i32 = row.get(3);
            let lon: i32 = row.get(4);

            let faces = result.entry(place_id).or_insert_with(|| HashMap::default());
            let links = faces.entry(face_id).or_insert_with(|| HashMap::default());
            let points = links.entry(link_id).or_insert_with(|| Vec::default());

            points.push(MapPoint::new(lat, lon));
        }

        Ok(result)
    }

    fn query_road_geometry(
        &self,
        connection: &Connection,
        ids: &str,
    ) -> Result<HashMap<i64, HashMap<i32, Vec<MapPoint>>>, DatabaseError> {
        let query = self.config.points_for_roads();
        let rows = connection.query(&query.replace("{ids}", &ids), &[])?;
        let mut result = HashMap::default();

        for row in rows.into_iter() {
            let road_id: i64 = row.get(0);
            let link_id: i32 = row.get(1);
            let lat: i32 = row.get(2);
            let lon: i32 = row.get(3);

            let links = result.entry(road_id).or_insert_with(|| HashMap::default());
            let points = links.entry(link_id).or_insert_with(|| Vec::default());

            points.push(MapPoint::new(lat, lon));
        }

        Ok(result)
    }

    fn query_place_names(
        &self,
        connection: &Connection,
        ids: &str,
    ) -> Result<HashMap<i64, String>, DatabaseError> {
        let query = self.config.names_for_places();
        let rows = connection.query(&query.replace("{ids}", &ids), &[])?;
        let mut result = HashMap::default();

        for row in rows.into_iter() {
            let place_id: i64 = row.get(0);
            let name: String = row.get(1);

            result.insert(place_id, name);
        }

        Ok(result)
    }

    fn query_road_names(
        &self,
        connection: &Connection,
        ids: &str,
    ) -> Result<HashMap<i64, Vec<String>>, DatabaseError> {
        let query = self.config.names_for_roads();
        let rows = connection.query(&query.replace("{ids}", &ids), &[])?;
        let mut result = HashMap::default();

        for row in rows.into_iter() {
            let place_id: i64 = row.get(0);
            let name: String = row.get(1);

            result
                .entry(place_id)
                .or_insert_with(|| Vec::new())
                .push(name);
        }

        Ok(result)
    }
}

fn collect_place(
    place_id: i64,
    place_name: String,
    id: i64,
    geometry: &HashMap<i64, HashMap<i32, HashMap<i32, Vec<MapPoint>>>>,
) -> MapPlace {
    if let Some(all_polygons) = geometry.get(&id) {
        let mut polygons = Vec::with_capacity(all_polygons.len());

        for all_links in all_polygons.values() {
            let mut links = Vec::with_capacity(all_links.len());

            for all_points in all_links.values() {
                let points = all_points.clone();

                links.push(MapLink::new(points));
            }

            polygons.push(MapPolygon::new(links));
        }

        MapPlace::with_name_geometry(place_id, place_name, polygons)
    } else {
        MapPlace::with_name(place_id, place_name)
    }
}

fn collect_road(
    road_id: i64,
    road_names: Vec<String>,
    id: i64,
    geometry: &HashMap<i64, HashMap<i32, Vec<MapPoint>>>,
) -> MapRoad {
    if let Some(all_links) = geometry.get(&id) {
        let mut links = Vec::with_capacity(all_links.len());

        for all_points in all_links.values() {
            let points = all_points.clone();

            links.push(MapLink::new(points));
        }

        MapRoad::with_names_geometry(road_id, road_names, links)
    } else {
        MapRoad::with_names(road_id, road_names)
    }
}

fn join<D, T>(sep: D, iter: T) -> String
where
    D: Display,
    T: IntoIterator,
    T::Item: Display,
{
    let mut result = String::new();
    let mut iter = iter.into_iter();

    if let Some(item) = iter.next() {
        write!(&mut result, "{}", item).unwrap();

        for item in iter {
            write!(&mut result, "{}{}", sep, item).unwrap();
        }
    }

    result
}
