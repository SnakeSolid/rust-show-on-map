"use strict";

define([
	"knockout",
	"reqwest",
	"openLayers",
	"localStorage",
	"mapModel",
], function(ko, reqwest, ol, storage, map) {
	return function() {
		const self = this;

		this.isConnectionVisible = ko.observable(false);
		this.isPlacesVisible = ko.observable(false);
		this.isRoadsVisible = ko.observable(false);
		this.isClearVisible = ko.observable(false);
		this.isPlacesEnabled = ko.observable(false);
		this.isRoadsEnabled = ko.observable(false);
		this.isClearEnabled = ko.observable(false);
		this.features = ko.observableArray([]);

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

			self.isPlacesVisible(false);
			self.isRoadsVisible(false);
			self.isConnectionVisible(!oldState);
		};

		this.hideConnection = function() {
			self.isConnectionVisible(false);
		};

		this.showPlaces = function() {
			const oldState = self.isPlacesVisible();

			self.isPlacesVisible(!oldState);
			self.isRoadsVisible(false);
			self.isConnectionVisible(false);
		};

		this.hidePlaces = function() {
			self.isPlacesVisible(false);
		};

		this.clearShapes = function() {
			self.map.clearShapes();
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
