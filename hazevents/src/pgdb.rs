//use futures::prelude::*;
use chrono::prelude::*;
use std::time::{SystemTime};
use chrono::{Duration, FixedOffset, TimeZone, Utc};
use futures_util::future;
use chrono::{DateTime, NaiveDateTime};
use tokio_postgres::types::{FromSql, Type};
use tokio_postgres::{NoTls, Error};
use serde_json::Value;

use crate::generic;
use crate::eonet;


#[derive(Debug, Clone)]
pub struct Pgdb {
    pub dburl: String,
    pub dbport: u16,
    pub dbname: String,
    pub dbuser: String,
    pub dbpassword: String,
}

impl Pgdb {
    pub async fn get_alert_events(&self, dt_start: DateTime<Utc>, distance: i32, acct: String, url: String) -> Result<(), Error> {

        let query = format!("WITH ds AS (
            SELECT DISTINCT category_id, COALESCE(distance, 0.0) AS distance, COUNT(*) AS tot, MAX(magnitudevalue) AS magnitude
            FROM event evt
            JOIN geometry geo ON (event_id = evt.id)
            WHERE inserted BETWEEN '{0}' AND now()
            AND distance < {1}
            GROUP BY 1,2
            ORDER BY 1)
            SELECT category_id as title, CONCAT(category_id, ' Dist: ', distance, 'km Mag: ', magnitude) as category_id, tot FROM ds
            ", dt_start, distance);
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
        let mut message: String = "".to_string();

        let rows = client
           .query(query.as_str(), &[])
           .await?;
           for row in rows {
               let tit: String = row.get("title");
               let cat: String = row.get("category_id");
               let tot: i64 = row.get("tot");
               generic::logthis(format!("Sending alert to phone {:?}", cat).as_str(), "INFO");

               push_phone::push(acct.as_str(), url.as_str(),
                   cat.as_str(),
                   tit.as_str()
                   , "2").await;
           //message.push_str(format!("{} {}", tot.to_string().as_str(), cat.as_str()).as_str());
           }
           //Ok(message.to_string())
           Ok(())
    }
    pub async fn get_last_record(&self) -> Result<DateTime<Utc>, Error> {
        let query = "SELECT date FROM eonet_calls ORDER BY date DESC LIMIT 1";
        let connect_string = format!("host={} port={} user={} password={} dbname={}",
            &self.dburl, &self.dbport, &self.dbuser, &self.dbpassword, &self.dbname);

        let today: DateTime<chrono::Utc> = SystemTime::now().clone().into();
        let mut st_date = today - Duration::days(90);

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
           .query(query, &[])
           .await?;
           for row in rows {
               let timestamp = row.get::<usize,SystemTime>(0);
               st_date = timestamp.clone().into();
           }
           Ok(st_date)
    }
    pub async fn check_connection(&self) -> Result<(), Box<dyn std::error::Error>> {
         generic::logthis(format!("DB Check connection").as_str(), "INFO");

        let query = "SELECT title FROM category WHERE id = 6";
        let clt = (&self).connect_select(query).await;

        Ok(())
    }
    pub async fn insert_full_event(&self, ev: eonet::Event) -> Result<(), Box<dyn std::error::Error>> {
        generic::logthis(format!("DB Insert or Update event").as_str(), "INFO");

        let event_id = ev.id.clone();

        (&self).insert_event(&ev).await;

        for ge in ev.geometry {
            (&self).insert_geo(&ge, &event_id).await;
        }
        // sources

        for se in ev.sources {
            //println!("New source {:?}", se);
            (&self).insert_sources(&se.id.unwrap(), &se.url.unwrap(), &event_id).await;
        }

        Ok(())
    }
    async fn insert_sources(&self, source_id: &String, source_url: &String, id: &String) -> Result<(), Box<dyn std::error::Error>> {
        //let client = (&self).connect_insert();

        let query = format!("
            INSERT INTO source (id, url)
            VALUES ('{0}', '{1}')
            ON CONFLICT (id)
            DO NOTHING;",
            source_id, source_url
        );

        let res = (&self).connect_insert(query.as_str()).await;

        let query_cross = format!("INSERT INTO event_source (source_id, event_id)
            VALUES('{0}', '{1}')
            ON CONFLICT (source_id, event_id)
            DO NOTHING;",
            source_id, id
        );

         let res = (&self).connect_insert(query_cross.as_str()).await;

         Ok(())
    }
    async fn insert_geo(&self, ge: &eonet::Geometry, id: &String) -> Result<(), Box<dyn std::error::Error>> {
        let query = format!("
            INSERT INTO geometry (dt, type, coordinates, event_id, magnitudevalue, magnitudeunit)
            VALUES ('{0}', '{1}', point({2}, {3}), '{4}', {5}, '{6}')
            ON CONFLICT (event_id, dt)
            DO UPDATE SET type = '{1}', coordinates = point({2}, {3}), magnitudevalue = {5}, magnitudeunit = {5}
            WHERE geometry.event_id = '{4}' AND geometry.dt = '{0}';",
            ge.date, ge.r#type, ge.coordinates[0], ge.coordinates[1], id,
            ge.magnitudeValue.unwrap_or(1.0), ge.magnitudeUnit.clone().unwrap_or("".to_string())
        );
        let res = (&self).connect_insert(query.as_str()).await;

        Ok(())
    }
    async fn insert_event(&self, ev: &eonet::Event) -> Result<(), Box<dyn std::error::Error>> {
        let today: DateTime<chrono::Utc> = SystemTime::now().clone().into();
        let query = format!("
            INSERT INTO event (id, title, description, link, category_id, closed, distance) VALUES ('{0}', '{1}', '{2}', '{3}', '{4}', '{5}', {6})
            ON CONFLICT (id)
            DO UPDATE SET title = '{1}', description = '{2}', category_id = '{4}', link = '{3}', distance = {6} WHERE event.id = '{0}';",
            &ev.id, &ev.title, &ev.description.clone().unwrap_or("".to_string()), &ev.link,
            &ev.categories[0].id.to_string(), &ev.closed.unwrap_or(today), &ev.distance.unwrap_or(0.0));

         let res = (&self).connect_insert(query.as_str()).await;

         Ok(())
    }
   pub async fn insert_call_log(&self) -> Result<(), Box<dyn std::error::Error>> {
        let query = format!("INSERT INTO public.eonet_calls(date, method) VALUES (now(), 'API');");
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

        if rows.len() > 0 {
            let value: &str = rows[0].get(0);
        }
        //assert_eq!(value, "Drought");

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
