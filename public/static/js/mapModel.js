"use strict";

define([ "knockout", "openLayers", "exports" ], function(ko, ol, exports) {
	const MapModel = function() {
		const self = this;

		this.deferred_add_places = ko.observableArray();
		this.deferred_add_roads = ko.observableArray();
		this.clear = ko.observable(false);
	};

	exports.create = function() {
		return new MapModel();
	};
});
