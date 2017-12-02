"use strict";

define([ "knockout" ], function(ko) {
	return function(params) {
		const self = this;

		this.isVisible = params.visible;
		this.map = params.map;
		this.callback = params.callback;

		this.places = ko.observable("");
		this.areUnique = ko.observable(false);
		this.isPlacesValid = ko.observable(true);

		this.isPlacesInvalid = ko.pureComputed(function() {
			return !this.isPlacesValid();
		}, this);

		this.validate = function() {
			const places = self.places();
			let result = true;

			// not empty, not white space and contains only separators and digits.
			if (places.length === 0 || places.match( /^\s+$/ ) || !places.match( /^[\s,;0-9]+$/ )) {
				self.isPlacesValid(false);

				result = false;
			} else {
				self.isPlacesValid(true);
			}

			return result;
		};

		this.save = function() {
			if (self.validate()) {
				console.log("query data and add it to the map");

				self.callback();
			}
		};

		this.hide = function() {
			self.callback();
		};

		this.clear = function() {
			self.places("");
			self.areUnique(false);
		};
	};
});
