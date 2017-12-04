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
./target/release/show-on-map -r -c example.toml
```

WEB server will be available on [localhost|http://localhost:8080/].

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
	`polygon_id::integer`, `link_id::integer`, `latitude::integer` and `longitude::integer`;
* `points_for_roads` - contains query to get line for roads. Query must return five fields: `road_id::bigint`,
	`link_id::integer`, `latitude::integer` and `longitude::integer`;
* `unique_place_ids` - contains query to get place ids by unique ids;
* `unique_road_ids` - contains query to get road ids by unique ids;
* `names_for_places` - contains query to get names for places;
* `names_for_roads` - contains query to get names for places.

Every query must contain place holder `{ids}`. This placeholder will be replaces with actual place or road ids in
runtime before query execution. All map coordinates must be represented as point in Mercator projection multiplied
by 10_000.

## License
[license]: #license

Source code is primarily distributed under the terms of the MIT license. See LICENSE for details.
