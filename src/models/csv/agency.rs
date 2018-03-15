#[derive(Debug,Deserialize)]
pub struct AgencyCSV {
    pub agency_id: String,
    pub agency_name: String,
    pub agency_url: String,
    pub agency_timezone: String,
    pub agency_lang: String,
    pub agency_phone: String
}