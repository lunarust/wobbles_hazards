# Wobble Alert

## Description

The application will query via API calls all recent natural events.

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
Copy the file one of the properties to create a Default.toml, replace the values between angle brackets.

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
|id|String|API|Unique id for this event.|
|title|String|API|The title of the event.|
|description|String|API|Optional longer description of the event. Most likely only a sentence or two.|
|link|String|API|The full link to the API endpoint for this specific event.|
|closed|DT|API|An event is deemed “closed” when it has ended. The closed field will include a date/time when the event has ended. Depending upon the nature of the event, the closed value may or may not accurately represent the absolute ending of the event. If the event is open, this will show “null”.|
|categories|String|API|One or more categories assigned to the event.|
|sources|Vec<String>|API|One or more sources that refer to more information about the event.|
|geometry|Vec<>|One or more event geometries are the pairing of a specific date/time with a location. The date/time will most likely be 00:00Z unless the source provided a particular time. The geometry will be a GeoJSON object of either type “Point” or “Polygon.”|
|magnitudeValue/Unit/Description|String|API|Information regarding the event magnitude, if available, is displayed here.|
|Distance|f64|Calculated|Distance to the location defined in the properties|




<details>
<summary>JSON Data Sample:</summary>

```json
{
    "id": "EONET_15983",
    "title": "Grey&#039;s Ditch Wildfire, Cameron, Louisiana",
    "description": "4 Miles N from Johnson Bayou, LA",
    "link": "https://eonet.gsfc.nasa.gov/api/v3/events/EONET_15983",
    "closed": null,
    "categories": [
        {
            "id": "wildfires",
            "title": "Wildfires"
        }
    ],
    "sources": [
        {
            "id": "IRWIN",
            "url": "https://irwin.doi.gov/observer/incidents/aa22960f-2a0b-43c0-b227-bbefcd7ed1ce"
        }
    ],
    "geometry": [
        {
            "magnitudeValue": 601.00,
            "magnitudeUnit": "acres",
            "date": "2025-11-17T16:54:00Z",
            "type": "Point",
            "coordinates": [
                -93.757778,
                29.838611
            ]
        }
    ]
}
```
</details>


## Acknowledgments
