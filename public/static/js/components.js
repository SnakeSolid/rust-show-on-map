"use strict";

define([ "knockout" ], function(ko) {
	ko.components.register("ko-connection-settings", {
		viewModel: { require: "components/ConnectionSettings" },
		template: { require: "text!components/ConnectionSettings.html" }
	});

	ko.components.register("ko-places", {
		viewModel: { require: "components/Places" },
		template: { require: "text!components/Places.html" }
	});
});
