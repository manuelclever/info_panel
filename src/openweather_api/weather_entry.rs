use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct WeatherEntry {
    pub city: String,
    pub timezone: i64,
    pub time_of_forecast: NaiveDateTime,
    pub main: Main,
    pub weather: Weather,
    pub rain: Rain,
    pub clouds: Clouds,
    pub wind: Wind,
    pub visibility: u16,
    pub precipitation_probability: u8,
    pub sys: Sys
}

impl WeatherEntry {
    pub(crate) fn new() -> Self {
        WeatherEntry{
            city: String::from(" "),
            timezone: 0,
            time_of_forecast: Default::default(),
            main: Main::new(),
            weather: Weather::new(),
            rain: Rain::new(),
            clouds: Clouds::new(),
            wind: Wind::new(),
            visibility: 0,
            precipitation_probability: 0,
            sys: Sys::new(),
        }
    }
}

impl PartialEq for WeatherEntry {
    fn eq(&self, other: &Self) -> bool {
        self.time_of_forecast == other.time_of_forecast &&
            self.main == other.main &&
            self.weather == other.weather &&
            self.clouds == other.clouds &&
            self.wind == other.wind &&
            self.visibility == other.visibility &&
            self.precipitation_probability == other.precipitation_probability &&
            self.sys == other.sys
    }
}

#[derive(Debug)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: i32,
    pub pressure_sea_level: i32,
    pub pressure_grnd_level: i32,
    pub humidity: u8
}

impl Main {
    pub(crate) fn new() -> Self {
        Main{
            temp: 0.0,
            feels_like: 0.0,
            temp_min: 0.0,
            temp_max: 0.0,
            pressure: 0,
            pressure_sea_level: 0,
            pressure_grnd_level: 0,
            humidity: 0,
        }
    }
}

impl PartialEq for Main {
    fn eq(&self, other: &Self) -> bool {
        self.temp == other.temp &&
            self.feels_like == other.feels_like &&
            self.temp_min == other.temp_min &&
            self.temp_max == other.temp_max &&
            self.pressure == other.pressure &&
            self.pressure_sea_level == other.pressure_sea_level &&
            self.pressure_grnd_level == other.pressure_grnd_level &&
            self.humidity == other.humidity
    }
}

#[derive(Debug)]
pub struct Weather {
    pub main: String,
    pub description: String,
    pub icon: String
}

impl Weather {
    pub(crate) fn new() -> Self {
        Weather{
            main: "".to_string(),
            description: "".to_string(),
            icon: "".to_string(),
        }
    }
}

impl PartialEq for Weather {
    fn eq(&self, other: &Self) -> bool {
        self.main == other.main &&
            self.description == other.description &&
            self.icon == other.icon
    }
}

#[derive(Debug)]
pub struct Rain {
    pub hour_1: f32,
    pub hour_3: f32
}

impl Rain {
    pub(crate) fn new() -> Self {
        Rain{
            hour_1: 0.0,
            hour_3: 0.0}
    }
}

impl PartialEq for Rain {
    fn eq(&self, other: &Self) -> bool {
        self.hour_1 == other.hour_1 &&
            self.hour_3 == other.hour_3
    }
}

#[derive(Debug)]
pub struct Clouds {
    pub cloudiness: u8
}

impl Clouds {
    pub(crate) fn new() -> Self {
        Clouds{ cloudiness: 0 }
    }
}

impl PartialEq for Clouds {
    fn eq(&self, other: &Self) -> bool {
        self.cloudiness == other.cloudiness
    }
}

#[derive(Debug)]
pub struct Wind {
    pub speed: f32,
    pub direction_deg: i16,
    pub gust: f32
}

impl Wind {
    pub(crate) fn new() -> Self {
        Wind{
            speed: 0.0,
            direction_deg: 0,
            gust: 0.0,
        }
    }
}

impl PartialEq for Wind {
    fn eq(&self, other: &Self) -> bool {
        self.speed == other.speed &&
            self.direction_deg == other.direction_deg &&
            self.gust == other.gust
    }
}

#[derive(Debug)]
pub struct Sys {
    pub part_of_day: char,
    pub country: String,
    pub sunrise: NaiveDateTime,
    pub sunset: NaiveDateTime
}

impl Sys {
    pub(crate) fn new() -> Self {
        Sys{
            part_of_day: ' ',
            country: String::from(" "),
            sunrise: NaiveDateTime::default(),
            sunset: NaiveDateTime::default()
        }
    }
}

impl PartialEq for Sys {
    fn eq(&self, other: &Self) -> bool {
        self.part_of_day == other.part_of_day &&
            self.country == other.country &&
            self.sunrise == other.sunrise &&
            self.sunset == other.sunset
    }
}