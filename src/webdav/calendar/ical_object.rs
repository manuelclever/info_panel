use std::fmt;

use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct ICalObject {
    pub cal_scale: String,
    pub uid: String,
    pub date_start: NaiveDateTime,
    pub date_end: NaiveDateTime,
    pub is_all_day: bool,
    pub summary: String,
    pub desc: String,
    pub date_timestamp: NaiveDateTime
}

impl ICalObject {
    pub(crate) fn new() -> Self{
        return ICalObject {
            cal_scale: String::new(),
            uid: String::new(),
            date_start: NaiveDateTime::default(),
            date_end: NaiveDateTime::default(),
            is_all_day: false, // Set the boolean field as needed
            summary: String::new(),
            desc: String::new(),
            date_timestamp: NaiveDateTime::default(),
        };
    }
}

impl PartialEq for ICalObject {
    fn eq(&self, other: &Self) -> bool {
        self.cal_scale == other.cal_scale &&
            self.uid == other.uid &&
            self.date_start == other.date_start &&
            self.date_end == other.date_end &&
            self.is_all_day == other.is_all_day &&
            self.summary == other.summary &&
            self.desc == other.desc &&
            self.date_timestamp == other.date_timestamp
    }
}

impl fmt::Display for ICalObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[calscale: {}, uid: [{}], date_start: {}, date_end: {}, is_all_day: {}, summary:{}, desc: {}, date_stamp: {}]",
               self.cal_scale,
               self.uid,
               self.date_start,
               self.date_end,
               self.is_all_day,
               self.summary,
               self.desc,
               self.date_timestamp)
    }
}