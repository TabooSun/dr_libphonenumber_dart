use std::os::raw::c_char;

use crate::clean_up::free_memory::freeCChar;

/// Check https://countrycode.org/ for detail.
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct RegionInfo {
    /// The region code or calling code.
    pub region_code: u16,

    /// The phone number excluding the [region_code].
    pub phone_number_value: u64,

    /// The country code.
    pub country_code: *mut c_char,

    /// The formatted phone number with combination of [region_code] & [phone_number_value].
    pub formatted_number: *mut c_char,
}

impl Drop for RegionInfo {
    fn drop(&mut self) {
        freeCChar(self.country_code);
        freeCChar(self.formatted_number);
    }
}
