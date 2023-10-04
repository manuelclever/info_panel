use std::fmt;

use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct VTimezone {
    pub version: String,
    pub cal_scale: String,
    pub prodid: String,
    
    pub tzid: String,
    
    pub dl_tz_offset_from: String,
    pub dl_tz_offset_to: String,
    pub dl_tz_name: String,
    pub dl_date_start: NaiveDateTime,
    pub dl_rrule_freq: String,
    pub dl_rrule_by_month: String,
    pub dl_rrule_by_day: String,

    pub df_tz_offset_from: String,
    pub df_tz_offset_to: String,
    pub df_tz_name: String,
    pub df_date_start: NaiveDateTime,
    pub df_rrule_freq: String,
    pub df_rrule_by_month: String,
    pub df_rrule_by_day: String
}

impl VTimezone {
    pub(crate) fn new() -> Option<Self> {
        None
    }
    /*
    pub(crate) fn new(icalendar: &ICalendar) -> Option<Self> {
        let mut version: String = String::new();
        let mut cal_scale: String = String::new();
        let mut prodid: String = String::new();

        let mut tzid: String = String::new();

        let mut dl_tz_offset_from: String = String::new();
        let mut dl_tz_offset_to: String = String::new();
        let mut dl_tz_name: String = String::new();
        let mut dl_date_start: NaiveDateTime = NaiveDateTime::default();
        let mut dl_rrule_freq: String = String::new();
        let mut dl_rrule_by_month: String = String::new();
        let mut dl_rrule_by_day: String = String::new();

        let mut df_tz_offset_from: String = String::new();
        let mut df_tz_offset_to: String = String::new();
        let mut df_tz_name: String = String::new();
        let mut df_date_start: NaiveDateTime = NaiveDateTime::default();
        let mut df_rrule_freq: String = String::new();
        let mut df_rrule_by_month: String = String::new();
        let mut df_rrule_by_day: String = String::new();

        for property in &icalendar.properties {
            match property.key() {
                "VERSION" => version = property.value().to_string(),
                "CALSCALE" => cal_scale = property.value().to_string(),
                "PRODID" => prodid = property.value().to_string(),
                _default => (),
            }
        }

        //todo
        //iCalendar only has events and todos, but not timezones.
            
        None
    }
     */
}

impl fmt::Display for VTimezone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[]")
    }
}