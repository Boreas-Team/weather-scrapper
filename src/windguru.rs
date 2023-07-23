use std::ops::Add;

use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use time::{OffsetDateTime, Duration};

#[derive(Deserialize, Debug)]
struct SpotForecast {
    fcst: Vec<SpotForecastModel>,
}

#[derive(Deserialize, Debug)]
struct SpotForecastModel {
    id_model: u16,
    initstr: String,
}

#[derive(Deserialize, Debug)]
struct ModelForecastRoot {
    fcst: ModelForecast,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct ModelForecast {
    initstamp: u32,
    GUST: Vec<f64>,
    WINDSPD: Vec<f64>,
    hours: Vec<u16>,
}

pub struct WindguruClient {
    client: reqwest::Client,
}

impl Default for WindguruClient {
    fn default() -> Self {
        Self::new()
    }
}

impl WindguruClient {
    const URL: &str = "https://www.windguru.cz/int/iapi.php";
    const GFS13: u16 = 3;

    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        // Windguru does not authorize requests without a Referer field set
        headers.insert("Referer", 
            HeaderValue::from_static("https://www.windguru.cz/map/spot"));
        
        Self {
            client: reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Only Referer header is set. Client creation should not fail.")
        }
    } 

    async fn get_spot_forecast(&self, spot_id: u32) -> Result<SpotForecast, reqwest::Error> {       
        self.client.get(format!("{}?q=forecast_spot&id_spot={}", WindguruClient::URL, spot_id))
            .send().await?
            .json::<SpotForecast>().await
    }

    async fn get_model_forecast(&self, spot_id: u32, model_id: u16, initstr: &str) -> Result<ModelForecastRoot, reqwest::Error> {
        self.client.get(format!("{}?q=forecast&id_spot={}&id_model={}&initstr={}", WindguruClient::URL, spot_id, model_id, initstr))
            .send().await?
            .json::<ModelForecastRoot>().await
    }

    pub async fn get_forecast(&self, spot_id: u32) -> Result<Forecast, reqwest::Error> {
        let spot = self.get_spot_forecast(spot_id).await?;
        let gfs13 = spot.fcst.iter().find(|model| model.id_model == WindguruClient::GFS13)
            .expect("Other models than GFS13 are not supported");

        let forecast = self.get_model_forecast(spot_id, 
            WindguruClient::GFS13, 
            gfs13.initstr.as_str()).await?;

        let start_time = OffsetDateTime::from_unix_timestamp(forecast.fcst.initstamp.into())
            .expect("Windguru API should return correct timestamp");
        let mut result = Forecast {
            entries: vec![]
        };

        for (index, hour) in forecast.fcst.hours.iter().enumerate() {
            result.entries.push(ForecastEntry {
                time: start_time.clone().add(Duration::hours((*hour).into())),
                wind_speed: forecast.fcst.WINDSPD[index],
                wind_gusts: forecast.fcst.GUST[index],
            });
        }

        Ok(result)
    }
}


#[derive(Debug)]
pub struct Forecast {
    entries: Vec<ForecastEntry>,
}

#[derive(Debug)]
pub struct ForecastEntry {
    pub time: OffsetDateTime,
    pub wind_speed: f64,
    pub wind_gusts: f64
}