use std::{
    cell::UnsafeCell,
    fmt::{self, Display, Formatter},
    ptr::NonNull,
};

/// A struct that holds a pointer to a memory location of type.
pub struct Ptr<T> {
    ptr: NonNull<UnsafeCell<T>>,
}

impl<T> Ptr<T> {
    pub fn new(value: T) -> Self {
        let boxed = Box::new(UnsafeCell::new(value));
        Self {
            ptr: NonNull::new(Box::into_raw(boxed)).expect("Allocation failed"),
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.ptr.as_ref().get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.ptr.as_ref().get() }
    }

    pub fn get_clone(&self) -> T
    where
        T: Clone,
    {
        unsafe { (*self.ptr.as_ref().get()).clone() }
    }

    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.ptr.as_ref().get() }
    }

    pub fn get_ref(&self) -> &T {
        unsafe { &*self.ptr.as_ref().get() }
    }
}

impl<T> Drop for Ptr<T> {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.ptr.as_ptr()));
        }
    }
}

impl<T: Display> Display for Ptr<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.get_ref())
    }
}

#[cfg(test)]
mod ptr_tests {
    use super::Ptr;

    #[test]
    fn test_get() {
        let ptr = Ptr::new(42);
        assert_eq!(ptr.get(), 42);
    }

    #[test]
    fn test_get_clone() {
        let ptr = Ptr::new(42);
        assert_eq!(ptr.get_clone(), 42);
    }

    #[test]
    fn test_set() {
        let ptr = Ptr::new(42);
        ptr.set(43);
        assert_eq!(ptr.get(), 43);
    }

    #[test]
    fn test_get_mut() {
        let ptr = Ptr::new(42);
        *ptr.get_mut() = 43;
        assert_eq!(ptr.get(), 43);
    }

    #[test]
    fn test_get_ref() {
        let ptr = Ptr::new(42);
        assert_eq!(*ptr.get_ref(), 42);
    }

    mod display {
        use super::Ptr;

        #[test]
        fn test_i32() {
            let value: i32 = 42;
            let ptr = Ptr::new(value);
            assert_eq!(format!("{}", ptr), format!("{}", value));
        }

        #[test]
        fn test_string() {
            let value = "Hello, World!";
            let ptr = Ptr::new(value);
            assert_eq!(format!("{}", ptr), format!("{}", value));
        }

        #[derive(Copy, Clone, PartialEq)]
        struct TestStruct {
            value: i32,
        }

        impl std::fmt::Display for TestStruct {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        #[test]
        fn test_struct() {
            let value = TestStruct { value: 42 };
            let ptr = Ptr::new(value);
            assert_eq!(format!("{}", ptr), format!("{}", value));
        }
    }
}
