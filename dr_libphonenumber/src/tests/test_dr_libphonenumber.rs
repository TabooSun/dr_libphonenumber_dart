#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use std::os::raw::c_char;
    
    use crate::dr_libphonenumber;
    use crate::string_helper;
    use crate::dr_libphonenumber::PhoneNumberFormat;
    use crate::free_memory::free_memory;
    use crate::utils::number_type;

    static PHONE_NUMBER: &'static str = "0129602189";
    static ISO_CODE: &'static str = "MY";

    #[test]
    fn format_phone_number_in_e164_format() {
        let result = dr_libphonenumber::format(parse_str_to_c_char(PHONE_NUMBER), parse_str_to_c_char(ISO_CODE), PhoneNumberFormat::E164);
        unsafe {
            let result = Box::from_raw(result);
            assert_eq!(string_helper::parse_c_char_to_str(result.data, "phone_number"), String::from("+60129602189"));
        }
    }

    #[test]
    fn format_phone_number_in_international_format() {
        let result = dr_libphonenumber::format(parse_str_to_c_char(PHONE_NUMBER), parse_str_to_c_char(ISO_CODE), PhoneNumberFormat::International);
        unsafe {
            let result = Box::from_raw(result);
            assert_eq!(string_helper::parse_c_char_to_str(result.data, "phone_number"), String::from("+60 12-960 2189"));
        }
    }

     #[test]
     fn format_phone_number_in_national_format() {
         let result = dr_libphonenumber::format(parse_str_to_c_char(PHONE_NUMBER), parse_str_to_c_char(ISO_CODE), PhoneNumberFormat::National);
         unsafe {
             let result = Box::from_raw(result);
             assert_eq!(string_helper::parse_c_char_to_str(result.data, "phone_number"), String::from("012-960 2189"));
         }
     }
 
     #[test]
     fn format_phone_number_in_rfc3966_format() {
         let result = dr_libphonenumber::format(parse_str_to_c_char(PHONE_NUMBER), parse_str_to_c_char(ISO_CODE), PhoneNumberFormat::Rfc3966);
         unsafe {
             let result = Box::from_raw(result);
             assert_eq!(string_helper::parse_c_char_to_str(result.data, "phone_number"), String::from("tel:+60-12-960-2189"));
         }
     }
 
     #[test]
     fn format_phone_number_with_lowercase_iso_code() {
         let result = dr_libphonenumber::format(parse_str_to_c_char(PHONE_NUMBER), parse_str_to_c_char("my"), PhoneNumberFormat::International);
         unsafe {
             let result = Box::from_raw(result);
             assert_eq!(string_helper::parse_c_char_to_str(result.data, "phone_number"), String::from("+60 12-960 2189"));
         }
     }
 
     #[test]
     fn get_number_type() {
         let phone_number_type = dr_libphonenumber::get_number_type(parse_str_to_c_char(PHONE_NUMBER), parse_str_to_c_char(ISO_CODE));
         assert_eq!(phone_number_type, number_type::PhoneNumberType::Mobile);
     }
 
     #[test]
     fn get_region_code_for_country_code() {
         let region_code = dr_libphonenumber::getRegionCodeForCountryCode(60);
         assert_eq!(string_helper::parse_c_char_to_str(region_code, "region_code"), ISO_CODE);
         free_memory(region_code);
     }

    #[test]
    fn get_region_info() {
        let region_info = dr_libphonenumber::getRegionInfo(parse_str_to_c_char(PHONE_NUMBER), parse_str_to_c_char(ISO_CODE));
        unsafe {
            let region_info = Box::from_raw(region_info);
            assert_eq!(region_info.region_code, 60);
            assert_eq!(string_helper::parse_c_char_to_str(region_info.country_code, "country_code"), ISO_CODE);
            assert_eq!(region_info.phone_number_value, 129602189);
            assert_eq!(string_helper::parse_c_char_to_str(region_info.formatted_number, "phone_number_value"), "012-960 2189");
        }
    }

    #[test]
    fn is_valid_phone_number() {
        let is_valid_phone_number = dr_libphonenumber::isValidPhoneNumber(parse_str_to_c_char(PHONE_NUMBER), parse_str_to_c_char(ISO_CODE));
        assert_eq!(is_valid_phone_number, true);
    }

    fn parse_str_to_c_char(s: &str) -> *const c_char {
        CString::new(s).unwrap().into_raw()
    }
}
