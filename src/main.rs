use std::env;
use dotenv;
use anyhow;

mod cli;
use cli::*;

mod weather;
use weather::Coordinates;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let api_key = env::var("API_KEY")?;

    let cli = Cli::new();

    match cli.command {
        Commands::Current(args) => {
            let coords = weather::get_coordinates(&args.city, &api_key).await;
            let weather_url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units={}&appid={}", coords.lat, coords.lon, "metric", api_key);
            let weather_current = weather::fetch_current_weather(&weather_url).await.unwrap();
            weather::print_weather_current(&weather_current);
        },
        Commands::Forecast(args) => {
            let coords = weather::get_coordinates(&args.city, &api_key).await;

            let weather_url = format!("https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&units={}&appid={}", coords.lat, coords.lon, "metric", api_key);
            let weather_forecast = weather::fetch_weather_forecast(&weather_url).await.unwrap();

            println!("{}", weather_url);
            
            weather::print_forecast_daily(&weather_forecast);
        },
    }

    Ok(())
}
