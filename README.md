# teru2bot -- A lambda function to notify a weather in Japan via https://weather.tsukumijima.net

## How to use it?

* **Read [this document](https://weather.tsukumijima.net) carefully and understand.**
* Build and deploy to AWS Lambda(see: https://www.cargo-lambda.info).
* Set suitable Lambda function trigger(s).
  * AWS Cloudwatch Logs assumed.

## Warning

This app depends a APIs below. Heavy traffic will affect them. **USE THIS CODE MODEST**.

* https://weather.tsukumijima.net
* Discord Webhook
* Japan Meteorological Agency Website API

## Thanks

* @tsukumijima(https://github.com/tsukumijima)
* Japan Meteological Agency
