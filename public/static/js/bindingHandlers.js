"use strict";

define(["knockout", "openLayers"], function(ko, ol) {
	const KEY_INDEX = "index";

	const createToggleControl = function(callback) {
		const button = document.createElement("button");
		button.title = "Toggle world map";
		button.innerHTML = "W";
		button.addEventListener("click", callback, false);
		button.addEventListener("touchstart", callback, false);

		const element = document.createElement("div");
		element.className = "toggle-world-map ol-unselectable ol-control";
		element.appendChild(button);

		return new ol.control.Control({ element: element });
	};

	const OpenLayersMap = function(element, params) {
		this.objectFeatures = {};
		this.objectStyles = {};

		params.mapObjects.subscribe(this.updateGeometry.bind(this));

		// Initialize layers - background - OSM, foreground - vector
		const sourceOsm = new ol.source.OSM();
		const layerTile = new ol.layer.Tile({ source: sourceOsm });
		const sourceVector = new ol.source.Vector({ wrapX: false });
		const layerVector = new ol.layer.Vector({
			source: sourceVector,
			style: this.getFeatureStyle.bind(this),
		});

		// Create control buttons: zoom, attribution and toggle map
		const controls = ol.control
			.defaults({
				attribution: true,
				zoom: true,
			})
			.extend([new ol.control.ScaleLine(), createToggleControl(this.toggleTiles.bind(this))]);

		// Create default view
		const view = new ol.View({
			center: [-2000000.0, 5000000.0],
			zoom: 4,
		});

		// Create map
		const map = new ol.Map({
			controls: controls,
			layers: [layerTile, layerVector],
			target: element,
			view: view,
		});

		// Add callback handler for object selection.
		const interactSelect = new ol.interaction.Select();
		interactSelect.on("select", params.selectionCallback);
		map.addInteraction(interactSelect);

		this.tileLayer = layerTile;
		this.sourceVector = sourceVector;
		this.map = map;
	};

	OpenLayersMap.prototype.updateGeometry = function(mapObjects) {
		// Add new map objects
		for (const index in mapObjects) {
			if (index in this.objectFeatures && index in this.objectStyles) {
				continue;
			}

			const mapObject = mapObjects[index];
			const name = mapObject.names.join(", ") + " (" + mapObject.id + ")";
			let geometry;
			let style;

			if (mapObject.type === "MultiLineString") {
				const points = mapObject.lines.map(this.lineToCoordinate.bind(this));

				geometry = new ol.geom.MultiLineString(points, "XY");
				style = this.getLineStyle();
			} else if (mapObject.type === "MultiPolygon") {
				const points = mapObject.polygons.map(this.polygonToCoordinate.bind(this));

				geometry = new ol.geom.MultiPolygon(points, "XY");
				style = this.getPolygonStyle();
			}

			const feature = new ol.Feature({ geometry, name });
			feature.set(KEY_INDEX, index);

			this.objectFeatures[index] = feature;
			this.objectStyles[index] = style;
			this.sourceVector.addFeature(feature);
		}

		// Remove unused map objects
		for (const index in this.objectFeatures) {
			if (!(index in mapObjects)) {
				const feature = this.objectFeatures[index];

				this.sourceVector.removeFeature(feature);
				delete this.objectFeatures[index];
				delete this.objectStyles[index];
			}
		}

		// Zoom to fit all features if features present on the map
		if (this.sourceVector.getFeatures().length > 0) {
			const extent = this.sourceVector.getExtent();
			const view = this.map.getView();

			view.fit(extent, {
				padding: [30, 20, 30, 20],
			});
		}
	};

	// Polygons represents places
	OpenLayersMap.prototype.getPolygonStyle = function() {
		const color_r = 0 + Math.floor(64.0 * Math.random());
		const color_g = 128 + Math.floor(64.0 - 128.0 * Math.random());
		const color_b = 255 - Math.floor(64.0 * Math.random());
		const stroke = new ol.style.Stroke({ color: [color_r, color_g, color_b], width: 1 });
		const fill = new ol.style.Fill({ color: [color_r, color_g, color_b, 0.1] });

		return new ol.style.Style({ stroke: stroke, fill: fill });
	};

	// Lines represents roads and links
	OpenLayersMap.prototype.getLineStyle = function() {
		const color_r = 255 - Math.floor(64.0 * Math.random());
		const color_g = 128 + Math.floor(64.0 - 128.0 * Math.random());
		const color_b = 0 + Math.floor(64.0 * Math.random());
		const stroke = new ol.style.Stroke({ color: [color_r, color_g, color_b], width: 2 });

		return new ol.style.Style({ stroke: stroke });
	};

	OpenLayersMap.prototype.toggleTiles = function(feature) {
		const tileVisible = !this.tileLayer.getVisible();

		this.tileLayer.setVisible(tileVisible);
	};

	OpenLayersMap.prototype.getFeatureStyle = function(feature) {
		const id = feature.get(KEY_INDEX);

		return this.objectStyles[id];
	};

	// Project polygon from latitude/longitude to WEB Mercator.
	OpenLayersMap.prototype.polygonToCoordinate = function(polygon) {
		return [polygon.map(this.pointToCoordinate)];
	};

	// Project line from latitude/longitude to WEB Mercator.
	OpenLayersMap.prototype.lineToCoordinate = function(line) {
		return line.map(this.pointToCoordinate);
	};

	// Project single point from latitude/longitude to WEB Mercator.
	OpenLayersMap.prototype.pointToCoordinate = function(point) {
		return ol.proj.transform([point.lon, point.lat], "EPSG:4326", "EPSG:3857");
	};

	// OpenLayers binding
	ko.bindingHandlers.asMap = {
		init: function(element, valueAccessor, allBindings, _, bindingContext) {
			const value = valueAccessor();
			const valueUnwrapped = ko.unwrap(value);

			valueUnwrapped._map = new OpenLayersMap(element, valueUnwrapped);
		},
	};
});
