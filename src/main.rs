use std::fs;
use std::fs::File;
use std::net::SocketAddr;

use axum::handler::HandlerWithoutStateExt;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use chrono::Utc;
use icalendar::Component;
use log::*;
use simplelog::*;

use crate::filesystem::FileSystemHandler;
use crate::openweather_api::OpenWeatherClient;
use crate::openweather_api::parsing::{parse_json_open_weather, utc_to_local_date_time, WeatherOrCity};

mod webdav;
mod openweather_api;
pub mod filesystem;

#[tokio::main]
async fn main() {
    let filesystem_handler = match FileSystemHandler::new() {
        Ok(handler) => handler,
        Err(_) => {
            eprintln!("Failed to create FileSystemHandler");
            return;
        }
    };

    // create logger
    match CombinedLogger::init(
        vec![
            TermLogger::new(
                LevelFilter::Info,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto),
            WriteLogger::new(
                LevelFilter::Info,
                Config::default(),
                File::create(format!("{}/log_{}.log", &filesystem_handler.home_directory_software, Utc::now().format("%Y-%m-%d").to_string())).unwrap(),
            ),
        ]
    ) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Failed to create Logger");
            return
        }
    }

    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.

    // A closure or a function can be used as handler.
    let app = Router::new()
        .nest("/weather_icons", axum_static::static_router(&format!("{}/weather_icons", &filesystem_handler.home_directory_software)))
        .nest("/styles", axum_static::static_router("data/styles"))
        .route("/", get(handler));

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    info!("Starting service...");
    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn handler() -> Html<String> {
    let path = "src/website/index.html";
    let html_file = fs::read_to_string(path);

    return match html_file {
        Ok(html_content) => {
            return match set_weather_data(&html_content).await {
                Ok(modified_html) => Html(modified_html),
                Err(e) => {
                    error!("Error setting weather data: {}", e);
                    Html(html_content)
                }
            }


        },
        Err(_) => {
            error!("Error reading HTML file. Is the path '{}' correct?", path);
            Html(format!("Error reading HTML file. Check the log file."))
        }
    }
}

async fn set_weather_data(html_content: &String) -> Result<String, String> {
    let client = match OpenWeatherClient::new("data/openweathermap_prod.conf") {
        Ok(client) => client,
        Err(e) => return Err(format!("Couldn't create OpenWeatherClient: {}", e)),
    };

    let json = match client.make_request_forecast_3h_5d().await {
        Ok(json) => json,
        Err(e) => return Err(format!("Error making forecast request: {}", e)),
    };

    let weather_data = match parse_json_open_weather(&json) {
        Some(data) => data,
        None => return Err(format!("Couldn't parse json into weather entries: {}", json)),
    };

    let mut modified_html_content = html_content.clone();

    for entry in weather_data {
        match entry {
            WeatherOrCity::Weather(weather_entries) => {
                let weather_entry_current = weather_entries.get(0).unwrap();
                let weather_entry_3 = weather_entries.get(1).unwrap();
                let weather_entry_6 = weather_entries.get(2).unwrap();
                let weather_entry_9 = weather_entries.get(3).unwrap();

                modified_html_content = modified_html_content.replace("#temp_current", weather_entry_current.main.temp.to_string().as_str());
                modified_html_content = modified_html_content.replace("#desc", weather_entry_current.weather.description.to_string().as_str());
                modified_html_content = modified_html_content.replace("#feel_like_current", weather_entry_current.main.feels_like.to_string().as_str());
                modified_html_content = modified_html_content.replace("#humidity_current", weather_entry_current.main.humidity.to_string().as_str());
                modified_html_content = modified_html_content.replace("#max_temp_current", weather_entry_current.main.temp_max.to_string().as_str());
                modified_html_content = modified_html_content.replace("#rain_current", weather_entry_current.precipitation_probability.to_string().as_str());

                modified_html_content = modified_html_content.replace("#time_+3", utc_to_local_date_time(weather_entry_3.time_of_forecast).time().to_string().as_str());
                modified_html_content = modified_html_content.replace("#temp_+3", weather_entry_3.main.temp.to_string().as_str());
                modified_html_content = modified_html_content.replace("#rain_+3", weather_entry_3.precipitation_probability.to_string().as_str());

                modified_html_content = modified_html_content.replace("#time_+6", utc_to_local_date_time(weather_entry_6.time_of_forecast).time().to_string().as_str());
                modified_html_content = modified_html_content.replace("#temp_+6", weather_entry_6.main.temp.to_string().as_str());
                modified_html_content = modified_html_content.replace("#rain_+6", weather_entry_6.precipitation_probability.to_string().as_str());

                modified_html_content = modified_html_content.replace("#time_+9", utc_to_local_date_time(weather_entry_9.time_of_forecast).time().to_string().as_str());
                modified_html_content = modified_html_content.replace("#temp_+9", weather_entry_9.main.temp.to_string().as_str());
                modified_html_content = modified_html_content.replace("#rain_+9", weather_entry_9.precipitation_probability.to_string().as_str());


                match client.download_icon(&weather_entry_current.weather.icon).await  {
                    Ok(_) => debug!("Success reading icon current weather."),
                    Err(msg) => error!("{}",msg)

                }

                match client.download_icon(&weather_entry_3.weather.icon).await  {
                    Ok(_) => debug!("Success reading icon weather+3."),
                    Err(msg) => error!("{}",msg)

                }

                match client.download_icon(&weather_entry_6.weather.icon).await  {
                    Ok(_) => debug!("Success reading icon weather+6."),
                    Err(msg) => error!("{}",msg)

                }

                match client.download_icon(&weather_entry_9.weather.icon).await  {
                    Ok(_) => debug!("Success reading icon weather+9."),
                    Err(msg) => error!("{}",msg)

                }

                modified_html_content = modified_html_content.replace("#icon_current", &format!("/weather_icons/{}.png", &weather_entry_current.weather.icon)).to_string();
                modified_html_content = modified_html_content.replace("#icon_+3", &format!("/weather_icons/{}.png", &weather_entry_3.weather.icon)).to_string();
                modified_html_content = modified_html_content.replace("#icon_+6", &format!("/weather_icons/{}.png", &weather_entry_6.weather.icon)).to_string();
                modified_html_content = modified_html_content.replace("#icon_+9", &format!("/weather_icons/{}.png", &weather_entry_9.weather.icon)).to_string();
            },
            WeatherOrCity::City(city_entry) => {
                modified_html_content = modified_html_content.replace("#sunrise", utc_to_local_date_time(city_entry.sunrise).time().to_string().as_str());
                modified_html_content = modified_html_content.replace("#sunset", utc_to_local_date_time(city_entry.sunset).time().to_string().as_str());
            }
            _ => (),
        }
    }

    Ok(modified_html_content)
}