#![allow(warnings)]
#[macro_use]
extern crate lazy_static;
use std::fs;
use chrono::prelude::*;
use chrono::{Duration, Utc};
mod generic;
mod settings;
mod earthquake;
mod influxdb;
mod firms;

lazy_static! {
    pub static ref CONFIG: settings::Settings =
    settings::Settings::new().expect("config can't be loaded ٩(＾◡＾)۶, you are doomed!!!");
}

#[tokio::main]
async fn main() {
    generic::logthis(format!("Good day ▼(´ᴥ`)▼ ").as_str(), "INFO");
    let influxdb = influxdb::Influxdb {
        dburl: CONFIG.db.dburl.clone(),
        //dbport: CONFIG.db.dbport.clone(),
        dbname: CONFIG.db.dbname.clone(),
        dbapi: CONFIG.db.dbapi.clone(),
        dborg: CONFIG.db.dborg.clone(),
    };

    let lg = CONFIG.location.longitude.clone();
    let lt = CONFIG.location.latitude.clone();
    let rd = CONFIG.location.radius.clone();
    let file = CONFIG.location.file.clone();

    let last_entry: DateTime<FixedOffset> = influxdb::Influxdb::check_connection(&influxdb).await;
    //println!("{:?}", last_entry.format("%Y-%m-%dT%H:%M:%S").to_string());
    let call_dbdate = last_entry + Duration::minutes(10);
    engage(file.as_str(), lg, lt, rd, call_dbdate.format("%Y-%m-%dT%H:%M:%S").to_string()).await.map_err(|err| println!("{:?}", err)).ok();

    // Get fire events
    let mapkey = CONFIG.nasa.mapkey.clone();
    let coordbox = CONFIG.nasa.coordbox.clone();
    // Introducing and testing fires alerts and detecting per nasa
    firms::handle_call(mapkey, coordbox, lg, lt, rd, CONFIG.clone())
        .await
        .map_err(|err| println!("{:?}", err)).ok();

        
}

async fn engage(myfile: &str, long: f64, lat: f64, rad: i32, stdate: String) -> Result<(), Box<dyn std::error::Error>> {
    //let stdate = (Utc::now()).format("%Y-%m-%dT%H:%M:00");
    //let stdate = DateTime::parse_from_rfc3339("2025-06-01T00:00:00").unwrap();
    let eddate = (Utc::now()).format("%Y-%m-%dT%H:%M:00");

    //println!("Engaging with {} to {}", stdate, eddate);
    generic::logthis(format!("Engaging for date: {} {}", stdate, eddate).as_str(), "INFO");

    earthquake::handle_call(stdate.to_string(), eddate.to_string(), long, lat, rad, myfile, CONFIG.clone())
        .await
        .map_err(|err| println!("{:?}", err)).ok();

    Ok(())
}


pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
