"use strict";

define(["knockout"], function(ko) {
	ko.components.register("ko-connection-settings", {
		viewModel: { require: "components/ConnectionSettings" },
		template: { require: "text!components/ConnectionSettings.html" },
	});

	ko.components.register("ko-objects", {
		viewModel: { require: "components/Objects" },
		template: { require: "text!components/Objects.html" },
	});

	ko.components.register("ko-selected-names", {
		viewModel: { require: "components/SelectedNames" },
		template: { require: "text!components/SelectedNames.html" },
	});

	ko.components.register("ko-messages", {
		viewModel: { require: "components/Messages" },
		template: { require: "text!components/Messages.html" },
	});
});
