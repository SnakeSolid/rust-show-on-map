"use strict";

define([], function() {
	const storage = window.localStorage;
	const CONNECTION_KEY = "connection";
	const RECENT_CONNECTION_KEY = "recent";
	const connectionListeners = [];
	const recentListeners = [];
	let connectionSettings;
	let recentConnections;

	const readOrDefault = function(storageKey, defaultValue) {
		const storageValue = storage.getItem(storageKey);

		if (storageValue !== null) {
			return JSON.parse(storageValue);
		} else {
			return defaultValue;
		}
	};

	const write = function(storageKey, value) {
		const storageValue = JSON.stringify(value);

		storage.setItem(storageKey, storageValue);
	};

	const pushRecentConnection = function(settings) {
		recentConnections = recentConnections.filter(function(item) {
			return (
				item.host !== settings.host ||
				item.port !== settings.port ||
				item.database !== settings.database ||
				item.role !== settings.role
			);
		});
		recentConnections = recentConnections.slice(0, 9);
		recentConnections = [settings].concat(recentConnections);

		for (const listener of recentListeners) {
			listener(recentConnections);
		}

		write(RECENT_CONNECTION_KEY, recentConnections);
	};

	const setConnectionSettings = function(host, port, database, role, password) {
		connectionSettings = { host, database, role, password };

		if (port) {
			connectionSettings.port = port | 0;
		}

		pushRecentConnection(connectionSettings);

		for (const listener of connectionListeners) {
			listener(connectionSettings);
		}

		write(CONNECTION_KEY, connectionSettings);
	};

	const getConnectionSettings = function() {
		return connectionSettings;
	};

	const getRecentConnections = function() {
		return recentConnections;
	};

	const addConnectionListener = function(callback) {
		connectionListeners.push(callback);
	};

	const addRecentListener = function(callback) {
		recentListeners.push(callback);
	};

	connectionSettings = readOrDefault(CONNECTION_KEY, null);
	recentConnections = readOrDefault(RECENT_CONNECTION_KEY, []);

	return {
		// Connection settings.
		setConnectionSettings: setConnectionSettings,
		getConnectionSettings: getConnectionSettings,
		addConnectionListener: addConnectionListener,

		// Recent connection handling.
		getRecentConnections: getRecentConnections,
		addRecentListener: addRecentListener,
	};
});
