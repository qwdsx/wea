use clap::{Args, Parser, Subcommand};

const ABOUT: &str= "weather";

#[derive(Parser)]
#[clap(about = ABOUT, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get current weather
    Current(Query),
    /// Get weather forecast (wip)
    Forecast(Query)
}

#[derive(Args)]
pub struct Query {
    pub city: String,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}