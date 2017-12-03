"use strict";

define([
	"knockout",
	"openLayers"
], function(ko, ol) {
	const KIND = "kind";
	const KIND_PLACE = "place";
	const KIND_ROAD = "road";
	const STROKE_PLACE = new ol.style.Stroke({ color: [ 0, 128, 255 ], width: 3 });
	const STROKE_ROAD = new ol.style.Stroke({ color: [ 255, 0, 0 ], width: 3 });
	const STROKE_DEFAULT = new ol.style.Stroke({ color: [ 0, 0, 0 ], width: 3 });
	const FILL_PLACE = new ol.style.Fill({ color: [ 0, 128, 255, 0.1 ] });
	const FILL_DEFAULT = new ol.style.Fill({ color: [ 0, 0, 0, 0.1 ] });
	const STYLE_PLACE = new ol.style.Style({ stroke: STROKE_PLACE, fill: FILL_PLACE });
	const STYLE_ROAD = new ol.style.Style({ stroke: STROKE_ROAD });
	const STYLE_DEFAULT = new ol.style.Style({ stroke: STROKE_DEFAULT, fill: FILL_DEFAULT });

	const pointToCoordinate = function(point) {
		return ol.proj.transform([ point.lon, point.lat ], "EPSG:4326", "EPSG:3857");
	};

	const getFeatureStyle = function(feature) {
		const kind = feature.get(KIND);

		if (kind === KIND_PLACE) {
			return STYLE_PLACE;
		} else if (kind === KIND_ROAD) {
			return STYLE_ROAD;
		}else {
			return STYLE_DEFAULT;
		}
	}

	// OpenLayers binding
	ko.bindingHandlers.asMap = {
		init: function(element, valueAccessor, allBindings, _, bindingContext) {
			const value = valueAccessor();
			const valueUnwrapped = ko.unwrap(value);
			const onSelected = valueUnwrapped.onSelected;
			const sourceOsm = new ol.source.OSM();
			const layerTile = new ol.layer.Tile({ source: sourceOsm });
			const sourceVector = new ol.source.Vector({ wrapX: false });
			const layerVector = new ol.layer.Vector({ source: sourceVector, style: getFeatureStyle });
			const interactSelect = new ol.interaction.Select();
			const controls = ol.control.defaults({
				attribution: true,
				zoom: true,
			}).extend([
				new ol.control.ScaleLine()
			]);
			const view = new ol.View({
				center: [ -2000000.0, 5000000.0 ],
				zoom: 4
			});
			const map = new ol.Map({
				controls: controls,
				layers: [ layerTile, layerVector ],
				target: element,
				view: view
			});

			interactSelect.on("select", onSelected);
			map.addInteraction(interactSelect);

			valueUnwrapped._vector = sourceVector;
			valueUnwrapped._map = map;
		}, update: function(element, valueAccessor, allBindings) {
			const value = valueAccessor();
			const valueUnwrapped = ko.unwrap(value);
			const places = valueUnwrapped.deferred_add_places();
			const roads = valueUnwrapped.deferred_add_roads();
			const clear = valueUnwrapped.clear();
			const map = valueUnwrapped._map;
			const vector = valueUnwrapped._vector;

			if (clear) {
				vector.clear();
				valueUnwrapped.clear(false);
			}

			if (places.length > 0) {
				const features = [];

				for (const place of places) {
					const name = place.name + " (" + place.id + ")";
					const allCoordinates = [];

					place.polygons.forEach(function(polygon) {
						allCoordinates.push([ polygon.map(pointToCoordinate) ]);
					});

					const geometry = new ol.geom.MultiPolygon(allCoordinates, "XY");
					const feature = new ol.Feature({ geometry, name });

					feature.set(KIND, KIND_PLACE);
					features.push(feature);
				}

				vector.addFeatures(features);
				valueUnwrapped.deferred_add_places([]);
			}

			if (roads.length > 0) {
				const features = [];

				for (const road of roads) {
					const name = road.names.sort().join(", ") + " (" + road.id + ")";
					const allCoordinates = [];

					road.lines.forEach(function(line) {
						allCoordinates.push(line.map(pointToCoordinate));
					});

					const geometry = new ol.geom.MultiLineString(allCoordinates, "XY");
					const feature = new ol.Feature({ geometry, name });

					feature.set(KIND, KIND_ROAD);
					features.push(feature);
				}

				vector.addFeatures(features);
				valueUnwrapped.deferred_add_roads([]);
			}

			if (vector.getFeatures().length > 0) {
				const extent = vector.getExtent();
				const view = map.getView();

				view.fit(extent, {
					padding: [ 30, 20, 30, 20 ],
				});
			}
		}
	};
});
