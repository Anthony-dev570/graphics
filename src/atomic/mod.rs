use std::sync::{Arc, Mutex};

pub mod atomic_macro;

pub struct Atomic<T>(pub Arc<Mutex<T>>);

impl <T> Atomic<T> {
    pub fn new(value: T) -> Atomic<T> {
        Self(Arc::new(Mutex::new(value)))
    }
}

impl <T> Clone for Atomic<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl <T: Default> Default for Atomic<T> {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(T::default())))
    }
}