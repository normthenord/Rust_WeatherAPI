

use std::io::{self, Write};

use colored::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    location: Location,
    current: Current,
}

#[derive(Deserialize, Debug)]
struct Location {
    name: String,
    region: String,
    country: String,
    localtime: String,
}
#[derive(Deserialize, Debug)]
struct Current {
    temp_f: f64,
    feelslike_f: f64,
    humidity: f64,
    condition: Condition,
    wind_mph: f64,
    gust_mph: f64,
    uv: f64,
    vis_miles: f64,
}

#[derive(Deserialize, Debug)]
struct Condition {
    text: String,
}

pub fn get_weather_data(api_key: &str, location: &str) -> Result<WeatherResponse, reqwest::Error> {
    let request = format!(
        "http://api.weatherapi.com/v1/forecast.json?key={}&q={}",
        api_key, location
    );
    let response = reqwest::blocking::get(request)?;

    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

pub fn display_weather(response: &WeatherResponse) {
    let temperature_colorized = colorized_temp(response.current.temp_f);

    let weather_text = format!(
        "Weather for: {}, {} {}
> Temperature: {}°F
> Feels like: {}°F
> Humidity: {:.1}%
> Current Conditions: {}",
        response.location.name,
        response.location.region,
        response.location.country,
        temperature_colorized,
        response.current.feelslike_f,
        response.current.humidity,
        response.current.condition.text
    );
    let weather_text_colored = weather_text;
    println!("{}", weather_text_colored);
}

fn colorized_temp(temperature: f64) -> ColoredString {
    let temperature_colorized: ColoredString;

    match temperature {
        ..=0.0 => temperature_colorized = format!("{:.1}", temperature).to_string().blue().bold(),
        32.0..=50.0 => temperature_colorized = format!("{:.1}", temperature).bright_blue().bold(),
        50.0..=70.0 => temperature_colorized = format!("{:.1}", temperature).truecolor(255, 255, 0).bold(), //yellow
        70.0..=80.0 => temperature_colorized = format!("{:.1}", temperature).truecolor(255, 165, 0).bold(), //orange
        _ => temperature_colorized = format!("{:.1}", temperature).red().bold(),
    }

    temperature_colorized
}

pub fn get_city() -> String {
    print!("\nWhich city woudld you like to have weather data for? (exit to cancel) ");
    let _ = io::stdout().flush().unwrap();
    let mut city = String::new();
    io::stdin().read_line(&mut city).unwrap();
    return city.trim().to_lowercase();
}
