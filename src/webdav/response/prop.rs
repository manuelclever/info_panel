#[derive(Debug)]
pub struct Prop {
    pub resourcetype: String,
    pub displayname: String,
    pub calendar_timezone: String,
    pub last_modified: String,
    pub content_length: u32,
    pub e_tag: String,
    pub content_type: String,
}

impl PartialEq for Prop {
    fn eq(&self, other: &Self) -> bool {
        self.resourcetype == other.resourcetype &&
            self.displayname == other.displayname &&
            self.calendar_timezone == other.calendar_timezone &&
            self.last_modified == other.last_modified &&
            self.content_length == other.content_length &&
            self.e_tag == other.e_tag &&
            self.content_type == other.content_type
    }
}