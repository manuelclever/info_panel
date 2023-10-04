use std::fmt;

use icalendar::Calendar as ICalendar;

use crate::webdav::calendar::vevent::VEvent;
use crate::webdav::calendar::vtimezone::VTimezone;
use crate::webdav::calendar::vtodo::VTodo;

pub(crate) mod vevent;
pub(crate) mod vtodo;
pub(crate) mod vtimezone;

pub struct Calendar {
    pub name: String,
    pub events: Vec<VEvent>,
    pub todos: Vec<VTodo>,
    pub timezone: Option<VTimezone>,
}

impl Calendar {
    pub(crate) fn new(name: String, icals: Vec<ICalendar>) -> Self {
        Calendar{
            name: "".to_string(),
            events: vec![],
            todos: vec![],
            timezone: VTimezone::new()
        }
    }
}


impl fmt::Display for Calendar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Name: {}, events: {}, todos:{}]",
               self.name,
               self.events.iter()
                   .map(|obj| obj.to_string())
                   .collect::<Vec<String>>()
                   .join(", "),
               self.todos.iter()
                   .map(|obj| obj.to_string())
                   .collect::<Vec<String>>()
                   .join(", "))
    }
}