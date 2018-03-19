use super::agency::Agency;

#[derive(Debug,Serialize,Deserialize)]
pub struct Route {
    pub uid: String,
    #[serde(skip_serializing)]
    pub id: String,
    pub agency_id: Option<String>,
    pub short_name: String,
    pub long_name: String,
    pub description: String,
    #[serde(rename="type")]
    pub route_type: i32,
    #[serde(skip_serializing)]
    pub feed_id: String
}