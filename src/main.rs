use reqwest;
use serde_json;
use geolocation;
use local_ip_address::{local_ip};
use std::net::IpAddr;
use std::env;

fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    // Convert kelvin to fahrenheit
    (kelvin - 273.15) * 9.0 / 5.0 + 32.0
}
fn get_weather_data(lat: String, lon: String, api_key: &str) -> String {
    // Make api call to openweathermap.org
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}", lat, lon, api_key);
    let response = reqwest::blocking::get(url).unwrap();
    let body = response.text().unwrap();
    body
}

fn get_lat_lon(ip: &str) -> (String, String) {
    // Get latitude and longitude from current location
    let location = geolocation::find(ip).unwrap();
    let lat = location.latitude;
    let lon = location.longitude;
    (lat, lon)
}

fn construct_weather_string(parsed_data: &serde_json::Value) -> String {
    // Construct weather string from parsed data
    let temp = kelvin_to_fahrenheit(parsed_data["main"]["temp"].as_f64().unwrap());
    let feels_like = kelvin_to_fahrenheit(parsed_data["main"]["feels_like"].as_f64().unwrap());
    let humidity = parsed_data["main"]["humidity"].as_f64().unwrap();
    let wind_speed = parsed_data["wind"]["speed"].as_f64().unwrap();
    let wind_direction = parsed_data["wind"]["deg"].as_f64().unwrap();
    let weather_description = parsed_data["weather"][0]["description"].as_str().unwrap();
    let output_string = format!("The current temperature is {} degrees fahrenheit. It feels like {} degrees fahrenheit. The humidity is {} percent. The wind speed is {} miles per hour. The wind direction is {} degrees. The weather description is {}.", temp, feels_like, humidity, wind_speed, wind_direction, weather_description);
    output_string
}

fn alternative_get_ip(_: local_ip_address::Error) -> Result<IpAddr, std::net::AddrParseError> {
    //println!("Error could not get IP address, Curling for it instead: {}", e);
    // Get IP address from ifconfig.me
    let response = reqwest::blocking::get("https://ifconfig.me/ip").unwrap();
    let body = response.text().unwrap();
    let ip: Result<IpAddr, std::net::AddrParseError> = body.parse();
    ip
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let api_key = &args[1];
    // get user's current IP address and base location off of this
    let ip = local_ip().or_else(alternative_get_ip).unwrap().to_string();
    
    // Get current latitude and longitude
    let (lat, lon) = get_lat_lon(&ip);

    // Get weather data from openweathermap.org
    let weather_data = get_weather_data(lat, lon, api_key);

    // Parse weather data
    let parsed_data: serde_json::Value = serde_json::from_str(&weather_data).unwrap();

    let output_string = construct_weather_string(&parsed_data);

    println!("{}", output_string);
}

