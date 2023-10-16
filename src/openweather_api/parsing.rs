use chrono::NaiveDateTime;
use json::JsonValue;

use crate::openweather_api::city_entry::CityEntry;
use crate::openweather_api::weather_entry::{Clouds, Main, Weather, WeatherEntry, Wind};

#[derive(Debug)]
pub(crate) enum WeatherOrCity {
    Weather(Vec<WeatherEntry>),
    City(CityEntry),
}

impl PartialEq for WeatherOrCity {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (WeatherOrCity::Weather(weather1), WeatherOrCity::Weather(weather2)) => {
                weather1 == weather2
            }
            (WeatherOrCity::City(city1), WeatherOrCity::City(city2)) => {
                city1 == city2
            }
            _ => false,
        }
    }
}

pub(crate) fn parse_json_open_weather(json_string: &str) -> Option<Vec<WeatherOrCity>> {
    let mut vec: Vec<WeatherOrCity> = Vec::new();

    let weather_obj = json::parse(&json_string);

    let mut weather_entries: Vec<WeatherEntry> = Vec::new();
    let mut city_entry: CityEntry = CityEntry::new();

    match &weather_obj {
        Ok(val) => {
            let list = &val["list"];

            println!("{}", list.to_string());

            for member in list.members() {
                let entry: WeatherEntry = convert_to_weather_entry(member);

                weather_entries.push(entry);
            }
            vec.push(WeatherOrCity::Weather(weather_entries));

            let city = &val["city"];
            city_entry = convert_to_city_entry(city);

            vec.push(WeatherOrCity::City(city_entry));
        },
        Err(_) => return None
    }
    Some(vec)
}

fn convert_to_weather_entry(json: &JsonValue) -> WeatherEntry {
    let mut weather_entry: WeatherEntry = WeatherEntry::new();

    for entry in json.entries() {
        match entry.0 {
            "dt" => weather_entry.time_of_forecast = NaiveDateTime::from_timestamp_opt(entry.1.to_string().parse().unwrap_or_default(),0).unwrap_or_default(),
            "main" => weather_entry.main = convert_to_main(entry.1),
            "weather" => weather_entry.weather = convert_to_weather(entry.1),
            "clouds" => weather_entry.clouds = convert_to_clouds(entry.1),
            "wind" => weather_entry.wind = convert_to_wind(entry.1),
            "visibility" => weather_entry.visibility = entry.1.to_string().parse().unwrap_or_default(),
            "pop" => weather_entry.precipitation_probability = entry.1.to_string().parse().unwrap_or_default(),
            "sys" => weather_entry.part_of_day = convert_to_pod(entry.1),
            _default => ()
        }
    }
    weather_entry
}

fn convert_to_main(json: &JsonValue) -> Main {
    let mut main: Main = Main::new();

    for entry in json.entries() {
        match entry.0 {
            "temp" => main.temp = entry.1.to_string().parse().unwrap_or_default(),
            "feels_like" => main.feels_like = entry.1.to_string().parse().unwrap_or_default(),
            "temp_min" => main.temp_min = entry.1.to_string().parse().unwrap_or_default(),
            "temp_max" => main.temp_max = entry.1.to_string().parse().unwrap_or_default(),
            "pressure" => main.pressure = entry.1.to_string().parse().unwrap_or_default(),
            "sea_level" => main.pressure_sea_level = entry.1.to_string().parse().unwrap_or_default(),
            "grnd_level" => main.pressure_grnd_level = entry.1.to_string().parse().unwrap_or_default(),
            "humidity" => main.humidity = entry.1.to_string().parse().unwrap_or_default(),
            _default => ()
        }
    }
    main
}

fn convert_to_weather(json: &JsonValue) -> Weather {
    let mut weather: Weather = Weather::new();

    match json.members().next() {
        Some(jsonValue) => {

            for entry in jsonValue.entries() {
                match entry.0 {
                    "main" => weather.main = entry.1.to_string(),
                    "description" => weather.description = entry.1.to_string(),
                    "icon" => weather.icon = entry.1.to_string(),
                    _default => ()
                }
            }
        },
        None => (),
    }
    weather
}

fn convert_to_clouds(json: &JsonValue) -> Clouds {
    let mut clouds: Clouds = Clouds::new();

    for entry in json.entries() {
        match entry.0 {
            "all" => clouds.cloudiness = entry.1.to_string().parse().unwrap_or_default(),
            _default => ()
        }
    }
    clouds
}

fn convert_to_wind(json: &JsonValue) -> Wind {
    let mut wind: Wind = Wind::new();

    for entry in json.entries() {
        match entry.0 {
            "speed" => wind.speed = entry.1.to_string().parse().unwrap_or_default(),
            "deg" => wind.direction_deg = entry.1.to_string().parse().unwrap_or_default(),
            "gust" => wind.gust = entry.1.to_string().parse().unwrap_or_default(),
            _default => ()
        }
    }
    wind
}

fn convert_to_pod(json: &JsonValue) -> char {
    for entry in json.entries() {
        match entry.0 {
            "pod" => return entry.1.to_string().chars().next().unwrap_or_default(),
            _default => ()
        }
    }
    ' '
}

fn convert_to_city_entry(json: &JsonValue) -> CityEntry {
    let mut city_entry: CityEntry = CityEntry::new();

    for entry in json.entries() {
        match entry.0 {
            "sunrise" => city_entry.sunrise = NaiveDateTime::from_timestamp_opt(entry.1.to_string().parse().unwrap_or_default(),0).unwrap_or_default().time(),
            "sunset" => city_entry.sunset = NaiveDateTime::from_timestamp_opt(entry.1.to_string().parse().unwrap_or_default(),0).unwrap_or_default().time(),
            _default => ()
        }
    }

    city_entry
}

#[cfg(test)]
mod test {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use crate::openweather_api::city_entry::CityEntry;
    use crate::openweather_api::parsing::{parse_json_open_weather, WeatherOrCity};
    use crate::openweather_api::weather_entry::{Clouds, Main, Weather, WeatherEntry, Wind};

    #[test]
    fn json_parsing() {
        let input = r#"{"cod":"200","message":0,"cnt":40,"list":[{"dt":1696496400,"main":{"temp":12.71,"feels_like":12.31,"temp_min":12.71,"temp_max":14.26,"pressure":1026,"sea_level":1026,"grnd_level":995,"humidity":87,"temp_kf":-1.55},"weather":[{"id":804,"main":"Clouds","description":"overcast clouds","icon":"04d"}],"clouds":{"all":100},"wind":{"speed":3.33,"deg":258,"gust":6.51},"visibility":10000,"pop":0,"sys":{"pod":"d"},"dt_txt":"2023-10-05 09:00:00"},{"dt":1696507200,"main":{"temp":13.77,"feels_like":13.21,"temp_min":13.77,"temp_max":15.89,"pressure":1026,"sea_level":1026,"grnd_level":995,"humidity":77,"temp_kf":-2.12},"weather":[{"id":804,"main":"Clouds","description":"overcast clouds","icon":"04d"}],"clouds":{"all":100},"wind":{"speed":3.38,"deg":278,"gust":5.34},"visibility":10000,"pop":0,"sys":{"pod":"d"},"dt_txt":"2023-10-05 12:00:00"}],"city":{"id":2866990,"name":"Nastätten","coord":{"lat":50.1991,"lon":7.8582},"country":"DE","population":4310,"timezone":7200,"sunrise":1697435481,"sunset":1697474205}}"#;

        let parsed = parse_json_open_weather(&input);
        match parsed {
            Some(output) => {
                let output_expected: Vec<WeatherOrCity> =
                    vec![
                        WeatherOrCity::Weather(vec![
                            WeatherEntry{
                                time_of_forecast: NaiveDateTime::new(
                                    NaiveDate::from_ymd_opt(2023,10,05).unwrap(),
                                    NaiveTime::from_hms_opt(09,00,00).unwrap()),
                                main: Main {
                                    temp: 12.71,
                                    feels_like: 12.31,
                                    temp_min: 12.71,
                                    temp_max: 14.26,
                                    pressure: 1026,
                                    pressure_sea_level: 1026,
                                    pressure_grnd_level: 995,
                                    humidity: 87,
                                },
                                weather: Weather {
                                    main: "Clouds".to_string(),
                                    description: "overcast clouds".to_string(),
                                    icon: "04d".to_string(),
                                },
                                clouds: Clouds { cloudiness: 100 },
                                wind: Wind {
                                    speed: 3.33,
                                    direction_deg: 258,
                                    gust: 6.51,
                                },
                                visibility: 10000,
                                precipitation_probability: 0,
                                part_of_day: 'd',
                            },
                            WeatherEntry{
                                time_of_forecast: NaiveDateTime::new(
                                    NaiveDate::from_ymd_opt(2023,10,05).unwrap(),
                                    NaiveTime::from_hms_opt(12,00,00).unwrap()),
                                main: Main {
                                    temp: 13.77,
                                    feels_like: 13.21,
                                    temp_min: 13.77,
                                    temp_max: 15.89,
                                    pressure: 1026,
                                    pressure_sea_level: 1026,
                                    pressure_grnd_level: 995,
                                    humidity: 77,
                                },
                                weather: Weather {
                                    main: "Clouds".to_string(),
                                    description: "overcast clouds".to_string(),
                                    icon: "04d".to_string(),
                                },
                                clouds: Clouds { cloudiness: 100 },
                                wind: Wind {
                                    speed: 3.38,
                                    direction_deg: 278,
                                    gust: 5.34,
                                },
                                visibility: 10000,
                                precipitation_probability: 0,
                                part_of_day: 'd',
                            }
                        ]),
                        WeatherOrCity::City(CityEntry{
                            sunrise: NaiveTime::from_hms_opt(05,51,21).unwrap(),
                            sunset: NaiveTime::from_hms_opt(16,36,45).unwrap() })
                    ];
                assert_eq!(output,output_expected);
            },
            None => assert!(false)
        }

    }
}