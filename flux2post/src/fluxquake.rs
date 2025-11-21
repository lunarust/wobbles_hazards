use futures::prelude::*;
use chrono::{DateTime, FixedOffset};
use chrono::prelude::*;
use influxdb2::models::Query;
use influxdb2::{Client, FromDataPoint};
use influxdb2_derive::WriteDataPoint;
use crate::pgdb;
use crate::generic;

#[derive(Debug, Clone)]
pub struct Fluxquake {
    pub dburl: String,
    pub dborg: String,
    pub dbname: String,
    pub dbapi: String,
}

#[derive(Debug, FromDataPoint, Clone)]
pub struct Quake {
   pub url: String,
   pub alert: String,
   pub code: String,
   pub magnitude: f64,
   pub distance: f64,
   pub longitude: f64,
   pub latitude: f64,
   pub depth: f64,
   pub time: DateTime<FixedOffset>,
}

impl Default for Quake {
    fn default() -> Self {
        Self {
            url: "".to_string(),
            alert: "".to_string(),
            code: "".to_string(),
            magnitude: 0.0,
            distance: 0.0,
            longitude: 0.0,
            latitude: 0.0,
            depth: 0.0,
            time: FixedOffset::west_opt(0).unwrap().with_ymd_and_hms(2024, 01, 01, 0, 0, 0).unwrap(),
        }
    }
}


impl Fluxquake {

    pub async fn get_quakes(&self, pg: &pgdb::Pgdb) -> Result<(), Box<dyn std::error::Error>> {
        //println!("{:?}", &self);

	    let host = &self.dburl;
	    let org =  &self.dborg;
	    let token = &self.dbapi;
	    let bucket = &self.dbname;
	    let client = Client::new(host, org, token);

        let qs = format!("from(bucket: \"{}\")
		          |> range(start: -14d)
                  |> filter(fn: (r) => r[\"_measurement\"] == \"quake\")
                  |> group(columns: [\"_field\"])
                  |> sort(columns: [\"_time\"], desc: false)
                  ", bucket);
        // println!("qs:{}", qs);

    	let query = Query::new(qs.to_string());
        //println!("INFLUXDB >>> Query: {}", qs);

        let res = client.query::<Quake>(Some(query)).await.unwrap_or_default();
        generic::logthis(format!("QUAKES: {:?}", res.len()).as_str(), "INFO");

        for q in res {
            pgdb::Pgdb::insert_quake(&pg, q).await;
        }
        Ok(())
	}
}
