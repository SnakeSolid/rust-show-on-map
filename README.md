# Show on Map

Show objects from database on the interactive map.

## Quick Start
[quick-start]: #quick-start

To build `show-on-map` from source code use following command:

```sh
cargo build --release
```

To start `show-on-map` with configuration file `example.yaml` listening on `localhost:8080` use following command:

```sh
./target/release/show-on-map -c example.yaml
```

WEB server will be available on [localhost](http://localhost:8080/).

## Commandline options
[commandline-options]: #commandline-options

* `-h` (`--help`) - show short description for all commend line options and exit;
* `-b HOST` (`--bind HOST`), optional - address to bind WEB server on. Default value: `localhost`;
* `-p PORT` (`--port PORT`), optional - port to listen for WEB server. Default value: `8080`;
* `-c FILE` (`--config FILE`), optional - path to configuration file. Detailed information about configuration file
	content see in [configuration] section. Default value: `config.yaml`.

## Configuration
[configuration]: #configuration

Configuration file must be written in `yaml` format. Configuration file has single section - `formats`. This section
contains map format name to format settings. Names can be any unique string.

Format settings contains three required fields:

* `format_type` - Type of geometry query result set for this format. Following types are available: `PlainLines`,
`PlainPolygons` and `Wkt`;
* `names_query` - SQL query. This query must return result set with two fields - bigint, varchar. First field will be
used as object identifier and must match with identifiers in query form. Second field represents as name. Single object
can have different names;
* `geometry_query` - SQL query. Query must returns object points or WKT string depending on chosen format (see format
description below);

All queries must have single parameter (`$1`). This parameter will represent array of object identifiers with `bigint`
type (PostgreSQL type `bigint[]`). Common usage is to add where clause `where object_id::bigint = any( $1 )` to select
only required object.

## PlainLines

Geometry query must contain lines and points (latitude and longitude). Single object must contain at least one line.
Every line must contain at least two points. Line with one point are useless, it can't be shown.

Query result set must contains following fields:

* `bigint` - Object identifier;
* `bigint` - Line identifier. This value used only to find points related to the line;
* `real` - Latitude of a point;
* `real` - Longitude of a point.

## PlainPolygons

Geometry query must contain polygons and points (latitude and longitude). Single object must contain at least one
polygon. Every polygon must contain at least three points. Polygon with one or two point are useless, it can't be
shown.

Query result set must contains following fields:

* `bigint` - Object identifier;
* `bigint` - Polygon identifier. This value used only to find points related to the polygon;
* `real` - Latitude of a point;
* `real` - Longitude of a point.

## Wkt

Geometry query must contain line and point (latitude and longitude). Single object must contain at least one line.
Every line must contain at least two points. Line with one point are useless, it can't be shown.

Query result set must contains following fields:

* `bigint` - Object identifier;
* `varchar` - Object geometry in [WKT](http://www.opengeospatial.org/standards/sfa) format. WKT value can be received
from [PostGIS](https://postgis.net/)'s `geometry` type using [ST_AsText](https://postgis.net/docs/ST_AsText.html)
function.

## Configuration file example
[config-example]: #config-example

```yaml
formats:
  "Example lines":
    format_type: PlainLines
    names_query: |
      select distinct
        rn.road_id::bigint as id,
        rn.name as name
      from road as rn
      where road_id::bigint = any( $1 ) ;
    geometry_query: |
      select
        links.road_id::bigint as id,
        links.link_id::bigint as line,
        (points.lat / 100000.0)::real as latitude,
        (points.lon / 100000.0)::real as longitude
      from road_link as links
        inner join link_points as points on ( points.link_id = links.link_id )
      where links.road_id::bigint = any( $1 )
      order by links.road_id, links.link_id, points.seq_num ;

  "Example polygons":
    format_type: PlainPolygons
    names_query: |
      select distinct
        fn.feature_id::bigint as id,
        fn.name as name
      from feature_name as nn using ( name_id )
      where fn.feature_id::bigint = any( $1 ) ;
    geometry_query: |
      select
        feature.feature_id::bigint as id,
        polygon.polygon_id::bigint as polygon_id,
        (points.lat / 100000.0)::real as latitude,
        (points.lon / 100000.0)::real as longitude
      from feature as feature
        inner join feature_polygon as polygon on ( polygon.feature_id = feature.feature_id )
        inner join polygon_link as link on ( link.polygon_id = polygon.polygon_id )
        inner join link_points as points on ( points.link_id = link.link_id )
      where feature.feature_id::bigint = any( $1 )
      order by polygon.face_id, link.seq_num, points.seq_num ;

  "Example WKT":
    format_type: Wkt
    names_query: |
      select distinct
        ll.link_id::bigint as id,
        ll.name as name
      from road_link as ll
      where ll.link_id::bigint = any( $1 ) ;
    geometry_query: |
      select
        rl.link_id::bigint as id,
        st_astext(st_force2d(fg.geometry)) as geometry
      from road_link as rl
        inner join link_geometry as fg using ( link_id )
      where rl.link_id::bigint = any( $1 ) ;
```

## License
[license]: #license

Source code is primarily distributed under the terms of the MIT license. See LICENSE for details.
