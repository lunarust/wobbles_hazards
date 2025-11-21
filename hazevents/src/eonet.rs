use reqwest::Client;
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use chrono::{DateTime, Utc, NaiveDateTime};
use geoutils::Location;

use crate::settings;
use crate::generic;
use crate::pgdb;

const RESURL: &str = "https://eonet.gsfc.nasa.gov/api/v2.1/events";

#[derive(Deserialize, Debug)]
struct EonetList {
    events: Vec<Event>,
}
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub link: String,
    pub categories: Vec<Categories>,
    pub sources: Vec<Sources>,
    pub geometries: Vec<Geometries>,
    pub distance: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Categories {
    pub id: i32,
    pub title: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Sources {
    pub id: Option<String>,
    pub url: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Geometries {
    pub date: DateTime<Utc>,
    pub r#type: String,
    pub coordinates: Vec<f64>,
}


pub async fn handle_call(pgdb: pgdb::Pgdb, cfg: settings::Settings ) ->  Result<()> {
    generic::logthis(format!("EONET: Entering Handle Call").as_str(), "INFO");

    let home = Location::new(cfg.location.latitude, cfg.location.longitude);
    let radius = cfg.location.radius;

    let res = run_call().await;

    for mut el in res.events {
        let quake_location = Location::new(el.geometries[0].coordinates[0], el.geometries[0].coordinates[1]);
        let distance = (home.haversine_distance_to(&quake_location).meters()) / 1000.0;
        el.distance = Some(distance);
        //println!("{:?}  ", el);

        pgdb::Pgdb::insert_full_event(&pgdb, el).await
            .map_err(|err| println!("{:?}", err)).ok();

    }

    Ok(())
}

async fn run_call() -> EonetList {
    let myurl = format!("{}?days=1",
        RESURL);
    //status=open&&limit=1
    generic::logthis(format!("EONET: Executing API call [{}]", myurl).as_str(), "INFO");


    let doge: Value = Client::new()
        .get(myurl)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("failed to get response")
        .json()
        .await
        .expect("failed to get payload");

    //println!("Doge raw {:?}", doge);

    let it: EonetList = serde_json::from_value(doge).unwrap();
    it
}
