use weather_scrapper::windguru;


#[tokio::main]
async fn main() {
    // 48722 Ria de Alvor
    windguru::get_spot_forecast(48772).await;
}
