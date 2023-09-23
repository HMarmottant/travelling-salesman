use rayon::prelude::*;
use std::fs;

use crate::CityUtil;

pub fn compute(cities: &CityUtil::CityList) {
    // let (tx, rx) = channel();

    // let mut mat: CityDistMatrix = CityDistMatrix::new();

    let num_cities = cities.len();

    (0..num_cities).into_par_iter().for_each(|i| {
        let lat1 = cities[i].1.lat;
        let phi1 = lat1 * std::f64::consts::PI / 180f64;
        let lng1 = cities[i].1.lng;
        let tht1 = lng1 * std::f64::consts::PI / 180f64;
        let earth_radius = 6371000f64;
        let mut list = CityUtil::CityDistList::new();
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
    });
}
