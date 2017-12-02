"use strict";

define([ "knockout", "localStorage" ], function(ko, storage) {
	return function(params) {
		const self = this;

		this.isVisible = params.visible;
		this.callback = params.callback;

		this.host = ko.observable("");
		this.port = ko.observable("");
		this.database = ko.observable("");
		this.role = ko.observable("");
		this.password = ko.observable("");
		this.isHostValid = ko.observable(true);
		this.isPortValid = ko.observable(true);
		this.isDatabaseValid = ko.observable(true);
		this.isRoleValid = ko.observable(true);

		this.isHostInvalid = ko.pureComputed(function() {
			return !this.isHostValid();
		}, this);

		this.isPortInvalid = ko.pureComputed(function() {
			return !this.isPortValid();
		}, this);

		this.isDatabaseInvalid = ko.pureComputed(function() {
			return !this.isDatabaseValid();
		}, this);

		this.isRoleInvalid = ko.pureComputed(function() {
			return !this.isRoleValid();
		}, this);

		this.validate = function() {
			const host = self.host();
			const port = self.port();
			const database = self.database();
			const role = self.role();
			const password = self.password();
			let result = true;

			if (host.length === 0) {
				self.isHostValid(false);

				result = false;
			} else {
				self.isHostValid(true);
			}

			if (port.length > 0 && !port.match( /^[0-9]+$/ )) {
				self.isPortValid(false);

				result = false;
			} else {
				self.isPortValid(true);
			}

			if (database.length === 0) {
				self.isDatabaseValid(false);

				result = false;
			} else {
				self.isDatabaseValid(true);
			}

			if (role.length === 0) {
				self.isRoleValid(false);

				result = false;
			} else {
				self.isRoleValid(true);
			}

			return result;
		};

		this.save = function() {
			if (self.validate()) {
				const host = self.host();
				const port = self.port() || 5432;
				const database = self.database();
				const role = self.role();
				const password = self.password();

				storage.setConnectionSettings(host, port, database, role, password);

				self.callback();
			}
		};

		this.hide = function() {
			self.callback();
		};

		storage.addConnectionListener(function(connectionSettings) {
			self.host(connectionSettings.host);
			self.port("" + connectionSettings.port);
			self.database(connectionSettings.database);
			self.role(connectionSettings.role);
			self.password(connectionSettings.password);
		});
	};
});
