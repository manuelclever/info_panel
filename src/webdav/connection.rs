use std::borrow::Cow;
use std::fs::File;
use std::io::{BufRead, BufReader};

use reqwest::Error;
use rustydav::client::Client;

use crate::webdav::parsing;
use crate::webdav::response::Response;

#[derive(Debug)]
pub struct Connection {
    pub url: String,
    pub webdav_client: Client
}

impl<'a> Connection {

    pub(crate) fn new(path_config: &str) -> Result<Self, String> {
        let mut url = String::new();
        let mut user = String::new();
        let mut pw = String::new();

        let file = File::open(path_config).map_err(|e| {
            format!("Failed to open '{}': {}", path_config, e)
        })?;

        //read config
        let reader = BufReader::new(file);
        for line_result in reader.lines() {
            let line = line_result.map_err(|e| {
                format!("Error reading '{}': {}", path_config, e)
            })?;

            let split: Vec<&str> = line.split('=').collect();
            if let [key, value] = split.as_slice() {
                match *key {
                    "url" => url = value.to_string(),
                    "user" => user = value.to_string(),
                    "password" => pw = value.to_string(),
                    _ => {}
                }
            }
        }

        //return self
        if url.is_empty() | user.is_empty() | pw.is_empty() {
            Err(format!("Error parsing content of '{}'", path_config))
        } else {
            Ok(Self { url, webdav_client: Client::init(user.as_str(), pw.as_str())})
        }
    }

    pub(crate) fn get_ics_file(&self, path: &str) -> Result<String, Error> {
        let url = &format!("{}{}", self.url, path);
        println!("url: {}",url);

        match self.webdav_client.get(url) {
            Ok(value) => {
                match value.text() {
                    Ok(value) => Ok(value),
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }

    pub(crate) fn get_responses(&self, name: &str) -> Result<Vec<Response>,String> {
        let xml: String = self.get_xml_of_calendar(name).map_err(|e|{
            format!("Error extracting xml: {}", e)
        })?;
        Ok(self.parse_responses(&xml))
    }

    fn get_xml_of_calendar(&self, name: &str) -> Result<String, String> {
        let url_calendar_name = &format!("{}{}", self.url, name);

        println!("Listing: {}", url_calendar_name);
        let result = self.webdav_client.list(url_calendar_name, "infinity");

        let response = result.map_err(|e| {
            format!("Failed to download '{}/{}': {}", self.url, name , e)
        })?;

        match response.text() {
            Ok(xml) => Ok(xml),
            Err(e) => Err(e.to_string())
        }
    }

    fn parse_responses(&self, xml: &String) -> Vec<Response> {
        let xml_responses: Vec<Cow<str>> = parsing::extract_response_xml(xml);
        let mut responses: Vec<Response> = Vec::new();

        for xml_response in xml_responses {
            let href = parsing::extract_href_xml(xml_response.as_ref());
            let propstat = parsing::extract_propstat_xml(xml_response.as_ref());
            let prop = parsing::parse_prop(propstat.as_ref());
            let response: Response = Response::new(href.as_ref(), prop);
            responses.push(response);
        }
        responses
    }



}

impl<'a> PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

#[cfg(test)]
mod tests {
    use rustydav::client::Client;

    use crate::webdav::connection::Connection;

    #[test]
    fn new() {
        let calendar = Connection::new("data/test/default_test.config");

        assert_eq!(calendar.unwrap(), Connection {
            url: String::from("https://diesisteintest.de/webdavoderso"),
            webdav_client: Client::init("user", "geheim")
        });
    }
}