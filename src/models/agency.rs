//! Agency related structs and implementations
#[derive(Debug,Serialize,Deserialize)]
pub struct Agency {
    pub uid: String,
    #[serde(skip_serializing)]
    pub id: String,
    pub name: String,
    pub url: String,
    pub timezone: String,
    pub lang: String,
    pub phone: String,
    #[serde(skip_serializing)]
    pub feed_id: String
}