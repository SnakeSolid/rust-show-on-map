"use strict";

define(["knockout", "reqwest", "messageModel", "localStorage", "integerParser"], function(
	ko,
	reqwest,
	message,
	storage,
	parser
) {
	const Objects = function(params) {
		const self = this;

		this.showCallback = params.showCallback;
		this.closeCallback = params.closeCallback;
		this.messageCallback = params.messageCallback;

		this.objects = ko.observable("");
		this.availableFormats = ko.observableArray();
		this.selectedFormat = ko.observable();
		this.isObjectsValid = ko.observable(true);
		this.isLoading = ko.observable(false);

		this.isObjectsInvalid = ko.pureComputed(function() {
			return !this.isObjectsValid();
		}, this);

		this.validate = function() {
			const valid = parser.validate(self.objects());

			self.isObjectsValid(valid);

			return valid;
		};

		this.processResponce = function(responce, expectedIds) {
			if (responce.success) {
				const actualIds = {};

				for (const place of responce.result) {
					const id = place.id;

					actualIds[id] = true;
				}

				for (const id of expectedIds) {
					if (!(id in actualIds)) {
						self.messageCallback(message.warn("Object with id " + id + " was not found."));
					}
				}

				self.showCallback(responce.result);
				self.closeCallback();
			} else {
				this.messageCallback(message.error(responce.message, "Error occurred"));
			}

			self.isLoading(false);
		};

		this.processFail = function() {
			self.closeCallback();
			self.isLoading(false);
		};

		this.show = function() {
			const connection = storage.getConnectionSettings();

			if (connection && self.validate()) {
				const ids = parser.parse(self.objects());
				const data = {
					host: connection.host,
					port: connection.port,
					database: connection.database,
					role: connection.role,
					password: connection.password,
					format: self.selectedFormat(),
					ids: ids,
				};

				reqwest({
					url: "/api/v1/object",
					method: "post",
					data: JSON.stringify(data),
					type: "json",
					contentType: "application/json",
				})
					.then(function(responce) {
						self.processResponce(responce, ids);
					})
					.fail(self.processFail);

				self.isLoading(true);
			}
		};

		this.hide = function() {
			self.closeCallback();
		};

		this.clear = function() {
			self.objects("");
		};

		this.loadAvailableFormats();
	};

	Objects.prototype.loadAvailableFormats = function() {
		reqwest({
			url: "/api/v1/format",
			method: "post",
			type: "json",
			contentType: "application/json",
		})
			.then(
				function(responce) {
					this.availableFormats(responce.result);
				}.bind(this)
			)
			.fail(this.processFail.bind(this));
	};

	return Objects;
});
