use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use http_client::h1::H1Client as Client;
use http_client::http_types::{Method, Request};
use http_client::HttpClient;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::env;
use webhook::client::WebhookClient;
use webhook::models::Embed;

const WEATHER_FORECAST_API_DEFAULT_URL: &str =
    "https://weather.tsukumijima.net/api/forecast/city/130010";
const WEBHOOK_DEFAULT_USERNAME: &str = "お天気太郎";

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<CloudWatchEvent>) -> Result<Response, Error> {
    let forecast = fetch_forecast().await.unwrap();

    send_to_discord(forecast).await;
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Command executed."),
    };
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

async fn fetch_forecast() -> Result<AreaForcast, http_client::http_types::Error> {
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

async fn send_to_discord(forecast: AreaForcast) {
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

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
struct AreaForcast {
    title: String,
    forecasts: Vec<ForcastByDay>,
    public_time_formatted: String,
    link: String,
    copyright: Copyright,
}

impl ToString for AreaForcast {
    fn to_string(&self) -> String {
        format!(
            r#"
        {}
        {}
        "#,
            self.title,
            self.forecasts
                .iter()
                .map(|forecast| { forecast.to_string() })
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl AreaForcast {
    fn todays_weather_image(self) -> ForecastImage {
        self.forecasts[0].image.clone()
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
struct ForcastByDay {
    date_label: String,
    detail: WeatherDetail,
    temperature: Temperatures,
    chance_of_rain: ChanceOfRain,
    image: ForecastImage,
}

impl ToString for ForcastByDay {
    fn to_string(&self) -> String {
        format!(
            r#"
{}
:thermometer: {}
:umbrella2: {}
        "#,
            self.detail.to_string(),
            self.temperature.to_string(),
            self.chance_of_rain.to_string()
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
struct WeatherDetail {
    weather: Option<String>,
}

impl ToString for WeatherDetail {
    fn to_string(&self) -> String {
        match self.clone().weather {
            Some(w) => w,
            None => "不明".to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
struct Temperatures {
    min: Temperature,
    max: Temperature,
}

impl ToString for Temperatures {
    fn to_string(&self) -> String {
        format!(
            "{} ～ {}",
            self.min.clone().to_string(),
            self.max.clone().to_string()
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
struct Temperature {
    celsius: Option<String>,
}

impl ToString for Temperature {
    fn to_string(&self) -> String {
        format!("{} ℃", self.celsius.clone().unwrap_or("-".to_string()))
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE", serialize = "snake_case"))]
struct ChanceOfRain {
    t00_06: String,
    t06_12: String,
    t12_18: String,
    t18_24: String,
}

impl ToString for ChanceOfRain {
    fn to_string(&self) -> String {
        format!(
            "{} → {} → {} → {}        ",
            self.t00_06, self.t06_12, self.t12_18, self.t18_24
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
struct ForecastImage {
    title: String,
    url: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
struct Copyright {
    title: String,
    image: CopyrightImage,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
struct CopyrightImage {
    title: String,
    url: String,
    link: String,
}
