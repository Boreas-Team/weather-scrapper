use weather_scrapper::windguru;


#[tokio::main]
async fn main() {
    // 48968 Ria de Alvor
    let windguru = windguru::WindguruClient::new();
    println!("{:?}", windguru.get_forecast(48968).await.unwrap());
}
