# Show on Map

Show objects from database on the interactive map.

## Quick Start
[quick-start]: #quick-start

To build `show-on-map` from source code use following command:

```sh
cargo build --release
```

To start `show-on-map` with configuration file `example.toml` listening on `localhost:8080` use following command:

```sh
./target/release/show-on-map -c example.toml
```

WEB server will be available on [localhost](http://localhost:8080/).

## Commandline options
[commandline-options]: #commandline-options

* `-h` (`--help`) - show short description for all commend line options and exit;
* `-b HOST` (`--bind HOST`), optional - address to bind WEB server on. Default value: `localhost`;
* `-p PORT` (`--port PORT`), optional - port to listen for WEB server. Default value: `8080`;
* `-c FILE` (`--config FILE`), optional - path to configuration file. Detailed information about configuration file
	content see in [configuration] section. Default value: `config.toml`.

## Configuration
[configuration]: #configuration

Configuration file must be written in `toml` format. Configuration file six main parameters representing every query:

* `points_for_places` - contains query to get polygons for places. Query must return five fields: `place_id::bigint`,
	`polygon_id::integer`, `link_id::integer`, `latitude::integer` and `longitude::integer`. Coordinates within single
	link must be ordered;
* `points_for_roads` - contains query to get line for roads. Query must return five fields: `road_id::bigint`,
	`link_id::integer`, `latitude::integer` and `longitude::integer`. Coordinates within single	link must be ordered;
* `unique_place_ids` - contains query to get place ids by unique ids. Query must return two fields: `place_id::bigint`,
	`unique_id::integer`;
* `unique_road_ids` - contains query to get road ids by unique ids. Query must return two fields: `road_id::bigint` and
	`unique_id::integer`;
* `names_for_places` - contains query to get names for places. Query must return two fields: `place_id::bigint` and
	`name::string`. If query returns several names for single place only last name will be used as place name;
* `names_for_roads` - contains query to get names for places. Query must return two fields: `road_id::bigint` and
	`name::string`. If query returns several names for single road all names will be collected to single list.

Every query must contain place holder `{ids}`. This placeholder will be replaces with actual place or road ids in
runtime before query execution. All map coordinates must be represented as point in Mercator projection multiplied
by 10_000.

For all queries it's important to order all fields in order from query description.

## Configuration file example
[config-example]: #config-example

```toml
points_for_places = """
SELECT
  places.place_id::bigint AS place_id,
  places.face_id AS polygon_id,
  places.link_id AS link_id,
  places.lat AS lat,
  places.lon AS lon
FROM map.places AS places
WHERE places.place_id IN ( {ids} )
ORDER by places.place_id, places.face_id, places.seq_num ;
"""

points_for_roads = """
SELECT
  roads.road_id AS road_id,
  roads.link_id::integer AS link_id,
  roads.lat AS lat,
  roads.lon AS lon
FROM map.roads AS roads
ORDER BY roads.road_id, roads.link_id, roads.seq_num ;
"""

unique_place_ids = """
SELECT
  obj.object_id AS place_id,
  obj.unique_id AS unique_id
FROM map.objects AS obj
WHERE obj.unique_id IN ( {ids} ) ;
"""

unique_road_ids = """
SELECT
  obj.object_id AS road_id,
  obj.unique_id AS unique_id
FROM map.objects AS obj
WHERE obj.unique_id IN ( {ids} ) ;
"""

names_for_places = """
SELECT
  names.place_id::bigint AS place_id,
  names.name AS name
FROM map.place_names AS names
WHERE names.place_id IN ( {ids} )
ORDER BY names.place_id, names.is_exonym = 'N' ;
"""

names_for_roads = """
SELECT
  names.road_id AS road_id,
  names.name AS name
FROM map.road_name AS names
WHERE names.road_id IN ( {ids} )
ORDER BY names.road_id, names.name ;
"""
```

## License
[license]: #license

Source code is primarily distributed under the terms of the MIT license. See LICENSE for details.
