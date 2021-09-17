use std::ffi::CString;
use std::os::raw::c_char;
use crate::models::region_info::RegionInfo;

#[no_mangle]
pub extern "C" fn freeCChar(str: *mut c_char) {
    unsafe {
        if str.is_null() { return; }
        CString::from_raw(str)
    };
}

#[no_mangle]
pub extern "C" fn freeRegionInfo(unsafe_struct: *mut RegionInfo) {
    if unsafe_struct.is_null() { return; }
    unsafe {
        Box::from_raw(unsafe_struct);
    }
}
