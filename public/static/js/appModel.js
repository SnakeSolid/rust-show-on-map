"use strict";

define([ "knockout", "reqwest", "openLayers", "mapModel" ], function(ko, reqwest, ol, map) {
	return function() {
		const self = this;

		this.map = map.create();
	};
});
