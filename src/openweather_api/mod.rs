use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use reqwest::Client;

use crate::filesystem::FileSystemHandler;

mod weather_entry;
pub(crate) mod parsing;

pub struct OpenWeatherClient {
    api_key: String,
    pub url: String,
    pub url_img: String,
    pub lat: f64,
    pub lon: f64,
    pub units: String,
    pub lang: String

}

impl OpenWeatherClient {
    pub(crate) fn new(path_config: &str) -> Result<Self,String> {
        let mut api_key: String = String::from("");
        let mut url: String = String::from("");
        let mut url_img: String = String::from("");
        let mut lat: f64 = 0f64;
        let mut lon: f64 = 0f64;
        let mut units: String = String::from("");
        let mut lang: String = String::from("");


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
                            "url_img" => url_img = value.to_string(),
                            "units" => units = value.to_string(),
                            "lang" => lang = value.to_string(),
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
            url_img,
            lat,
            lon,
            units,
            lang
        })
    }

    pub(crate) async fn make_request_forecast_3h_5d(&self) -> String {
        let geocoding_url = format!("{}?lat={}&lon={}&units={}&lang={}&appid={}",
                                    self.url, self.lat,self.lon,self.units,self.lang,self.api_key);

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

    pub(crate) async fn download_icon(&self, icon: &str) -> bool {

        let request_url = format!("{}/{}@2x.png", self.url_img, icon);

        println!("New Request: {}", request_url);
        let client = Client::new();

        return match client.get(request_url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    return match response.bytes().await {
                        Ok(image_bytes) => {
                            return match FileSystemHandler::new() {
                                Ok(filesystem_handler) => {
                                    return match filesystem_handler.create_directory("weather_icons") {
                                        Ok(absolute_path) => {
                                            return match File::create(format!("{}/{}", absolute_path, icon)) {
                                                Ok(mut file) => {
                                                    println!("Write to file: '{}'", absolute_path);
                                                    return match file.write_all(&image_bytes) {
                                                        Ok(_) => true,
                                                        Err(_) => false
                                                    }
                                                },
                                                Err(e) => false

                                            }
                                        },
                                        Err(e) => false
                                    }
                                },
                                Err(_) => false
                            }
                        },
                        Err(_) => false
                    }
                } else {
                    false
                }
            },
            _ => false
        };









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
                let json_answer = client.make_request_forecast_3h_5d();

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