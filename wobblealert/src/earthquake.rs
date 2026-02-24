use reqwest::Client;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
//use std::io::Write;
use std::f32;
use num_traits::pow::Pow;
use serde_json::{Result, Value};
use geoutils::Location;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use crate::influxdb;
use crate::settings;
use crate::generic;
use crate::pgdb;

const RESTURL: &str = "https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson";

#[derive(Deserialize, Debug)]
struct EventList {
    features: Vec<Feature>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Feature {
	//new: String,
	properties: Properties,
	geometry: Geometry,
	id: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Geometry {
	coordinates: [f64; 3],
}
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Properties {
   //#[serde(deserialize_with = "deserialize_null_default")]
   	mag: f32,
	place: Option<String>,
	time: i64,
	updated: i64,
	url: Option<String>,
	detail: Option<String>,
	felt: Option<i32>,
	cdi: Option<f32>,
	mmi: Option<f32>,
	alert: Option<String>,
	status: Option<String>,
	tsunami: i32,
	sig: i32,
	net: Option<String>,
	code: Option<String>,
	nst: i32,
	dmin: f32,
	rms: f32,
	gap: i32,
	magType: Option<String>,
	title: Option<String>,
}

pub async fn handle_call(stdt: String, endt: String, lg: f64, lt: f64, rd: i32, output_file: &str, cfg: settings::Settings) ->  Result<()> {
	//testing set

	let duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
	let timestamp_nanos = duration_since_epoch.as_nanos(); // u128
    let res = run_call(stdt, endt, lg, lt, rd, cfg.nasa.europebox).await;

	let mut update_event: String = "".to_string();

	let mut index: i32 = 0;

    let dbconn = pgdb::Pgdb{
       dburl: cfg.dbpg.dburl.clone(),
       dbport: cfg.dbpg.dbport.clone(),
       dbname: cfg.dbpg.dbname.clone(),
       dbuser: cfg.dbpg.dbuser.clone(),
       dbpassword: cfg.dbpg.dbpassword.clone(),
    };

    let mut quake_list: Vec<influxdb::Quake> = vec![];
    let mut report_list: Vec<influxdb::LastReport> = vec![];

	for el in &res.features {
		// Getting distance in km between the earthquake and my home
        // using haversine method, this is enough for this little pet project
		let quake_location = Location::new(el.geometry.coordinates[1], el.geometry.coordinates[0]);
		let home = Location::new(lt, lg);

        let depth = el.geometry.coordinates[2];

		let dist = (home.haversine_distance_to(&quake_location).meters()) / 1000.0;
        let d_hypo = ((dist.pow(2)  as f64) + depth).sqrt();
        let p_arrival = d_hypo / cfg.wavespeed.pwave;
        let s_arrival = d_hypo / cfg.wavespeed.swave;

		let nano_time = (el.properties.time)*1000000;
//        let dt_time = DateTime::from_timestamp(el.properties.time, 0);

        //let dt_nano_utc = (el.properties.time/1000000000) as u64;
        let test = el.properties.time / 1000;
        let d = UNIX_EPOCH + Duration::from_secs(test.try_into().unwrap());
	    let datetime = DateTime::<Utc>::from(d);
	    let timestamp_str = datetime.format("%v %H:%M").to_string();

        let message_to_send = format!("@{:?} Dist. {:.2}km, Mag {} [P: {:.2}s - S: {:.2}s] depth: {:.2}",
           timestamp_str, dist, el.properties.mag, p_arrival, s_arrival,el.geometry.coordinates[2]);

        //println!("timestamp_str: {:?} datetime: {:?} from el {:?}", timestamp_str, datetime, el.properties.time);

        pgdb::Pgdb::insert_quake(&dbconn,
            el.properties.url.clone().unwrap_or("".to_string()).clone(),
            el.properties.alert.clone().unwrap_or("green".to_string()).clone(),
	    	el.properties.code.clone().unwrap_or("".to_string()).clone(),
	    	(el.properties.mag as f64),
	    	dist,
            el.geometry.coordinates[1],
            el.geometry.coordinates[0],
            el.geometry.coordinates[2],
            timestamp_str,
            d_hypo,
            p_arrival,
            s_arrival
        ).await;

	    let qu: influxdb::Quake = influxdb::Quake {
            url: el.properties.url.clone().unwrap_or("".to_string()).clone(),
	    	alert: el.properties.alert.clone().unwrap_or("green".to_string()).clone(),
	    	code: el.properties.code.clone().unwrap_or("".to_string()).clone(),
	    	magnitude: (el.properties.mag as f64),
	    	distance: dist,
            dhypo: d_hypo,
            parrival: p_arrival,
            sarrival: s_arrival,
	    	longitude: el.geometry.coordinates[0],
	    	latitude: el.geometry.coordinates[1],
	    	depth: el.geometry.coordinates[2],
	    	time: nano_time as i64,
	    };
	    quake_list.push(qu);


        push_phone::push(cfg.alertzy.account.as_str(),
           cfg.alertzy.url.as_str(),
           message_to_send.as_str(),
           "Quake alert",
            "2")
           .await;

		index = index+1;
		update_event = el.properties.code.clone().unwrap_or("".to_string()).clone();
	}

	//println!("quake list len: {:?}", quake_list.len());
	if quake_list.len() > 0 {
		let iterator = (quake_list).iter().next().unwrap();
		let mut i3_output: String = "".to_string();

		let dt_nano_utc = (iterator.time/1000000000) as u64;
	    let d = UNIX_EPOCH + Duration::from_secs(dt_nano_utc);
	    let datetime = DateTime::<Utc>::from(d);
	    let timestamp_str = datetime.format("%v %H:%M").to_string();
    	let mut color = iterator.alert.clone();
		if iterator.magnitude < 4.0 {
			color = "green".to_string();
		}
		else if iterator.magnitude < 5.0 { color = "yellow".to_string(); }
		else if iterator.magnitude < 6.0 { color = "orange".to_string(); }
		else { color = "red".to_string(); }

		match color.as_str() {
			"orange" => { i3_output = format!(r#"<span background="{}">"#, cfg.color.orange); },
			"red" => { i3_output = format!(r#"<span background="{}">"#, cfg.color.red); },
			"yellow" => { i3_output = format!(r#"<span background="{}">"#, cfg.color.yellow); },
			_ => { i3_output = format!(r#"<span background="{}">"#, cfg.color.green); }
		}
		i3_output.push_str(
			format!(r#" [{}] M.{:.1} Dist.{:.2} </span>"#,
				timestamp_str,
				iterator.magnitude,
				iterator.distance).as_str());

	    std::fs::write(format!("{}", output_file), format!("{}", i3_output))
        .expect("Should be able to write to i3 config");

		// Lastly reporting to influxdb
        /*let rep: influxdb::LastReport = influxdb::LastReport {
			code: iterator.code.clone(),
			result: quake_list.len() as u64,
			time: timestamp_nanos as i64,
		};
		report_list.push(rep);
        */
        //let _ = influxdb::Influxdb::dump_report(&inflx.clone(), report_list).await;
	    generic::logthis(format!("Event recorded: M:{} D:{} @{}", iterator.magnitude, iterator.distance, timestamp_str).as_str(), "ALERT")

	}

	Ok(())
}

async fn run_call(stdt: String, endt: String, lg: f64, lt: f64, rd: i32, bx: Vec<i32>) -> EventList {
	//building query
    //let myparam = format!("starttime={}&endtime={}&latitude={}&longitude={}&maxradiuskm={}", stdt, endt, lt, lg, rd);

    let myparam = format!("starttime={}&endtime={}&minlatitude={}&minlongitude={}&maxlatitude={}&maxlongitude={}",
        stdt, endt, bx[0], bx[1], bx[2], bx[3]);

    let doge: Value = Client::new()
        .get(format!("{}&{}", RESTURL, myparam))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("failed to get response")
        .json()
        .await
        .expect("failed to get payload");

    //println!("Trying to get full Payload: {:#?}", doge); //:#?
	let it: EventList = serde_json::from_value(doge).unwrap();
    it
}
