#![allow(dead_code)]
mod utility;
use clap::Parser;
use dotenv::dotenv;
use utility::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 3)]
    days: u8,

    #[arg(short, long)]
    location: Option<String>,
}

fn main() {
    dotenv().ok();
    let api_key = std::env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY must be set");
    let args = Args::parse();

    let mut location = args.location;
    loop {
        let city = match location {
            Some(loc) => loc,
            None => get_city(),
        };
        location = None;
        match city.as_str() {
            "exit" => return,
            city => match get_weather_data(&api_key, city, args.days) {
                Ok(response) => display_weather(&response),
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            },
        }
    }
}
