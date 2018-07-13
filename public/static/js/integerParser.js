"use strict";

define([], function() {
	const RE_DIGITS = new RegExp("[0-9]+", "g");
	const RE_VALID_CHARS = new RegExp("^[\\s,;0-9]+$");
	const RE_WHITESPACE = new RegExp("^\\s+$");

	const MAP_TO_INT = function(element) {
		return parseInt(element);
	};

	return {
		validate: function(text) {
			if (text.length === 0) {
				return false;
			}

			if (text.match(RE_WHITESPACE) !== null) {
				return false;
			}

			if (text.match(RE_VALID_CHARS) === null) {
				return false;
			}

			return true;
		},

		parse: function(text) {
			const matched = text.match(RE_DIGITS);

			if (matched === null) {
				return [];
			}

			return matched.map(MAP_TO_INT);
		}
	};
});
