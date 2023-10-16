use chrono::NaiveTime;

#[derive(Debug)]
pub struct CityEntry {
    pub sunrise: NaiveTime,
    pub sunset: NaiveTime
}

impl CityEntry {
    pub(crate) fn new() -> Self {
        CityEntry{
            sunrise: NaiveTime::default(),
            sunset: NaiveTime::default()
        }
    }
}

impl PartialEq for CityEntry {
    fn eq(&self, other: &Self) -> bool {
        self.sunrise == other.sunrise &&
            self.sunset == other.sunset
    }
}