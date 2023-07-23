use weather_scrapper::{windguru, db};


#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error>{
    let windguru = windguru::Client::new();
    let db = db::Client::new().await?;

    for spot in &db.get_all_spots().await? {
        let forecast = windguru.get_forecast(spot.windguru_spot_id)
            .await
            .expect("Windguru should return forecast for the spot");

        for entry in &forecast.entries {
            db.upsert_forecast(spot.spot_id, entry).await?;
        }
    }
    
    Ok(())
}
