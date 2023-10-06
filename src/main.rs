use std::fs;
use std::net::SocketAddr;

use axum::handler::HandlerWithoutStateExt;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use icalendar::Component;

use crate::openweather_api::OpenWeatherClient;
use crate::openweather_api::parsing::parse_json_open_weather;

mod webdav;
mod openweather_api;

#[tokio::main]
async fn main() {
    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.

    // A closure or a function can be used as handler.

    let app = Router::new()
        .nest("/images", axum_static::static_router("data/images"))
        .nest("/styles", axum_static::static_router("data/styles"))
        .route("/", get(handler));
    //        Router::new().route("/", get(|| async { "Hello, world!" }));

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
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
            let modified_html = set_weather_data(&html_content).await;

            Html(modified_html)
        },
        Err(_) => Html(format!("Error reading HTML file. Is the path '{}' correct?", path))
    }
}

async fn set_weather_data(html_content: &String) -> String {
    let result = OpenWeatherClient::new("data/openweathermap_prod.conf");

    return match result {
        Ok(client) => {
            let json = client.make_request_3h_5d().await;
            let option = parse_json_open_weather(&json);

            return match option {
                Some(weather) => {
                    let weather_entry_current = weather.get(0).unwrap();
                    let weather_entry_3 = weather.get(1).unwrap();
                    let weather_entry_6 = weather.get(2).unwrap();
                    let weather_entry_9 = weather.get(3).unwrap();

                    let mut modified_html_content = html_content.replace("#temp_current", weather_entry_current.main.temp.to_string().as_str());
                    modified_html_content = modified_html_content.replace("#desc", weather_entry_current.weather.description.to_string().as_str());
                    modified_html_content = modified_html_content.replace("#feel_like_current", weather_entry_current.main.feels_like.to_string().as_str());
                    modified_html_content = modified_html_content.replace("#humidity_current", weather_entry_current.main.humidity.to_string().as_str());
                    modified_html_content = modified_html_content.replace("#max_temp_current", weather_entry_current.main.temp_max.to_string().as_str());
                    modified_html_content = modified_html_content.replace("#rain_current", weather_entry_current.precipitation_probability.to_string().as_str());

                    modified_html_content = modified_html_content.replace("#time_+3", weather_entry_3.time_of_forecast.time().to_string().as_str());
                    modified_html_content = modified_html_content.replace("#temp_+3", weather_entry_3.main.temp.to_string().as_str());
                    modified_html_content = modified_html_content.replace("#rain_+3", weather_entry_3.precipitation_probability.to_string().as_str());

                    modified_html_content = modified_html_content.replace("#time_+6", weather_entry_6.time_of_forecast.time().to_string().as_str());
                    modified_html_content = modified_html_content.replace("#temp_+6", weather_entry_6.main.temp.to_string().as_str());
                    modified_html_content = modified_html_content.replace("#rain_+6", weather_entry_6.precipitation_probability.to_string().as_str());

                    modified_html_content = modified_html_content.replace("#time_+9", weather_entry_9.time_of_forecast.time().to_string().as_str());
                    modified_html_content = modified_html_content.replace("#temp_+9", weather_entry_9.main.temp.to_string().as_str());
                    modified_html_content = modified_html_content.replace("#rain_+9", weather_entry_9.precipitation_probability.to_string().as_str());

                    let icon_current = &weather_entry_current.weather.icon;




                    modified_html_content
                },
                None => html_content.clone()
            }
        },
        Err(_) => html_content.clone()
    }
}