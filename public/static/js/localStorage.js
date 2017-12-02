"use strict";

define([], function() {
	let connectionSettings = {};
	let connectionListeners = [];

	const setConnectionSettings = function(host, port, database, role, password) {
		connectionSettings = {
			host: host,
			database,
			role,
			password,
		};

		if (port) {
			connectionSettings.port = port | 0;
		}

		for (const listener of connectionListeners) {
			listener(connectionSettings);
		}
	};

	const getConnectionSettings = function() {
		return connectionSettings;
	};

	const addConnectionListener = function(callback) {
		connectionListeners.push(callback);
	};

	return {
		setConnectionSettings: setConnectionSettings,
		getConnectionSettings: getConnectionSettings,
		addConnectionListener: addConnectionListener,
	};
});
