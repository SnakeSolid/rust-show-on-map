"use strict";

define(["knockout", "exports", "messageModel"], function(ko, exports, message) {
	const MapModel = function(selectionHandler) {
		const self = this;

		this.selectionHandler = selectionHandler;
		this.isTilesVisible = ko.observable(true);
		this.deferred_add = ko.observableArray();
		this.clear = ko.observable(false);

		this.onSelected = function(event) {
			if (self.selectionHandler !== null) {
				self.selectionHandler(event.selected);
			}
		};

		this.tilesToggleCallback = function(event) {
			self.isTilesVisible(!self.isTilesVisible());
		};

		this.showObjects = function(objects, messageCallback) {
			self.deferred_add(objects);
		};

		this.clearShapes = function() {
			self.clear(true);
		};
	};

	exports.create = function(selectionHandler) {
		return new MapModel(selectionHandler);
	};
});
