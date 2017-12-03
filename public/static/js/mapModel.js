"use strict";

define([ "knockout", "exports" ], function(ko, exports) {
	const MapModel = function(selectionHandler) {
		const self = this;

		this.selectionHandler =selectionHandler;
		this.deferred_add_places = ko.observableArray();
		this.deferred_add_roads = ko.observableArray();
		this.clear = ko.observable(false);

		this.onSelected = function(event) {
			if (self.selectionHandler !== null) {
				self.selectionHandler(event.selected);
			}
		};

		this.showPlaces = function(places) {
			this.deferred_add_places(places);
		};

		this.clearShapes = function() {
			self.clear(true);
		};
	};

	exports.create = function(selectionHandler) {
		return new MapModel(selectionHandler);
	};
});
