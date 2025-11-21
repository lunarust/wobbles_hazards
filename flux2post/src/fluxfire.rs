use futures::prelude::*;
use chrono::{DateTime, FixedOffset};
use chrono::prelude::*;
use influxdb2::models::Query;
use influxdb2::{Client, FromDataPoint};
use influxdb2_derive::WriteDataPoint;
use crate::pgdb;
use crate::generic;

#[derive(Debug, Clone)]
pub struct Fluxfire {
    pub dburl: String,
    pub dborg: String,
    pub dbname: String,
    pub dbapi: String,
}


#[derive(Debug, FromDataPoint, Clone)]
pub struct Fire {
   pub instrument: String,
   pub confidence: String,
   pub daynight: String,
   pub satellite: String,
   pub latitude: f64,
   pub longitude: f64,
   pub bright_ti4: f64,
   pub frp: f64,
   pub distance: f64,
   pub typ: String,
   pub time: DateTime<FixedOffset>,
}
impl Default for Fire {
    fn default() -> Self {
        Self {
            instrument: "".to_string(),
            confidence: "".to_string(),
            daynight: "".to_string(),
            satellite: "".to_string(),
            longitude: 0.0,
            latitude: 0.0,
            bright_ti4: 0.0,
            frp: 0.0,
            distance: 0.0,
            typ: "".to_string(),
            time: FixedOffset::west_opt(0).unwrap().with_ymd_and_hms(2024, 01, 01, 0, 0, 0).unwrap(),
        }
    }
}



impl Fluxfire {

    pub async fn get_fires(&self, pg: &pgdb::Pgdb) ->Result<(), Box<dyn std::error::Error>>  {

	    let host = &self.dburl;
	    let org =  &self.dborg;
	    let token = &self.dbapi;
	    let bucket = &self.dbname;
	    let client = Client::new(host, org, token);

        let qs = format!("from(bucket: \"{}\")
		          |> range(start: -14d)
                  |> group(columns: [\"_field\"])
                  |> sort(columns: [\"_time\"], desc: false)
                  ", bucket);

    	let query = Query::new(qs.to_string());

        let res = client.query::<Fire>(Some(query)).await.unwrap_or_default();
        generic::logthis(format!("FIRES: {:?}", res.len()).as_str(), "INFO");

        for q in res {
            pgdb::Pgdb::insert_fire(&pg, q).await;
        }
        Ok(())
	}
}
