use std::env;

pub struct AppConfig {}

impl AppConfig {
    pub fn webhook_username() -> String {
        env::var("TERU2_WEBHOOK_USERNAME").unwrap_or("お天気太郎".to_string())
    }
    pub fn weather_forecast_api_url() -> String {
        env::var("TERU2_WEATHER_FORECAST_API_URL")
            .unwrap_or("https://weather.tsukumijima.net/api/forecast/city/130010".to_string())
    }
    pub fn webhook_url() -> String {
        env::var("TERU2_DISCORD_WEBHOOK_URL").unwrap()
    }
}
