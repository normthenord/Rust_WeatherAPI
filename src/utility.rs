use std::io::{self, Write};

use colored::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct WeatherResponse {
    location: Location,
    current: Current,
    forecast: Forecast,
}

#[derive(Deserialize, Debug, Clone)]
struct Location {
    name: String,
    region: String,
    country: String,
    localtime: String,
}
#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
struct Condition {
    text: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Forecast {
    forecastday: Vec<Day>,
}
#[derive(Deserialize, Debug, Clone)]
struct Day {
    date: String,
    day: DayConditions,
}
#[derive(Deserialize, Debug, Clone, Copy)]
struct DayConditions {
    maxtemp_f: f64,
    mintemp_f: f64,
    daily_chance_of_rain: i64,
}

pub fn get_weather_data(
    api_key: &str,
    location: &str,
    num_days: u8,
) -> Result<WeatherResponse, reqwest::Error> {
    let request = format!(
        "http://api.weatherapi.com/v1/forecast.json?key={}&q={}&days={num_days}",
        api_key, location
    );
    let response = reqwest::blocking::get(request)?;

    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

pub fn display_weather(response: &WeatherResponse) {
    let weather_text = format!(
        "Current Weather for: {}, {}, {}
> Temperature: {}°F
> Feels like: {}°F
> Humidity: {:.1}%
> Current Conditions: {}\n",
        response.location.name,
        response.location.region,
        response.location.country,
        colorized_temp(response.current.temp_f),
        colorized_temp(response.current.feelslike_f),
        response.current.humidity,
        response.current.condition.text,
    );
    println!("{}", weather_text);

    for day in &response.forecast.forecastday {
        print_daily_forecast(day, response.location.clone());
    }
}

fn print_daily_forecast(day: &Day, location: Location) {
    let weather_text = format!(
        "Daily Weather for {}, {} on {}:
> High of {}°F
> Low of {}°F
> Chance of rain: {}%\n",
        location.name,
        location.region,
        day.date,
        colorized_temp(day.day.maxtemp_f),
        colorized_temp(day.day.mintemp_f),
        day.day.daily_chance_of_rain
    );
    println!("{}", weather_text);
}

fn colorized_temp(temperature: f64) -> ColoredString {
    let temperature_colorized: ColoredString;

    match temperature {
        ..=0.0 => temperature_colorized = format!("{:.1}", temperature).to_string().blue().bold(),
        32.0..=50.0 => temperature_colorized = format!("{:.1}", temperature).bright_blue().bold(),
        50.0..=70.0 => {
            temperature_colorized = format!("{:.1}", temperature).truecolor(255, 255, 0).bold()
        } //yellow
        70.0..=80.0 => {
            temperature_colorized = format!("{:.1}", temperature).truecolor(255, 165, 0).bold()
        } //orange
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
