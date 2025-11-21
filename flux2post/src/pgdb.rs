//use futures::prelude::*;
use futures_util::future;
use std::future::Future;
use tokio_postgres::{Client, Error, Statement, NoTls};
use crate::generic;
use crate::fluxquake;
use crate::fluxfire;

#[derive(Debug, Clone)]
pub struct Pgdb {
    pub dburl: String,
    pub dbport: u16,
    pub dbname: String,
    pub dbuser: String,
    pub dbpassword: String,
}


impl Pgdb {
    pub async fn check_connection(&self) -> Result<(), Box<dyn std::error::Error>> {
        //Result<postgres::Client, postgres::Error>
        //postgresql://[user[:password]@][netloc][:port][/dbname][?param1=value1&...]
        generic::logthis(format!("DB Check connection").as_str(), "INFO");

         let query = "SELECT title FROM category WHERE id = 6";
         let clt = (&self).connect_select(query).await;

         Ok(())
    }
    pub async fn insert_quake(&self, qu: fluxquake::Quake)  -> Result<(), Box<dyn std::error::Error>> {
        let query = format!("
            INSERT INTO quake (url, alert, code, magnitude, distance, geometry, depth, time)
            VALUES ('{0}', '{1}', '{2}', {3}, {4}, Point({5}, {6}), {7}, '{8}')
            ON CONFLICT (code)
            DO NOTHING;",
            qu.url, qu.alert, qu.code, qu.magnitude, qu.distance, qu.latitude, qu.longitude, qu.depth, qu.time
        );
            //println!("{:?}", qu);

             let res = (&self).connect_insert(query.as_str()).await;
            Ok(())
    }
    pub async fn insert_fire(&self, qu: fluxfire::Fire)  -> Result<(), Box<dyn std::error::Error>> {
        let query = format!("
            INSERT INTO fire (instrument, confidence, daynight, satellite, geometry, bright_ti4, frp, distance, typ, time)
            VALUES ('{0}', '{1}', '{2}', '{3}', Point({4}, {5}), {6}, {7}, {8}, '{9}', '{10}')
            ON CONFLICT (instrument, bright_ti4, time)
            DO NOTHING;",
            qu.instrument, qu.confidence, qu.daynight, qu.satellite, qu.latitude, qu.longitude, qu.bright_ti4, qu.frp, qu.distance, qu.typ, qu.time
        );
            //println!("{:?}", qu);

             let res = (&self).connect_insert(query.as_str()).await;
            Ok(())
    }

    pub async fn insert_event_test(&self) -> Result<(), Box<dyn std::error::Error>> {
        let query = format!("
            INSERT INTO event (id, title, description, link, category_id)
            VALUES ('plop', 'test event', 'description', 'my link', 6)
            ON CONFLICT (id)
            DO UPDATE SET title = 'plop', description = 'updated',
            category_id = 6, link = 'updated' WHERE event.id = 'plop';");

        let res = (&self).connect_insert(query.as_str()).await;

        Ok(())
    }
    async fn connect_select(&self, qu: &str) ->  Result<(), Box<dyn std::error::Error>> {
        let connect_string = format!("host={} port={} user={} password={} dbname={}",
            &self.dburl, &self.dbport, &self.dbuser, &self.dbpassword, &self.dbname);

        let (client, connection) =
            tokio_postgres::connect(
                connect_string.as_str(),
                    NoTls).await.unwrap();

        tokio::spawn(async move{
          if let Err(e) = connection.await {
            eprintln!("{:?}", e);
          }
        });


        let rows = client
           .query(qu, &[])
           .await?;
        let value: &str = rows[0].get(0);
        assert_eq!(value, "Drought");
        //println!("QUERY: [{:?}] {}", qu, value);
        Ok(())
    }



    async fn connect_insert(&self, qu: &str) -> Result<(), Box<dyn std::error::Error>> {
        let connect_string = format!("host={} port={} user={} password={} dbname={}",
            &self.dburl, &self.dbport, &self.dbuser, &self.dbpassword, &self.dbname);

        //println!("QUERY: [{:?}]", qu);

        let (client, connection) =
            tokio_postgres::connect(
                connect_string.as_str(),
                    NoTls).await.unwrap();
        //println!("QUERY: [{:?}]", qu);


        tokio::spawn(async move{
          if let Err(e) = connection.await {
            eprintln!("{:?}", e);
          }
        });

        client
            .query(qu, &[])
            .await;

        //println!("QUERY: [{:?}]", qu);
        Ok(())
    }
}
