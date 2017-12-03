#[derive(Debug, Clone)]
pub struct MapPlace {
    id: i32,
    name: String,
    polygons: Vec<MapPolygon>,
}

#[derive(Debug, Clone)]
pub struct MapPolygon {
    links: Vec<MapLink>,
}

#[derive(Debug, Clone)]
pub struct MapLink {
    points: Vec<MapPoint>,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone)]
pub struct MapPoint {
    lat: i32,
    lon: i32,
}

impl MapPlace {
    pub fn with_name(id: i32, name: String) -> MapPlace {
        MapPlace {
            id: id,
            name: name,
            polygons: Vec::default(),
        }
    }

    pub fn with_name_geometry(id: i32, name: String, polygons: Vec<MapPolygon>) -> MapPlace {
        MapPlace { id, name, polygons }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn polygons(&self) -> &Vec<MapPolygon> {
        &self.polygons
    }
}

impl MapPolygon {
    pub fn new(links: Vec<MapLink>) -> MapPolygon {
        MapPolygon { links }
    }

    pub fn links(&self) -> &Vec<MapLink> {
        &self.links
    }
}

impl MapLink {
    pub fn new(points: Vec<MapPoint>) -> MapLink {
        MapLink { points }
    }

    pub fn points(&self) -> &Vec<MapPoint> {
        &self.points
    }
}

impl MapPoint {
    pub fn new(lat: i32, lon: i32) -> MapPoint {
        MapPoint { lat, lon }
    }

    pub fn lat(&self) -> i32 {
        self.lat
    }

    pub fn lon(&self) -> i32 {
        self.lon
    }
}
