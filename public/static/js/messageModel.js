"use strict";

define([ "knockout", "exports" ], function(ko, exports) {
	const MessageModel = function(message, header) {
		const self = this;

		this.header = header;
		this.message = message;

		this.isHeaderVisible = ko.pureComputed(function() {
			return this.header !== null;
		}, this);
	};

	exports.create = function(message, header) {
		return new MessageModel(message, header);
	};
});
