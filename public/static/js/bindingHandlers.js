"use strict";

define([ "knockout", "openLayers" ], function(ko, ol) {
	const pointToCoordinate = function(point) {
		return ol.proj.transform([ point.lon, point.lat ], "EPSG:4326", "EPSG:3857");
	};

	// OpenLayers binding
	ko.bindingHandlers.asMap = {
		init: function(element, valueAccessor, allBindings, _, bindingContext) {
			const value = valueAccessor();
			const valueUnwrapped = ko.unwrap(value);
			const sourceOsm = new ol.source.OSM();
			const layerTile = new ol.layer.Tile({ source: sourceOsm });
			const sourceVector = new ol.source.Vector({ wrapX: false });
			const layerVector = new ol.layer.Vector({ source: sourceVector });
			const controls = ol.control.defaults({
				attribution: true,
				zoom: true,
			}).extend([
				new ol.control.ScaleLine()
			]);
			const view = new ol.View({
				center: [0, 0],
				zoom: 3
			});
			const map = new ol.Map({
				controls: controls,
				layers: [ layerTile, layerVector ],
				target: element,
				view: view
			});

			valueUnwrapped._vector = sourceVector;
			valueUnwrapped._map = map;
		}, update: function(element, valueAccessor, allBindings) {
			const value = valueAccessor();
			const valueUnwrapped = ko.unwrap(value);
			const places = valueUnwrapped.deferred_add_places();
			const map = valueUnwrapped._map;
			const vector = valueUnwrapped._vector;

			if (places.length > 0) {
				const features = [];

				for (const place of places) {
					const name = place.name + " (" + place.id + ")";
					const allCoordinates = [];

					place.polygons.forEach(function(polygon) {
						allCoordinates.push([ polygon.map(pointToCoordinate) ]);
					});

					const geometry = new ol.geom.MultiPolygon(allCoordinates, "XY");
					const style = new ol.style.Style({
						stroke: new ol.style.Stroke({
							color: 'blue',
							width: 3
						}),
						fill: new ol.style.Fill({
							color: 'rgba(0, 0, 255, 0.1)'
						})
					});
					const feature = new ol.Feature({
						geometry: geometry,
						style: style,
						name: name,
					});

					features.push(feature);
				}

				vector.addFeatures(features);
				valueUnwrapped.deferred_add_places([]);
			}
		}
	};
});
