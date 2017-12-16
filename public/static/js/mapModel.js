"use strict";

define([
	"knockout",
	"exports",
	"messageModel",
], function(ko, exports, message) {
	const MapModel = function(selectionHandler) {
		const self = this;

		this.selectionHandler = selectionHandler;
		this.deferred_add_places = ko.observableArray();
		this.deferred_add_roads = ko.observableArray();
		this.clear = ko.observable(false);

		this.onSelected = function(event) {
			if (self.selectionHandler !== null) {
				self.selectionHandler(event.selected);
			}
		};

		this.showPlaces = function(places, messageCallback) {
			places.forEach(function(place) {
				if (place.polygons === null || place.polygons.length === 0) {
					const text = "Place " + place.name + " (" + place.id + ") has no polygons.";

					messageCallback(message.error(text));
				} else {
					self.deferred_add_places.push(place);
				}
			});
		};

		this.showRoads = function(roads, messageCallback) {
			roads.forEach(function(road) {
				if (road.lines === null || road.lines.length === 0) {
					const text = "Road " + road.names.join(", ") + " (" + road.id + ") has no lines.";

					messageCallback(message.error(text));
				} else {
					self.deferred_add_roads.push(road);
				}
			});
		};

		this.clearShapes = function() {
			self.clear(true);
		};
	};

	exports.create = function(selectionHandler) {
		return new MapModel(selectionHandler);
	};
});
