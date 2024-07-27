#![allow(dead_code)]
mod utility;
use utility::*;
use dotenv::dotenv;


fn main() {
    dotenv().ok();
    let api_key = std::env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY must be set");

    loop {
        let city = get_city();
        match city.as_str() {
            "exit" => return,
            city => match get_weather_data(&api_key, city) {
                Ok(response) => display_weather(&response),
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            },
        }
    }
}
