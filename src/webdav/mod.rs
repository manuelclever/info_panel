use reqwest::Error as ReqwestError;

use crate::webdav::calendar::Calendar;
use crate::webdav::calendar::ical_object::ICalObject;
use crate::webdav::connection::Connection;
use crate::webdav::parsing::parse_ical_object;

pub mod parsing;
pub mod response;
mod connection;
pub mod calendar;

pub(crate) fn read_calendar(conf: &str, calendar_name: &str) -> Result<Calendar,ReqwestError> {
    let result: Result<Connection,String> = Connection::new(&format!("data/{}",conf));
    let connection: Connection = result.unwrap();

    let mut calendar: Calendar = Calendar::new();
    let mut events: Vec<ICalObject> = Vec::new();
    let responses: Vec<response::Response> = connection.get_responses(&format!("//{}",calendar_name)).unwrap();
    for response in responses {

        if response.href.ends_with("ics") {
            let event_string: Result<String, ReqwestError> = connection.get_event(&format!("//{}/{}", calendar_name, response.ical_file));

            match event_string {
                Ok(event_string) => {
                    events.push(parse_ical_object(&event_string))
                },
                Err(e) => return Err(e)
            }
        } else {
            calendar.name = response.prop.displayname;
            calendar.timezone = parse_ical_object(&response.prop.calendar_timezone);
        }
    }
    calendar.events = events;
    Ok(calendar)
}