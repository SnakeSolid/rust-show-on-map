"use strict";

define([ "knockout", "exports" ], function(ko, exports) {
	const MapModel = function() {
		const self = this;

		this.deferred_add_places = ko.observableArray();
		this.deferred_add_roads = ko.observableArray();
		this.clear = ko.observable(false);

		this.onSelected = function(a, b, c, d) {
			console.log(a, b, c, d);
		};

		this.showPlaces = function(places) {
			this.deferred_add_places(places);
		};

		this.clearShapes = function() {
			self.clear(true);
		};
	};

	exports.create = function() {
		return new MapModel();
	};
});
