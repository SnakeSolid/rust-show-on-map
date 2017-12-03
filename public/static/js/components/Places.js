"use strict";

define([
	"knockout",
	"reqwest",
	"localStorage",
], function(ko, reqwest, storage) {
	const mapToInt = function(element) {
		return parseInt(element);
	};

	return function(params) {
		const self = this;

		this.isVisible = params.visible;
		this.map = params.map;
		this.callback = params.callback;

		this.places = ko.observable("");
		this.areUnique = ko.observable(false);
		this.isPlacesValid = ko.observable(true);
		this.isLoading = ko.observable(false);

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

		this.show = function() {
			const connection = storage.getConnectionSettings();

			if (connection && self.validate()) {
				const data = {
					host: connection.host,
					port: connection.port,
					database: connection.database,
					role: connection.role,
					password: connection.password,
					ids: self.places().split( /[ ,;]+/ ).map(mapToInt),
					unique: self.areUnique(),
				};

				this.isLoading(true);

				reqwest({
					url: "/api/v1/place",
					method: "post",
					data: JSON.stringify(data),
					type: "json",
					contentType: "application/json"
				}).then(function (resp) {
					if (resp.ok) {
						self.map.showPlaces(resp.places);
					} else {
						// check for error message
					}

					self.isLoading(false);
					self.callback();
				}).fail(function(err) {
					self.isLoading(false);
					self.callback();
				});
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
