#[derive(Debug, Deserialize)]
pub struct RouteCSV {
    pub route_id: String,
    pub agency_id: String,
    pub route_short_name: String,
    pub route_long_name: String,
    pub route_desc: String,
    pub route_type: String,
}
