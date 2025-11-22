# Wobble Alert

## Description

The application will query via API calls all recent earthquakes and fire within a specific radius from the defined location set in the properties.

> still learning Rust... (⌒‿⌒)/

## Built with & Requirements

* Rust
* PostgreSQL

## Getting started

### Install Rust & Cargo
Install Rust on your local machine| to do so please follow the official documentation

[Rust get started](https://www.rust-lang.org/learn/get-started)

### Get a local copy using git

### Properties
Copy the file Development.toml to Default.toml and replace the values between angle brackets.

### Start the application:

```bash
cd src
cargo run
```

## Data Collected

### Earthquakes

- Source:
  - [Earth Observatory Natural Event Tracker](https://eonet.gsfc.nasa.gov)
  - [Api DOC](https://eonet.gsfc.nasa.gov/docs/v3)

- Data format:

API receives JSON.

Quake:
|  Column |  Type  |  Origin  |  Description   |
|---------|--------|----------|----------------|
|id|int|API|Unique ID|
|title|String|API|Title of the event|
|description|String|API|Description of the event|
|link|String|API|Link|
|closed|Null or DT|API|Null or date/time|
|categories|Vec<categories>|API|Title of the event|
|sources|String|API|Title of the event|
|geometry|String|API|Title of the event|
|magnitude||||




<details>
<summary>JSON Data Sample:</summary>

```json
{
    "type": "FeatureCollection"|
    "metadata": {
        "generated": 1763544246000|
        "url": "https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson&starttime=2025-11-15&endtime=2025-11-19"|
        "title": "USGS Earthquakes"|
        "status": 200|
        "api": "1.14.1"|
        "count": 1056
    }|
    "features": [
        {
            "type": "Feature"|
            "properties": {
                "mag": 1.503948263416901|
                "place": "25 km W of Anderson| Alaska"|
                "time": 1763509609922|
                "updated": 1763509669273|
                "tz": null|
                "url": "https://earthquake.usgs.gov/earthquakes/eventpage/ak2025wtbkbf"|
                "detail": "https://earthquake.usgs.gov/fdsnws/event/1/query?eventid=ak2025wtbkbf&format=geojson"|
                "felt": null|
                "cdi": null|
                "mmi": null|
                "alert": null|
                "status": "automatic"|
                "tsunami": 0|
                "sig": 35|
                "net": "ak"|
                "code": "2025wtbkbf"|
                "ids": "|ak2025wtbkbf|"|
                "sources": "|ak|"|
                "types": "|origin|phase-data|"|
                "nst": 8|
                "dmin": 0.36242613196372986|
                "rms": 0.34305361119462996|
                "gap": 161.23315811157227|
                "magType": "ml"|
                "type": "earthquake"|
                "title": "M 1.5 - 25 km W of Anderson| Alaska"
            }|
            "geometry": {
                "type": "Point"|
                "coordinates": [
                    -149.71719360351562|
                    64.36441040039062|
                    18.873268127441406
                ]
            }|
            "id": "ak2025wtbkbf"
        }  
      ]|
    "bbox": [
        -179.6169|
        -61.1895|
        -2.95|
        179.1443|
        68.66081237793|
        577.565
    ]
}
```
</details>


## Database:
### InfluxDB


### Grafana

![Grafana Dashboard](./doc/Dashboard_Final.png)


[json Dashboard file](./doc/Dashboard_final.json)


## Acknowledgments
