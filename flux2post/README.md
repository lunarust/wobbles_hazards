# Flux to PostgreSQL

## Description

Needed to import all the data stored in InfluxDB2 to PostgreSQL, having one single entry DB makes it easier to build dashboards on Grafana with multiple layers on a world map.  

## Built with & Requirements

* Rust
* PostgreSQL
* InfluxDB2

## Getting started

### Install Rust & Cargo
Install Rust on your local machine| to do so please follow the official documentation

[Rust get started](https://www.rust-lang.org/learn/get-started)

### Get a local copy using git
```bash
git clone git@github.com:lunarust/flux2post.git
```

### Properties
Copy the file Development.toml to Default.toml and replace the values between angle brackets.

### Start the application:

```bash
cd src
cargo run
```

## Acknowledgments
