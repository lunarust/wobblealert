# This project will query the latest earthquake alerts

> still learning... (⌒‿⌒)/

Querying the last earthquake from my location with a radius of 320km,
saving a smaller set of data in influxdb for grafana & in a small file for my i3Status bar.
Data are queried from https://earthquake.usgs.gov

# Tasks

## TODO
  - [ ] Clean 
  - [ ] Optimization


# Getting started

## Instal Rust & Cargo
Install Rust on your local machine, to do so please follow the official documentation

[Rust get started](https://www.rust-lang.org/learn/get-started)


## Get a local copy using git

```bash
git clone git@github.com:lunarust/wobblealert.git
```

## Start the application:

Copy ./src/config/Development.toml to Default.toml

```bash
cd src
cargo run
```

## Details:
### InfluxDB 

### Grafana

![Grafana Earthquake Dashboard](./img/Grafana_Dasboard.png)

*Note: Need to point to a influxdb datasource - Import not tested -*

[json Dashboard](./grafana/quakes.json)


### I3Status

![I3Status Module](./img/i3_alert_status_bar.png)

Configuration file:
```yaml
order += "read_file wobblealert"

read_file wobblealert {
        path = "~/.config/i3/alerti3"
        format = "%content"
        separator_block_width = 0
}
```

# Ref.
## USGS API DOC
https://earthquake.usgs.gov/fdsnws/event/1/


# MIT License



