# Wobbles hazards


> All done mostly to learn & play with Rust... (⌒‿⌒)/

3 subprojects used to monitor natural disaters

1. [wobblealert](./wobblealert/README.md)
   fetching earthquakes & fires from usgs.gov & firms.
   Storing data into influxDB2

2. [hazevents](./hazevents/README.md)
   fetching several type of disaters recorded from eonet/NASA
   Storing data into PostgreSQL

3. [flux2post](./flux2post/README.md)
   simple code used to copy the data stored in influxDB2 to PostgreSQL.



  _ workflows Builds & tests all 3 subprojects:
   [![Rust](https://github.com/lunarust/wobbles_hazards/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/lunarust/wobbles_hazards/actions/workflows/rust.yml)


# Grafana Dashboard Sample:

![Grafana Dashboard](/doc/Grafana_Dashboard_all.png)

[json Dashboard file](/doc/Dashboard_final.json)

# TODO
 - [ ] Cleanup
 - [ ] DB all queries and init script
 - [ ] wobblealert doesn't build yet on github, might be cache related.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
