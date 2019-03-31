"use strict";

define(["knockout", "localStorage"], function(ko, storage) {
	return function(params) {
		const self = this;

		this.saveCallback = params.saveCallback;
		this.closeCallback = params.closeCallback;

		this.host = ko.observable("");
		this.port = ko.observable("");
		this.database = ko.observable("");
		this.role = ko.observable("");
		this.password = ko.observable("");
		this.recent = ko.observableArray();
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

		this.isRecentVisible = ko.pureComputed(function() {
			return this.recent().length > 0;
		}, this);

		this.saveConnection = function(settings, event) {
			self.setConnectionSettings(settings);
			self.save();
		};

		this.editConnection = function(settings, event) {
			self.setConnectionSettings(settings);
		};

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

			if (port.length === 0 || !port.match(/^[0-9]+$/)) {
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
				const port = self.port() | 0;
				const database = self.database();
				const role = self.role();
				const password = self.password();

				storage.setConnectionSettings(host, port, database, role, password);

				self.saveCallback();
			}
		};

		this.clear = function() {
			self.host("");
			self.port("");
			self.database("");
			self.role("");
			self.password("");
			self.isHostValid(true);
			self.isPortValid(true);
			self.isDatabaseValid(true);
			self.isRoleValid(true);
		};

		this.hide = function() {
			self.closeCallback();
		};

		this.setConnectionSettings = function(connectionSettings) {
			self.host(connectionSettings.host);
			self.port("" + connectionSettings.port);
			self.database(connectionSettings.database);
			self.role(connectionSettings.role);
			self.password(connectionSettings.password);
		};

		this.setRecentConnections = function(recentConnections) {
			self.recent(recentConnections);
		};

		const connectionSettings = storage.getConnectionSettings();
		const recentConnections = storage.getRecentConnections();

		if (connectionSettings !== null) {
			this.setConnectionSettings(connectionSettings);
		}

		if (recentConnections !== null) {
			this.setRecentConnections(recentConnections);
		}

		storage.addConnectionListener(this.setConnectionSettings);
		storage.addRecentListener(this.setRecentConnections);
	};
});
