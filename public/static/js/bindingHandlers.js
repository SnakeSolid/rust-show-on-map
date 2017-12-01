"use strict";

define([ "knockout", "openLayers" ], function(ko, ol) {
	// OpenLayers binding
	ko.bindingHandlers.asMap = {
		init: function(element, valueAccessor, allBindings, _, bindingContext) {
			const value = valueAccessor();
			const valueUnwrapped = ko.unwrap(value);
			const source_osm = new ol.source.OSM();
			const leyer_tile = new ol.layer.Tile({ source: source_osm });
			const source_vector = new ol.source.Vector({ wrapX: false });
			const leyer_vector = new ol.layer.Vector({ source: source_vector });
			const controls = ol.control.defaults({
				attribution: true,
				zoom: true,
			}).extend([
				new ol.control.ScaleLine()
			]);
			const view = new ol.View({
				center: [0, 0],
				zoom: 2
			});
			const map = new ol.Map({
				controls: controls,
				layers: [ leyer_tile, leyer_vector ],
				target: 'map',
				view: view
			});

			valueUnwrapped._tile = leyer_tile;
			valueUnwrapped._vector = leyer_vector;
			valueUnwrapped._map = map;
		}, update: function(element, valueAccessor, allBindings) {
			const value = valueAccessor();
			const valueUnwrapped = ko.unwrap(value);
			const map = valueUnwrapped._map;

			// some interactions here.
		}
	};
});
