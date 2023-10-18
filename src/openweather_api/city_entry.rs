use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct CityEntry {
    pub sunrise: NaiveDateTime,
    pub sunset: NaiveDateTime
}

impl CityEntry {
    pub(crate) fn new() -> Self {
        CityEntry{
            sunrise: NaiveDateTime::default(),
            sunset: NaiveDateTime::default()
        }
    }
}

impl PartialEq for CityEntry {
    fn eq(&self, other: &Self) -> bool {
        self.sunrise == other.sunrise &&
            self.sunset == other.sunset
    }
}