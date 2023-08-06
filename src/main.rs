use std::sync::Arc;

use tokio::sync::Mutex;
use weather_scrapper::{db, windguru};

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error> {
    let db = Arc::new(Mutex::new(db::Client::new().await?));
    let spots = db.lock().await.get_all_spots().await?;

    let mut futures = vec![];

    for spot in &spots {
        let db = Arc::clone(&db);

        let windguru_spot_id = spot.windguru_spot_id;
        let spot_id = spot.spot_id;
        let name = spot.name.clone();

        futures.push(tokio::spawn(async move {
            let windguru = windguru::Client::new();

            let forecast = windguru
                .get_forecast(windguru_spot_id)
                .await
                .expect("Windguru should return forecast for the spot");
            println!("Downloaded forecast for {spot_id}: {name}");

            for entry in &forecast.entries {
                db.lock()
                    .await
                    .upsert_forecast(spot_id, entry)
                    .await
                    .expect("should upsert properly in the db")
            }

            println!("Upserted forecast for {spot_id}: {name}");
        }))
    }

    for future in futures {
        let _ = future.await;
    }

    Ok(())
}
