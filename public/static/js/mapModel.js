"use strict";

define(["knockout", "exports", "messageModel"], function(ko, exports, message) {
	const MapModel = function(selectionCallback) {
		const self = this;

		this.selectionChanged = selectionCallback;
		this.isTilesVisible = ko.observable(true);
		this.mapObjects = ko.observableArray();

		this.selectionCallback = function(selectedIds) {
			if (this.selectionChanged !== null) {
				const mapObjects = this.mapObjects();
				const selectedMapObjects = selectedIds.map(function(id) {
					return mapObjects[id];
				});

				this.selectionChanged(selectedMapObjects);
			}
		}.bind(this);

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
