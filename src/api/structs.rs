use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct AreaForcast {
    pub title: String,
    pub forecasts: Vec<ForcastByDay>,
    pub public_time_formatted: String,
    pub link: String,
    pub copyright: Copyright,
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
    // fn todays_weather_image(self) -> ForecastImage {
    //     self.forecasts[0].image.clone()
    // }

    pub fn title_with_weather_summary(&self) -> String {
        format!("{}({})", self.title, self.forecasts[0].detail.to_string())
    }

    #[cfg(test)]
    pub fn from_json_file(path: &std::path::Path) -> Result<Self, std::io::Error> {
        // Open the file in read-only mode with buffer.

        use std::{fs::File, io::BufReader};
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let u = serde_json::from_reader(reader)?;

        Ok(u)
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct ForcastByDay {
    pub date_label: String,
    detail: WeatherDetail,
    temperature: Temperatures,
    chance_of_rain: ChanceOfRain,
    // image: ForecastImage,
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
    // title: String,
    // url: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct Copyright {
    pub title: String,
    pub image: CopyrightImage,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct CopyrightImage {
    pub title: String,
    pub url: String,
    pub link: String,
}
