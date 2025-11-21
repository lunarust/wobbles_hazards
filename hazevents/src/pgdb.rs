//use futures::prelude::*;

use futures_util::future;
use std::future::Future;
use tokio_postgres::{Client, Error, Statement, NoTls};
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
    pub async fn check_connection(&self) -> Result<(), Box<dyn std::error::Error>> {
        //Result<postgres::Client, postgres::Error>
        //postgresql://[user[:password]@][netloc][:port][/dbname][?param1=value1&...]
        generic::logthis(format!("DB Check connection").as_str(), "INFO");


         let query = "SELECT title FROM category WHERE id = 6";
         let clt = (&self).connect_select(query).await;


         Ok(())
    }


    pub async fn insert_full_event(&self, ev: eonet::Event) -> Result<(), Box<dyn std::error::Error>> {
        generic::logthis(format!("DB Insert or Update event").as_str(), "INFO");

        let event_id = ev.id.clone();

        (&self).insert_event(&ev).await;

        for ge in ev.geometries {
            //println!("New geo {:?}", ge);
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
    async fn insert_geo(&self, ge: &eonet::Geometries, id: &String) -> Result<(), Box<dyn std::error::Error>> {
        let query = format!("
            INSERT INTO geometry (dt, type, coordinates, event_id)
            VALUES ('{0}', '{1}', point({2}, {3}), '{4}')
            ON CONFLICT (event_id, dt)
            DO UPDATE SET type = '{1}', coordinates = point({2}, {3})
            WHERE geometry.event_id = '{4}' AND geometry.dt = '{0}';",
            ge.date, ge.r#type, ge.coordinates[0], ge.coordinates[1], id
        );
        let res = (&self).connect_insert(query.as_str()).await;

        Ok(())
    }
    async fn insert_event(&self, ev: &eonet::Event) -> Result<(), Box<dyn std::error::Error>> {
        let query = format!("
            INSERT INTO event (id, title, description, link, category_id) VALUES ('{0}', '{1}', '{2}', '{3}', {4})
            ON CONFLICT (id)
            DO UPDATE SET title = '{1}', description = '{2}', category_id = {4}, link = '{3}' WHERE event.id = '{0}';",
            &ev.id, &ev.title, &ev.description.clone().unwrap(), &ev.link, &ev.categories[0].id.to_string());

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
