#![allow(warnings)]
#[macro_use]
extern crate lazy_static;
use std::fs;
use chrono::prelude::*;
use chrono::{Duration, Utc};
mod generic;
mod settings;
mod pgdb;
mod fluxquake;
mod fluxfire;

lazy_static! {
    pub static ref CONFIG: settings::Settings =
    settings::Settings::new().expect("config can't be loaded ٩(＾◡＾)۶, you are doomed!!!");
}
#[tokio::main]
async fn main() {
   generic::logthis(format!("Good day ▼(´ᴥ`)▼ ").as_str(), "INFO");

   let pgdb = pgdb::Pgdb {
       dburl: CONFIG.dbpg.dburl.clone(),
       dbport: CONFIG.dbpg.dbport.clone(),
       dbname: CONFIG.dbpg.dbname.clone(),
       dbuser: CONFIG.dbpg.dbuser.clone(),
       dbpassword: CONFIG.dbpg.dbpassword.clone(),
   };

    let fluxquake = fluxquake::Fluxquake {
        dburl: CONFIG.dbflux.dburl.clone(),
        dbname: CONFIG.dbflux.dbname.clone(),
        dbapi: CONFIG.dbflux.dbapi.clone(),
        dborg: CONFIG.dbflux.dborg.clone(),
    };
    let fluxfire = fluxfire::Fluxfire {
        dburl: CONFIG.dbflux.dburl.clone(),
        dbname: "hotties".to_string(),
        dbapi: CONFIG.dbflux.dbapi.clone(),
        dborg: CONFIG.dbflux.dborg.clone(),
    };
    //println!("{:?}", fluxquake);

    fluxquake::Fluxquake::get_quakes(&fluxquake, &pgdb).await;

    fluxfire::Fluxfire::get_fires(&fluxfire, &pgdb).await;

    //println!("{:?}", entries);
}
