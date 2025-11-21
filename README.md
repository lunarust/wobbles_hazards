# Wobbles hazards

3 subprojects used to monitor natural disaters

1. wobblealert
   fetching earthquakes & fires from usgs.gov & firms.
   Storing data into influxDB2
   [![Rust](https://github.com/lunarust/wobbles_hazards/blob/main/wobblealert/.github/workflows/rust.yml/badge.svg?branch=main)](https://github.com/lunarust/wobbles_hazards/blob/main/wobblealert/.github/workflows/rust.yml)

2. hazevents
   fetching several type of disaters recorded from eonet/NASA
   Storing data into PostgreSQL
   [![Rust](https://github.com/lunarust/wobbles_hazards/blob/main/hazevents/.github/workflows/rust.yml/badge.svg?branch=main)](https://github.com/lunarust/wobbles_hazards/blob/main/hazevents/.github/workflows/rust.yml)

3. flux2post
   simple code used to copy the data stored in influxDB2 to PostgreSQL.
   [![Rust](https://github.com/lunarust/wobbles_hazards/blob/main/flux2post/.github/workflows/rust.yml/badge.svg?branch=main)](https://github.com/lunarust/wobbles_hazards/blob/main/flux2post/.github/workflows/rust.yml)
