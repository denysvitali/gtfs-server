#[derive(Debug,Deserialize)]
pub struct FeedCSV {
    pub feed_publisher_name: String,
    pub feed_publisher_url: String,
    pub feed_lang: String,
    pub feed_start_date: String,
    pub feed_end_date: String,
    pub feed_version: String
}