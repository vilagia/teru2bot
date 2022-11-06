use std::env;

use http_client::HttpClient;
use http_client::h1::H1Client as Client;
use http_client::{Request, http_types::Method};
use reqwest::{Url};
use webhook::client::WebhookClient;
use webhook::models::Embed;

use self::structs::{AreaForcast, ForcastByDay};

mod structs;

const WEBHOOK_DEFAULT_USERNAME: &str = "お天気太郎";
const WEATHER_FORECAST_API_DEFAULT_URL: &str =
"https://weather.tsukumijima.net/api/forecast/city/130010";

pub async fn fetch_forecast() -> Result<AreaForcast, http_client::http_types::Error> {
    let weather_forecast_api_url: String = env::var("TERU2_WEATHER_FORECAST_API_URL")
        .unwrap_or(WEATHER_FORECAST_API_DEFAULT_URL.to_string());
    let client = Client::new();
    let mut request = Request::new(
        Method::Get,
        Url::parse(weather_forecast_api_url.as_str()).unwrap(),
    );
    request.append_header("User-Agent", "teru2bot");
    let mut response = client.send(request).await?;
    response.body_json::<AreaForcast>().await
}


 pub async fn send_to_discord(forecast: AreaForcast) {
    let webhook_url = env::var("TERU2_DISCORD_WEBHOOK_URL").unwrap();
    let webhook_client = WebhookClient::new(webhook_url.as_str());
    let chatbot_name: String =
        env::var("TERU2_WEBHOOK_USERNAME").unwrap_or(WEBHOOK_DEFAULT_USERNAME.to_string());

    webhook_client
        .send(|message| {
            message.username(chatbot_name.as_str()).embed(|emb| {
                forecast
                    .forecasts
                    .iter()
                    .fold(emb, |e: &mut Embed, forecast: &ForcastByDay| {
                        e.field(&forecast.date_label, forecast.to_string().as_str(), false)
                    })
                    .title(
                        format!(
                            "{}({})",
                            forecast.title,
                            forecast.forecasts[0].detail.to_string()
                        )
                        .as_str(),
                    )
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
        .unwrap();
}
