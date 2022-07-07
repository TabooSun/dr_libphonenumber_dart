use std::os::raw::{c_char, c_void};
use crate::free_memory::free_memory;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct LibPhoneNumberResult<T> {
    pub data: *mut T,
    pub error: *mut c_char,
}

impl<T> Drop for LibPhoneNumberResult<T> {
    fn drop(&mut self) {
        free_memory(self.data as *mut c_void);
        free_memory(self.error as *mut c_void);
    }
}
