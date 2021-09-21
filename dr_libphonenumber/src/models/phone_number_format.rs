use phonenumber::Mode;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum PhoneNumberFormat {
    /// E.164 formatting, no spaces, no decorations.
    #[allow(dead_code)]
    E164,

    /// International formatting, contains country code and country dependent
    /// formatting.
    #[allow(dead_code)]
    International,

    /// National formatting, no country code and country dependent formatting.
    #[allow(dead_code)]
    National,

    /// RFC3966 formatting, see the RFC.
    #[allow(dead_code)]
    Rfc3966,
}

impl PhoneNumberFormat {
    pub fn parse_to_mode(&self) -> Mode {
         match self {
             PhoneNumberFormat::E164 => { Mode::E164 }
             PhoneNumberFormat::International => { Mode::International }
             PhoneNumberFormat::National => { Mode::National }
             PhoneNumberFormat::Rfc3966 => { Mode::Rfc3966 }
         }
    }
}
