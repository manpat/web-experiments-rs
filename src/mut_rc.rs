use std::cell::{RefCell, RefMut, Ref};
use std::rc::Rc;

pub struct MutRc<T> (Rc<RefCell<T>>);

impl<T> MutRc<T> {
	pub fn new(d: T) -> Self {
		MutRc(Rc::new(RefCell::new(d)))
	}

	pub fn borrow_mut(&self) -> RefMut<T> {
		self.0.borrow_mut()
	}

	pub fn borrow(&self) -> Ref<T> {
		self.0.borrow()
	}
}

impl<T> Clone for MutRc<T> {
	fn clone(&self) -> Self {
		MutRc(self.0.clone())
	}
}