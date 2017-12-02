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

		this.isConnectionVisible = ko.observable(true);
		this.isPlacesVisible = ko.observable(false);
		this.isRoadsVisible = ko.observable(false);
		this.isClearShapesVisible = ko.observable(false);
		this.isPlacesEnabled = ko.observable(false);
		this.isRoadsEnabled = ko.observable(false);
		this.isClearShapesEnabled = ko.observable(false);
		this.map = map.create();

		this.isPlacesDisabled = ko.pureComputed(function() {
			return !this.isPlacesEnabled();
		}, this);

		this.isRoadsDisabled = ko.pureComputed(function() {
			return !this.isRoadsEnabled();
		}, this);

		this.isClearShapesDisabled = ko.pureComputed(function() {
			return !this.isClearShapesEnabled();
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

		storage.addConnectionListener(function(connectionSettings) {
			self.isPlacesEnabled(true);
			self.isRoadsEnabled(true);
		});
	};
});
