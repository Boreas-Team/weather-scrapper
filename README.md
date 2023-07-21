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