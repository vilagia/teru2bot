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

#[cfg(test)]
mod tests {

    mod webhook_username {
        use crate::config::AppConfig;

        #[test]
        fn not_env() {
            assert_eq!(AppConfig::webhook_username(), "お天気太郎");
        }

        #[test]
        fn env() {
            let webhook_username = "test_webhook_name";
            std::env::set_var("TERU2_WEBHOOK_USERNAME", webhook_username.to_string());
            assert_eq!(AppConfig::webhook_username(), webhook_username.to_string());
            std::env::remove_var("TERU2_WEBHOOK_USERNAME");
        }
    }
    mod weather_forecast_api_url {
        use crate::config::AppConfig;
        #[test]
        fn not_env() {
            assert_eq!(
                AppConfig::weather_forecast_api_url(),
                "https://weather.tsukumijima.net/api/forecast/city/130010"
            );
        }

        #[test]
        fn env() {
            let url = "https://weather.example.com";
            std::env::set_var("TERU2_WEATHER_FORECAST_API_URL", url.to_string());
            assert_eq!(AppConfig::weather_forecast_api_url(), url.to_string());
            std::env::remove_var("TERU2_WEATHER_FORECAST_API_URL");
        }
    }

    mod webhook_url {
        use crate::config::AppConfig;
        #[test]
        #[should_panic]
        fn not_env() {
            assert_eq!(
                AppConfig::webhook_url(),
                "https://weather.tsukumijima.net/api/forecast/city/130010"
            );
        }

        #[test]
        fn env() {
            let url = "https://discord.example.com";
            std::env::set_var("TERU2_DISCORD_WEBHOOK_URL", url.to_string());
            assert_eq!(AppConfig::webhook_url(), url.to_string());
            std::env::remove_var("TERU2_DISCORD_WEBHOOK_URL");
        }
    }
}
