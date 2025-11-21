use futures::prelude::*;
use chrono::{DateTime, FixedOffset};
use chrono::prelude::*;
use influxdb2::models::Query;
use influxdb2::{Client, FromDataPoint};
use influxdb2_derive::WriteDataPoint;

#[derive(Debug, Clone)]
pub struct Influxdb {
    pub dburl: String,
//   pub dbport: u16,
    pub dborg: String,
    pub dbname: String,
    pub dbapi: String,
}

#[derive(Debug, Default, WriteDataPoint)]
#[measurement = "quake"]
pub struct Quake {
	#[influxdb(tag)]
	pub url: String,
	#[influxdb(tag)]
	pub alert: String,
	#[influxdb(tag)]
	pub code: String,
	#[influxdb(field)]
	pub magnitude: f64,
	#[influxdb(field)]
	pub distance: f64,
	#[influxdb(field)]
	pub longitude: f64,
	#[influxdb(field)]
	pub latitude: f64,
	#[influxdb(field)]
	pub depth: f64,
    #[influxdb(timestamp)]
    pub time: i64,
}

#[derive(Debug, Default, WriteDataPoint)]
#[measurement = "ready"]
pub struct LastReport {
	#[influxdb(tag)]
	pub code: String,
	#[influxdb(field)]
	pub result: u64,
    #[influxdb(timestamp)]
    pub time: i64,
}

#[derive(Debug, FromDataPoint, Clone)]
pub struct LastEntry {
    time: DateTime<FixedOffset>,
    stop: DateTime<FixedOffset>,
    code: String,
    result: i64,
}
impl Default for LastEntry {
    fn default() -> Self {
        Self {
            code: "".to_string(),
            time: FixedOffset::west_opt(0).unwrap().with_ymd_and_hms(2024, 01, 01, 0, 0, 0).unwrap(),
            stop: FixedOffset::west_opt(0).unwrap().with_ymd_and_hms(2025, 01, 01, 0, 0, 0).unwrap(),
            result: 1,
        }
    }
}


#[derive(Debug, Default, WriteDataPoint)]
#[measurement = "fire"]
pub struct Fire {
    #[influxdb(tag)]
    pub instrument: String,
    #[influxdb(field)]
    pub confidence: String,
    #[influxdb(field)]
    pub daynight: String,
    #[influxdb(field)]
    pub satellite: String,
	#[influxdb(field)]
    pub latitude: f64,
	#[influxdb(field)]
    pub longitude: f64,
	#[influxdb(field)]
    pub bright_ti4: f64,
	#[influxdb(field)]
    pub frp: f64,
	#[influxdb(field)]
    pub distance: f64,
    #[influxdb(field)]
    pub typ: String,
    #[influxdb(timestamp)]
    pub time: i64,
}


impl Influxdb {

    pub async fn check_connection(&self) -> DateTime<FixedOffset> {
    //DateTime<FixedOffset> { //Result<(), Box<dyn std::error::Error>> {
	    let host = &self.dburl;
	    let org =  &self.dborg;
	    let token = &self.dbapi;
	    let bucket = &self.dbname;
	    let client = Client::new(host, org, token);

        let qs = format!("from(bucket: \"{}\")
		          |> range(start: 0, stop: now())
                  |> group(columns: [\"_field\"])
                  |> sort(columns: [\"_time\"], desc: false)
                  |> last(column: \"_stop\")", bucket);


    	let query = Query::new(qs.to_string());

        //println!("INFLUXDB >>> Query: {}", qs);
        let res = client.query::<LastEntry>(Some(query)).await.unwrap_or_default();
        let mut iterator = LastEntry::default();
        if res.len() != 0 {

            iterator = (res).iter().next().unwrap().clone();
            //println!("INFLUXDB >>> {:?}, {}", res, iterator.time);
        }
        iterator.time
	}
	pub async fn dump_report(&self, le: Vec<LastReport>) -> Result<(), Box<dyn std::error::Error>>  {

	    let host = &self.dburl;
	    let org =  &self.dborg;
	    let token = &self.dbapi;
	    let bucket = &self.dbname;
	    let client = Client::new(host, org, token);
	    //println!("{:?}", le);
	    println!("{:?}", le);

	    client.write(bucket,  stream::iter(le)).await?;

	    Ok(())
	}
	pub async fn dump(&self, qu: Vec<Quake>) -> Result<(), Box<dyn std::error::Error>>  {

	    let host = &self.dburl;
	    let org =  &self.dborg;
	    let token = &self.dbapi;
	    let bucket = &self.dbname;
	    let client = Client::new(host, org, token);
	    //println!("{:?} > {:?}", qu.len(), qu);

	    client.write(bucket, stream::iter(qu)).await?;

	    Ok(())
	}
    pub async fn dump_fire(&self, qu: Vec<Fire>) ->  Result<(), Box<dyn std::error::Error>> {
        let host = &self.dburl;
        let org = &self.dborg;
        let token = &self.dbapi;
        let bucket = "hotties";
        let client = Client::new(host, org, token);
        println!("{:?}", qu);

        client.write(bucket, stream::iter(qu)).await?;
	    Ok(())

    }
}
