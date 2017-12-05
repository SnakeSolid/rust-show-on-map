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

		this.roads = ko.observable("");
		this.areUnique = ko.observable(false);
		this.isRoadsValid = ko.observable(true);
		this.isLoading = ko.observable(false);
		this.messageHeader = ko.observable("");
		this.messageText = ko.observable("");

		this.isRoadsInvalid = ko.pureComputed(function() {
			return !this.isRoadsValid();
		}, this);

		this.isMessageVisible = ko.pureComputed(function() {
			return this.messageHeader().length > 0 && this.messageText().length > 0;
		}, this);

		this.validate = function() {
			const roads = self.roads();
			let result = true;

			// not empty, not white space and contains only separators and digits.
			if (roads.length === 0 || roads.match( /^\s+$/ ) || !roads.match( /^[\s,;0-9]+$/ )) {
				self.isRoadsValid(false);

				result = false;
			} else {
				self.isRoadsValid(true);
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
					ids: self.roads().split( /[ ,;]+/ ).map(mapToInt),
					unique: self.areUnique(),
				};

				self.isLoading(true);

				reqwest({
					url: "/api/v1/road",
					method: "post",
					data: JSON.stringify(data),
					type: "json",
					contentType: "application/json"
				}).then(function (resp) {
					if (resp.ok) {
						self.map.showRoads(resp.roads);
						self.messageHeader("");
						self.messageText("");
						self.callback();
					} else {
						self.messageHeader("Error occurred");
						self.messageText(resp.message);
					}

					self.isLoading(false);
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
			self.roads("");
			self.areUnique(false);
		};
	};
});
