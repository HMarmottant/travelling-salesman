use serde::Deserialize;
use std::collections::HashMap;
use std::f64;
#[derive(Deserialize)]
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

struct CityPos {
    lat: f64,
    lng: f64,
}

type CityList = HashMap<String, CityPos>;
type CityDistList = Vec<f64>;
type CityDistMatrix = Vec<CityDistList>;

fn compute_spherical_d(cities: CityList) -> CityDistMatrix {
    let earth_radius = 6371000f64;
    let mut mat: CityDistMatrix = CityDistMatrix::new();
    for city1 in &cities {
        let lat1 = city1.1.lat;
        let phi1 = lat1 * std::f64::consts::PI / 180f64;
        let lng1 = city1.1.lng;
        let tht1 = lng1 * std::f64::consts::PI / 180f64;

        let mut list = CityDistList::new();
        for city2 in &cities {
            let lat2 = city2.1.lat;
            let phi2 = lat2 * std::f64::consts::PI / 180f64;
            let lng2 = city2.1.lng;
            let tht2 = lng2 * std::f64::consts::PI / 180f64;

            let dphi = phi2 - phi1;
            let dtht = tht2 - tht1;

            let a = f64::powi(f64::sin(dphi / 2f64), 2)
                + f64::cos(phi1) * f64::cos(phi2) * f64::powi(f64::sin(dtht / 2f64), 2);
            let c = 2f64 * f64::atan2(f64::sqrt(a), f64::sqrt(1f64 - a));
            let d = earth_radius * c;
            list.push(d);
        }

        mat.push(list);
    }
    return mat;
}

fn main() -> Result<(), csv::Error> {
    let reader = csv::Reader::from_path("./worldcities.csv");

    let mut cities: CityList = CityList::new();

    for record in reader?.deserialize() {
        let record: Record = record?;
        let city = CityPos {
            lat: record.lat,
            lng: record.lng,
        };
        cities.insert(record.city_ascii, city);
        // if record.country == "France" {
        // }
    }

    use std::time::Instant;
    let now = Instant::now();

    let mat = compute_spherical_d(cities);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
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
