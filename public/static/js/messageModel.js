"use strict";

define(["knockout", "exports"], function(ko, exports) {
	const TYPE_WARN = "warn";
	const TYPE_ERROR = "error";

	const MessageModel = function(type, message, header) {
		const self = this;

		this.type = type;
		this.header = header;
		this.message = message;

		this.isWarning = ko.pureComputed(function() {
			return this.type === TYPE_WARN;
		}, this);

		this.isError = ko.pureComputed(function() {
			return this.type === TYPE_ERROR;
		}, this);

		this.isHeaderVisible = ko.pureComputed(function() {
			return this.header !== null;
		}, this);
	};

	exports.warn = function(message, header) {
		return new MessageModel(TYPE_WARN, message, header);
	};

	exports.error = function(message, header) {
		return new MessageModel(TYPE_ERROR, message, header);
	};
});
