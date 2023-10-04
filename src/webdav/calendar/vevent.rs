use std::fmt;

use chrono::NaiveDateTime;
use icalendar::{Calendar as ICalendar, Component};

use crate::webdav::parsing::parse_date;

#[derive(Debug)]
pub struct VEvent {
    pub version: String,
    pub cal_scale: String,
    pub prodid: String,

    pub desc: String,
    pub summary: String,
    pub uid: String,

    pub date_start: NaiveDateTime,
    pub date_end: NaiveDateTime,
    pub date_timestamp: NaiveDateTime,
    pub is_all_day: bool,
}

impl VEvent {
    pub(crate) fn new(icalendar: &ICalendar) -> Option<Self> {
        let mut version: String = String::new();
        let mut cal_scale: String = String::new();
        let mut prodid: String = String::new();

        let mut desc: String = String::new();
        let mut summary: String = String::new();
        let mut uid: String = String::new();

        let mut date_start: NaiveDateTime = NaiveDateTime::default();
        let mut date_end: NaiveDateTime = NaiveDateTime::default();
        let mut date_timestamp: NaiveDateTime = NaiveDateTime::default();
        let mut is_all_day: bool = false;

        for property in &icalendar.properties {
            match property.key() {
                "VERSION" => version = property.value().to_string(),
                "CALSCALE" => cal_scale = property.value().to_string(),
                "PRODID" => prodid = property.value().to_string(),
                _default => (),
            }
        }

        for component in &icalendar.components {
            match component.as_event() {
                Some(event) => {
                    for property in event.properties() {
                        match property.0.as_str() {
                            "DESCRIPTION" => desc = property.1.value().to_string(),
                            "DTSTART" => date_start = parse_date(property.1),
                            "DTEND" => date_end = parse_date(property.1),
                            "DTSTAMP" => date_timestamp = parse_date(property.1),
                            "SUMMARY" => summary = property.1.value().to_string(),
                            "TRANSP" => (),
                            "UID" => uid = property.1.value().to_string(),
                            "URL" => (),
                            "X-FUNAMBOL-ALLDAY" => is_all_day = property.1.value() == "1",
                            _default => (),
                        }
                    }
                },
                None => return None
            }
        }

        Some(VEvent {
            version,
            cal_scale,
            prodid,
            desc,
            summary,
            uid,
            date_start,
            date_end,
            date_timestamp,
            is_all_day
        })
    }
}

impl PartialEq for VEvent {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version &&
            self.cal_scale == other.cal_scale &&
            self.prodid == other.prodid &&
            self.desc == other.desc &&
            self.summary == other.summary &&
            self.uid == other.uid &&
            self.date_start == other.date_start &&
            self.date_end == other.date_end &&
            self.date_timestamp == other.date_timestamp &&
            self.is_all_day == other.is_all_day
    }
}

impl fmt::Display for VEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[version: {}, cal_scale: [{}], prodid: {}, desc: {}, summary: {}, uid:{}, date_start: {}, date_end: {}, date_timestamp: {}, is_all_day: {}]",
               self.version,
               self.cal_scale,
               self.prodid,
               self.desc,
               self.summary,
               self.uid,
               self.date_start,
               self.date_end,
               self.date_timestamp,
               self.is_all_day)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use icalendar::parser::{read_calendar as read_icalendar, unfold};

    use crate::webdav::calendar::vevent::VEvent;

    #[test]
    fn create_event() {
        let mut file = File::open(format!("data/test/vevent.ics")).unwrap();
        let mut ics = String::new();
        file.read_to_string(&mut ics).unwrap();

        let unfolded = unfold(&ics);
        let result = read_icalendar(&unfolded);

        match result {
            Ok(icalendar) => {
                let option = VEvent::new(&icalendar.into());

                match option {
                    Some(vevent) => {
                        let vevent_expected = VEvent{
                            version: "2.0".to_string(),
                            cal_scale: "GREGORIAN".to_string(),
                            prodid: "-//hacksw/handcal//NONSGML v1.0//EN".to_string(),
                            desc: "".to_string(),
                            summary: "Restmülltonne\\, Biotonne\\, Altpapiertonne\\, Gelber Sack".to_string(),
                            uid: "082c600294b2948e371fee12ae989ff5@eaw-rtk.de".to_string(),
                            date_start: NaiveDateTime::new(
                                NaiveDate::from_ymd_opt(2022,07,26).unwrap(),
                                NaiveTime::default()),
                            date_end: NaiveDateTime::new(
                                NaiveDate::from_ymd_opt(2022,07,27).unwrap(),
                                NaiveTime::default()),
                            date_timestamp: NaiveDateTime::new(
                                NaiveDate::from_ymd_opt(2022,08,22).unwrap(),
                                NaiveTime::from_hms_opt(18,10,09).unwrap()),
                            is_all_day: true,
                        };
                        assert_eq!(vevent, vevent_expected);
                    }
                    None => assert!(false)
                }
            },
            Err(_) => assert!(false)
        }
    }
}