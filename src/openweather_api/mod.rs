use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::time::{Duration, SystemTime};

use reqwest::Client;

use crate::filesystem::FileSystemHandler;

mod weather_entry;
pub(crate) mod parsing;

pub struct OpenWeatherClient {
    api_key: String,
    pub url_current: String,
    pub url_5d_3h: String,
    pub url_img: String,
    pub lat: f64,
    pub lon: f64,
    pub units: String,
    pub lang: String

}

impl OpenWeatherClient {
    pub(crate) fn new(path_config: &str) -> Result<Self,String> {
        let mut api_key: String = String::from("");
        let mut url_current: String = String::from("");
        let mut url_5d_3h: String = String::from("");
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
                            "url_current" => url_current = value.to_string(),
                            "url_5d_3h" => url_5d_3h = value.to_string(),
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
            url_current,
            url_5d_3h,
            url_img,
            lat,
            lon,
            units,
            lang
        })
    }

    pub(crate) async fn make_request_current(&self) -> Result<String, String> {
        let geocoding_url = format!("{}?lat={}&lon={}&units={}&lang={}&appid={}",
                                    self.url_current, self.lat, self.lon, self.units, self.lang, self.api_key);

        println!("New Request (current): {}", geocoding_url);
        let client = Client::new();

        return match client.get(geocoding_url).send().await {
            Ok(response) => {

                if response.status().is_success() {
                    match response.text().await {
                        Ok(json) => Ok(json),
                        _ => Err(format!("Request failed"))
                    }
                } else {
                    Err(format!("Request failed with status code {}.", response.status().to_string()))
                }
            },
            _ => Err(format!("Request failed"))
        }
    }

    pub(crate) async fn make_request_forecast_3h_5d(&self) -> Result<String, String> {
        let geocoding_url = format!("{}?lat={}&lon={}&units={}&lang={}&appid={}",
                                    self.url_5d_3h, self.lat, self.lon, self.units, self.lang, self.api_key);

        println!("New Request (forecast 3h 5d): {}", geocoding_url);
        let client = Client::new();

        return match client.get(geocoding_url).send().await {
            Ok(response) => {

                if response.status().is_success() {
                    match response.text().await {
                        Ok(json) => Ok(json),
                        _ => Err(format!("Request failed"))
                    }
                } else {
                    Err(format!("Request failed with status code {}.", response.status().to_string()))
                }
            },
            _ => Err(format!("Request failed"))
        }
    }

    pub(crate) async fn download_icon(&self, icon: &str) -> Result<(), String> {
        let request_url = format!("{}/{}@2x.png", self.url_img, icon);
        println!("New Request: {}", request_url);

        let client = Client::new();
        let response = client.get(request_url).send().await
            .or(Err("Failed to send request"))?;

        if !response.status().is_success() {
            return Err("Failed to download image".into());
        }

        let image_bytes = response.bytes().await
            .or(Err("Failed to read bytes of response"))?;

        let filesystem_handler = FileSystemHandler::new()
            .or(Err("Failed to create FileHandler"))?;

        let absolute_path = filesystem_handler.create_directory("weather_icons")
            .or(Err("Failed to create 'weather_icons' directory"))?;

        let icon_path = format!("{}/{}.png", absolute_path, icon);

        if fs::metadata(&icon_path).is_err() && need_new_file(&icon_path) {
            let mut file = File::create(&icon_path)
                .or(Err(format!("Failed to create file '{}'", icon_path)))?;

            return if file.write_all(&image_bytes).is_ok() {
                Ok(())
            } else {
                Err(format!("Failed to write image bytes to file '{}'", icon_path))
            }
        }

        Ok(())
    }
}

fn need_new_file(path: &str) -> bool {
    let metadata = fs::metadata(path);

    // if last_modification date can't be read, just return false, so a new file will be downloaded
    if metadata.is_err() {
        return true
    }

    let now: SystemTime = SystemTime::now();
    let months: u64 = 3;

    let modified_time = metadata.unwrap().modified();
    if modified_time.is_err() {
        return true
    }

    let duration = now.duration_since(modified_time.unwrap());
    if duration.is_err() {
        return true
    }

    let three_months = Duration::from_secs(60 * 60 * 24 * 30 * months);
    if duration.unwrap() > three_months {
        true
    } else {
        false
    }
}


#[cfg(test)]
mod test {
    use crate::openweather_api::OpenWeatherClient;

    #[tokio::test]
    async fn make_request_3h_5d() {
        let result: Result<OpenWeatherClient,String> = OpenWeatherClient::new(&"data/openweathermap_prod.conf");

        match result {
            Ok(client) => {
                let json_answer = match client.make_request_forecast_3h_5d().await {
                    Ok(json_answer) => json_answer,
                    Err(_) => return assert!(false)
                };

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