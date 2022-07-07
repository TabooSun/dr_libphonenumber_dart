use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null_mut;
use std::str::FromStr;

use phonenumber::{
    country::Id,
    metadata::DATABASE,
    Mode,
    PhoneNumber,
};
use crate::lib_phonenumber_result::{LibPhoneNumberResult, MutableLibPhoneNumberResult};

use crate::region_info::RegionInfo;
use crate::{number_type, PhoneNumberFormat, string_helper};
use crate::utils::number_type::PhoneNumberType;

/// Format the [phone_number] using the [phone_number_format].
#[no_mangle]
pub extern "C" fn format(phone_number: *const c_char, iso_code: *const c_char, phone_number_format: PhoneNumberFormat) -> *mut MutableLibPhoneNumberResult<c_char> {
    let country = match parse_iso_code(iso_code) {
        Ok(country) => country,
        Err(error) => {
            return Box::into_raw(Box::new(MutableLibPhoneNumberResult {
                data: null_mut(),
                error: parse_string_to_c_char(error.as_str()),
            }));
        }
    };

    let phone_number = match parse_phone_number(phone_number, country) {
        Ok(phone_number) => phone_number,
        Err(error) => {
            return Box::into_raw(Box::new(MutableLibPhoneNumberResult {
                data: null_mut(),
                error: parse_string_to_c_char(error.as_str()),
            }));
        }
    };

    let phone_number_format = phone_number_format.parse_to_mode();

    Box::into_raw(Box::new(MutableLibPhoneNumberResult {
        data: parse_string_to_c_char(phone_number.format().mode(phone_number_format).to_string().as_str()),
        error: null_mut(),
    }))
}

#[no_mangle]
pub extern "C" fn format_as_you_type(phone_number: *const c_char, iso_code: *const c_char, phone_number_format: PhoneNumberFormat) -> *mut MutableLibPhoneNumberResult<c_char> {
    let country = match parse_iso_code(iso_code).ok() {
        None => {
            return Box::into_raw(Box::new(MutableLibPhoneNumberResult {
                data: null_mut(),
                error: parse_string_to_c_char(string_helper::parse_c_char_to_str(phone_number, "phone_number").as_str()),
            }));
        }
        Some(country) => country,
    };

    let phone_number = match parse_phone_number(phone_number, country) {
        Ok(phone_number) => phone_number,
        Err(error) => {
            return Box::into_raw(Box::new(MutableLibPhoneNumberResult {
                data: null_mut(),
                error: parse_string_to_c_char(error.as_str()),
            }));
        }
    };

    let phone_number_format = phone_number_format.parse_to_mode();
    Box::into_raw(Box::new(MutableLibPhoneNumberResult {
        data: parse_string_to_c_char(phone_number.format().mode(phone_number_format).to_string().as_str()),
        error: null_mut(),
    }))
}

#[no_mangle]
pub extern "C" fn get_number_type(phone_number: *const c_char, iso_code: *const c_char) -> *mut LibPhoneNumberResult<PhoneNumberType> {
    let country = match parse_iso_code(iso_code) {
        Ok(country) => country,
        Err(error) => {
            return Box::into_raw(Box::new(LibPhoneNumberResult {
                data: PhoneNumberType::Unknown,
                error: parse_string_to_c_char(error.as_str()),
            }));
        }
    };

    let phone_number = match parse_phone_number(phone_number, country) {
        Ok(phone_number) => phone_number,
        Err(error) => {
            return Box::into_raw(Box::new(LibPhoneNumberResult {
                data: PhoneNumberType::Unknown,
                error: parse_string_to_c_char(error.as_str()),
            }));
        }
    };

    let metadata = match phone_number.metadata(&*DATABASE) {
        None => {
            return Box::into_raw(Box::new(LibPhoneNumberResult {
                data: PhoneNumberType::Unknown,
                error: parse_string_to_c_char("Unable to retrieve metadata."),
            }));
        }
        Some(metadata) => metadata
    };
    let national_phone_number = phone_number.national().to_string();

    Box::into_raw(Box::new(LibPhoneNumberResult {
        data: number_type::get_number_type(metadata, &national_phone_number),
        error: null_mut(),
    }))
}

#[no_mangle]
pub extern "C" fn get_region_code_for_country_code(calling_code: u16) -> *mut MutableLibPhoneNumberResult<c_char> {
    let region_code = match (*DATABASE).by_code(&calling_code) {
        None => {
            return Box::into_raw(Box::new(MutableLibPhoneNumberResult {
                data: null_mut(),
                error: parse_string_to_c_char(format!("Invalid calling_code: {}", calling_code).to_string().as_str()),
            }));
        }
        Some(region_code) => region_code,
    };
    let region_code = match region_code.first() {
        None => {
            return Box::into_raw(Box::new(MutableLibPhoneNumberResult {
                data: null_mut(),
                error: parse_string_to_c_char(format!("Unable to locate the region code for calling_code: {}", calling_code).to_string().as_str()),
            }));
        }
        Some(region_code) => region_code,
    };
    Box::into_raw(Box::new(MutableLibPhoneNumberResult {
        data: parse_string_to_c_char(region_code.id()),
        error: null_mut(),
    }))
}

#[no_mangle]
pub extern "C" fn get_region_info(phone_number: *const c_char, iso_code: *const c_char) -> *mut MutableLibPhoneNumberResult<RegionInfo> {
    let country = match parse_iso_code(iso_code) {
        Ok(country) => country,
        Err(error) => {
            return Box::into_raw(Box::new(MutableLibPhoneNumberResult {
                data: null_mut(),
                error: parse_string_to_c_char(error.as_str()),
            }));
        }
    };
    let phone_number = match parse_phone_number(phone_number, country) {
        Ok(phone_number) => phone_number,
        Err(error) => {
            return Box::into_raw(Box::new(MutableLibPhoneNumberResult {
                data: null_mut(),
                error: parse_string_to_c_char(error.as_str()),
            }));
        }
    };

    Box::into_raw(Box::new(MutableLibPhoneNumberResult {
        data: Box::into_raw(Box::new(RegionInfo {
            region_code: phone_number.country().code(),
            phone_number_value: phone_number.national().value(),
            country_code: parse_string_to_c_char(country.as_ref()),
            formatted_number: parse_string_to_c_char(phone_number.format().mode(Mode::National).to_string().as_str()),
        })),
        error: null_mut(),
    }))
}

#[no_mangle]
pub extern "C" fn is_valid_phone_number(phone_number: *const c_char, iso_code: *const c_char) -> *mut LibPhoneNumberResult<bool> {
    let country = match parse_iso_code(iso_code) {
        Ok(country) => country,
        Err(error) => {
            return Box::into_raw(Box::new(LibPhoneNumberResult {
                data: false,
                error: parse_string_to_c_char(error.as_str()),
            }));
        }
    };

    let phone_number = match parse_phone_number(phone_number, country) {
        Ok(phone_number) => phone_number,
        Err(error) => {
            return Box::into_raw(Box::new(LibPhoneNumberResult {
                data: false,
                error: parse_string_to_c_char(error.as_str()),
            }));
        }
    };

    Box::into_raw(Box::new(LibPhoneNumberResult {
        data: phone_number.is_valid(),
        error: null_mut(),
    }))
}

fn parse_phone_number(phone_number: *const c_char, country: Id) -> Result<PhoneNumber, String> {
    let phone_number = string_helper::parse_c_char_to_str(phone_number, "phone_number");
    return match phonenumber::parse(Some(country), phone_number.clone()) {
        Ok(phone_number) => {
            Ok(phone_number)
        }
        Err(error) => {
            Err(format!("Unable to parse phone number: {}. The error was: {:?}", phone_number, error))
        }
    };
}

fn parse_iso_code(iso_code: *const c_char) -> Result<Id, String> {
    let iso_code = string_helper::parse_c_char_to_str(iso_code, "iso_code");
    return match Id::from_str(iso_code.to_uppercase().as_str()) {
        Ok(country) => {
            Ok(country)
        }
        Err(_) => {
            Err(format!("Invalid ISO code: {}", iso_code))
        }
    };
}

fn parse_string_to_c_char(str: &str) -> *mut c_char {
    CString::new(str).unwrap().into_raw()
}
