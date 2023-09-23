use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
use std::f64;
use std::fs;
// use std::path;
// use std::sync::mpsc::Receiver;
// use std::sync::mpsc::Sender;
// use std::sync::mpsc::{channel, RecvError};
// use std::thread;
use rayon::prelude::*;
// use std::sync::mpsc::channel;
use std::time::Instant;
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

#[derive(Clone)]
struct CityPos {
    lat: f64,
    lng: f64,
}

type CityList = Vec<(String, CityPos)>;
type CityDistList = Vec<f64>;
type CityDistMatrix = Vec<CityDistList>;

fn compute_spherical_d(cities: &CityList) {
    // let (tx, rx) = channel();

    // let mut mat: CityDistMatrix = CityDistMatrix::new();

    let num_cities = cities.len();

    (0..num_cities).into_par_iter().for_each(|i| {
        let lat1 = cities[i].1.lat;
        let phi1 = lat1 * std::f64::consts::PI / 180f64;
        let lng1 = cities[i].1.lng;
        let tht1 = lng1 * std::f64::consts::PI / 180f64;
        let earth_radius = 6371000f64;
        let mut list = CityDistList::new();
        for city2 in 0..i {
            let lat2 = cities[city2].1.lat;
            let phi2 = lat2 * std::f64::consts::PI / 180f64;
            let lng2 = cities[city2].1.lng;
            let tht2 = lng2 * std::f64::consts::PI / 180f64;

            let dphi = phi2 - phi1;
            let dtht = tht2 - tht1;

            let a = f64::powi(f64::sin(dphi / 2f64), 2)
                + f64::cos(phi1) * f64::cos(phi2) * f64::powi(f64::sin(dtht / 2f64), 2);
            let c = 2f64 * f64::atan2(f64::sqrt(a), f64::sqrt(1f64 - a));
            let d = earth_radius * c;
            list.push(d);
        }
        let data = serde_json::to_string(&list).unwrap();
        let mut filepath: String = "./data/".to_owned();
        filepath += &i.to_string();
        filepath += &".dat";

        fs::write(filepath, data).expect("Unable to write file");

        // tx.send(list).expect("Could not send data!");
    });

    // for _ in 0..num_cities {
    //     let list = rx.recv().unwrap();
    //     mat.push(list);
    // }

    // return mat;
}

fn main() -> Result<(), csv::Error> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();
    let reader = csv::Reader::from_path("./worldcities.csv");

    let mut cities: CityList = CityList::new();
    let mut i = 0;

    for record in reader?.deserialize() {
        let record: Record = record?;
        if i < 10000 {
            let city = CityPos {
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

    compute_spherical_d(&cities);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
