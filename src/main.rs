use reqwest::Error as ReqwestError;

use crate::webdav::calendar::Calendar;
use crate::webdav::read_calendar;

mod webdav;

fn main() {
    let result: Result<Calendar,ReqwestError> = read_calendar("default.config","abfall");

    match result {
        Ok(calendar) => println!("{}",calendar),
        Err(e) => println!("Error reading calendar: {}", e)
    }
}