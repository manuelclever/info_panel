use icalendar::Component;

use crate::webdav::read_calendar;

mod webdav;

fn main() {
    let option = read_calendar("default.config","persnlich");

    match option {
        Some(calendar) => println!("{}", calendar),
        None => ()
    }
}