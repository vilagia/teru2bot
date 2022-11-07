use http_client::h1::H1Client as Client;
use http_client::HttpClient;
use http_client::{http_types::Method, Request};
use reqwest::Url;
use webhook::client::WebhookClient;
use webhook::models::Embed;

use self::structs::{AreaForcast, ForcastByDay};

mod structs;

use super::config::AppConfig;

pub async fn fetch_forecast() -> Result<AreaForcast, http_client::http_types::Error> {
    let client = Client::new();
    let mut request = Request::new(
        Method::Get,
        Url::parse(AppConfig::weather_forecast_api_url().as_str()).unwrap(),
    );
    request.append_header("User-Agent", "teru2bot");
    let mut response = client.send(request).await?;
    response.body_json::<AreaForcast>().await
}

pub async fn send_to_discord(forecast: AreaForcast) -> bool {
    let webhook_client = WebhookClient::new(AppConfig::webhook_url().as_str());
    let chatbot_name: String = AppConfig::webhook_username();
    webhook_client
        .send(|message| {
            message.username(chatbot_name.as_str()).embed(|emb| {
                forecast
                    .forecasts
                    .iter()
                    .fold(emb, |e: &mut Embed, forecast: &ForcastByDay| {
                        e.field(&forecast.date_label, forecast.to_string().as_str(), false)
                    })
                    .title(forecast.title_with_weather_summary().as_str())
                    .description(forecast.public_time_formatted.as_str())
                    .url(forecast.link.as_str())
                    .author(
                        forecast.copyright.title.as_str(),
                        Some(forecast.clone().copyright.image.link),
                        Some(forecast.clone().copyright.image.url),
                    )
            })
        })
        .await
        .unwrap()
}

#[cfg(test)]
mod tests {
    mod send_to_discord {
        use std::path::Path;

        use httpmock::prelude::*;

        use crate::api;
        use crate::api::structs::AreaForcast;
        use crate::config::AppConfig;

        #[tokio::test]
        async fn return_true_if_success() {
            let server = MockServer::start();
            let discord_mock = server.mock(|when, then| {
                when.any_request();
                then.status(204);
            });
            std::env::set_var("TERU2_DISCORD_WEBHOOK_URL", server.url("/discord"));
            let fixture_file_path = Path::new("./fixtures/forecast.example.json");
            let forecast = AreaForcast::from_json_file(fixture_file_path).unwrap();
            let url = AppConfig::webhook_url();
            assert_eq!(url, server.url("/discord"));
            let is_success = api::send_to_discord(forecast).await;
            std::env::remove_var("TERU2_DISCORD_WEBHOOK_URL");
            discord_mock.assert();
            assert!(is_success);
        }

        #[tokio::test]
        #[should_panic]
        async fn panic_if_error() {
            let server = MockServer::start();
            server.mock(|when, then| {
                when.method(POST).path("/discord");
                then.status(400);
            });
            std::env::set_var("TERU2_DISCORD_WEBHOOK_URL", server.url("/discord"));
            let fixture_file_path = Path::new("./fixtures/forecast.example.json");
            let forecast = AreaForcast::from_json_file(fixture_file_path).unwrap();
            let url = AppConfig::webhook_url();
            assert_eq!(url, server.url("/discord"));
            api::send_to_discord(forecast).await;
        }
    }
}
