use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
use std::f64;

// use std::path;
// use std::sync::mpsc::Receiver;
// use std::sync::mpsc::Sender;
// use std::sync::mpsc::{channel, RecvError};
// use std::thread;

// use std::sync::mpsc::channel;
use std::time::Instant;

mod compute_spherical_d;
mod CityUtil;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    city: String,
    city_ascii: String,
    lat: f64,
    lng: f64,
    country: String,
    iso2: String,
    iso3: String,
    admin_name: String,
    capital: String,
    population: String,
    id: String,
}

fn main() -> Result<(), csv::Error> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap();
    let reader = csv::Reader::from_path("./worldcities.csv");

    let mut cities: CityUtil::CityList = CityUtil::CityList::new();
    let mut i = 0;

    for record in reader?.deserialize() {
        let record: Record = record?;
        if true {
            let city = CityUtil::CityPos {
                lat: record.lat,
                lng: record.lng,
            };
            cities.push((record.city_ascii, city));
        } else {
            break;
        }
        i += 1;
    }

    let now = Instant::now();
    
    compute_spherical_d::compute(&cities);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
