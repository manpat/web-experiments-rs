#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ffi::CString;

pub trait Interop {
	fn as_int(self, _: &mut Vec<CString>) -> i32;
}

impl Interop for i32 {
	fn as_int(self, _: &mut Vec<CString>) -> i32 {
		return self;
	}
}

impl<'a> Interop for &'a str {
	fn as_int(self, arena: &mut Vec<CString>) -> i32 {
		let c = CString::new(self).unwrap();
		let ret = c.as_ptr() as i32;
		arena.push(c);
		return ret;
	}
}

impl<'a> Interop for *const u8 {
	fn as_int(self, _: &mut Vec<CString>) -> i32 {
		return self as i32;
	}
}

#[macro_export]
macro_rules! js {
	( ($( $x:expr ),*) $y:expr ) => {
		{
			use std::ffi::CString;
			let mut arena: Vec<CString> = Vec::new();
			#[allow(dead_code)]
			const LOCAL: &'static [u8] = $y;
			
			#[allow(unused_unsafe)]
			unsafe { $crate::emscripten_asm_const_int(&LOCAL[0] as *const _ as *const u8, $($crate::Interop::as_int($x, &mut arena)),*) }
		}
	};
	( $y:expr ) => {
		{
			#[allow(dead_code)]
			const LOCAL: &'static [u8] = $y;

			#[allow(unused_unsafe)]
			unsafe { $crate::emscripten_asm_const_int(&LOCAL[0] as *const _ as *const u8) }
		}
	};
}

pub const EMSCRIPTEN_RESULT_SUCCESS: i32             =  0;
pub const EMSCRIPTEN_RESULT_DEFERRED: i32            =  1;
pub const EMSCRIPTEN_RESULT_NOT_SUPPORTED: i32       = -1;
pub const EMSCRIPTEN_RESULT_FAILED_NOT_DEFERRED: i32 = -2;
pub const EMSCRIPTEN_RESULT_INVALID_TARGET: i32      = -3;
pub const EMSCRIPTEN_RESULT_UNKNOWN_TARGET: i32      = -4;
pub const EMSCRIPTEN_RESULT_INVALID_PARAM: i32       = -5;
pub const EMSCRIPTEN_RESULT_FAILED: i32              = -6;
pub const EMSCRIPTEN_RESULT_NO_DATA: i32             = -7;
pub const EMSCRIPTEN_RESULT_TIMED_OUT: i32           = -8;

pub type emscripten_align1_short = i16;
pub type emscripten_align4_int64 = i64;
pub type emscripten_align2_int64 = i64;
pub type emscripten_align1_int64 = i64;
pub type emscripten_align2_int = i32;
pub type emscripten_align1_int = i32;
pub type emscripten_align2_float = f32;
pub type emscripten_align1_float = f32;
pub type emscripten_align4_double = f64;
pub type emscripten_align2_double = f64;
pub type emscripten_align1_double = f64;

pub type em_callback_func = Option< unsafe extern "C" fn () >;
pub type em_arg_callback_func = Option< unsafe extern "C" fn(arg1: *mut CVoid) >;
pub type em_str_callback_func = Option< unsafe extern "C" fn(arg1: *const CChar) >;
pub type em_socket_callback = Option< unsafe extern "C" fn(fd: i32, userData: *mut CVoid) >;
pub type em_socket_error_callback = Option< unsafe extern "C" fn(fd: i32, err: i32, msg: *const CChar, userData: *mut CVoid) >;
pub type em_async_wget_onload_func = Option< unsafe extern "C" fn(arg1: *mut CVoid, arg2: *mut CVoid, arg3: i32) >;
pub type em_async_wget2_onload_func = Option< unsafe extern "C" fn(arg1: u32, arg2: *mut CVoid, arg3: *const CChar) >;
pub type em_async_wget2_onstatus_func = Option< unsafe extern "C" fn(arg1: u32, arg2: *mut CVoid, arg3: i32) >;
pub type em_async_wget2_data_onload_func = Option< unsafe extern "C" fn(arg1: u32, arg2: *mut CVoid, arg3: *mut CVoid, arg4: u32) >;
pub type em_async_wget2_data_onerror_func = Option< unsafe extern "C" fn(arg1: u32, arg2: *mut CVoid, arg3: i32, arg4: *const CChar) >;
pub type em_async_wget2_data_onprogress_func = Option< unsafe extern "C" fn(arg1: u32, arg2: *mut CVoid, arg3: i32, arg4: i32) >;
pub type em_idb_exists_func = Option< unsafe extern "C" fn(arg1: *mut CVoid, arg2: i32) >;
pub type em_run_preload_plugins_data_onload_func = Option< unsafe extern "C" fn(arg1: *mut CVoid, arg2: *const CChar) >;
pub type em_worker_callback_func = Option< unsafe extern "C" fn(arg1: *mut CChar, arg2: i32, arg3: *mut CVoid) >;
pub type em_key_callback_func = Option< unsafe extern "C" fn(eventType: i32, keyEvent: *const EmscriptenKeyboardEvent, userData: *mut CVoid) -> i32>;
pub type em_mouse_callback_func = Option< unsafe extern "C" fn(eventType: i32, mouseEvent: *const EmscriptenMouseEvent, userData: *mut CVoid) -> i32>;
pub type em_wheel_callback_func = Option< unsafe extern "C" fn(eventType: i32, wheelEvent: *const EmscriptenWheelEvent, userData: *mut CVoid) -> i32>;
pub type em_ui_callback_func = Option< unsafe extern "C" fn(eventType: i32, uiEvent: *const EmscriptenUiEvent, userData: *mut CVoid) -> i32>;
pub type em_focus_callback_func = Option< unsafe extern "C" fn(eventType: i32, focusEvent: *const EmscriptenFocusEvent, userData: *mut CVoid) -> i32>;
pub type em_deviceorientation_callback_func = Option< unsafe extern "C" fn(eventType: i32, deviceOrientationEvent: *const EmscriptenDeviceOrientationEvent, userData: *mut CVoid) -> i32>;
pub type em_devicemotion_callback_func = Option< unsafe extern "C" fn(eventType: i32, deviceMotionEvent: *const EmscriptenDeviceMotionEvent, userData: *mut CVoid) -> i32>;
pub type em_orientationchange_callback_func = Option< unsafe extern "C" fn(eventType: i32, orientationChangeEvent: *const EmscriptenOrientationChangeEvent, userData: *mut CVoid) -> i32>;
pub type em_fullscreenchange_callback_func = Option< unsafe extern "C" fn(eventType: i32, fullscreenChangeEvent: *const EmscriptenFullscreenChangeEvent, userData: *mut CVoid) -> i32>;
pub type em_canvasresized_callback_func = Option< unsafe extern "C" fn(eventType: i32, reserved: *const CVoid, userData: *mut CVoid) -> i32>;
pub type em_pointerlockchange_callback_func = Option< unsafe extern "C" fn(eventType: i32, pointerlockChangeEvent: *const EmscriptenPointerlockChangeEvent, userData: *mut CVoid) -> i32>;
pub type em_pointerlockerror_callback_func = Option< unsafe extern "C" fn(eventType: i32, reserved: *const CVoid, userData: *mut CVoid) -> i32>;
pub type em_visibilitychange_callback_func = Option< unsafe extern "C" fn(eventType: i32, visibilityChangeEvent: *const EmscriptenVisibilityChangeEvent, userData: *mut CVoid) -> i32>;
pub type em_touch_callback_func = Option< unsafe extern "C" fn(eventType: i32, touchEvent: *const EmscriptenTouchEvent, userData: *mut CVoid) -> i32>;
pub type em_gamepad_callback_func = Option< unsafe extern "C" fn(eventType: i32, gamepadEvent: *const EmscriptenGamepadEvent, userData: *mut CVoid) -> i32>;
pub type em_battery_callback_func = Option< unsafe extern "C" fn(eventType: i32, batteryEvent: *const EmscriptenBatteryEvent, userData: *mut CVoid) -> i32>;
pub type em_beforeunload_callback = Option< unsafe extern "C" fn(eventType: i32, reserved: *const CVoid, userData: *mut CVoid) -> *const CChar>;
pub type em_webgl_context_callback = Option< unsafe extern "C" fn(eventType: i32, reserved: *const CVoid, userData: *mut CVoid) -> i32>;

pub type CChar = ::std::os::raw::c_char;
pub type CVoid = ::std::os::raw::c_void;
type FILE = CVoid;

extern "C" {
	pub fn emscripten_run_script(script: *const CChar); 
	pub fn emscripten_run_script_int(script: *const CChar) -> i32; 
	pub fn emscripten_run_script_string(script: *const CChar) -> *mut CChar; 
	pub fn emscripten_async_run_script(script: *const CChar, millis: i32); 
	pub fn emscripten_async_load_script(script: *const CChar, onload: em_callback_func, onerror: em_callback_func); 
	pub fn emscripten_set_main_loop(func: em_callback_func, fps: i32, simulate_infinite_loop: i32); 
	pub fn emscripten_set_main_loop_timing(mode: i32, value: i32) -> i32; 
	pub fn emscripten_get_main_loop_timing(mode: *mut i32, value: *mut i32); 
	pub fn emscripten_set_main_loop_arg(func: em_arg_callback_func, arg: *mut CVoid, fps: i32, simulate_infinite_loop: i32) -> !;
	pub fn emscripten_pause_main_loop (); 
	pub fn emscripten_resume_main_loop (); 
	pub fn emscripten_cancel_main_loop (); 

	pub fn emscripten_asm_const_int(s: *const u8, ...) -> i32;
}


extern "C" {
	pub fn emscripten_set_socket_error_callback(userData: *mut CVoid, callback: em_socket_error_callback); 
	pub fn emscripten_set_socket_open_callback(userData: *mut CVoid, callback: em_socket_callback); 
	pub fn emscripten_set_socket_listen_callback(userData: *mut CVoid, callback: em_socket_callback); 
	pub fn emscripten_set_socket_connection_callback(userData: *mut CVoid, callback: em_socket_callback); 
	pub fn emscripten_set_socket_message_callback(userData: *mut CVoid, callback: em_socket_callback); 
	pub fn emscripten_set_socket_close_callback(userData: *mut CVoid, callback: em_socket_callback); 
	pub fn _emscripten_push_main_loop_blocker(func: em_arg_callback_func, arg: *mut CVoid, name: *const CChar); 
	pub fn _emscripten_push_uncounted_main_loop_blocker(func: em_arg_callback_func, arg: *mut CVoid, name: *const CChar); 
	pub fn emscripten_set_main_loop_expected_blockers(num: i32); 
	pub fn emscripten_async_call(func: em_arg_callback_func, arg: *mut CVoid, millis: i32); 
	pub fn emscripten_exit_with_live_runtime (); 
	pub fn emscripten_force_exit(status: i32); 
	pub fn emscripten_get_device_pixel_ratio () -> f64; 
	pub fn emscripten_hide_mouse (); 
	pub fn emscripten_set_canvas_size(width: i32, height: i32); 
	pub fn emscripten_get_canvas_size(width: *mut i32, height: *mut i32, isFullscreen: *mut i32); 
	pub fn emscripten_get_now () -> f64; 
	pub fn emscripten_random () -> f32; 
	pub fn emscripten_async_wget(url: *const CChar, file: *const CChar, onload: em_str_callback_func, onerror: em_str_callback_func); 
}

extern "C" {
	pub fn emscripten_async_wget_data(url: *const CChar, arg: *mut CVoid, onload: em_async_wget_onload_func, onerror: em_arg_callback_func); 
}

extern "C" {
	pub fn emscripten_async_wget2(url: *const CChar, file: *const CChar, requesttype: *const CChar, param: *const CChar, arg: *mut CVoid, onload: em_async_wget2_onload_func, onerror: em_async_wget2_onstatus_func, onprogress: em_async_wget2_onstatus_func) -> i32; 
}

extern "C" {
	pub fn emscripten_async_wget2_data(url: *const CChar, requesttype: *const CChar, param: *const CChar, arg: *mut CVoid, free: i32, onload: em_async_wget2_data_onload_func, onerror: em_async_wget2_data_onerror_func, onprogress: em_async_wget2_data_onprogress_func) -> i32; 
	pub fn emscripten_async_wget2_abort(handle: i32); 
	pub fn emscripten_wget(url: *const CChar, file: *const CChar); 
	pub fn emscripten_wget_data(url: *const CChar, pbuffer: *mut *mut CVoid, pnum: *mut i32, perror: *mut i32); 
	pub fn emscripten_idb_async_load(db_name: *const CChar, file_id: *const CChar, arg: *mut CVoid, onload: em_async_wget_onload_func, onerror: em_arg_callback_func); 
	pub fn emscripten_idb_async_store(db_name: *const CChar, file_id: *const CChar, ptr: *mut CVoid, num: i32, arg: *mut CVoid, onstore: em_arg_callback_func, onerror: em_arg_callback_func); 
	pub fn emscripten_idb_async_delete(db_name: *const CChar, file_id: *const CChar, arg: *mut CVoid, ondelete: em_arg_callback_func, onerror: em_arg_callback_func); 
}

extern "C" {
	pub fn emscripten_idb_async_exists(db_name: *const CChar, file_id: *const CChar, arg: *mut CVoid, oncheck: em_idb_exists_func, onerror: em_arg_callback_func); 
	pub fn emscripten_idb_load(db_name: *const CChar, file_id: *const CChar, pbuffer: *mut *mut CVoid, pnum: *mut i32, perror: *mut i32); 
	pub fn emscripten_idb_store(db_name: *const CChar, file_id: *const CChar, buffer: *mut CVoid, num: i32, perror: *mut i32); 
	pub fn emscripten_idb_delete(db_name: *const CChar, file_id: *const CChar, perror: *mut i32); 
	pub fn emscripten_idb_exists(db_name: *const CChar, file_id: *const CChar, pexists: *mut i32, perror: *mut i32); 
	pub fn emscripten_idb_load_blob(db_name: *const CChar, file_id: *const CChar, pblob: *mut i32, perror: *mut i32); 
	pub fn emscripten_idb_store_blob(db_name: *const CChar, file_id: *const CChar, buffer: *mut CVoid, num: i32, perror: *mut i32); 
	pub fn emscripten_idb_read_from_blob(blob: i32, start: i32, num: i32, buffer: *mut CVoid); 
	pub fn emscripten_idb_free_blob(blob: i32); 
	pub fn emscripten_run_preload_plugins(file: *const CChar, onload: em_str_callback_func, onerror: em_str_callback_func) -> i32; 
}

extern "C" {
	pub fn emscripten_run_preload_plugins_data(data: *mut CChar, size: i32, suffix: *const CChar, arg: *mut CVoid, onload: em_run_preload_plugins_data_onload_func, onerror: em_arg_callback_func); 
}

pub type worker_handle = i32;

extern "C" {
	pub fn emscripten_create_worker(url: *const CChar) -> worker_handle; 
	pub fn emscripten_destroy_worker(worker: worker_handle); 
}

extern "C" {
	pub fn emscripten_call_worker(worker: worker_handle, funcname: *const CChar, data: *mut CChar, size: i32, callback: em_worker_callback_func, arg: *mut CVoid); 
	pub fn emscripten_worker_respond(data: *mut CChar, size: i32); 
	pub fn emscripten_worker_respond_provisionally(data: *mut CChar, size: i32); 
	pub fn emscripten_get_worker_queue_size(worker: worker_handle) -> i32; 
	pub fn emscripten_get_compiler_setting(name: *const CChar) -> i32; 
	pub fn emscripten_debugger (); 
	pub fn emscripten_get_preloaded_image_data(path: *const CChar, w: *mut i32, h: *mut i32) -> *mut CChar; 
	pub fn emscripten_get_preloaded_image_data_from_FILE(file: *mut FILE, w: *mut i32, h: *mut i32) -> *mut CChar; 
	pub fn emscripten_log(flags: i32, ... ); 
	pub fn emscripten_get_callstack(flags: i32, out: *mut CChar, maxbytes: i32) -> i32; 
	pub fn emscripten_print_double(x: f64, to: *mut CChar, max: i32) -> i32; 
	pub fn emscripten_sleep(ms: u32); 
	pub fn emscripten_sleep_with_yield(ms: u32); 
}

pub type emscripten_coroutine = *mut CVoid;

extern "C" {
	pub fn emscripten_coroutine_create(func: em_arg_callback_func, arg: *mut CVoid, stack_size: i32) -> emscripten_coroutine; 
	pub fn emscripten_coroutine_next(arg1: emscripten_coroutine) -> i32; 
	pub fn emscripten_yield (); 
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EmscriptenKeyboardEvent {
	pub key: [CChar; 32],
	pub code: [CChar; 32],
	pub location: u32,
	pub ctrlKey: i32,
	pub shiftKey: i32,
	pub altKey: i32,
	pub metaKey: i32,
	pub repeat: i32,
	pub locale: [CChar; 32],
	pub charValue: [CChar; 32],
	pub charCode: u32,
	pub keyCode: u32,
	pub which: u32
}

extern "C" {
	pub fn emscripten_set_keypress_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_key_callback_func) -> i32; 
	pub fn emscripten_set_keydown_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_key_callback_func) -> i32; 
	pub fn emscripten_set_keyup_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_key_callback_func) -> i32; 
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EmscriptenMouseEvent {
	pub timestamp: f64,
	pub screenX: i32,
	pub screenY: i32,
	pub clientX: i32,
	pub clientY: i32,
	pub ctrlKey: i32,
	pub shiftKey: i32,
	pub altKey: i32,
	pub metaKey: i32,
	pub button: u16,
	pub buttons: u16,
	pub movementX: i32,
	pub movementY: i32,
	pub targetX: i32,
	pub targetY: i32,
	pub canvasX: i32,
	pub canvasY: i32,
	pub padding: i32
}

extern "C" {
	pub fn emscripten_set_click_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_mouse_callback_func) -> i32; 
	pub fn emscripten_set_mousedown_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_mouse_callback_func) -> i32; 
	pub fn emscripten_set_mouseup_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_mouse_callback_func) -> i32; 
	pub fn emscripten_set_dblclick_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_mouse_callback_func) -> i32; 
	pub fn emscripten_set_mousemove_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_mouse_callback_func) -> i32; 
	pub fn emscripten_set_mouseenter_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_mouse_callback_func) -> i32; 
	pub fn emscripten_set_mouseleave_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_mouse_callback_func) -> i32; 
	pub fn emscripten_set_mouseover_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_mouse_callback_func) -> i32; 
	pub fn emscripten_set_mouseout_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_mouse_callback_func) -> i32; 
	pub fn emscripten_get_mouse_status(mouseState: *mut EmscriptenMouseEvent) -> i32; 
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EmscriptenWheelEvent {
	pub mouse: EmscriptenMouseEvent,
	pub deltaX: f64,
	pub deltaY: f64,
	pub deltaZ: f64,
	pub deltaMode: u32
}

extern "C" {
	pub fn emscripten_set_wheel_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_wheel_callback_func) -> i32; 
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EmscriptenUiEvent {
	pub detail: i32,
	pub documentBodyClientWidth: i32,
	pub documentBodyClientHeight: i32,
	pub windowInnerWidth: i32,
	pub windowInnerHeight: i32,
	pub windowOuterWidth: i32,
	pub windowOuterHeight: i32,
	pub scrollTop: i32,
	pub scrollLeft: i32
}

extern "C" {
	pub fn emscripten_set_resize_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_ui_callback_func) -> i32; 
	pub fn emscripten_set_scroll_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_ui_callback_func) -> i32; 
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EmscriptenFocusEvent {
	pub nodeName: [CChar; 128],
	pub id: [CChar; 128]
}

extern "C" {
	pub fn emscripten_set_blur_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_focus_callback_func) -> i32; 
	pub fn emscripten_set_focus_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_focus_callback_func) -> i32; 
	pub fn emscripten_set_focusin_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_focus_callback_func) -> i32; 
	pub fn emscripten_set_focusout_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_focus_callback_func) -> i32; 
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EmscriptenDeviceOrientationEvent {
	pub timestamp: f64,
	pub alpha: f64,
	pub beta: f64,
	pub gamma: f64,
	pub absolute: i32
}

extern "C" {
	pub fn emscripten_set_deviceorientation_callback(userData: *mut CVoid, useCapture: i32, callback: em_deviceorientation_callback_func) -> i32; 
	pub fn emscripten_get_deviceorientation_status(orientationState: *mut EmscriptenDeviceOrientationEvent) -> i32; 
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EmscriptenDeviceMotionEvent {
	pub timestamp: f64,
	pub accelerationX: f64,
	pub accelerationY: f64,
	pub accelerationZ: f64,
	pub accelerationIncludingGravityX: f64,
	pub accelerationIncludingGravityY: f64,
	pub accelerationIncludingGravityZ: f64,
	pub rotationRateAlpha: f64,
	pub rotationRateBeta: f64,
	pub rotationRateGamma: f64
}

extern "C" {
	pub fn emscripten_set_devicemotion_callback(userData: *mut CVoid, useCapture: i32, callback: em_devicemotion_callback_func) -> i32; 
	pub fn emscripten_get_devicemotion_status(motionState: *mut EmscriptenDeviceMotionEvent) -> i32; 
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EmscriptenOrientationChangeEvent {
	pub orientationIndex: i32,
	pub orientationAngle: i32
}

extern "C" {
	pub fn emscripten_set_orientationchange_callback(userData: *mut CVoid, useCapture: i32, callback: em_orientationchange_callback_func) -> i32; 
	pub fn emscripten_get_orientation_status(orientationStatus: *mut EmscriptenOrientationChangeEvent) -> i32; 
	pub fn emscripten_lock_orientation(allowedOrientations: i32) -> i32; 
	pub fn emscripten_unlock_orientation () -> i32; 
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EmscriptenFullscreenChangeEvent {
	pub isFullscreen: i32,
	pub fullscreenEnabled: i32,
	pub nodeName: [CChar; 128],
	pub id: [CChar; 128],
	pub elementWidth: i32,
	pub elementHeight: i32,
	pub screenWidth: i32,
	pub screenHeight: i32
}

extern "C" {
	pub fn emscripten_set_fullscreenchange_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_fullscreenchange_callback_func) -> i32; 
	pub fn emscripten_get_fullscreen_status(fullscreenStatus: *mut EmscriptenFullscreenChangeEvent) -> i32; 
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EmscriptenFullscreenStrategy {
	pub scaleMode: i32,
	pub canvasResolutionScaleMode: i32,
	pub filteringMode: i32,
	pub canvasResizedCallback: em_canvasresized_callback_func,
	pub canvasResizedCallbackUserData: *mut CVoid,
}

extern "C" {
	pub fn emscripten_request_fullscreen(target: *const CChar, deferUntilInEventHandler: i32) -> i32; 
	pub fn emscripten_request_fullscreen_strategy(target: *const CChar, deferUntilInEventHandler: i32, fullscreenStrategy: *const EmscriptenFullscreenStrategy) -> i32; 
	pub fn emscripten_exit_fullscreen () -> i32; 
	pub fn emscripten_enter_soft_fullscreen(target: *const CChar, fullscreenStrategy: *const EmscriptenFullscreenStrategy) -> i32; 
	pub fn emscripten_exit_soft_fullscreen () -> i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EmscriptenPointerlockChangeEvent {
	pub isActive: i32,
	pub nodeName: [CChar; 128],
	pub id: [CChar; 128]
}

extern "C" {
	pub fn emscripten_set_pointerlockchange_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_pointerlockchange_callback_func) -> i32; 
}

extern "C" {
	pub fn emscripten_set_pointerlockerror_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_pointerlockerror_callback_func) -> i32; 
	pub fn emscripten_get_pointerlock_status(pointerlockStatus: *mut EmscriptenPointerlockChangeEvent) -> i32; 
	pub fn emscripten_request_pointerlock(target: *const CChar, deferUntilInEventHandler: i32) -> i32; 
	pub fn emscripten_exit_pointerlock () -> i32; 
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EmscriptenVisibilityChangeEvent {
	pub hidden: i32,
	pub visibilityState: i32
}

extern "C" {
	pub fn emscripten_set_visibilitychange_callback(userData: *mut CVoid, useCapture: i32, callback: em_visibilitychange_callback_func) -> i32; 
	pub fn emscripten_get_visibility_status(visibilityStatus: *mut EmscriptenVisibilityChangeEvent) -> i32; 
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EmscriptenTouchPoint {
	pub identifier: i32,
	pub screenX: i32,
	pub screenY: i32,
	pub clientX: i32,
	pub clientY: i32,
	pub pageX: i32,
	pub pageY: i32,
	pub isChanged: i32,
	pub onTarget: i32,
	pub targetX: i32,
	pub targetY: i32,
	pub canvasX: i32,
	pub canvasY: i32
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EmscriptenTouchEvent {
	pub numTouches: i32,
	pub ctrlKey: i32,
	pub shiftKey: i32,
	pub altKey: i32,
	pub metaKey: i32,
	pub touches: [EmscriptenTouchPoint; 32]
}

extern "C" {
	pub fn emscripten_set_touchstart_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_touch_callback_func) -> i32; 
	pub fn emscripten_set_touchend_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_touch_callback_func) -> i32; 
	pub fn emscripten_set_touchmove_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_touch_callback_func) -> i32; 
	pub fn emscripten_set_touchcancel_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_touch_callback_func) -> i32; 
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EmscriptenGamepadEvent {
	pub timestamp: f64,
	pub numAxes: i32,
	pub numButtons: i32,
	pub axis: [f64; 64],
	pub analogButton: [f64; 64],
	pub digitalButton: [i32; 64],
	pub connected: i32,
	pub index: i32,
	pub id: [CChar; 64],
	pub mapping: [CChar; 64]
}

extern "C" {
	pub fn emscripten_set_gamepadconnected_callback(userData: *mut CVoid, useCapture: i32, callback: em_gamepad_callback_func) -> i32; 
	pub fn emscripten_set_gamepaddisconnected_callback(userData: *mut CVoid, useCapture: i32, callback: em_gamepad_callback_func) -> i32; 
	pub fn emscripten_get_num_gamepads () -> i32; 
	pub fn emscripten_get_gamepad_status(index: i32, gamepadState: *mut EmscriptenGamepadEvent) -> i32; 
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EmscriptenBatteryEvent {
	pub chargingTime: f64,
	pub dischargingTime: f64,
	pub level: f64,
	pub charging: i32
}

extern "C" {
	pub fn emscripten_set_batterychargingchange_callback(userData: *mut CVoid, callback: em_battery_callback_func) -> i32; 
	pub fn emscripten_set_batterylevelchange_callback(userData: *mut CVoid, callback: em_battery_callback_func) -> i32; 
	pub fn emscripten_get_battery_status(batteryState: *mut EmscriptenBatteryEvent) -> i32; 
	pub fn emscripten_vibrate(msecs: i32) -> i32; 
	pub fn emscripten_vibrate_pattern(msecsArray: *mut i32, numEntries: i32) -> i32; 
}

extern "C" {
	pub fn emscripten_set_beforeunload_callback(userData: *mut CVoid, callback: em_beforeunload_callback) -> i32; 
}

pub type EMSCRIPTEN_WEBGL_CONTEXT_HANDLE = i32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EmscriptenWebGLContextAttributes {
	pub alpha: i32,
	pub depth: i32,
	pub stencil: i32,
	pub antialias: i32,
	pub premultipliedAlpha: i32,
	pub preserveDrawingBuffer: i32,
	pub preferLowPowerToHighPerformance: i32,
	pub failIfMajorPerformanceCaveat: i32,
	pub majorVersion: i32,
	pub minorVersion: i32,
	pub enableExtensionsByDefault: i32,
	pub explicitSwapControl: i32, 
}

extern "C" {
	pub fn emscripten_webgl_init_context_attributes(attributes: *mut EmscriptenWebGLContextAttributes); 
	pub fn emscripten_webgl_create_context(target: *const CChar, attributes: *const EmscriptenWebGLContextAttributes) -> EMSCRIPTEN_WEBGL_CONTEXT_HANDLE; 
	pub fn emscripten_webgl_make_context_current(context: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE) -> i32; 
	pub fn emscripten_webgl_get_current_context () -> EMSCRIPTEN_WEBGL_CONTEXT_HANDLE; 
	pub fn emscripten_webgl_destroy_context(context: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE) -> i32; 
	pub fn emscripten_webgl_enable_extension(context: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE, extension: *const CChar) -> i32;
}

extern "C" {
	pub fn emscripten_set_webglcontextlost_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_webgl_context_callback) -> i32; 
	pub fn emscripten_set_webglcontextrestored_callback(target: *const CChar, userData: *mut CVoid, useCapture: i32, callback: em_webgl_context_callback) -> i32; 
	pub fn emscripten_is_webgl_context_lost(target: *const CChar) -> i32; 
	pub fn emscripten_webgl_commit_frame () -> i32; 
	pub fn emscripten_set_element_css_size(target: *const CChar, width: f64, height: f64) -> i32; 
	pub fn emscripten_get_element_css_size(target: *const CChar, width: *mut f64, height: *mut f64) -> i32; 
}

