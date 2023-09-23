use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
use std::f64;
use std::fs;
// use std::sync::mpsc::Receiver;
// use std::sync::mpsc::Sender;
// use std::sync::mpsc::{channel, RecvError};
// use std::thread;
use std::time::Instant;
use std::sync::mpsc::channel;
use rayon::prelude::*;
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

fn compute_spherical_d(cities: CityList) -> CityDistMatrix {
    let (tx, rx) = channel();

    // let l = cities.to_vec();
    // let copy = cities.to_vec();

    let mut mat: CityDistMatrix = CityDistMatrix::new();

    let num_cities = cities.len();

    (0..num_cities).into_par_iter().for_each( |i| {
        let lat1 = cities[i].1.lat;
        let phi1 = lat1 * std::f64::consts::PI / 180f64;
        let lng1 = cities[i].1.lng;
        let tht1 = lng1 * std::f64::consts::PI / 180f64;
        let earth_radius = 6371000f64;
        for city2 in 0..num_cities {
            let mut list = CityDistList::new();
            {
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
            tx.send(list).expect("Could not send data!");
        }
    });

    for _ in 0..num_cities {
        let list = rx.recv().unwrap();
        mat.push(list);
    }

    return mat;
}

fn main() -> Result<(), csv::Error> {
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();
    let reader = csv::Reader::from_path("./worldcities.csv");

    let mut cities: CityList = CityList::new();

    for record in reader?.deserialize() {
        let record: Record = record?;
        let city = CityPos {
            lat: record.lat,
            lng: record.lng,
        };
        cities.push((record.city_ascii, city));
        // if record.country == "France" {
        // }
    }

    let now = Instant::now();

    let mat = compute_spherical_d(cities);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let data = serde_json::to_string(&mat).unwrap();
    fs::write("./data/test.txt", data).expect("Unable to write file");
    // for list in mat {
    //     print!("Distance to : {}", list.0);
    //     for dist in list.1 {
    //         print!("{} : {}", dist.0, dist.1);
    //     }
    //     println!(" ")
    // }
    println!("Done!");

    Ok(())
}
