use crate::webdav::response::prop::Prop;

pub mod prop;

#[derive(Debug)]
pub struct Response {
    pub href: String,
    pub ical_file: String,
    pub prop: Prop,

}

impl Response {
    pub(crate) fn new(href: &str, prop: Prop) -> Self {
        let vec: Vec<&str> = href.split("/").collect();

        Response{
            href: href.to_string(),
            ical_file: vec.get(vec.len()-1).unwrap().to_string(),
            prop,
        }

    }
}

impl PartialEq for Response {
    fn eq(&self, other: &Self) -> bool {
        self.href == other.href && self.prop == other.prop
    }
}
