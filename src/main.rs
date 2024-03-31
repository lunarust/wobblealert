#[macro_use]
extern crate lazy_static;
use std::fs;
use chrono::{Utc, Duration};
use chrono::prelude::*;

mod generic;
mod settings;
mod earthquake;
mod influxdb;

lazy_static! {
    pub static ref CONFIG: settings::Settings = 
    settings::Settings::new().expect("config can't be loaded ٩(＾◡＾)۶, you are doomed!!!");
}

#[tokio::main]
async fn main() {
    println!("Good day ▼(´ᴥ`)▼");

    let influxdb = influxdb::Influxdb {
        dburl: CONFIG.db.dburl.clone(),
        dbport: CONFIG.db.dbport.clone(),
        dbname: CONFIG.db.dbname.clone(),
        dbapi: CONFIG.db.dbapi.clone(), 
        dborg: CONFIG.db.dborg.clone(),         
    };

    let lg = CONFIG.location.longitude.clone();
    let lt = CONFIG.location.latitude.clone();
    let rd = CONFIG.location.radius.clone();
    let file = CONFIG.location.file.clone();            

    let last_entry: DateTime<FixedOffset> = influxdb::Influxdb::check_connection(&influxdb).await;
    println!("{:?}", last_entry.format("%Y-%m-%dT%H:%M:%S").to_string());

    engage(file.as_str(), lg, lt, rd, last_entry.format("%Y-%m-%dT%H:%M:%S").to_string()).await.map_err(|err| println!("{:?}", err)).ok();

}

async fn engage(myfile: &str, long: f64, lat: f64, rad: i32, stdate: String) -> Result<(), Box<dyn std::error::Error>> {
    //downlaoded a bigger set of data 
    //let mut stdate = (Utc::now() + Duration::minutes(-262800)).format("%Y-%m-%dT%H:%M:00");
    let eddate = (Utc::now()).format("%Y-%m-%dT%H:%M:00");

    println!("Engaging with {} to {}", stdate, eddate);

    earthquake::handle_call(stdate.to_string(), eddate.to_string(), long, lat, rad, myfile, CONFIG.clone())
        .await
        .map_err(|err| println!("{:?}", err)).ok();

    Ok(())
}

/*
fn file_modified_time_in_seconds(path: &str) -> u64 {
    fs::metadata(path)
    .unwrap()
    .modified()
    .unwrap()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs()
}
*/
pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
