"use strict";

define([ "knockout", "reqwest", "openLayers", "mapModel" ], function(ko, reqwest, ol, map) {
	return function() {
		const self = this;

		this.isConnectionVisible = ko.observable(false);
		this.isShowPlacesVisible = ko.observable(false);
		this.isShowRoadsVisible = ko.observable(false);
		this.isClearShapesVisible = ko.observable(false);
		this.isShowPlacesEnabled = ko.observable(false);
		this.isShowRoadsEnabled = ko.observable(false);
		this.isClearShapesEnabled = ko.observable(false);
		this.map = map.create();

		this.isShowPlacesDisabled = ko.pureComputed(function() {
			return !this.isShowPlacesEnabled();
		}, this);

		this.isShowRoadsDisabled = ko.pureComputed(function() {
			return !this.isShowRoadsEnabled();
		}, this);

		this.isClearShapesDisabled = ko.pureComputed(function() {
			return !this.isClearShapesEnabled();
		}, this);

		this.showConnection = function() {
			const oldState = self.isConnectionVisible();

			self.isShowPlacesVisible(false);
			self.isShowRoadsVisible(false);
			self.isConnectionVisible(!oldState);
		};

		this.saveConnection = function() {
			console.log("saved");
		};
	};
});
