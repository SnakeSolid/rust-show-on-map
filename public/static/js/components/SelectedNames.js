"use strict";

define(["knockout"], function(ko) {
	return function(params) {
		this.names = params.names;

		this.isVisible = ko.pureComputed(function() {
			return this.names().length > 0;
		}, this);
	};
});
