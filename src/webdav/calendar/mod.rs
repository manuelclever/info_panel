use std::fmt;

use crate::webdav::calendar::ical_object::ICalObject;

pub(crate) mod ical_object;

pub struct Calendar {
    pub name: String,
    pub events: Vec<ICalObject>,
    pub timezone: ICalObject,
}

impl Calendar {
    pub(crate) fn new() -> Self {
        Calendar{
            name: "".to_string(),
            events: vec![],
            timezone: ICalObject::new()
        }
    }
}

impl fmt::Display for Calendar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Name: {}, events: {}, timezone: {}]",
               self.name,
               self.events.iter()
                   .map(|obj| obj.to_string())
                   .collect::<Vec<String>>()
                   .join(", "),
               self.timezone)
    }
}
