"use strict";

requirejs.config({
	baseUrl: "/static/js",
	paths: {
		knockout: ["https://cdnjs.cloudflare.com/ajax/libs/knockout/3.4.2/knockout-min", "lib/knockout-min"],
		openLayers: ["https://cdnjs.cloudflare.com/ajax/libs/openlayers/4.5.0/ol", "lib/ol"],
		reqwest: ["https://cdnjs.cloudflare.com/ajax/libs/reqwest/2.0.5/reqwest.min", "lib/reqwest.min"],
		semantic: ["https://cdnjs.cloudflare.com/ajax/libs/semantic-ui/2.2.13/semantic.min", "lib/semantic.min"],
		text: ["https://cdnjs.cloudflare.com/ajax/libs/require-text/2.0.12/text.min", "lib/text.min"],
	},
	shim: {
		openLayers: {
			exports: "OpenLayers",
		},
		reqwest: {
			exports: "reqwest",
		},
	},
	waitSeconds: 15,
});

// Start the main application logic.
requirejs(
	["knockout", "appModel", "components", "bindingHandlers"],
	function(ko, appModel) {
		ko.applyBindings(new appModel());
	},
	function(err) {
		console.log(err.requireType);

		if (err.requireType === "timeout") {
			console.log("modules: " + err.requireModules);
		}

		throw err;
	}
);
