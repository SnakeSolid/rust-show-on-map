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

	ko.components.register("ko-roads", {
		viewModel: { require: "components/Roads" },
		template: { require: "text!components/Roads.html" }
	});

	ko.components.register("ko-selected-names", {
		viewModel: { require: "components/SelectedNames" },
		template: { require: "text!components/SelectedNames.html" }
	});
});
