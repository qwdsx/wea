
use std::time;

use chrono::{DateTime, Utc};
use reqwest;
use serde::Deserialize;
use thiserror;

#[derive(Deserialize, Debug, Clone)]
pub struct Coordinates {
    pub name: String,
    pub country: String,
    pub lat: f32,
    pub lon: f32
}

#[derive(Deserialize, Debug)]
pub struct WeatherForecast {
    pub list: Vec<Forecast>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Forecast {
    pub dt: u64,
    pub main: Temperature,
    pub weather: Vec<Weather>,
    pub dt_txt: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct Temperature {
    pub temp: f32,
    pub feels_like: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Weather {
    pub main: Temperature,
    pub weather: Vec<WeatherDescription>
}

#[derive(Deserialize, Debug, Clone)]
pub struct WeatherDescription {
    pub main: String,
    pub description: String
}

pub async fn fetch_current_weather(url: &str) -> Result<Weather, Box<dyn std::error::Error>> {
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(serde_json::from_str(&body)?)
}

pub async fn fetch_weather_forecast(url: &str) -> Result<WeatherForecast, Box<dyn std::error::Error>> {
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(serde_json::from_str(&body)?)
}

async fn fetch_coordinates(url: &str) -> Result<Vec<Coordinates>, Box<dyn std::error::Error>> {
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(serde_json::from_str(&body)?)
}

pub async fn get_coordinates(city: &str, api_key: &str) -> Coordinates {
    let coordinates_url = format!("http://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}", city, api_key);
    let coordinates = fetch_coordinates(&coordinates_url).await.unwrap();
    let coords = &coordinates[0];
    coords.clone()
}

pub fn format_forecast_time(forecast: &Forecast) -> String {
    let d = time::UNIX_EPOCH + time::Duration::from_secs(forecast.dt);
    let date_time = DateTime::<Utc>::from(d);
    date_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn print_weather_current(weather_current: &Weather) {
    println!("Temperature: {:.1}°C", weather_current.main.temp);
    println!("Description: {}", weather_current.weather[0].main);
}

pub fn print_forecast_daily(weather_forecast: &WeatherForecast) {
    println!("{:#?}", weather_forecast.list);
}

fn avg_temp(vec: &Vec<Forecast>) -> f32 {
    let mut avg_temp = 0.0;
    for f in vec {
        avg_temp += f.main.temp;
    }
    avg_temp / vec.len() as f32
}

fn group_by_date(vec: &Vec<Forecast>) -> Vec<Vec<Forecast>> {
    let mut vec_result = Vec::<Vec<Forecast>>::new();
    // vec_result.push(Vec::<Forecast>::new());
    // vec_result.push(Vec::<Forecast>::new());
    // vec_result.push(Vec::<Forecast>::new());
    // vec_result.push(Vec::<Forecast>::new());
    // vec_result.push(Vec::<Forecast>::new());

    // let mut index = 0;
    // let mut day = 32;

    // for i in vec {
    //     let date = i.dt_txt.split(" ").collect::<Vec<&str>>()[0];
    //     let day_temp = date.split("-").collect::<Vec<&str>>()[2].parse::<u32>().unwrap();

    //     println!("{:#?}", vec.len());
        
    //     if day_temp > day {
    //         vec_result[index].push(i.clone());
    //         index += 1;
    //         continue;
    //     } else {
    //         vec_result[index].push(i.clone());
    //     }

    //     day = day_temp;
    // }

    vec_result
}