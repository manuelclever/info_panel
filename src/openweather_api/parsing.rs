use std::ops::Add;

use chrono::{Duration, Local, NaiveDateTime};
use json::JsonValue;
use log::debug;

use crate::openweather_api::weather_entry::{Clouds, Main, Rain, Sys, Weather, WeatherEntry, Wind};

pub(crate) fn parse_json_current(json_string: &str) -> Option<WeatherEntry> {
    return match json::parse(&json_string) {
        Ok(weather_entry_obj) => Some(convert_to_weather_entry(&weather_entry_obj)),
        Err(_) => None
    }
}

pub(crate) fn parse_json_forecast(json_string: &str) -> Option<Vec<WeatherEntry>> {
    let weather_obj = json::parse(&json_string);

    let mut weather_entries: Vec<WeatherEntry> = Vec::new();

    match &weather_obj {
        Ok(val) => {
            let list = &val["list"];

            debug!("parsing json: {}", list.to_string());

            for member in list.members() {
                let entry: WeatherEntry = convert_to_weather_entry(member);
                weather_entries.push(entry);
            }
        },
        Err(_) => return None
    }
    Some(weather_entries)
}

fn convert_to_weather_entry(json: &JsonValue) -> WeatherEntry {
    let mut weather_entry: WeatherEntry = WeatherEntry::new();

    for entry in json.entries() {
        match entry.0 {
            "name" => weather_entry.city = entry.1.to_string(),
            "timezone" => weather_entry.timezone = entry.1.to_string().parse().unwrap_or_default(),
            "dt" => weather_entry.time_of_forecast = NaiveDateTime::from_timestamp_opt(entry.1.to_string().parse().unwrap_or_default(),0).unwrap_or_default(),
            "main" => weather_entry.main = convert_to_main(entry.1),
            "weather" => weather_entry.weather = convert_to_weather(entry.1),
            "rain" => weather_entry.rain = convert_to_rain(entry.1),
            "clouds" => weather_entry.clouds = convert_to_clouds(entry.1),
            "wind" => weather_entry.wind = convert_to_wind(entry.1),
            "visibility" => weather_entry.visibility = entry.1.to_string().parse().unwrap_or_default(),
            "pop" => weather_entry.precipitation_probability = entry.1.to_string().parse().unwrap_or_default(),
            "sys" => weather_entry.sys = convert_to_sys(entry.1),
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

fn convert_to_rain(json: &JsonValue) -> Rain {
    let mut rain: Rain = Rain::new();

    for entry in json.entries() {
        match entry.0 {
            "1h" => rain.hour_1 = entry.1.to_string().parse().unwrap_or_default(),
            "3h" => rain.hour_3 = entry.1.to_string().parse().unwrap_or_default(),
            _default => ()
        }
    }
    rain
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

fn convert_to_sys(json: &JsonValue) -> Sys {
    let mut sys: Sys = Sys::new();

    for entry in json.entries() {
        match entry.0 {
            "pod" => sys.part_of_day = entry.1.to_string().chars().next().unwrap_or_default(),
            "country" => sys.country = entry.1.to_string(),
            "sunrise" => sys.sunrise = NaiveDateTime::from_timestamp_opt(entry.1.to_string().parse().unwrap_or_default(),0).unwrap_or_default(),
            "sunset" => sys.sunset = NaiveDateTime::from_timestamp_opt(entry.1.to_string().parse().unwrap_or_default(),0).unwrap_or_default(),
            _default => ()
        }
    }
    sys
}

pub(crate) fn utc_to_local_date_time(naiveDateTime: NaiveDateTime) -> NaiveDateTime {
    let offset: i64 = Local::now().offset().local_minus_utc().into();

    naiveDateTime.add(Duration::seconds(offset))
}

#[cfg(test)]
mod test {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use crate::openweather_api::parsing::{parse_json_current, parse_json_forecast};
    use crate::openweather_api::weather_entry::{Clouds, Main, Rain, Sys, Weather, WeatherEntry, Wind};

    #[test]
    fn json_parsing_current() {
        let input = r#"{"coord":{"lon":7.8582,"lat":50.1991},"weather":[{"id":500,"main":"Rain","description":"Leichter Regen","icon":"10n"}],"base":"stations","main":{"temp":12.06,"feels_like":11.23,"temp_min":10.96,"temp_max":12.59,"pressure":1001,"humidity":73,"sea_level":1001,"grnd_level":971},"visibility":10000,"wind":{"speed":3.17,"deg":124,"gust":9.39},"rain":{"1h":0.16},"clouds":{"all":100},"dt":1697656291,"sys":{"type":2,"id":2016993,"country":"DE","sunrise":1697608475,"sunset":1697646761},"timezone":7200,"id":2866990,"name":"Nastätten","cod":200}"#;

        let parsed = parse_json_current(&input);
        match parsed {
            Some(output) => {
                let output_expected: WeatherEntry =
                    WeatherEntry{
                        city: "Nastätten".to_string(),
                        timezone: 7200,
                        time_of_forecast: NaiveDateTime::new(
                            NaiveDate::from_ymd_opt(2023,10,18).unwrap(),
                            NaiveTime::from_hms_opt(19,11,31).unwrap()),
                        main: Main {
                            temp: 12.06,
                            feels_like: 11.23,
                            temp_min: 10.96,
                            temp_max: 12.59,
                            pressure: 1001,
                            pressure_sea_level: 1001,
                            pressure_grnd_level: 971,
                            humidity: 73,
                        },
                        weather: Weather {
                            main: "Rain".to_string(),
                            description: "Leichter Regen".to_string(),
                            icon: "10n".to_string(),
                        },
                        rain: Rain {
                            hour_1: 0.16,
                            hour_3: 0.0 },
                        clouds: Clouds { cloudiness: 100 },
                        wind: Wind {
                            speed: 3.17,
                            direction_deg: 124,
                            gust: 9.39,
                        },
                        visibility: 10000,
                        precipitation_probability: 0,
                        sys: Sys {
                            part_of_day: ' ',
                            country: "DE".to_string(),
                            sunrise: NaiveDateTime::new(
                                NaiveDate::from_ymd_opt(2023,10,18).unwrap(),
                                NaiveTime::from_hms_opt(05,54,35).unwrap()),
                            sunset: NaiveDateTime::new(
                                NaiveDate::from_ymd_opt(2023,10,18).unwrap(),
                                NaiveTime::from_hms_opt(16,32,41).unwrap()),
                        },
                    };
                assert_eq!(output,output_expected);
            },
            None => assert!(false)
        }

    }

    #[test]
    fn json_parsing_forecast() {
        let input = r#"{"cod":"200","message":0,"cnt":40,"list":[{"dt":1696496400,"main":{"temp":12.71,"feels_like":12.31,"temp_min":12.71,"temp_max":14.26,"pressure":1026,"sea_level":1026,"grnd_level":995,"humidity":87,"temp_kf":-1.55},"weather":[{"id":804,"main":"Clouds","description":"overcast clouds","icon":"04d"}],"clouds":{"all":100},"wind":{"speed":3.33,"deg":258,"gust":6.51},"rain":{"3h":0.23},"visibility":10000,"pop":40,"sys":{"pod":"d"},"dt_txt":"2023-10-05 09:00:00"},{"dt":1696507200,"main":{"temp":13.77,"feels_like":13.21,"temp_min":13.77,"temp_max":15.89,"pressure":1026,"sea_level":1026,"grnd_level":995,"humidity":77,"temp_kf":-2.12},"weather":[{"id":804,"main":"Clouds","description":"overcast clouds","icon":"04d"}],"clouds":{"all":100},"wind":{"speed":3.38,"deg":278,"gust":5.34},"visibility":10000,"pop":0,"sys":{"pod":"d"},"dt_txt":"2023-10-05 12:00:00"}],"city":{"id":2866990,"name":"Nastätten","coord":{"lat":50.1991,"lon":7.8582},"country":"DE","population":4310,"timezone":7200,"sunrise":1697435481,"sunset":1697474205}}"#;

        let parsed = parse_json_forecast(&input);
        match parsed {
            Some(output) => {
                let output_expected: Vec<WeatherEntry> =
                    vec![
                        WeatherEntry{
                            city: " ".to_string(),
                            timezone: 0,
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
                            rain: Rain {
                                hour_1: 0.0,
                                hour_3: 0.4 },
                            clouds: Clouds { cloudiness: 100 },
                            wind: Wind {
                                speed: 3.33,
                                direction_deg: 258,
                                gust: 6.51,
                            },
                            visibility: 10000,
                            precipitation_probability: 40,
                            sys: Sys {
                                part_of_day: 'd',
                                country: " ".to_string(),
                                sunrise: Default::default(),
                                sunset: Default::default(),
                            },
                        },
                        WeatherEntry{
                            city: " ".to_string(),
                            timezone: 0,
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
                            rain: Rain {
                                hour_1: 0.0,
                                hour_3: 0.0 },
                            clouds: Clouds { cloudiness: 100 },
                            wind: Wind {
                                speed: 3.38,
                                direction_deg: 278,
                                gust: 5.34,
                            },
                            visibility: 10000,
                            precipitation_probability: 0,
                            sys: Sys {
                                part_of_day: 'd',
                                country: " ".to_string(),
                                sunrise: Default::default(),
                                sunset: Default::default(),
                            },
                        }
                    ];
                assert_eq!(output,output_expected);
            },
            None => assert!(false)
        }

    }
}