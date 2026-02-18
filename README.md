# Wobbles hazards

> All done mostly to learn & play with Rust... (⌒‿⌒)/

4 subprojects used to monitor natural disasters

1. [wobblealert](./wobblealert/README.md)
   - fetching earthquakes & fires from usgs.gov & firms.
   - Storing data in influxDB2
   - Adding Slipper-window metric predictor > per event arrival time aknowledgement
    - d_hypo (hypocentral distance 3D) = sqrt(d_surface² + depth²)
      calculate P-wave arrival time (d_hypo / Vp) & S-wave arrival time (d_hypo / Vs)
      Using average constante: Typical wave speeds
      In continental crust:
      P-wave speed ≈ 6 km/s
      S-wave speed ≈ 3.5 km/s

2. [hazevents](./hazevents/README.md)
   - fetching several type of disasters recorded from EONET/NASA
   - Storing data IN PostgreSQL [sql db file](/doc/wobbly_dump_001.sql)
     No automatic DB creation with this app for now.

3. [flux2post](./flux2post/README.md)
   - simple code used to copy the data stored in influxDB2 to PostgreSQL.

4. [push_phone](./push_phone/README.md)
   - Mini library to push notification on phone [alertzy](https://alertzy.app/)

  > [!NOTE]
  >  workflows Builds & tests all 3 subprojects:

  [![Rust](https://github.com/lunarust/wobbles_hazards/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/lunarust/wobbles_hazards/actions/workflows/rust.yml)


# Sample queries:

[sql](QUERIES.md)


# Grafana Dashboard Sample:

![Grafana Dashboard](/doc/Grafana_Dashboard_all.png)

[json Dashboard file](/doc/Dashboard_final.json)

# TODO / Could DO
 - [ ] Turn generic log into a library.
 - [ ] Run DB script on start if the schema doesn't exist.
 - [ ] Offer option to store the data into PostgreSQL in the settings for wobblealert.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
