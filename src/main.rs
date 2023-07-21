//! This module contains a program that retrieves weather forecast data from the OpenWeatherMap API.
//!
//! The program takes two command-line arguments: the latitude and longitude of the location to retrieve the forecast for.
//!
//! The `get_weather_data` function retrieves the forecast data from the API and returns it as a `WeatherData` struct.
//!
//! The `WeatherData` struct contains the following fields:
//!
//! - `cod`: A string representing the HTTP status code of the response.
//! - `message`: A float representing the message of the response.
//! - `cnt`: An integer representing the number of forecast data points returned.
//! - `list`: A vector of `WeatherDataList` structs representing the forecast data points.
//! - `city`: A `City` struct representing the city for which the forecast data was retrieved.
//!
//! The `WeatherDataList` struct contains the following fields:
//!
//! - `dt`: An integer representing the timestamp of the forecast data point.
//! - `main`: A `Main` struct representing the main weather parameters of the forecast data point.
//! - `weather`: A vector of `Weather` structs representing the weather conditions of the forecast data point.
//! - `clouds`: A `Clouds` struct representing the cloudiness of the forecast data point.
//! - `wind`: A `Wind` struct representing the wind conditions of the forecast data point.
//! - `visibility`: An integer representing the visibility of the forecast data point.
//! - `pop`: A float representing the probability of precipitation of the forecast data point.
//! - `rain`: An optional `Rain` struct representing the rain conditions of the forecast data point.
//! - `sys`: A `Sys` struct representing the system information of the forecast data point.
//! - `dt_txt`: A string representing the date and time of the forecast data point.
//!
//! The `Main` struct contains the following fields:
//!
//! - `temp`: A float representing the temperature of the forecast data point.
//! - `feels_like`: A float representing the "feels like" temperature of the forecast data point.
//! - `temp_min`: A float representing the minimum temperature of the forecast data point.
//! - `temp_max`: A float representing the maximum temperature of the forecast data point.
//! - `pressure`: An integer representing the atmospheric pressure of the forecast data point.
//! - `sea_level`: An integer representing the sea level atmospheric pressure of the forecast data point.
//! - `grnd_level`: An integer representing the ground level atmospheric pressure of the forecast data point.
//! - `humidity`: An integer representing the humidity of the forecast data point.
//! - `temp_kf`: A float representing the temperature change of the forecast data point.
//!
//! The `Weather` struct contains the following fields:
//!
//! - `id`: An integer representing the weather condition ID.
//! - `main`: A string representing the weather condition.
//! - `description`: A string representing the weather condition description.
//! - `icon`: A string representing the weather condition icon.
//!
//! The `Clouds` struct contains the following fields:
//!
//! - `all`: An integer representing the cloudiness percentage.
//!
//! The `Wind` struct contains the following fields:
//!
//! - `speed`: A float representing the wind speed.
//! - `deg`: An integer representing the wind direction in degrees.
//! - `gust`: A float representing the wind gust speed.
//!
//! The `Rain` struct contains the following fields:
//!
//! - `h3`: A float representing the rain volume for the last 3 hours.
//!
//! The `Sys` struct contains the following fields:
//!
//! - `pod`: A string representing the part of the day (day or night).
//!
//! The `City` struct contains the following fields:
//!
//! - `id`: An integer representing the city ID.
//! - `name`: A string representing the city name.
//! - `coord`: A `Coord` struct representing the city coordinates.
//! - `country`: A string representing the country code of the city.
//! - `population`: An integer representing the population of the city.
//! - `timezone`: An integer representing the timezone offset of the city.
//! - `sunrise`: An integer representing the sunrise time of the city.
//! - `sunset`: An integer representing the sunset time of the city.
//!
//! The `Coord` struct contains the following fields:
//!
//! - `lat`: A float representing the latitude of the city.
//! - `lon`: A float representing the longitude of the city.
//!
//! # Examples
//!
//! ```
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let lat = "37.7749";
//!     let lon = "-122.4194";
//!
//!     let body = get_weather_data(lat, lon).await?;
//!     println!("Forecast => {:?}", body);
//!
//!     Ok(())
//! }
//! ```

use std::env;
use std::error::Error;
use serde::{Deserialize, Serialize};

/// Retrieves weather forecast data from the OpenWeatherMap API.
///
/// # Arguments
///
/// * `lat` - A string slice representing the latitude of the location to retrieve the forecast for.
/// * `lon` - A string slice representing the longitude of the location to retrieve the forecast for.
///
/// # Examples
///
/// ```
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let lat = "37.7749";
///     let lon = "-122.4194";
///
///     let body = get_weather_data(lat, lon).await?;
///     println!("Forecast => {:?}", body);
///
///     Ok(())
/// }
/// ```
 async fn get_weather_data(lat: &str, lon: &str) -> Result<WeatherData, Box<dyn Error>> {
    let api_key = "API_KEY GOES HERE"; // TODO: Replace with your API key from https://home.openweathermap.org/api_keys
    let url = format!("https://api.openweathermap.org/data/2.5/forecast?lat={lat}&lon={lon}&appid={api_key}");
    let response = reqwest::get(&url).await?.json::<WeatherData>().await?;
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize)]
struct WeatherData {
    cod: String,
    message: f64,
    cnt: i64,
    list: Vec<WeatherDataList>,
    city: City,

}

#[derive(Debug, Serialize, Deserialize)]
struct WeatherDataList {
    dt: i64,
    main: Main,
    weather: Vec<Weather>,
    clouds: Clouds,
    wind: Wind,
    visibility: i64,
    pop: f64,
    rain: Option<Rain>,
    sys: Sys,
    dt_txt: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i64,
    sea_level: i64,
    grnd_level: i64,
    humidity: i64,
    temp_kf: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Weather {
    id: i64,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Clouds {
    all: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Wind {
    speed: f64,
    deg: i64,
    gust: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Rain {
    #[serde(rename = "3h")]
    h3: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sys {
    pod: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct City {
    id: i64,
    name: String,
    coord: Coord,
    country: String,
    population: i64,
    timezone: i64,
    sunrise: i64,
    sunset: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Coord {
    lat: f64,
    lon: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} LATITUDE LONGITUDE", args[0]);
        return Ok(());
    }
    let lat = &args[1];
    let lon = &args[2];

    let body =  get_weather_data(&lat, &lon).await?;
    print!("Forecast => {:?}", body);

    Ok(())
}