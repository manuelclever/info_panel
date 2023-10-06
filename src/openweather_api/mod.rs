use std::fs::File;
use std::io::{BufRead, BufReader};

use reqwest::Client;

mod weather_entry;
pub(crate) mod parsing;

pub struct OpenWeatherClient {
    api_key: String,
    pub url: String,
    pub lat: f64,
    pub lon: f64,
    pub units: String

}

impl OpenWeatherClient {
    pub(crate) fn new(path_config: &str) -> Result<Self,String> {
        let mut api_key: String = String::from("");
        let mut url: String = String::from("");
        let mut lat: f64 = 0f64;
        let mut lon: f64 = 0f64;
        let mut units: String = String::from("");


        let file = File::open(path_config).map_err(|e| {
            format!("Failed to open '{}': {}", path_config, e)
        })?;

        //read config
        let reader = BufReader::new(file);
        for line_result in reader.lines() {
            let line: String = line_result.map_err(|e| {
                format!("Error reading '{}': {}", path_config, e)
            })?;

            if let Some(first_char) = line.chars().next() {
                if first_char != '#' {
                    let split: Vec<&str> = line.split('=').collect();
                    if let [key, value] = split.as_slice() {
                        match *key {
                            "url" => url = value.to_string(),
                            "units" => units = value.to_string(),
                            "key" => api_key = value.to_string(),
                            "lat" => lat = value.to_string().parse::<f64>().map_err(|e| e.to_string())?,
                            "lon" => lon = value.to_string().parse::<f64>().map_err(|e| e.to_string())?,
                            _ => {}
                        }
                    }
                }
            }
        }

        Ok(OpenWeatherClient {
            api_key,
            url,
            lat,
            lon,
            units
        })
    }

    pub(crate) async fn make_request_3h_5d(&self) -> String {
        let geocoding_url = format!("{}?lat={}&lon={}&units={}&appid={}",
                                    self.url, self.lat,self.lon,self.units,self.api_key);

        println!("New Request: {}", geocoding_url);
        let client = Client::new();

        return match client.get(geocoding_url).send().await {
            Ok(response) => {

                if response.status().is_success() {
                    match response.text().await {
                        Ok(json) => json,
                        _ => format!("Request failed")
                    }
                } else {
                    format!("Request failed with status code {}.", response.status().to_string())
                }
            },
            _ => format!("Request failed")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::openweather_api::OpenWeatherClient;

    #[test]
    fn make_request_3h_5d() {
        let result: Result<OpenWeatherClient,String> = OpenWeatherClient::new(&"data/openweathermap_prod.conf");

        match result {
            Ok(client) => {
                let json_answer = client.make_request_3h_5d();

                match json::parse(&json_answer) {
                    Ok(json_value) => {
                        for entry in json_value.entries() {
                            match entry.0 {
                                "cod" => assert_eq!(entry.1.to_string(),"200"),
                                _default => ()
                            }
                        }
                    },
                    Err(_) => assert!(false)
                }
            },
            Err(_) => assert!(false)
        }
    }
}