extern crate chrono;

use std::borrow::Cow;

use chrono::{NaiveDate, NaiveDateTime};
use quick_xml::events::{BytesStart, Event as QuickXmlEvent};
use quick_xml::reader::Reader;

use response::prop::Prop;

use crate::webdav::response;
use crate::webdav::calendar::ical_object::ICalObject;


pub fn extract_response_xml (string: &str) -> Vec<Cow<str>> {
    let mut reader = Reader::from_str(string);
    reader.trim_text(true);

    let start_response = BytesStart::new("d:response");
    let end_response   = start_response.to_end().into_owned();

    let mut xml_responses: Vec<Cow<str>> = Vec::new();

    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(QuickXmlEvent::Eof) => break,
            Ok(QuickXmlEvent::Start(e)) => {
                match e.name().as_ref() {
                    name if name == start_response.name().as_ref() => {
                        let inner_xml = reader.read_text(end_response.name()).unwrap();
                        xml_responses.push(inner_xml);
                    },
                    _ => (),
                }
            }
            _ => (),
        }
    }
    xml_responses
}

pub fn extract_href_xml (string: &str) -> Cow<str> {
    let mut reader = Reader::from_str(string);
    reader.trim_text(true);

    let start_response = BytesStart::new("d:href");
    let end_response   = start_response.to_end().into_owned();

    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(QuickXmlEvent::Eof) => break,
            Ok(QuickXmlEvent::Start(e)) => {
                e.name().into_inner();
                match e.name().as_ref() {
                    name if name == start_response.name().as_ref()  => {
                        let inner_xml = reader.read_text(end_response.name()).unwrap();
                        return inner_xml
                    },
                    _ => (),
                }
            }
            _ => (),
        }
    }
    Cow::from("")
}

pub fn extract_propstat_xml (string: &str) -> Cow<str> {
    let mut reader = Reader::from_str(string);
    reader.trim_text(true);

    let start_response = BytesStart::new("d:propstat");
    let end_response   = start_response.to_end().into_owned();

    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(QuickXmlEvent::Eof) => break,
            Ok(QuickXmlEvent::Start(e)) => {
                e.name().into_inner();
                match e.name().as_ref() {
                    name if name == start_response.name().as_ref() => {
                        let inner_xml = reader.read_text(end_response.name()).unwrap();
                        return inner_xml
                    },
                    _ => (),
                }
            }
            _ => (),
        }
    }
    Cow::from("")
}

pub fn parse_prop(string: &str) -> Prop {
    let mut reader = Reader::from_str(string);
    reader.trim_text(true);

    let resourcetype_start = BytesStart::new("d:resourcetype");
    let resourcetype_end = resourcetype_start.to_end().into_owned();

    let displayname_start = BytesStart::new("d:displayname");
    let displayname_end = displayname_start.to_end().into_owned();

    let calendar_timezone_start = BytesStart::new("cal:webdav-timezone");
    let calendar_timezone_end = calendar_timezone_start.to_end().into_owned();

    let last_modified_start = BytesStart::new("d:getlastmodified");
    let last_modified_end = last_modified_start.to_end().into_owned();

    let content_length_start = BytesStart::new("d:getcontentlength");
    let content_length_end = content_length_start.to_end().into_owned();

    let e_tag_start = BytesStart::new("d:getetag");
    let e_tag_end = e_tag_start.to_end().into_owned();

    let content_type_start = BytesStart::new("d:getcontenttype");
    let content_type_end = content_type_start.to_end().into_owned();

    let mut prop: Prop = Prop {
        resourcetype: String::from(""),
        displayname: String::from(""),
        calendar_timezone: String::from(""),
        last_modified: String::from(""),
        content_length: 0,
        e_tag: String::from(""),
        content_type: String::from(""),
    };

    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(QuickXmlEvent::Eof) => break,
            Ok(QuickXmlEvent::Start(e)) => {
                e.name().into_inner();
                match e.name().as_ref() {
                    name if name == resourcetype_start.name().as_ref() => {
                        let inner_xml = reader.read_text(resourcetype_end.name()).unwrap();
                        prop.resourcetype = inner_xml.into_owned();
                    },
                    name if name == displayname_start.name().as_ref() => {
                        let inner_xml = reader.read_text(displayname_end.name()).unwrap();
                        prop.displayname = inner_xml.into_owned();
                    },
                    name if name == calendar_timezone_start.name().as_ref() => {
                        let inner_xml = reader.read_text(calendar_timezone_end.name()).unwrap();
                        prop.calendar_timezone = inner_xml.into_owned();
                    },
                    name if name == last_modified_start.name().as_ref() => {
                        let inner_xml = reader.read_text(last_modified_end.name()).unwrap();
                        prop.last_modified = inner_xml.into_owned();
                    },
                    name if name == content_length_start.name().as_ref() => {
                        let inner_xml = reader.read_text(content_length_end.name()).unwrap();
                        prop.content_length = inner_xml.as_ref().parse().unwrap();
                    },
                    name if name == e_tag_start.name().as_ref() => {
                        let inner_xml = reader.read_text(e_tag_end.name()).unwrap();
                        prop.e_tag = inner_xml.into_owned();
                    },
                    name if name == content_type_start.name().as_ref() => {
                        let inner_xml = reader.read_text(content_type_end.name()).unwrap();
                        prop.content_type = inner_xml.into_owned();
                    },
                    _ => (),
                }
            }
            _ => (),
        }
    }
    prop
}

pub fn parse_ical_object(string: &str) -> ICalObject {

    let mut event: ICalObject = ICalObject::new();

    let mut is_all_day: bool = false;
    let mut date_start: &str = "";
    let mut date_end: &str = "";

    for line in string.lines() {
        let pair: Vec<&str> = line.splitn(2, |c| c == ':' || c == ';').collect();

        if pair.len() >= 2 {
            let key: &str = pair.get(0).unwrap();
            let value: &str = pair.get(1).unwrap();

            match key {
                "CALSCALE" => event.cal_scale = value.to_string(),
                "UID" => event.uid = value.to_string(),
                "DTSTART" => date_start = value,
                "DTEND" => date_end = value,
                "X-FUNAMBOL-ALLDAY" => {
                    is_all_day = value == "1";
                    event.is_all_day = is_all_day},
                "SUMMARY" => event.summary = value.to_string(),
                "DESCRIPTION" => event.desc = value.to_string(),
                "DTSTAMP" => event.date_timestamp = parse_date_time(value),
                _ => {}
            }
        }
    }

    if is_all_day {
        event.date_start = parse_date(date_start).and_hms_opt(0, 0, 0).unwrap();
        event.date_end = parse_date(date_end).and_hms_opt(0, 0, 0).unwrap();
    } else {
        event.date_start = parse_date_time(date_start);
        event.date_end = parse_date_time(date_end);
    }

    event
}

fn parse_date(string: &str) -> NaiveDate {
    if string != "" {
        let date: &str = string.split(":").collect::<Vec<&str>>().get(1).unwrap();

        return match NaiveDate::parse_from_str(date, "%Y%m%d") {
            Ok(date) => {
                date
            }
            Err(_) => {
                NaiveDate::default()
            }
        }
    }
    NaiveDate::default()
}

fn parse_date_time(string: &str) -> NaiveDateTime {
    match NaiveDateTime::parse_from_str(string, "%Y%m%dT%H%M%S%.fZ") {
        Ok(date) => {
            date
        }
        Err(_) => {
            NaiveDateTime::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::fs::File;
    use std::io::Read;

    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use crate::webdav::calendar::ical_object::ICalObject;
    use crate::webdav::parsing;
    use crate::webdav::parsing::{parse_ical_object, parse_prop};
    use crate::webdav::response::prop::Prop;

    fn get_xml(name: &str) -> String {
        let mut file = File::open(format!("data/test/{name}")).unwrap();

        let mut xml = String::new();
        file.read_to_string(&mut xml).unwrap();
        xml
    }

    #[test]
    fn response_extraction() {
        let xml = get_xml("calendar.xml");

        let xml_responses: Vec<Cow<str>> = parsing::extract_response_xml(&xml);
        let mut expected = Vec::new();
        expected.push(Cow::from("
        <d:href>/nextcloud/remote.php/dav/calendars/user/abfall/</d:href>
        <d:propstat>
            <d:prop>
                <d:resourcetype>
                    <d:collection/>
                    <cal:calendar/>
                </d:resourcetype>
                <cs:getctag>http://sabre.io/ns/sync/32</cs:getctag>
                <s:sync-token>32</s:sync-token>
                <cal:supported-calendar-component-set>
                    <cal:comp name=\"VEVENT\"/>
                </cal:supported-calendar-component-set>
                <cal:schedule-calendar-transp>
                    <cal:opaque/>
                </cal:schedule-calendar-transp>
                <oc:owner-principal>principals/users/user</oc:owner-principal>
                <d:displayname>Abfall</d:displayname>
                <cal:calendar-timezone>BEGIN:VCALENDAR
                    PRODID:-//IDN nextcloud.com//Calendar app 3.4.2//EN
                    CALSCALE:GREGORIAN
                    VERSION:2.0
                    BEGIN:VTIMEZONE
                    TZID:Europe/Berlin
                    BEGIN:DAYLIGHT
                    TZOFFSETFROM:+0100
                    TZOFFSETTO:+0200
                    TZNAME:CEST
                    DTSTART:19700329T020000
                    RRULE:FREQ=YEARLY;BYMONTH=3;BYDAY=-1SU
                    END:DAYLIGHT
                    BEGIN:STANDARD
                    TZOFFSETFROM:+0200
                    TZOFFSETTO:+0100
                    TZNAME:CET
                    DTSTART:19701025T030000
                    RRULE:FREQ=YEARLY;BYMONTH=10;BYDAY=-1SU
                    END:STANDARD
                    END:VTIMEZONE
                    END:VCALENDAR</cal:calendar-timezone>
                <x1:calendar-order
                        xmlns:x1=\"http://apple.com/ns/ical/\">0
                </x1:calendar-order>
                <x1:calendar-color
                        xmlns:x1=\"http://apple.com/ns/ical/\">#D09E6D
                </x1:calendar-color>
                <x2:owner-displayname
                        xmlns:x2=\"http://nextcloud.com/ns\">User
                </x2:owner-displayname>
            </d:prop>
            <d:status>HTTP/1.1 200 OK</d:status>
        </d:propstat>"));
        expected.push(Cow::from("
        <d:href>/nextcloud/remote.php/dav/calendars/user/abfall/D9F0AFEB-6B0A-434A-99B8-EE64C8E27526.ics</d:href>
        <d:propstat>
            <d:prop>
                <d:getlastmodified>Mon, 22 Aug 2022 18:10:09 GMT</d:getlastmodified>
                <d:getcontentlength>465</d:getcontentlength>
                <d:resourcetype/>
                <d:getetag>&quot;a86c24c6146b1965dff7da97f2e433cf&quot;</d:getetag>
                <d:getcontenttype>text/calendar; charset=utf-8; component=vevent</d:getcontenttype>
            </d:prop>
            <d:status>HTTP/1.1 200 OK</d:status>
        </d:propstat>"));
        expected.push(Cow::from("
    <d:href>/nextcloud/remote.php/dav/calendars/user/abfall/BFB6E10D-1C74-4B62-A566-1F75F8BD0893.ics</d:href>
    <d:propstat>
        <d:prop>
            <d:getlastmodified>Mon, 22 Aug 2022 18:10:09 GMT</d:getlastmodified>
            <d:getcontentlength>513</d:getcontentlength>
            <d:resourcetype/>
            <d:getetag>&quot;742dd33eb021a68e783e046bc77a6f97&quot;</d:getetag>
            <d:getcontenttype>text/calendar; charset=utf-8; component=vevent</d:getcontenttype>
        </d:prop>
        <d:status>HTTP/1.1 200 OK</d:status>
    </d:propstat>"));

        assert_eq!(xml_responses, expected);
    }

    #[test]
    fn href_extraction() {
        let xml = get_xml("response.xml");

        let href = parsing::extract_href_xml(xml.as_ref());
        let expected: Cow<str> = Cow::from("/nextcloud/remote.php/dav/calendars/user/abfall/D9F0AFEB-6B0A-434A-99B8-EE64C8E27526.ics");

        assert_eq!(expected, href);
    }

    #[test]
    fn propstat_extraction() {
        let xml = get_xml("response.xml");

        let propstat = parsing::extract_propstat_xml(xml.as_ref());
        let expected: Cow<str> = Cow::from("
        <d:prop>
            <d:getlastmodified>Mon, 22 Aug 2022 18:10:09 GMT</d:getlastmodified>
            <d:getcontentlength>465</d:getcontentlength>
            <d:resourcetype/>
            <d:getetag>&quot;a86c24c6146b1965dff7da97f2e433cf&quot;</d:getetag>
            <d:getcontenttype>text/calendar; charset=utf-8; component=vevent</d:getcontenttype>
        </d:prop>
        <d:status>HTTP/1.1 200 OK</d:status>");

        assert_eq!(expected, propstat);
    }

    #[test]
    fn prop_parsing() {
        let prop_string = get_xml("prop.xml");

        let prop: Prop = parse_prop(&prop_string);

        let expected_prop: Prop = Prop {
                    resourcetype: String::from(""),
                    displayname: String::from(""),
                    calendar_timezone: String::from(""),
                    last_modified: "Mon, 22 Aug 2022 18:10:09 GMT".parse().unwrap(),
                    content_length: 465,
                    e_tag: "&quot;a86c24c6146b1965dff7da97f2e433cf&quot;".parse().unwrap(),
                    content_type: "text/calendar; charset=utf-8; component=vevent".parse().unwrap() };

        assert_eq!(prop, expected_prop);
    }

    #[test]
    fn event_parsing() {
        let event: ICalObject = parse_ical_object(
            "BEGIN:VCALENDAR
VERSION:2.0
CALSCALE:GREGORIAN
PRODID:-//hacksw/handcal//NONSGML v1.0//EN
BEGIN:VEVENT
UID:ab2abbc6854d1c58daa84520d6198074@eaw-rtk.de
DTSTART;VALUE=DATE:20221102
DTEND;VALUE=DATE:20221103
X-FUNAMBOL-ALLDAY:1
X-MICROSOFT-CDO-ALLDAYEVENT:TRUE
TRANSP:TRANSPARENT
SUMMARY:Tonnentausch
DESCRIPTION:
URL;VALUE=URI:https://www.eaw-rtk.de/abfallkalender/?cityid=2&districtid=9&
 streetid=191
DTSTAMP:20220822T181011Z
END:VEVENT
END:VCALENDAR");

        let expectet_event = ICalObject {
            cal_scale:"GREGORIAN".to_string(),
            uid: "ab2abbc6854d1c58daa84520d6198074@eaw-rtk.de".to_string(),
            date_start: NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2022, 11, 2).unwrap(),
                NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            date_end: NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2022, 11, 3).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            is_all_day: true,
            summary: "Tonnentausch".to_string(),
            desc: "".to_string(),
            date_timestamp: NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2022, 8, 22).unwrap(),
                NaiveTime::from_hms_opt(18, 10, 11).unwrap())};

        assert_eq!(event, expectet_event);

    }
}