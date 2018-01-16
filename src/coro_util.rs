use std::ops::{Generator, GeneratorState};
use bindings::emscripten::*;

pub fn set_coro_as_main_loop<T>(coro: T) -> ! where T: Generator<Yield=(), Return=()> {
	unsafe {
		let coro: Box<Generator<Yield=(), Return=()>> = box coro;
		emscripten_set_main_loop_arg(Some(resume_main_coro), Box::into_raw(box coro) as _, 0, 1)
	}
}

extern "C" fn resume_main_coro(ctx: *mut CVoid) {
	use std::mem::transmute;
	use std::ops::GeneratorState::*;

	let coro: &mut Box<Generator<Yield=(), Return=()>> = unsafe{ transmute(ctx) };

	match coro.resume() {
		Yielded(()) => {}
		Complete(()) => unsafe {
			println!("Main coro has returned");

			Box::from_raw(ctx as _);

			emscripten_cancel_main_loop();
		}
	}
}

pub struct Coro<Y> {
	pub value: Option<Y>,
	
	valid: bool,
	coro: Box<Generator<Yield=Y, Return=()>>,
}

impl<Y> Coro<Y> {
	pub fn is_valid(&self) -> bool { self.valid }
}

impl<Y, G> From<G> for Coro<Y> where G: 'static + Generator<Yield=Y, Return=()> {
	fn from(gen: G) -> Self {
		Coro {
			coro: box gen,
			value: None,
			valid: true,
		}
	}
}

impl<Y> Iterator for Coro<Y> {
	type Item = Y;
	default fn next(&mut self) -> Option<Self::Item> {
		if !self.valid { return None }

		if let GeneratorState::Yielded(yielded_value) = self.coro.resume() {
			Some(yielded_value)
		} else {
			self.valid = false;
			None
		}
	}
}

impl<Y: Clone> Iterator for Coro<Y> {
	fn next(&mut self) -> Option<Self::Item> {
		if !self.valid { return None }

		if let GeneratorState::Yielded(yielded_value) = self.coro.resume() {
			self.value = Some(yielded_value);
			self.value.clone()
		} else {
			self.valid = false;
			None
		}
	}
}

pub struct StackCoro<Y, G: Generator<Yield=Y, Return=()>> {
	pub value: Option<Y>,

	valid: bool,
	coro: G,
}

impl<Y, G> StackCoro<Y, G> where G: Generator<Yield=Y, Return=()> {
	pub fn is_valid(&self) -> bool { self.valid }
}

impl<Y, G> From<G> for StackCoro<Y, G> where G: Generator<Yield=Y, Return=()> {
	fn from(gen: G) -> Self {
		StackCoro {
			coro: gen,
			value: None,
			valid: true,
		}
	}
}

impl<Y, G> Iterator for StackCoro<Y, G> where G: Generator<Yield=Y, Return=()> {
	type Item = Y;
	default fn next(&mut self) -> Option<Self::Item> {
		if !self.valid { return None }

		if let GeneratorState::Yielded(yielded_value) = self.coro.resume() {
			Some(yielded_value)
		} else {
			self.valid = false;
			None
		}
	}
}

impl<Y, G> Iterator for StackCoro<Y, G> where Y: Clone, G: Generator<Yield=Y, Return=()> {
	fn next(&mut self) -> Option<Self::Item> {
		if !self.valid { return None }

		if let GeneratorState::Yielded(yielded_value) = self.coro.resume() {
			self.value = Some(yielded_value);
			self.value.clone()
		} else {
			self.valid = false;
			None
		}
	}
}

#[macro_export]
macro_rules! parameter_lerp {
	( ($rc_obj:expr).$param:ident -> $to:expr, $duration:tt @ $delay:expr, $ease:ident ) => {{
		let rc_obj = $rc_obj.clone();

		let from = rc_obj.borrow().$param;
		let to = $to;

		let delay_frames = ($delay * 60.0) as u32; 
		let num_frames = ($duration * 60.0) as u32;

		Coro::from(move || {
			for _ in 0..delay_frames { yield }

			for i in 0..num_frames {
				let prog = i as f32 / num_frames as f32;
				rc_obj.borrow_mut().$param = prog.$ease(from, to);
				yield;
			}

			rc_obj.borrow_mut().$param = to;
		})
	}};

	( ($rc_obj:expr).$param:ident -> $to:expr, $duration:expr, $ease:ident ) => {{
		parameter_lerp!( ($rc_obj).$param -> $to, $duration @ 0.0, $ease )
	}};

	( $rc_obj:ident.$param:ident -> $to:expr, $duration:tt, $ease:ident ) => {{
		parameter_lerp!( ($rc_obj).$param -> $to, $duration @ 0.0, $ease )
	}};
	( $rc_obj:ident.$param:ident -> $to:expr, $duration:tt @ $delay:expr, $ease:ident ) => {{
		parameter_lerp!( ($rc_obj).$param -> $to, $duration @ $delay, $ease )
	}};
}
