use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Serialize, Deserialize};

// id_spot=48968 => Ria de Alvor
// https://www.windguru.cz/int/iapi.php?q=forecast_spot&id_spot=48968
#[derive(Deserialize, Debug)]
struct SpotForecast {
    fcst: Vec<ForecastModel>,
}

#[derive(Deserialize, Debug)]
struct ForecastModel {
    id_model: u16,
    initstr: String,
}

pub async fn get_spot_forecast(spot_id: u32) {
    let mut headers = HeaderMap::new();
    headers.insert("Referer", HeaderValue::from_static("https://www.windguru.cz/map/spot"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let result: SpotForecast = client.get(format!("https://www.windguru.cz/int/iapi.php?q=forecast_spot&id_spot={spot_id}"))
        .send()
        .await
        .unwrap()
        .json()
        .await.unwrap();
    println!("{:?}", result)
}