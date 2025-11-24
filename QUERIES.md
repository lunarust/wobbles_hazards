## Grafana queries

Example of some queries used in the dashboard

### PostgreSQL

Query natural disasters push notification:

```sql
WITH ds AS (
		SELECT DISTINCT category_id, COALESCE(distance, 0.0) AS distance, COUNT(*) AS tot, MAX(magnitudevalue) AS magnitude
		FROM event evt
		JOIN geometry geo ON (event_id = evt.id)
		WHERE inserted BETWEEN '{0}' AND now()
		AND distance < {1}
		GROUP BY 1,2
		ORDER BY 1)
SELECT category_id as title, CONCAT(category_id, ' Dist: ', distance, 'km Mag: ', magnitude) as category_id, tot FROM ds
```


Query all natural events for Grafana:

```sql
WITH al AS (
SELECT 'fire' as category, count(*) AS evnt, time
FROM fire GROUP BY time, category
	UNION
SELECT 'quake' as category, count(*) AS evnt, time
FROM quake GROUP BY time, category
	UNION
SELECT evt.category_id AS category, count(evt.id) AS evnt, dt AS time
FROM event evt
JOIN geometry geo ON (event_id = evt.id) GROUP BY evt.category_id, dt
	)
SELECT category, evnt, time FROM al
WHERE time > $__timeFrom()  AND time < $__timeTo()
		AND category IS NOT NULL
```



### influxDB2

Fetch last entry:

```sql
from(bucket: "{}")
		|> range(start: 0, stop: now())
		|> group(columns: ["_field"])
		|> sort(columns: ["_time"], desc: false)
		|> last(column: "_stop")
```

Query all earthquakes:

```sql
from(bucket: "wobbly")
	|> range(start: v.timeRangeStart, stop: v.timeRangeStop)
	|> filter(fn: (r) => r["_measurement"] == "quake")
	|> pivot(rowKey: ["_time"], columnKey: ["_field"], valueColumn: "_value")
```
