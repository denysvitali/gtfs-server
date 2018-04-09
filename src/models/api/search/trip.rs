#[derive(FromForm,Serialize,Deserialize)]
pub struct TripSearch {
    pub stops_visited: Option<String>,
    pub route : Option<String>
}