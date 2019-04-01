"use strict";

define(["knockout", "exports", "messageModel"], function(ko, exports, message) {
	const MapModel = function(selectionCallback) {
		const self = this;

		this.selectionCallback = selectionCallback;
		this.isTilesVisible = ko.observable(true);
		this.mapObjects = ko.observableArray();

		this.onSelected = function(event) {
			// TODO: fix selection
			if (self.selectionCallback !== null) {
				self.selectionCallback(event.selected);
			}
		};

		this.tilesToggleCallback = function(event) {
			self.isTilesVisible(!self.isTilesVisible());
		};

		this.clearShapes = function() {
			self.mapObjects([]);
		};
	};

	MapModel.prototype.showObjects = function(mapObjects, messageCallback) {
		for (const mapObject of mapObjects) {
			this.mapObjects.push(mapObject);
		}
	};

	exports.create = function(selectionCallback) {
		return new MapModel(selectionCallback);
	};
});
