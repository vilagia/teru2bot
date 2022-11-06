
mod api;

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
    let forecast = api::fetch_forecast().await.unwrap();

    api::send_to_discord(forecast).await;
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

