#[derive(Debug,Serialize,Deserialize)]
pub struct Route {
    pub uid: String,
    #[serde(skip_serializing)]
    id: String,
    pub agency: Option<Agency>,
    pub short_name: String,
    pub long_name: String,
    pub description: String,
    #[serde(rename="type")]
    pub route_type: i32,
    #[serde(skip_serializing)]
    feed_id: String
}