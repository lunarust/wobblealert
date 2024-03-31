//use std::time::{SystemTime};
use futures::prelude::*;
//use influxdb2::models::{DataPoint, Query};
use influxdb2::models::Query;
use influxdb2::{Client, FromDataPoint};
use influxdb2_derive::WriteDataPoint;
//use chrono::prelude::*;
//use crate::error;
use chrono::{DateTime, FixedOffset};
use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct Influxdb {
    pub dburl: String,
    pub dbport: u16,
    pub dborg: String,
    pub dbname: String,
    pub dbapi: String,
}

#[derive(Debug, Default, WriteDataPoint)]
#[measurement = "quake"]
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

#[derive(Debug, Default, WriteDataPoint)]
#[measurement = "ready"]
pub struct LastReport {
	#[influxdb(tag)]
	pub code: String,
	#[influxdb(field)]
	pub result: u64,		
    #[influxdb(timestamp)]
    pub time: i64,	
}

#[derive(Debug, FromDataPoint, Clone)]
pub struct LastEntry {
    time: DateTime<FixedOffset>,
    code: String,
    result: i64,
}
impl Default for LastEntry {
    fn default() -> Self {
        Self {
            code: "".to_string(),
            //time: chrono::MIN_DATETIME.with_timezone(&chrono::FixedOffset::east(7 * 3600)),
            time: FixedOffset::west_opt(0).unwrap().with_ymd_and_hms(2024, 01, 01, 0, 0, 0).unwrap(),
            result: 1,
        }
    }
}
impl Influxdb {

	pub async fn check_connection(&self) -> DateTime<FixedOffset> { //Result<(), Box<dyn std::error::Error>> {
	    let host = &self.dburl;
	    let org =  &self.dborg;
	    let token = &self.dbapi;
	    let bucket = &self.dbname;
	    let client = Client::new(host, org, token);

    let qs = format!("from(bucket: \"{}\")
        |> range(start: 0, stop: now())  
        |> filter(fn: (r) => r[\"_measurement\"] == \"ready\")
        |> filter(fn: (r) => r[\"_field\"] == \"result\")
		|> sort(columns: [\"_time\"], desc: false)
		|> last()      
		",
		bucket);

    	let query = Query::new(qs.to_string());

	    //let mut last_updated = LastEntry::default();
    	let res = client.query::<LastEntry>(Some(query)).
	    	await.unwrap_or_default(); //unwrap_or(LastEntry::default());
	    let mut iterator = LastEntry::default();

	    if res.len() == 0 {
	    	println!("Set is empty:: {:?}", res.len());	    	
	    }	
	    else {
	    	println!("I have an entry:: {:?}", res.len());
			iterator = (res).iter().last().unwrap().clone();
	    }

	    	println!("this one:: {:?}", res.len());
	    //self.candle_series.last().unwrap_or(&Candle::default())
//	    let mut iterator = (res).iter().last().unwrap();

	    iterator.time
		//let mut iterator = (res).iter().next().unwrap_or(&LastEntry::default());
		//let mut iterator = (res).iter().next().unwrap();
    	//println!("iterator.time {}", iterator.time);
	    //iterator.time

	    //chrono::MIN_DATETIME.with_timezone(&chrono::FixedOffset::east(7 * 3600))
	}
	pub async fn dump_report(&self, le: Vec<LastReport>) -> Result<(), Box<dyn std::error::Error>>  {

	    let host = &self.dburl;
	    let org =  &self.dborg;
	    let token = &self.dbapi;
	    let bucket = &self.dbname;
	    let client = Client::new(host, org, token);
	    //println!("{:?}", le);

	    client.write(bucket,  stream::iter(le)).await?;
	    
	    Ok(())	    
	}
	pub async fn dump(&self, qu: Vec<Quake>) -> Result<(), Box<dyn std::error::Error>>  {

	    let host = &self.dburl;
	    let org =  &self.dborg;
	    let token = &self.dbapi;
	    let bucket = &self.dbname;
	    let client = Client::new(host, org, token);
	    println!("{:?} > {:?}", qu.len(), qu);

	    client.write(bucket, stream::iter(qu)).await?;
	    
	    Ok(())	    
	}
}