use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const INDEX_HTML_TEMPLATE: &'static str = 
r##"<html>
	<head>
		<meta charset="utf-8" />
		<meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0, user-scalable=no" />
		<meta name="apple-mobile-web-app-capable" content="yes" />
		<meta name="mobile-web-app-capable" content="yes" />

		<meta name="theme-color" content="#222" />
		<meta name="msapplication-navbutton-color" content="#222" />
		<meta name="apple-mobile-web-app-status-bar-style" content="#222" />

		<style>
			* {
				margin: 0;
				padding: 0;
				user-select: none;
				-moz-user-select: none;
				-khtml-user-select: none;
				-webkit-user-select: none;
				-o-user-select: none;
			}

			html, body {
				width: 100vw;
				height: 100vh;
				position: fixed;
				overflow: hidden;
			}

			canvas {
				position: absolute;
				top: 0;
				left: 0;
				width: 100%;
				height: 100%;

				overflow: hidden;
				display: block;
			}
		</style>
	</head>

	<body>
		<canvas id="canvas"></canvas>
		<script src="/[[pkg_name]]/[[binary_name]]/[[build_type]].js"></script>
	</body>
</html>"##;


const MAPPING_TEMPLATE: &'static str = 
r##"/experiments/[[binary_name]] => target/html/[[binary_name]].html
/experiments/[[binary_name]]/debug.js => target/asmjs-unknown-emscripten/debug/[[binary_name]].js
/experiments/[[binary_name]]/release.js => target/asmjs-unknown-emscripten/release/[[binary_name]].js

"##;

fn main() {
	let profile = env::var("PROFILE").unwrap();

	let html_target_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let html_target_dir = Path::new(&html_target_dir).join("target/html");
	std::fs::create_dir_all(&html_target_dir).unwrap();

	let bin_dir = Path::new("src/bin");
	let mut binaries = Vec::new();

	for path in bin_dir.read_dir().expect(&format!("Couldn't read {:?}", bin_dir)) {
		use std::ffi::OsStr;

		if let Ok(path) = path {
			let path = path.path();

			if let Some(Some(path)) = path.file_stem().map(OsStr::to_str) {
				binaries.push(path.to_owned());
			}
		}
	}

	let mapping_template = MAPPING_TEMPLATE.to_owned();

	let mut mappings_file = File::create("mappings.sb").unwrap();

	for binary in binaries.iter() {
		let index_html = INDEX_HTML_TEMPLATE.to_owned()
			.replace("[[build_type]]", &profile)
			.replace("[[pkg_name]]", env!("CARGO_PKG_NAME"))
			.replace("[[binary_name]]", binary);

		let mapping = mapping_template.replace("[[binary_name]]", binary);
		
		let path = html_target_dir.join(format!("{}.html", binary));
		let mut html_file = File::create(&path).unwrap();

		html_file.write_all(index_html.as_bytes()).unwrap();
		mappings_file.write_all(mapping.as_bytes()).unwrap();
	}

	if profile == "debug" {
		println!("cargo:rustc-cfg=debug");
	}

	// println!("cargo:rustc-cfg=dom_console");
}
