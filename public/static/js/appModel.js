"use strict";

define([
	"knockout",
	"localStorage",
	"messageModel",
	"mapModel",
], function(ko, storage, message, map) {
	return function() {
		const self = this;

		this.isConnectionVisible = ko.observable(false);
		this.isPlacesVisible = ko.observable(false);
		this.isRoadsVisible = ko.observable(false);
		this.isClearVisible = ko.observable(false);
		this.isPlacesEnabled = ko.observable(false);
		this.isRoadsEnabled = ko.observable(false);
		this.isClearEnabled = ko.observable(false);
		this.features = ko.observableArray();
		this.messages = ko.observableArray();

		this.clearMessages = function() {
			self.messages([]);
		};

		this.featuresSelected = function(features) {
			const featureNames = features.map(function(feature) {
				return feature.get("name");
			});

			self.features(featureNames);
		};

		this.map = map.create(this.featuresSelected);

		this.isPlacesDisabled = ko.pureComputed(function() {
			return !this.isPlacesEnabled();
		}, this);

		this.isRoadsDisabled = ko.pureComputed(function() {
			return !this.isRoadsEnabled();
		}, this);

		this.isClearDisabled = ko.pureComputed(function() {
			return !this.isClearEnabled();
		}, this);

		this.showConnection = function() {
			const oldState = self.isConnectionVisible();

			self.isConnectionVisible(!oldState);
			self.isPlacesVisible(false);
			self.isRoadsVisible(false);
		};

		this.hideConnection = function() {
			self.isConnectionVisible(false);
		};

		this.showPlaces = function() {
			if (self.isPlacesEnabled()) {
				const oldState = self.isPlacesVisible();

				self.isConnectionVisible(false);
				self.isPlacesVisible(!oldState);
				self.isRoadsVisible(false);
			}
		};

		this.showPlacesCallback = function(places) {
			self.map.showPlaces(places, self.pushMessage);
		}

		this.hidePlaces = function() {
			self.isPlacesVisible(false);
		};

		this.showRoads = function() {
			if (self.isRoadsEnabled()) {
				const oldState = self.isRoadsVisible();

				self.isConnectionVisible(false);
				self.isPlacesVisible(false);
				self.isRoadsVisible(!oldState);
			}
		};

		this.showRoadsCallback = function(roads) {
			self.map.showRoads(roads, self.pushMessage);
		}

		this.hideRoads = function() {
			self.isRoadsVisible(false);
		};

		this.clearShapes = function() {
			if (self.isClearEnabled()) {
				self.map.clearShapes();
			}
		};

		this.pushMessage = function(message) {
			self.messages.push(message);
		};

		const connectionSettings = storage.getConnectionSettings();

		if (connectionSettings === null) {
			this.isConnectionVisible(true);
		} else {
			self.isPlacesEnabled(true);
			self.isRoadsEnabled(true);
			self.isClearEnabled(true);
		}

		storage.addConnectionListener(function(connectionSettings) {
			self.isPlacesEnabled(true);
			self.isRoadsEnabled(true);
			self.isClearEnabled(true);
		});
	};
});
