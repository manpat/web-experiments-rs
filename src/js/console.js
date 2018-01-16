
var LibraryConsoleExtensions = {
	$ConsoleData: {
		console_el: null,
	},

	init_console: function() {
		console_el = document.createElement("div");
		console_el.style.marginLeft = '5px';
		console_el.style.color = '#eee';
		console_el.style.top = '5px';
		console_el.style.position = 'absolute';
		console_el.style.fontFamily = 'sans-serif';
		console_el.style.pointerEvents = 'none';

		document.getElementsByTagName("body")[0].appendChild(console_el);
	},

	set_console_text: function(str) {
		console_el.innerHTML = Pointer_stringify(str);
	},

	set_console_color: function(str) {
		console_el.style.color = Pointer_stringify(str);
	},
};

autoAddDeps(LibraryConsoleExtensions, '$ConsoleData');
mergeInto(LibraryManager.library, LibraryConsoleExtensions);