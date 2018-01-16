#![allow(unused_variables, unused_imports, dead_code)]

use std::collections::HashMap;
use bindings::console::*;

enum ConsoleDirtiness {
	Clean,
	Buffer,
	Map,
}

struct State {
	entries: HashMap<String, String>,
	buffer: String,
	dirty: ConsoleDirtiness
}

static mut CONSOLE_STATE: Option<State> = None;

fn get_state() -> &'static mut State {
	unsafe { CONSOLE_STATE.as_mut().unwrap() }
}

pub fn init() {
	#[cfg(dom_console)]
	unsafe {
		init_console();

		CONSOLE_STATE = Some(State {
			entries: HashMap::new(),
			buffer: String::new(),
			dirty: ConsoleDirtiness::Clean,
		});
	}
}

pub fn set_text(s: &str) {
	#[cfg(dom_console)] {
		get_state().buffer = String::from(s);
		get_state().dirty = ConsoleDirtiness::Buffer;
	}
}

pub fn set_section<S, S2>(sect: S, s: S2) where S: Into<String>, S2: Into<String> {
	#[cfg(dom_console)] {
		get_state().entries.insert(sect.into(), s.into());
		get_state().dirty = ConsoleDirtiness::Map;
	}
}

pub fn set_color<S>(s: S) where S: Into<Vec<u8>> {
	#[cfg(dom_console)] unsafe {
		use std::ffi::CString;
		set_console_color(CString::new(s).unwrap().as_ptr());
	}
}

pub fn update() {
	#[cfg(dom_console)] {
		use std::fmt::Write;
		use std::ffi::CString;
		use self::ConsoleDirtiness::*;
	
		match get_state().dirty {
			Buffer => unsafe {
				set_console_text(CString::new(get_state().buffer.as_str()).unwrap().as_ptr());
			}

			Map => unsafe {
				let buf = &mut get_state().buffer;
				buf.clear();

				for (k, v) in get_state().entries.iter() {
					write!(buf, "<h3>{}</h3><div>{}</div><br/>", k, v).unwrap();
				}

				set_console_text(CString::new(buf.as_str()).unwrap().as_ptr());
			}

			Clean => {}
		}

		get_state().dirty = Clean;
	}
}