"use strict";

define([ "knockout" ], function(ko) {
	ko.components.register("ko-connection-settings", {
		viewModel: { require: "components/ConnectionSettings" },
		template: { require: "text!components/ConnectionSettings.html" }
	});
});
