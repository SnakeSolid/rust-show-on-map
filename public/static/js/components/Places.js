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

		this.processResponce = function(responce, expectedIds) {
			if (responce.ok) {
				const actualIds = {};

				for (const place of responce.places) {
					const id = place.id;

					actualIds[id] = true;
				}

				for (const id of expectedIds) {
					if (!(id in actualIds)) {
						self.messageCallback(message.warn("Place with id " + id + " was not found."));
					}
				}

				self.showCallback(responce.places);
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
				const ids = self.places().split( /[ ,;]+/ ).map(mapToInt);
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
					url: "/api/v1/place",
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
			self.places("");
			self.areUnique(false);
		};
	};
});
