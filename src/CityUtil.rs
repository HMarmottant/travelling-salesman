#[derive(Clone)]
pub struct CityPos {
    pub lat: f64,
    pub lng: f64,
}

pub type CityList = Vec<(String, CityPos)>;
pub type CityDistList = Vec<f64>;
pub type CityDistMatrix = Vec<CityDistList>;