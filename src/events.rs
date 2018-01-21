use std::mem::transmute;

use bindings::emscripten::*;
use common::*;


pub enum Event {
	Resize(Vec2i),

	Down(Vec2i),
	Up(Vec2i),
	Move(Vec2i),

	Click(Vec2i),
}


pub unsafe fn initialise_ems_event_queue(queue: &mut Vec<Event>) {
	use std::ptr::null;

	js! { b"Module.canvas = document.getElementById('canvas')\0" };

	let evt_ptr = transmute(queue);

	let window_target = b"#window\0".as_ptr() as _;
	let canvas_target = b"#canvas\0".as_ptr() as _;

	on_resize(0, null(), evt_ptr);
	emscripten_set_resize_callback(window_target, evt_ptr, 1, Some(on_resize));
	emscripten_set_click_callback(canvas_target, evt_ptr, 1, Some(on_click));

	emscripten_set_mousemove_callback(canvas_target, evt_ptr, 1, Some(on_mouse_move));
	emscripten_set_mousedown_callback(canvas_target, evt_ptr, 1, Some(on_mouse_down));
	emscripten_set_mouseup_callback(canvas_target, evt_ptr, 1, Some(on_mouse_up));

	emscripten_set_touchstart_callback(canvas_target, evt_ptr, 1, Some(on_touch_start));
	emscripten_set_touchmove_callback(canvas_target, evt_ptr, 1, Some(on_touch_move));
	emscripten_set_touchend_callback(canvas_target, evt_ptr, 1, Some(on_touch_end));
	emscripten_set_touchcancel_callback(canvas_target, evt_ptr, 1, Some(on_touch_end));
}

unsafe extern "C"
fn on_resize(_: i32, _e: *const EmscriptenUiEvent, ud: *mut CVoid) -> i32 {
	let event_queue: &mut Vec<Event> = transmute(ud);

	// js! { b"Module.canvas = document.getElementById('canvas')\0" };

	// let mut screen_size = Vec2i::zero();
	// screen_size.x = js! { b"return (Module.canvas.width = Module.canvas.style.width = window.innerWidth)\0" };
	// screen_size.y = js! { b"return (Module.canvas.height = Module.canvas.style.height = window.innerHeight)\0" };
	// event_queue.push(Event::Resize(screen_size));

	let canvas_target = b"#canvas\0".as_ptr() as _;
	let (mut width, mut height) = (0.0, 0.0);

	emscripten_get_element_css_size(canvas_target, &mut width, &mut height);
	emscripten_set_canvas_size(width as i32, height as i32);

	event_queue.push(Event::Resize(Vec2i::new(width as i32, height as i32)));
	
	1
}

unsafe extern "C"
fn on_click(_: i32, e: *const EmscriptenMouseEvent, ud: *mut CVoid) -> i32 {
	let event_queue: &mut Vec<Event> = transmute(ud);
	let e: &EmscriptenMouseEvent = transmute(e);

	event_queue.push(Event::Click(Vec2i::new(e.clientX as _, e.clientY as _)));
	
	1
}


unsafe extern "C"
fn on_mouse_move(_: i32, e: *const EmscriptenMouseEvent, ud: *mut CVoid) -> i32 {
	let event_queue: &mut Vec<Event> = transmute(ud);
	let e: &EmscriptenMouseEvent = transmute(e);

	event_queue.push(Event::Move(Vec2i::new(e.clientX as _, e.clientY as _)));
	
	1
}
unsafe extern "C"
fn on_mouse_down(_: i32, e: *const EmscriptenMouseEvent, ud: *mut CVoid) -> i32 {
	let event_queue: &mut Vec<Event> = transmute(ud);
	let e: &EmscriptenMouseEvent = transmute(e);

	event_queue.push(Event::Down(Vec2i::new(e.clientX as _, e.clientY as _)));
	
	1
}
unsafe extern "C"
fn on_mouse_up(_: i32, e: *const EmscriptenMouseEvent, ud: *mut CVoid) -> i32 {
	let event_queue: &mut Vec<Event> = transmute(ud);
	let e: &EmscriptenMouseEvent = transmute(e);

	event_queue.push(Event::Up(Vec2i::new(e.clientX as _, e.clientY as _)));
	
	1
}


unsafe extern "C"
fn on_touch_move(_: i32, e: *const EmscriptenTouchEvent, ud: *mut CVoid) -> i32 {
	let event_queue: &mut Vec<Event> = transmute(ud);
	let e: &EmscriptenTouchEvent = transmute(e);

	if e.touches[0].identifier != 0 { return 0 }

	let pos = Vec2i::new(e.touches[0].clientX as _, e.touches[0].clientY as _);
	event_queue.push(Event::Move(pos));
	
	1
}

unsafe extern "C"
fn on_touch_start(_: i32, e: *const EmscriptenTouchEvent, ud: *mut CVoid) -> i32 {
	let event_queue: &mut Vec<Event> = transmute(ud);
	let e: &EmscriptenTouchEvent = transmute(e);

	if e.touches[0].identifier != 0 { return 0 }

	let pos = Vec2i::new(e.touches[0].clientX as _, e.touches[0].clientY as _);
	event_queue.push(Event::Down(pos));
	
	1
}

unsafe extern "C"
fn on_touch_end(_: i32, e: *const EmscriptenTouchEvent, ud: *mut CVoid) -> i32 {
	let event_queue: &mut Vec<Event> = transmute(ud);
	let e: &EmscriptenTouchEvent = transmute(e);

	if e.numTouches > 1 {
		use std::mem::uninitialized;

		// TODO: Make this requestable
		let mut fse: EmscriptenFullscreenChangeEvent = uninitialized();
		emscripten_get_fullscreen_status(&mut fse);

		if fse.isFullscreen == 0 {
			js!{ b"Module.requestFullscreen(1,1,0)" };
		}
	}

	if e.touches[0].identifier != 0 { return 0 }

	let pos = Vec2i::new(e.touches[0].clientX as _, e.touches[0].clientY as _);
	event_queue.push(Event::Up(pos));
	
	1
}
