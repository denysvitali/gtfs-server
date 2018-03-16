#[derive(Debug,Serialize,Deserialize)]
pub struct Agency {
    pub uid: String,
    #[serde(skip_serializing)]
    id: String,
    pub name: String,
    pub url: String,
    pub timezone: String,
    pub lang: String,
    pub phone: String,
    #[serde(skip_serializing)]
    feed_id: String
}