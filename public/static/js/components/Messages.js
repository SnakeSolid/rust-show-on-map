"use strict";

define([ "knockout" ], function(ko) {
	return function(params) {
		this.messages = params.messages;
		this.clearCallback = params.clearCallback;

		this.isVisible = ko.pureComputed(function() {
			return this.messages().length > 0;
		}, this);
	};
});
