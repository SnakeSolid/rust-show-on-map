"use strict";

define([ "knockout", "exports" ], function(ko, exports) {
	const MapModel = function() {
		const self = this;

		this.deferred_add_places = ko.observableArray();
		this.deferred_add_roads = ko.observableArray();
		this.clear = ko.observable(false);

		this.showPlaces = function(places) {
			this.deferred_add_places(places);
		};
	};

	exports.create = function() {
		return new MapModel();
	};
});
