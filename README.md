# Wobbles hazards

3 subprojects used to monitor natural disaters

1. wobblealert
   fetching earthquakes & fires from usgs.gov & firms.
   Storing data into influxDB2

2. hazevents
   fetching several type of disaters recorded from eonet/NASA
   Storing data into PostgreSQL
   [![Rust](https://github.com/lunarust/wobbles_hazards/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/lunarust/wobbles_hazards/actions/workflows/rust.yml)


3. flux2post
   simple code used to copy the data stored in influxDB2 to PostgreSQL.



## Grafana queries
```sql
with al as (
select 'fire' as category, count(*) as evnt, time
from fire group by time, category
	union
select 'quake' as category, count(*) as evnt, time
from quake group by time, category
	union
select cat.title as category, count(evt.id) as evnt, dt as time
	from event evt
	join geometry geo ON (event_id = evt.id)
	join category cat ON (evt.category_id=cat.id) group by cat.title, dt
	)
	select category, evnt, time from al
  where time > $__timeFrom()  and time < $__timeTo()

  ```


  select cat.title as category,
count(evt.id) as evnt,
dt as time
from event evt
left join geometry geo ON (event_id = evt.id)
left join event_source es ON (es.event_id=evt.id)
left join source s ON (es.source_id=s.id)
left join category cat ON (evt.category_id=cat.id)
where dt > $__timeFrom()  and dt < $__timeTo()
group by cat.title, dt
order by 1
