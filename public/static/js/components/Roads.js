"use strict";

define([
	"knockout",
	"reqwest",
	"messageModel",
	"localStorage",
], function(ko, reqwest, message, storage) {
	const mapToInt = function(element) {
		return parseInt(element);
	};

	return function(params) {
		const self = this;

		this.showCallback = params.showCallback;
		this.closeCallback = params.closeCallback;
		this.messageCallback = params.messageCallback;

		this.roads = ko.observable("");
		this.areUnique = ko.observable(false);
		this.isRoadsValid = ko.observable(true);
		this.isLoading = ko.observable(false);

		this.isRoadsInvalid = ko.pureComputed(function() {
			return !this.isRoadsValid();
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

		this.processResponce = function(responce, expectedIds) {
			if (responce.ok) {
				const actualIds = {};

				for (const road of responce.roads) {
					const id = road.id;

					actualIds[id] = true;
				}

				for (const id of expectedIds) {
					if (!(id in actualIds)) {
						self.messageCallback(message.warn("Road with id " + id + " was not found."));
					}
				}

				self.showCallback(responce.roads);
				self.closeCallback();
			} else {
				this.messageCallback(message.error(responce.message, "Error occurred"));
			}

			self.isLoading(false);
		};

		this.processFail = function() {
			self.closeCallback();
			self.isLoading(false);
		};

		this.show = function() {
			const connection = storage.getConnectionSettings();

			if (connection && self.validate()) {
				const ids = self.roads().trim().split( /[ ,;]+/ ).map(mapToInt);
				const data = {
					host: connection.host,
					port: connection.port,
					database: connection.database,
					role: connection.role,
					password: connection.password,
					ids: ids,
					unique: self.areUnique(),
				};


				reqwest({
					url: "/api/v1/road",
					method: "post",
					data: JSON.stringify(data),
					type: "json",
					contentType: "application/json"
				}).then(function(responce) {
					self.processResponce(responce, ids);
				}).fail(self.processFail);

				self.isLoading(true);
			}
		};

		this.hide = function() {
			self.closeCallback();
		};

		this.clear = function() {
			self.roads("");
			self.areUnique(false);
		};
	};
});
