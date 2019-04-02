"use strict";

define(["knockout", "localStorage", "messageModel", "mapModel"], function(ko, storage, message, map) {
	return function() {
		const self = this;

		this.isConnectionVisible = ko.observable(false);
		this.isObjectsVisible = ko.observable(false);
		this.isClearVisible = ko.observable(false);
		this.isObjectsEnabled = ko.observable(false);
		this.isClearEnabled = ko.observable(false);
		this.features = ko.observableArray();
		this.messages = ko.observableArray();

		this.clearMessages = function() {
			self.messages([]);
		};

		this.featuresSelected = function(features) {
			const featureNames = features.map(function(feature) {
				return feature.names.join(", ") + " (" + feature.id + ")";
			});

			self.features(featureNames);
		};

		this.map = map.create(this.featuresSelected);

		this.isObjectsDisabled = ko.pureComputed(function() {
			return !this.isObjectsEnabled();
		}, this);

		this.isClearDisabled = ko.pureComputed(function() {
			return !this.isClearEnabled();
		}, this);

		this.showConnection = function() {
			const oldState = self.isConnectionVisible();

			self.isConnectionVisible(!oldState);
			self.isObjectsVisible(false);
		};

		this.hideConnection = function() {
			self.isConnectionVisible(false);
		};

		this.showObjects = function() {
			if (self.isObjectsEnabled()) {
				const oldState = self.isObjectsVisible();

				self.isConnectionVisible(false);
				self.isObjectsVisible(!oldState);
			}
		};

		this.showObjectsCallback = function(objects) {
			self.map.showObjects(objects, self.pushMessage);
		};

		this.hideObjects = function() {
			self.isObjectsVisible(false);
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
			self.isObjectsEnabled(true);
			self.isClearEnabled(true);
		}

		storage.addConnectionListener(function(connectionSettings) {
			self.isObjectsEnabled(true);
			self.isClearEnabled(true);
		});
	};
});
