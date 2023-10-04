use std::fmt;

#[derive(Debug)]
pub struct VTimezone {
    pub tzid: String,
}

impl VTimezone {
    pub(crate) fn new() -> Self {
        VTimezone{
            tzid: "".to_string(),
        }
    }
}

impl fmt::Display for VTimezone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[]")
    }
}