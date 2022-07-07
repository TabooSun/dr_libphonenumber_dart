mod dr_libphonenumber;

pub mod clean_up {
    pub mod free_memory;
}

pub use crate::clean_up::free_memory;

mod tests {
    pub mod test_dr_libphonenumber;
}

pub use crate::tests::test_dr_libphonenumber;

mod utils {
    pub mod string_helper;
    pub mod number_type;
}

pub use crate::utils::string_helper;
pub use crate::utils::number_type;

pub mod models {
    pub mod region_info;
    pub mod lib_phonenumber_result;
    pub mod phone_number_format;
}

pub use crate::models::region_info;
pub use crate::models::lib_phonenumber_result;
pub use crate::models::phone_number_format::PhoneNumberFormat;
