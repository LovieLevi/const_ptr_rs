use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct Ptr<T> {
    ptr: Lazy<Arc<Mutex<T>>>,
}

impl<T: Default> Ptr<T> {
    pub const fn new() -> Self {
        Ptr {
            ptr: Lazy::new(|| Arc::new(Mutex::new(Default::default()))),
        }
    }

    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.ptr.lock().unwrap().clone()
    }

    pub fn get_ref(&self) -> MutexGuard<T> {
        self.ptr.lock().unwrap()
    }

    pub fn set(&self, value: T) {
        *self.ptr.lock().unwrap() = value;
    }
}
