#[derive(FromForm,Serialize,Deserialize)]
pub struct TripSearch {
    pub stops_visited: Option<String>
}