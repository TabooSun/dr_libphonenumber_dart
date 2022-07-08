#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use std::os::raw::c_char;

    use crate::{dr_libphonenumber, DrPhoneNumberFormat};
    use crate::string_helper;
    use crate::utils::number_type;

    static PHONE_NUMBER: &'static str = "0129602189";
    static ISO_CODE: &'static str = "MY";

    #[test]
    fn format_phone_number_in_e164_format() {
        let result = dr_libphonenumber::format(parse_str_to_c_char(PHONE_NUMBER), parse_str_to_c_char(ISO_CODE), DrPhoneNumberFormat::E164);
        unsafe {
            let result = Box::from_raw(result);
            assert_eq!(string_helper::parse_c_char_to_str(result.data, "phone_number"), String::from("+60129602189"));
        }
    }

    #[test]
    fn format_phone_number_in_international_format() {
        let result = dr_libphonenumber::format(parse_str_to_c_char(PHONE_NUMBER), parse_str_to_c_char(ISO_CODE), DrPhoneNumberFormat::International);
        unsafe {
            let result = Box::from_raw(result);
            assert_eq!(string_helper::parse_c_char_to_str(result.data, "phone_number"), String::from("+60 12-960 2189"));
        }
    }

    #[test]
    fn format_phone_number_in_national_format() {
        let result = dr_libphonenumber::format(parse_str_to_c_char(PHONE_NUMBER), parse_str_to_c_char(ISO_CODE), DrPhoneNumberFormat::National);
        unsafe {
            let result = Box::from_raw(result);
            assert_eq!(string_helper::parse_c_char_to_str(result.data, "phone_number"), String::from("012-960 2189"));
        }
    }

    #[test]
    fn format_phone_number_in_rfc3966_format() {
        let result = dr_libphonenumber::format(parse_str_to_c_char(PHONE_NUMBER), parse_str_to_c_char(ISO_CODE), DrPhoneNumberFormat::Rfc3966);
        unsafe {
            let result = Box::from_raw(result);
            assert_eq!(string_helper::parse_c_char_to_str(result.data, "phone_number"), String::from("tel:+60-12-960-2189"));
        }
    }

    #[test]
    fn format_phone_number_with_lowercase_iso_code() {
        let result = dr_libphonenumber::format(parse_str_to_c_char(PHONE_NUMBER), parse_str_to_c_char("my"), DrPhoneNumberFormat::International);
        unsafe {
            let result = Box::from_raw(result);
            assert_eq!(string_helper::parse_c_char_to_str(result.data, "phone_number"), String::from("+60 12-960 2189"));
        }
    }
    
    #[test]
    fn format_invalid_phone_number_in_e164_format() {
        let result = dr_libphonenumber::format(parse_str_to_c_char("0"), parse_str_to_c_char(ISO_CODE), DrPhoneNumberFormat::E164);
        unsafe {
            let result = Box::from_raw(result);
            assert!(result.data.is_null());
            assert!(!result.error.is_null());
            assert!(!string_helper::parse_c_char_to_str(result.error, "error").is_empty());
        }
    }

    #[test]
    fn get_number_type() {
        let phone_number_type = dr_libphonenumber::get_number_type(parse_str_to_c_char(PHONE_NUMBER), parse_str_to_c_char(ISO_CODE));
        unsafe {
            let phone_number_type = Box::from_raw(phone_number_type);
            assert_eq!(phone_number_type.data, number_type::DrPhoneNumberType::Mobile);
        }
    }

    #[test]
    fn get_region_code_for_country_code() {
        let region_code = dr_libphonenumber::get_region_code_for_country_code(60);
        unsafe {
            let region_code = Box::from_raw(region_code);
            assert_eq!(string_helper::parse_c_char_to_str(region_code.data, "region_code"), ISO_CODE);
        }
    }

    #[test]
    fn get_region_info() {
        let region_info = dr_libphonenumber::get_region_info(parse_str_to_c_char(PHONE_NUMBER), parse_str_to_c_char(ISO_CODE));
        unsafe {
            let region_info = &*(Box::from_raw(region_info).data);
            assert_eq!(region_info.region_code, 60);
            assert_eq!(string_helper::parse_c_char_to_str(region_info.country_code, "country_code"), ISO_CODE);
            assert_eq!(region_info.phone_number_value, 129602189);
            assert_eq!(string_helper::parse_c_char_to_str(region_info.formatted_number, "phone_number_value"), "012-960 2189");
        }
    }

    #[test]
    fn is_valid_phone_number() {
        let is_valid_phone_number = dr_libphonenumber::is_valid_phone_number(parse_str_to_c_char(PHONE_NUMBER), parse_str_to_c_char(ISO_CODE));
        unsafe {
            let is_valid_phone_number = Box::from_raw(is_valid_phone_number);
            assert_eq!(is_valid_phone_number.data, true);
        }
    }

    fn parse_str_to_c_char(s: &str) -> *const c_char {
        CString::new(s).unwrap().into_raw()
    }
}
