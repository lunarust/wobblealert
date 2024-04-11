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

[Influx API Doc](https://docs.influxdata.com/influxdb/v2/api/#operation/PostQuery)
Data stored as:
```rust
pub struct Quake {
  #[influxdb(tag)]
  pub url: String,
  #[influxdb(tag)]
  pub alert: String,
  #[influxdb(tag)]
  pub code: String,
  #[influxdb(field)]
  pub magnitude: f64,
  #[influxdb(field)]
  pub distance: f64,
  #[influxdb(field)]
  pub longitude: f64,
  #[influxdb(field)]
  pub latitude: f64,
  #[influxdb(field)]
  pub depth: f64,
    #[influxdb(timestamp)]
    pub time: i64,  
}
```

```bash
curl --request POST 'http://192.168.1.1:8086/api/v2/query?org=org.local' \
--header 'Content-Type: application/vnd.flux' \
--header 'Accept: application/json' \
--header "Authorization: Token APITOKEN" \
--data 'from(bucket: "wobbly") |> range(start: 0) |> filter(fn: (r) => r._measurement == "quake")'

```
|  _time  |  _value  |  _field  |  _measurement  |  alert  | code  |  url  |
|---------|----------|----------|----------------|---------|-------|-------|
|2023-10-13T03:33:14.082Z|10|depth|quake|green|6000lf9u|https://earthquake.usgs.gov/earthquakes/eventpage/us6000lf9u|
|2023-10-13T03:33:14.082Z|122.415017|distance|quake|green|6000lf9u|https://earthquake.usgs.gov/earthquakes/eventpage/us6000lf9u|
|2023-10-13T03:33:14.082Z|37.851|latitude|quake|green|6000lf9u|https://earthquake.usgs.gov/earthquakes/eventpage/us6000lf9u|
|2023-10-13T03:33:14.082Z|24.1553|longitude|quake|green|6000lf9u|https://earthquake.usgs.gov/earthquakes/eventpage/us6000lf9u|
|2023-10-13T03:33:14.082Z|4.40000009536743|magnitude|quake|green|6000lf9u|https://earthquake.usgs.gov/earthquakes/eventpage/us6000lf9u|
|2023-10-11T18:45:41.506Z|41.122|depth|quake|green|6000lg5z|https://earthquake.usgs.gov/earthquakes/eventpage/us6000lg5z|
|2023-10-11T18:45:41.506Z|211.05327|distance|quake|green|6000lg5z|https://earthquake.usgs.gov/earthquakes/eventpage/us6000lg5z|
|2023-10-11T18:45:41.506Z|39.1017|latitude|quake|green|6000lg5z|https://earthquake.usgs.gov/earthquakes/eventpage/us6000lg5z|
|2023-10-11T18:45:41.506Z|21.3788|longitude|quake|green|6000lg5z|https://earthquake.usgs.gov/earthquakes/eventpage/us6000lg5z|
|2023-10-11T18:45:41.506Z|4.30000019073486|magnitude|quake|green|6000lg5z|https://earthquake.usgs.gov/earthquakes/eventpage/us6000lg5z|
|

```influxql
from(bucket: "wobbly")
|>  range(start: 0, stop: now())  
|> filter(fn: (r) => r["_measurement"] == "ready")
|> filter(fn: (r) => r["_field"] == "result")
|> sort(columns: ["_time"], desc: false)
|> last()
```


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


### Zabbix & Telegram
/var/log/scripts/wobblealert

Create a Bot
https://core.telegram.org/bots/tutorial
BotFather will give you the ID upon creation.

Add your bot to a channel or group, your bot must have admin access.

Get your group id:
https://api.telegram.org/bot<TOKEN>/getUpdates


In Zabbix
> Data Collection > Templates
Create a new Template and Template group to easily identify your custom rules
Create an application and a trigger (with older version you need to create a graph as well)
Assign the template to your host.

> alert > media Type: 
Enable Telegram
Edit the entry and add your bot token

> Users > Users:
Create or edit an existing User, create an entry in tab Media for Telegram, with the ID of your channel.

> Alerts > Actions > Trigger action:
Create an action for your Telegram alert with all the required filter.


A default alert looks as follow:
```text
Problem: Wobble
Problem started at 11:54:26 on 2024.04.02
Problem name: Wobble
Host: Gumbys
Severity: Disaster
Operational data: 2024-04-02 10:27:01 [ALERT] M.7 D.156 
Original problem ID: 72518
```
still working on alerts / these are triggered but dont resolves themselves... yet
But you can configure the format of an alert either in alert > Media.


# Ref.
## USGS API DOC
https://earthquake.usgs.gov/fdsnws/event/1/


# MIT License





[![Rust](https://github.com/lunarust/wobblealert/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/lunarust/wobblealert/actions/workflows/rust.yml)