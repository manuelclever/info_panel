use std::fmt;

use chrono::NaiveDateTime;
use icalendar::{Calendar as ICalendar, Component};

use crate::webdav::parsing::parse_date;

#[derive(Debug)]
pub struct VTodo {
    pub version: String,
    pub cal_scale: String,
    pub prodid: String,

    pub summary: String,
    pub status: String,
    pub completed: NaiveDateTime,
    pub percent_complete: u8,
    pub sequence: String,
    pub uid: String,

    pub date_timestamp: NaiveDateTime,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

impl VTodo {
    pub(crate) fn new(icalendar: &ICalendar) -> Option<Self> {
        let mut version: String = String::new();
        let mut cal_scale: String = String::new();
        let mut prodid: String = String::new();

        let mut summary: String = String::new();
        let mut status: String = String::new();
        let mut completed: NaiveDateTime = NaiveDateTime::default();
        let mut percent_complete: u8 = 0;
        let mut sequence: String = String::new();
        let mut uid: String = String::new();

        let mut date_timestamp: NaiveDateTime = NaiveDateTime::default();
        let mut created: NaiveDateTime = NaiveDateTime::default();
        let mut last_modified: NaiveDateTime = NaiveDateTime::default();

        for property in &icalendar.properties {
            match property.key() {
                "VERSION" => version = property.value().to_string(),
                "CALSCALE" => cal_scale = property.value().to_string(),
                "PRODID" => prodid = property.value().to_string(),
                _default => (),
            }
        }

        for component in &icalendar.components {
            match component.as_todo() {
                Some(event) => {
                    for property in event.properties() {
                        match property.0.as_str() {
                            "DTSTAMP" => date_timestamp = parse_date(property.1),
                            "UID" => uid = property.1.value().to_string(),
                            "SEQUENCE" => sequence = property.1.value().to_string(),
                            "CREATED" => created = parse_date(property.1),
                            "LAST-MODIFIED" => last_modified = parse_date(property.1),
                            "SUMMARY" => summary = property.1.value().to_string(),
                            "STATUS" => status = property.1.value().to_string(),
                            "COMPLETED" => completed = parse_date(property.1),
                            "PERCENT-COMPLETE" => percent_complete = property.1.value().to_string().parse::<u8>().unwrap(),
                            _default => (),
                        }
                    }
                },
                None => return None
            }
        }

        Some(VTodo{
            version,
            cal_scale,
            prodid,
            summary,
            status,
            completed,
            percent_complete,
            sequence,
            uid,
            date_timestamp,
            created,
            last_modified,
        })
    }
}

impl PartialEq for VTodo {
    fn eq(&self, other: &Self) -> bool {
            self.version == other.version &&
                self.cal_scale == other.cal_scale &&
                self.prodid == other.prodid &&
                self.summary == other.summary &&
                self.status == other.status &&
                self.completed == other.completed &&
                self.percent_complete == other.percent_complete &&
                self.sequence == other.sequence &&
                self.uid == other.uid &&
                self.date_timestamp == other.date_timestamp &&
                self.created == other.created &&
                self.last_modified == other.last_modified
    }
}

impl fmt::Display for VTodo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[version: {}, cal_scale: [{}], prodid: {}, summary: {}, status: {}, completed:{}, percent_complete: {}, sequence: {}, uid {}, date_timestamp {}, created {}, last_modified {}]",
               self.version,
               self.cal_scale,
               self.prodid,
               self.summary,
               self.status,
               self.completed,
               self.percent_complete,
               self.sequence,
               self.uid,
               self.date_timestamp,
               self.created,
               self.last_modified)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use icalendar::parser::{read_calendar as read_icalendar, unfold};

    use crate::webdav::calendar::vtodo::VTodo;

    #[test]
    fn create_todo() {
        let mut file = File::open(format!("data/test/vtodo.ics")).unwrap();
        let mut ics = String::new();
        file.read_to_string(&mut ics).unwrap();

        let unfolded = unfold(&ics);
        let result = read_icalendar(&unfolded);

        match result {
            Ok(icalendar) => {
                let option = VTodo::new(&icalendar.into());

                match option {
                    Some(vtodo) => {
                        let vtodo_expected = VTodo{
                            version: "2.0".to_string(),
                            cal_scale: "".to_string(),
                            prodid: "+//IDN bitfire.at//ical4android (org.dmfs.tasks)".to_string(),
                            summary: "Küche Sockelleisten".to_string(),
                            status: "COMPLETED".to_string(),
                            completed: NaiveDateTime::new(
                                NaiveDate::from_ymd_opt(2023,09,01).unwrap(),
                                NaiveTime::from_hms_opt(11,33,29).unwrap()),
                            percent_complete: 100,
                            sequence: "1".to_string(),
                            uid: "9bb62d0c-0d01-4232-8b36-ac712e948cbf".to_string(),
                            date_timestamp: NaiveDateTime::new(
                                NaiveDate::from_ymd_opt(2023,09,01).unwrap(),
                                NaiveTime::from_hms_opt(11,33,53).unwrap()),
                            created: NaiveDateTime::new(
                                NaiveDate::from_ymd_opt(2023,05,14).unwrap(),
                                NaiveTime::from_hms_opt(20,35,05).unwrap()),
                            last_modified: NaiveDateTime::new(
                                NaiveDate::from_ymd_opt(2023,09,01).unwrap(),
                                NaiveTime::from_hms_opt(11,33,29).unwrap())
                        };
                        assert_eq!(vtodo, vtodo_expected);
                    },
                    None => assert!(false)
                }
            },
            Err(_) => assert!(false)
        }
    }
}