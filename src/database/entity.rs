#[derive(Debug, Clone)]
pub struct NamesGeometry {
    names: Vec<String>,
    geometry: Geometry,
}

impl NamesGeometry {
    pub fn names(&self) -> &[String] {
        &self.names
    }

    pub fn geometry(&self) -> &Geometry {
        &self.geometry
    }
}

impl From<(Vec<String>, Geometry)> for NamesGeometry {
    fn from(names_geometry: (Vec<String>, Geometry)) -> Self {
        NamesGeometry {
            names: names_geometry.0,
            geometry: names_geometry.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    lat: f32,
    lon: f32,
}

impl Point {
    pub fn new(lat: f32, lon: f32) -> Point {
        Point { lat, lon }
    }

    pub fn lat(&self) -> f32 {
        self.lat
    }

    pub fn lon(&self) -> f32 {
        self.lon
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    points: Vec<Point>,
}

impl Line {
    pub fn new(points: &[Point]) -> Line {
        Line {
            points: points.into(),
        }
    }

    pub fn points(&self) -> &[Point] {
        &self.points
    }
}

impl From<Vec<Point>> for Line {
    fn from(points: Vec<Point>) -> Self {
        Line { points }
    }
}

#[derive(Debug, Clone)]
pub struct MultiLine {
    lines: Vec<Line>,
}

impl MultiLine {
    pub fn new(lines: &[Line]) -> MultiLine {
        MultiLine {
            lines: lines.into(),
        }
    }

    pub fn lines(&self) -> &[Line] {
        &self.lines
    }
}

impl From<Vec<Vec<Point>>> for MultiLine {
    fn from(lines: Vec<Vec<Point>>) -> Self {
        MultiLine {
            lines: lines.into_iter().map(|line| line.into()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    pub fn new(points: &[Point]) -> Polygon {
        Polygon {
            points: points.into(),
        }
    }

    pub fn points(&self) -> &[Point] {
        &self.points
    }
}

impl From<Vec<Point>> for Polygon {
    fn from(points: Vec<Point>) -> Self {
        Polygon { points }
    }
}

#[derive(Debug, Clone)]
pub struct MultiPolygon {
    polygons: Vec<Polygon>,
}

impl MultiPolygon {
    pub fn new(polygons: &[Polygon]) -> MultiPolygon {
        MultiPolygon {
            polygons: polygons.into(),
        }
    }

    pub fn polygons(&self) -> &[Polygon] {
        &self.polygons
    }
}

impl From<Vec<Vec<Point>>> for MultiPolygon {
    fn from(polygons: Vec<Vec<Point>>) -> Self {
        MultiPolygon {
            polygons: polygons.into_iter().map(|line| line.into()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Geometry {
    MultiLine(MultiLine),
    MultiPolygon(MultiPolygon),
}
