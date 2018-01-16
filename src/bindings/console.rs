

#[allow(unused_attributes)]
#[link_args = "--js-library src/js/console.js"]
extern "C" {
	pub fn init_console();
	pub fn set_console_text(s: *const i8);
	pub fn set_console_color(s: *const i8);
}
