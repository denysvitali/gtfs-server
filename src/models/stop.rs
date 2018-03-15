#[derive(Debug,Deserialize)]
pub struct Stop {
    pub id: String,
    pub name: String,
    pub lat: f64,
    pub lng: f64,
    pub location_type: i32,
    pub parent_station: String,
    pub feed_id: String
}