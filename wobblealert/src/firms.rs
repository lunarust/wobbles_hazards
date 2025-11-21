use reqwest::Client;
//use serde_json::{Result, Value};
use csv::Error;

use std::time::{SystemTime, UNIX_EPOCH,Duration};
use chrono::{Utc,NaiveDateTime};

use std::fs;
use serde::{Deserialize, Serialize};
//use serde_json::error;
//use serde_xml_rs::from_str;
use crate::generic;
use crate::influxdb;
use crate::settings;
use geoutils::Location;


const RESTURL: &str = "https://firms.modaps.eosdis.nasa.gov/api";
const SAT: &str = "MODIS_SP";

pub async fn handle_call(mapkey: String, coordbox: String, lg: f64, lt: f64, rd: i32, cfg: settings::Settings) -> Result<(), csv::Error> {
    let mut ct = 0;

	//  building query
    //area_url = 'https://firms.modaps.eosdis.nasa.gov/api/area/csv/' + MAP_KEY + '/VIIRS_NOAA20_NRT/world/1'
    //https://firms.modaps.eosdis.nasa.gov/api/area/csv/322e359dab64a838314a1e1bed6a5f5e/VIIRS_SNPP_NRT/-50,0,50,50/1/2025-11-18
    let qudt = (Utc::now()).format("%Y-%m-%d");

    let inflx = influxdb::Influxdb {
        dburl: cfg.db.dburl,
   //     dbport: cfg.db.dbport,
        dbname: cfg.db.dbname,
        dborg: cfg.db.dborg,
        dbapi: cfg.db.dbapi,
    };
    let myparam = format!("{}/area/csv/{}/{}/{}/1/{}", RESTURL, mapkey, SAT, coordbox, qudt);
    //let myparam = format!("{}/area/csv/{}/{}/{}/7/{}", RESTURL, mapkey, SAT, "-180,-80,180,80", "2025-11-07");

    //println!("FIRMS: {}", myparam);
    let client = Client::new();


    let doge = client
            .get(myparam)
            .timeout(Duration::from_secs(180))
            .send()
            .await
            .expect("failed to get response")
            .text()
            .await
            .expect("failed to get payload");

    let home = Location::new(lt, lg);

    //let mut fds = Vec::new();
    let mut fire_list: Vec<influxdb::Fire> = vec![];
    let mut reader = csv::Reader::from_reader(doge.as_bytes());
    for record in reader.records() {
        let record = record?;

        let quake_location = Location::new(record[0].parse::<f64>().unwrap(), record[1].parse::<f64>().unwrap());
        let disthome = (home.haversine_distance_to(&quake_location).meters()) / 1000.0;

        //let duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        //let timestamp_nanos = duration_since_epoch.as_nanos(); // u128

        if disthome <= rd.to_string().parse::<f64>().unwrap() {
                //"2025-06-29 00:00:00",
            let bdtest = format!("{} 00:00:00", record[5].to_string());
            let test = NaiveDateTime::parse_from_str(
                &bdtest,
                "%Y-%m-%d %H:%M:%S");
            let test_unix = NaiveDateTime::timestamp(&test.unwrap())*1000000000;
            //println!("Dist {} - dt {:?} >> {}", disthome, bdtest, test_unix);

            let fi: influxdb::Fire = influxdb::Fire {
            latitude: record[0].parse::<f64>().unwrap(),
            longitude: record[1].parse::<f64>().unwrap(),
            bright_ti4: record[2].parse::<f64>().unwrap(),
            satellite: record[7].to_string(),
            instrument: record[8].to_string(),
            confidence: record[9].to_string(),
            frp: record[12].parse::<f64>().unwrap(),
            daynight: record[13].to_string(),
            distance: disthome.to_string().parse::<f64>().unwrap(),
            typ: record[14].to_string(),
            time: test_unix as i64,
        };
        ct +=1;
        fire_list.push(fi);
      }
    }
    generic::logthis(format!("Events recorded: {:?}", ct).as_str(), "INFO");


    //println!("FIRMS: {} event", ct);
    let _res = influxdb::Influxdb::dump_fire(&inflx.clone(), fire_list).await;

    Ok(())
}
