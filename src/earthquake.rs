use reqwest::Client;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::fs::{OpenOptions};
use std::io::Write;
use serde_json::{Result, Value};
use geoutils::Location;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::influxdb;
use crate::settings;

const RESTURL: &str = "https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson";

#[derive(Deserialize, Debug)]
struct EventList {
    features: Vec<Feature>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Feature {
	//new: String,	
	properties: Properties,
	geometry: Geometry,
	id: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Geometry {
	coordinates: [f64; 3],
}
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Properties {
   //#[serde(deserialize_with = "deserialize_null_default")]
   	mag: f32,
	place: String,
	time: i64,
	updated: i64,
	url: String,
	detail: String,
	felt: Option<i32>,
	cdi: Option<f32>,
	mmi: Option<f32>,
	alert: Option<String>,
	status: String,
	tsunami: i32,
	sig: i32,
	net: String,
	code: String,
	nst: i32,
	dmin: f32,
	rms: f32,
	gap: i32,
	magType: String,
	title: String,
}

pub async fn handle_call(stdt: String, endt: String, lg: f64, lt: f64, rd: i32, output_file: &str, cfg: settings::Settings) ->  Result<()> {
	println!("Entered async function in earthquake");
	//testing set
	//let dd = r#"{"type":"FeatureCollection","metadata":{"generated":1711715671000,"url":"https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson&starttime=2024-03-29T07:02:19&endtime=2024-03-29T10:22:19&latitude=37.57086135710454&longitude=22.80949188170508&maxradiuskm=320","title":"USGS Earthquakes","status":200,"api":"1.14.0","count":2},"features":[{"type":"Feature","properties":{"mag":5.8,"place":"30 km NW of Filiatrá, Greece","time":1711696367809,"updated":1711714445186,"tz":null,"url":"https://earthquake.usgs.gov/earthquakes/eventpage/us7000m8q9","detail":"https://earthquake.usgs.gov/fdsnws/event/1/query?eventid=us7000m8q9&format=geojson","felt":23,"cdi":7,"mmi":5.065,"alert":"green","status":"reviewed","tsunami":0,"sig":534,"net":"us","code":"7000m8q9","ids":",us7000m8q9,usauto7000m8q9,pt24089000,","sources":",us,usauto,pt,","types":",dyfi,internal-moment-tensor,internal-origin,losspager,moment-tensor,origin,phase-data,shakemap,","nst":127,"dmin":0.509,"rms":1.14,"gap":32,"magType":"mww","type":"earthquake","title":"M 5.8 - 30 km NW of Filiatrá, Greece"},"geometry":{"type":"Point","coordinates":[21.3156,37.3286,25.489]},"id":"us7000m8q9"},{"type":"Feature","properties":{"mag":4.1,"place":"29 km WNW of Filiatrá, Greece","time":1711696314868,"updated":1711699088040,"tz":null,"url":"https://earthquake.usgs.gov/earthquakes/eventpage/us7000m8qa","detail":"https://earthquake.usgs.gov/fdsnws/event/1/query?eventid=us7000m8qa&format=geojson","felt":null,"cdi":null,"mmi":null,"alert":null,"status":"reviewed","tsunami":0,"sig":259,"net":"us","code":"7000m8qa","ids":",us7000m8qa,","sources":",us,","types":",origin,phase-data,","nst":23,"dmin":0.526,"rms":1.04,"gap":205,"magType":"mb","type":"earthquake","title":"M 4.1 - 29 km WNW of Filiatrá, Greece"},"geometry":{"type":"Point","coordinates":[21.2747,37.2621,43.961]},"id":"us7000m8qa"}],"bbox":[21.2747,37.2621,25.489,21.3156,37.3286,43.961]}"#;
	//let dd = r#"{"type":"FeatureCollection","metadata":{"generated":1711715671000,"url":"https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson&starttime=2024-03-29T07:02:19&endtime=2024-03-29T10:22:19&latitude=37.57086135710454&longitude=22.80949188170508&maxradiuskm=320","title":"USGS Earthquakes","status":200,"api":"1.14.0","count":2},"features":[]}"#;
	//let res: EventList = serde_json::from_str(&dd).unwrap();

	let res = run_call(stdt, endt, lg, lt, rd).await;

	let mut update_event: String = "".to_string();
    let mut file_ref = OpenOptions::new()
        .write(true)
        .append(true)
    	.open(output_file)
    	.expect("Unable to open file");
	let mut index: i32 = 0;

    let inflx = influxdb::Influxdb {
        dburl: cfg.db.dburl,
        dbport: cfg.db.dbport,
        dbname: cfg.db.dbname,
        dborg: cfg.db.dborg,       
        dbapi: cfg.db.dbapi, 
    };

    let mut quake_list: Vec<influxdb::Quake> = vec![];
    let mut report_list: Vec<influxdb::LastReport> = vec![];

	for el in &res.features {
		// Getting distance in km between the earthquake and my home
		// using haversine method, this is enough for this little study
		let quake_location = Location::new(el.geometry.coordinates[1], el.geometry.coordinates[0]);
		let home = Location::new(lt, lg);
		let dist = (home.haversine_distance_to(&quake_location).meters()) / 1000.0;
		let nano_time = (el.properties.time)*1000000;

	    let qu: influxdb::Quake = influxdb::Quake {
	    	url: el.properties.url.clone(),
	    	alert: el.properties.alert.clone().unwrap_or("green".to_string()).clone(),
	    	code: el.properties.code.clone(),
	    	magnitude: (el.properties.mag as f64),
	    	distance: dist,
	    	longitude: el.geometry.coordinates[0],
	    	latitude: el.geometry.coordinates[1],
	    	depth: el.geometry.coordinates[2],
	    	time: nano_time as i64,
	    };
	    quake_list.push(qu);

		index = index+1;
		update_event = el.properties.code.clone();
	}

	//println!("quake list len: {:?}", quake_list.len());
	if quake_list.len() > 0 {
		let iterator = (quake_list).iter().next().unwrap();
		let mut i3_output: String = "".to_string();

		let dt_nano_utc = (iterator.time/1000000000) as u64;
	    let d = UNIX_EPOCH + Duration::from_secs(dt_nano_utc);
	    let datetime = DateTime::<Utc>::from(d);
	    let timestamp_str = datetime.format("%v %H:%M").to_string();
    	let color = iterator.alert.clone();

		match color.as_str() {
			"orange" => { i3_output = format!(r#"<span background="{}">"#, cfg.color.orange); },
			"red" => { i3_output = format!(r#"<span background="{}">"#, cfg.color.red); },
			"yellow" => { i3_output = format!(r#"<span background="{}">"#, cfg.color.yellow); },
			_ => { i3_output = format!(r#"<span background="{}">"#, cfg.color.green); }
		}
		i3_output.push_str(
			format!(r#" [{}] M.{:.1} Dist.{:.2} </span>"#, 
				timestamp_str, 
				iterator.magnitude, 
				iterator.distance).as_str());

		//println!("time: {:?} {:?}", dt_nano_utc, i3_output);	
	    std::fs::write(format!("{}i3", output_file), format!("{}", i3_output))
	    	.expect("Unable to write file");

		// Updating check file timestamp 
		file_ref.write_all(update_event.as_bytes()).expect("write failed");

		let duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
		let timestamp_nanos = duration_since_epoch.as_nanos(); // u128
		// Lastly reporting to influxdb
		let rep: influxdb::LastReport = influxdb::LastReport {
			code: iterator.code.clone(),
			result: quake_list.len() as u64,
			time: timestamp_nanos as i64,
		};
		report_list.push(rep);
		let _ = influxdb::Influxdb::dump_report(&inflx.clone(), report_list).await;
	}

	// pushing data to influxdb
	let res = influxdb::Influxdb::dump(&inflx.clone(), quake_list).await;
	println!("Pushing data... {:?}", res);


	Ok(())
}

async fn run_call(stdt: String, endt: String, lg: f64, lt: f64, rd: i32) -> EventList {
	//building query
	let myparam = format!("starttime={}&endtime={}&latitude={}&longitude={}&maxradiuskm={}",
		stdt, endt, lt, lg, rd);

	//println!("Entered async function run_call in earthquake [{}&{}]", RESTURL, myparam);

    let doge: Value = Client::new()
        .get(format!("{}&{}", RESTURL, myparam))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("failed to get response")
        .json()
        .await
        .expect("failed to get payload");

	//println!("Trying to get full Payload: {:#?}", doge); //:#?
	let it: EventList = serde_json::from_value(doge).unwrap();
    it
}
