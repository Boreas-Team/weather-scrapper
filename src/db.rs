use tokio_postgres::NoTls;

use crate::windguru;


#[derive(Debug)]
pub struct Spot {
    pub spot_id: i32,
    pub name: String,
    pub windguru_spot_id: i32
}

impl Client {
    pub async fn new() -> Result<Client, tokio_postgres::Error> {
        let (client, connection) = 
            tokio_postgres::connect("host=localhost user=postgres password=dinnerlady dbname=boreas", NoTls).await?;

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                panic!("connection error: {}", e);
            }
        });

        Ok(Client {
            client
        })
    }

    pub async fn get_all_spots(&self) -> Result<Vec<Spot>, tokio_postgres::Error> {
        let rows = self.client.query("SELECT spot_id, name, windguru_spot_id FROM spots", &[])
            .await?;
    
        let mut spots = vec![];
        for row in &rows { 
            spots.push(Spot {
                spot_id: row.get(0),
                name: row.get(1),
                windguru_spot_id: row.get(2)
            });
        }
    
        Ok(spots)
    }

    pub async fn upsert_forecast(&self, spot_id: i32, entry: &windguru::ForecastEntry) -> Result<(), tokio_postgres::Error> {
        self.client.execute("INSERT INTO spot_forecast 
            VALUES ($1, $2, $3, $4) ON CONFLICT (spot_id, forecast_time) 
            DO UPDATE SET wind_speed = $3, wind_gusts = $4", &[
                &spot_id,
                &entry.time,
                &entry.wind_speed,
                &entry.wind_gusts
            ]
        ).await?;

        Ok(())
    }
}

pub struct Client {
    client: tokio_postgres::Client
}

