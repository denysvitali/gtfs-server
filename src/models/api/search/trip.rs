#[derive(FromForm, Serialize, Deserialize)]
pub struct TripSearch {
    pub stops_visited: Option<String>,
    pub route: Option<String>,
    pub departure_after: Option<String>,
    pub arrival_before: Option<String>,
    pub offset : Option<i64>,
    pub per_page : Option<i64>
}
