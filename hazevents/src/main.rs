#![allow(warnings)]
#[macro_use]
extern crate lazy_static;
use std::fs;
use chrono::prelude::*;
use chrono::{Duration, Utc};
mod generic;
mod settings;
mod eonet;
mod pgdb;

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

  pgdb::Pgdb::check_connection(&pgdb).await
       .map_err(|err| println!("{:?}", err)).ok();

  let start_date = pgdb::Pgdb::get_last_record(&pgdb).await.unwrap();

   generic::logthis(format!("Main calling next EONET Starting from {:?}", start_date).as_str(), "INFO");

   eonet::handle_call(&pgdb, CONFIG.clone(), start_date.clone()).await
        .map_err(|err| println!("{:?}", err)).ok();

   // Check if I have to call mama
   let message_to_send = pgdb::Pgdb::get_alert_events(&pgdb, start_date, CONFIG.location.radius, CONFIG.clone().alertzy.account, CONFIG.clone().alertzy.url,).await
        .map_err(|err| println!("{:?}", err)).ok();


}
