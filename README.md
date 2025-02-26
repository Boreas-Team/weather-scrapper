# weather-scrapper

Database schema:

`spots`
`id | name | latitude | longitude | iko_id | windguru_spot_id`

`spot_forecasts`
`spot_id | forecast_time | wind | wind_gusts`
- forecast_time -> UTC time of the predicted weather (e.g 15knots at 15:00 UTC time)
- wind and wind_gusts in knots (float)

Algorithm `load_forecast(spot_id)`:

1. Fetch spots from the database `spots`
2. For each spot download wind forecast (windguru GFS 13km)
3. Clear current forecast for the spot
3. Save forecast in `spot_forecast`


```
[
    {
        "time": "123123", // UTC 
        "wind": 34, // knots
        "gusts": 35 // gusts
    }
]
```

Local Development:
```
docker run --name boreas-db -p 5432:5432 -e POSTGRES_PASSWORD=dinnerlady -d postgres
# db=postgres password=dinnerlady host=127.0.0.1:5432

brew install libpq
echo 'export PATH="/usr/local/opt/libpq/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

psql -h localhost -U postgres
```

```sql
CREATE TABLE spots (
    spot_id             integer                  NOT NULL,
    name                varchar(100)             NOT NULL,
    windguru_spot_id    integer                  NOT NULL
)

INSERT INTO spots VALUES (7, 'Lagos', 48968);

CREATE TABLE spot_forecast (
    spot_id integer NOT NULL,
    forecast_time timestamptz NOT NULL,
    wind_speed float(8) NOT NULL,
    wind_gusts float(8) NOT NULL,
    PRIMARY KEY (spot_id, forecast_time)
);
```